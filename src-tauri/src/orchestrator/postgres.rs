use std::process::Command;
use tokio::time::{sleep, Duration, Instant};

pub async fn wait_until_ready(timeout_secs: u64) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);

    while Instant::now() < deadline {
        let ok = Command::new("docker")
            .args(["exec", "fluxbooks-postgres", "pg_isready", "-U", "fluxbooks"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if ok {
            println!("[postgres] ready");
            return Ok(());
        }

        sleep(Duration::from_millis(500)).await;
    }

    Err("PostgreSQL did not become ready in time".into())
}