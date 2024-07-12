use std::process::Command;

pub fn send_notification() {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("display notification \"done!\" with title \"ptimer\" sound name \"Basso\"")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Notification sent successfully");
    } else {
        eprintln!("Failed to send notification: {}", String::from_utf8_lossy(&output.stderr));
    }
}
