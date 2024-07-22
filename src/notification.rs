use std::process::Command;

use crate::config::Config;

pub fn send_notification(config: &Config) {
    let title = if config.debug_mode { "ptimer-dev" } else { "ptimer" };
    let script = format!("display notification \"done!\" with title \"{}\" sound name \"Basso\"", title);
    let output = Command::new("osascript").arg("-e").arg(script).output().expect("Failed to execute command");

    if output.status.success() {
        println!("Notification sent successfully");
    } else {
        eprintln!("Failed to send notification: {}", String::from_utf8_lossy(&output.stderr));
    }
}
