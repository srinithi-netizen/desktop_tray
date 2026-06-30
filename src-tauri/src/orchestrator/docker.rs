use std::process::Command;
use tauri::AppHandle;
use crate::orchestrator::progress;

const CONTAINER: &str = "fluxbooks-postgres";

pub fn check_docker_installed(app: &AppHandle) -> Result<(), String> {
    let ok = Command::new("docker")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if ok {
        Ok(())
    } else {
        progress::emit(app, "error_no_docker",
            "Docker is not installed. Please install Docker Desktop from https://docker.com");
        Err("Docker not found".into())
    }
}

fn container_exists() -> bool {
    Command::new("docker")
        .args(["ps", "-a", "--filter", &format!("name=^{}$", CONTAINER), "--format", "{{.Names}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(CONTAINER))
        .unwrap_or(false)
}

fn container_running() -> bool {
    Command::new("docker")
        .args(["ps", "--filter", &format!("name=^{}$", CONTAINER), "--format", "{{.Names}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(CONTAINER))
        .unwrap_or(false)
}

fn create_container() -> Result<(), String> {
    println!("[docker] creating postgres container...");
    let status = Command::new("docker")
        .args([
            "run", "-d",
            "--name", CONTAINER,
            "-e", "POSTGRES_USER=fluxbooks",
            "-e", "POSTGRES_PASSWORD=fluxbooks123",
            "-e", "POSTGRES_DB=fluxbooks",
            "-p", "5432:5432",
            "-v", "fluxbooks_pgdata:/var/lib/postgresql/data",
            "--restart", "unless-stopped",
            "postgres:16",
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() { Ok(()) }
    else { Err("docker run failed".into()) }
}

fn start_container() -> Result<(), String> {
    println!("[docker] starting existing container...");
    Command::new("docker")
        .args(["start", CONTAINER])
        .status()
        .map_err(|e| e.to_string())
        .and_then(|s| if s.success() { Ok(()) } else { Err("docker start failed".into()) })
}

pub fn stop_container() {
    let _ = Command::new("docker").args(["stop", CONTAINER]).status();
}

pub fn ensure_postgres_container() -> Result<(), String> {
    if container_running() {
        println!("[docker] postgres already running");
        return Ok(());
    }
    if container_exists() {
        start_container()
    } else {
        create_container()
    }
}