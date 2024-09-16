use std::process::Command;

use crate::config::Config;

pub fn send_notification(config: &Config, message: &str) {
    let title = if config.debug_mode { "mm-pomodoro-dev" } else { "mm-pomodoro" };
    let script = format!(r#"display dialog "{message}" with title "{title}" buttons {{"OK"}} default button "OK""#);
    let output = Command::new("osascript").arg("-e").arg(script).output().expect("Failed to execute command");

    if output.status.success() {
        println!("Notification sent successfully");
    } else {
        eprintln!("Failed to send notification: {}", String::from_utf8_lossy(&output.stderr));
    }
}
