pub mod backend;
pub mod progress;

use tauri::AppHandle;

pub async fn run_startup(app: AppHandle, server_url: String) -> Result<(), String> {
    progress::emit(&app, "checking_server", "Connecting to server...");
    backend::ensure_backend(&server_url).await?;
    progress::emit(&app, "ready", "Ready");
    Ok(())
}