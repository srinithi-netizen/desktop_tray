use rusqlite::Connection;
use std::fs;
use tauri::{AppHandle, Manager};

pub fn app_data_dir(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("No app data dir")
}

pub fn uploads_dir(app: &AppHandle) -> std::path::PathBuf {
    let dir = app_data_dir(app).join("uploads");
    fs::create_dir_all(&dir).ok();
    dir
}

pub fn get_conn(app: &AppHandle) -> rusqlite::Result<Connection> {
    let db_path = app_data_dir(app).join("fluxbooks.db");
    Connection::open(db_path)
}

pub fn init_db(app: &AppHandle) -> rusqlite::Result<()> {
    let data_dir = app_data_dir(app);
    fs::create_dir_all(&data_dir).ok();

    let conn = get_conn(app)?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS uploads (
            id           TEXT PRIMARY KEY,
            file_name    TEXT NOT NULL,
            local_path   TEXT NOT NULL,
            file_size    INTEGER NOT NULL,
            status       TEXT NOT NULL DEFAULT 'pending',
            progress     INTEGER NOT NULL DEFAULT 0,
            total_chunks INTEGER NOT NULL DEFAULT 0,
            done_chunks  INTEGER NOT NULL DEFAULT 0,
            error_msg    TEXT,
            queued_at    TEXT NOT NULL,
            uploaded_at  TEXT
        );

        CREATE TABLE IF NOT EXISTS chunks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            upload_id   TEXT NOT NULL,
            chunk_index INTEGER NOT NULL,
            total       INTEGER NOT NULL,
            status      TEXT NOT NULL DEFAULT 'pending',
            size_bytes  INTEGER NOT NULL DEFAULT 0,
            sent_at     TEXT,
            error_msg   TEXT,
            FOREIGN KEY (upload_id) REFERENCES uploads(id) ON DELETE CASCADE,
            UNIQUE(upload_id, chunk_index)
        );
    ")?;
    Ok(())
}