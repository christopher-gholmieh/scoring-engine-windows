// Written by: Christopher Gholmieh
// Crates:
use notify_rust::{Notification, Timeout};

// Functions:
pub fn construct_positive_notification() -> Result<(), Box<dyn std::error::Error>> {
    // Notification:
    Notification::new()
        .timeout(Timeout::Milliseconds(4000))
        .summary("Scoring Engine")
        .body("You have gained points!")
        .show()?;

    // Unit:
    Ok(())
}

pub fn construct_negative_notification() -> Result<(), Box<dyn std::error::Error>> {
    // Notification:
    Notification::new()
        .timeout(Timeout::Milliseconds(4000))
        .summary("Scoring Engine")
        .body("You have lost points!")
        .show()?;

    // Unit:
    Ok(())
}
