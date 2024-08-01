use std::process::Command;

use crate::config::Config;

pub fn send_notification(config: &Config, message: &str) {
    let title = if config.debug_mode { "mm-pomodoro-dev" } else { "mm-pomodoro" };
    let mut script = format!("display notification \"{message}\" with title \"{title}\" sound name \"Basso\"");
    script.push_str("\ndisplay dialog \"mm-pomodoro\" buttons {\"OK\"} default button \"OK\"");
    let output = Command::new("osascript").arg("-e").arg(script).output().expect("Failed to execute command");

    if output.status.success() {
        println!("Notification sent successfully");
    } else {
        eprintln!("Failed to send notification: {}", String::from_utf8_lossy(&output.stderr));
    }
}
