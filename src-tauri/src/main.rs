#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, RunEvent,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();

            // --- Build the tray menu ---
            let open_item = MenuItem::with_id(handle, "open", "Open", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(handle, "quit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(handle, &[&open_item, &quit_item])?;

            // --- Load the tray icon ---
            // include_bytes! is a Rust macro that reads a file at compile time
            // and embeds its raw bytes directly into the binary.
            // The path is relative to this source file (src/main.rs),
            // so "../icons/tray-icon.png" resolves to src-tauri/icons/tray-icon.png.
            // This means no file path is needed at runtime — the bytes are
            // already baked in, which also works correctly in release builds.
            let icon_bytes = include_bytes!("../icons/tray-icon.png");

            // Use the `image` crate to decode the PNG bytes into raw RGBA pixels.
            // image::load_from_memory() parses any supported image format from a byte slice.
            // .to_rgba8() converts it to a flat RGBA byte buffer (4 bytes per pixel: R, G, B, A).
            let img = image::load_from_memory(icon_bytes)
                .expect("Failed to decode tray icon PNG")
                .to_rgba8();

            let (width, height) = img.dimensions();

            // tauri::image::Image::new_owned() takes:
            //   - a Vec<u8> of raw RGBA bytes
            //   - the width in pixels
            //   - the height in pixels
            // This is the constructor available in Tauri 2.11.x.
            let tauri_image = tauri::image::Image::new_owned(
                img.into_raw(), // consumes the RGBA buffer into a Vec<u8>
                width,
                height,
            );

            // --- Build the tray icon ---
            TrayIconBuilder::new()
                .icon(tauri_image)
                .menu(&menu)
                .show_menu_on_left_click(true)
                // on_tray_icon_event handles direct clicks on the icon itself
                // (not on a menu item). We use it to show the window on left-click.
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(handle)?;

            // --- Handle menu item clicks ---
            // In Tauri 2.11.x, menu events are registered on the app handle
            // separately, not as a builder method on TrayIconBuilder.
            // on_menu_event fires whenever any menu item anywhere in the app
            // is clicked — we match on the item id to handle ours specifically.
            app.on_menu_event(|app, event| {
                match event.id().as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } = event
            {
                if label == "main" {
                    api.prevent_close();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            }
        });
}