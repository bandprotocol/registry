use anyhow::{anyhow, Result};
use bothan_core::registry::{Invalid, Registry};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// CLI structure to handle command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the registry JSON file
    #[arg(short, long, default_value = "registry.json")]
    path: String,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();

    // Path to the registry.json file
    let registry_path = Path::new(&args.path);

    // Read and parse the registry file
    let registry: Registry<Invalid> = {
        let file = File::open(registry_path)
            .map_err(|e| anyhow!("Failed to open registry file at '{}': {e}", args.path))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .map_err(|e| anyhow!("Failed to parse registry file at '{}': {e}", args.path))?
    };

    // Validate the registry
    registry
        .validate()
        .map_err(|e| anyhow!("Invalid registry at '{}': {e}", args.path))?;

    println!("Registry is valid.");
    Ok(())
}
