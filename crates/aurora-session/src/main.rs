use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

struct SessionManager {
    processes: Vec<Child>,
}

impl SessionManager {
    fn new() -> Self {
        Self { processes: Vec::new() }
    }

    fn spawn(&mut self, binary: &str, args: &[&str]) {
        match Command::new(binary).args(args).spawn() {
            Ok(child) => {
                println!("[Aurora Session] Started {} (PID: {})", binary, child.id());
                self.processes.push(child);
            }
            Err(e) => {
                eprintln!("[Aurora Session] Failed to spawn {}: {}", binary, e);
            }
        }
    }

    fn stop_all(&mut self) {
        for mut child in self.processes.drain(..) {
            let _ = child.kill();
        }
    }
}

impl Drop for SessionManager {
    fn drop(&mut self) {
        self.stop_all();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("          Aurora M3 Desktop Environment Session             ");
    println!("  Material 3 Tonal Theming | macOS Ergonomics | Dwindle WM   ");
    println!("============================================================");

    let mut session = SessionManager::new();

    // 1. Start Compositor & Dwindle Window Manager
    session.spawn("aurora-wm", &[]);
    sleep(Duration::from_millis(300)).await;

    // 2. Start Desktop Shell (Top Bar & Dock)
    session.spawn("aurora-shell", &[]);

    println!("[Aurora Session] Desktop environment is active and running.");
    println!("[Aurora Session] Use `aurora-msg` to inspect or control the desktop environment.");

    // Keep session alive
    tokio::signal::ctrl_c().await?;
    println!("[Aurora Session] Exiting Aurora session...");
    session.stop_all();

    Ok(())
}
