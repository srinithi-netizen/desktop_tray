use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::time::{sleep, Duration, Instant};

static BACKEND: Lazy<Mutex<Option<()>>> = Lazy::new(|| Mutex::new(None)); // no local child anymore

pub async fn ensure_backend(server_url: &str) -> Result<(), String> {
    wait_until_healthy(server_url, 15).await
}

async fn check_health(server_url: &str) -> bool {
    reqwest::get(format!("{}/plaid/link-token", server_url))
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

pub async fn wait_until_healthy(server_url: &str, timeout_secs: u64) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);

    while Instant::now() < deadline {
        if check_health(server_url).await {
            println!("[backend] healthy");
            return Ok(());
        }
        sleep(Duration::from_millis(500)).await;
    }

    Err(format!("Backend at {} did not become healthy in time", server_url))
}

pub fn shutdown() {
    // nothing to kill anymore — server runs on a remote machine
}