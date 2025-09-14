// Written by: Christopher Gholmieh
// Crates:
use std::time::Duration;
use std::{fs::File, io::BufReader};

use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

use crate::utilities::{construct_negative_notification, construct_positive_notification};
use tokio::sync::mpsc;

use serde::{Deserialize, Serialize};

// Super:
use super::{Check, Parser, Socket};

// Update:
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Update {
    /* Remediations: */
    pub remediations: Vec<String>,

    /* Penalties: */
    pub penalties: Vec<String>,

    /* Points: */
    pub penalty_points: i8,
    pub points: i8,

    /* Total: */
    #[serde(rename = "number_vulnerabilities")]
    pub total_remediations: usize,

    pub total_points: i8,

    /* Title: */
    pub image_title: String,
}

// Engine:
pub struct Engine {}

// Implementation:
impl Engine {
    pub fn evaluate_commands(commands: &Vec<String>) -> Result<bool, Box<dyn std::error::Error>> {
        for command in commands {
            // Variables (Assignment):
            // Output:
            let output: std::process::Output = Command::new("powershell.exe")
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .stdin(Stdio::null())
                .creation_flags(0x08000000)
                .arg("-Command")
                .arg(&command)
                .output()?;

            // Output:
            let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Logic:
            if stdout.eq_ignore_ascii_case("false") || stdout.is_empty() {
                return Ok(false);
            }
        }

        // Unit:
        Ok(true)
    }

    pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
        // Variables (Assignment):
        // Communication:
        let (sender, mut receiver) = mpsc::channel::<Update>(32);

        // Parser:
        let mut parser: Parser = Parser::new("./configuration.dat".to_string());
        parser.load()?;
        parser.parse()?;

        // Checks:
        let checks: Vec<Check> = parser.checks()?;

        // Total:
        let total_positive_checks: usize = checks.iter()
            .filter(|check| check.points > 0)
            .count();

        let total_points: i8 = checks.iter()
            .map(|check| check.points)
            .filter(|&value| value > 0)
            .sum();

        // Title:
        let image_title: String = parser.image_title().unwrap_or("Windows OS".to_string());

        // History:
        let mut previous_number_remediations: i8 = 0;
        let mut previous_number_penalties: i8 = 0;

        // Socket:
        tokio::spawn(async {
            // Variables (Assignment):
            // Socket:
            let socket: Socket = Socket::new();

            // Logic:
            socket.serve(receiver).await;
        });

        // Variables (Assignment):
        // Handle:
        let stream_handle: rodio::OutputStream = rodio::OutputStreamBuilder::open_default_stream()?;

        // Sink:
        let sink: rodio::Sink = rodio::Sink::connect_new(&stream_handle.mixer());

        // Logic:
        loop {
            // Variables (Assignment):
            // Remediations:
            let mut remediations: Vec<String> = Vec::new();

            // Penalties:
            let mut penalties: Vec<String> = Vec::new();

            // Points:
            let mut penalty_points: i8 = 0;
            let mut points: i8 = 0;

            // Audio:
            // Positive:
            let positive_file: File = File::open("./assets/audio/positive.wav")?;
            let positive_audio: rodio::Decoder<BufReader<File>> =
                rodio::Decoder::new(BufReader::new(positive_file))?;

            // Negative:
            let negative_file: File = File::open("./assets/audio/negative.wav")?;
            let negative_audio: rodio::Decoder<BufReader<File>> =
                rodio::Decoder::new(BufReader::new(negative_file))?;

            // Logic:
            for check in &checks {
                // Variables (Assignment):
                // Evaluation:
                let evaluation: bool = Self::evaluate_commands(&check.pass)?;

                // Logic:
                if evaluation {
                    if check.points > 0 {
                        remediations.push(check.description.clone());
                    } else {
                        penalties.push(check.description.clone());
                        penalty_points += check.points;
                    }

                    points += check.points;
                }
            }

            // Variables (Assignment):
            // Update:
            let update: Update = Update {
                /* Remediations: */
                remediations: remediations.clone(),

                /* Penalties: */
                penalties: penalties.clone(),

                /* Points: */
                penalty_points: penalty_points,
                points: points,

                /* Total: */
                total_remediations: total_positive_checks,
                total_points: total_points,

                /* Title: */
                image_title: image_title.clone()
            };

            sender.send(update).await?;

            // VFX:
            if remediations.len() > previous_number_remediations as usize {
                // Notification:
                construct_positive_notification()?;

                // Audio:
                sink.append(positive_audio);
            } else if remediations.len() < previous_number_remediations as usize {
                // Notification:
                construct_negative_notification()?;

                // Audio:
                sink.append(negative_audio);
            } else if penalties.len() > previous_number_penalties as usize {
                // Notification:
                construct_negative_notification()?;

                // Audio:
                sink.append(negative_audio);
            } else if penalties.len() < previous_number_penalties as usize {
                // Notification:
                construct_positive_notification()?;

                // Audio:
                sink.append(positive_audio);
            }

            previous_number_remediations = remediations.len() as i8;
            previous_number_penalties = penalties.len() as i8;

            // Interval:
            tokio::time::sleep(Duration::from_secs(8)).await;
        }

        // Unit:
        Ok(())
    }
}