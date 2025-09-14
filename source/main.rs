// Written by: Christopher Gholmieh
// Modules (Declaration):
mod constants;
mod core;
mod utilities;

// Crate:
use crate::core::Engine;

// Main:
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Engine:
    Engine::execute().await?;

    // Unit:
    Ok(())
}
