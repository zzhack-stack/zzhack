// Build script to extract configuration from Trunk.toml
// This runs at compile time to generate constants for the application

use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to re-run this build script if Trunk.toml changes
    println!("cargo:rerun-if-changed=Trunk.toml");

    // Read and parse Trunk.toml
    let trunk_toml_path = Path::new("Trunk.toml");
    let public_url = if trunk_toml_path.exists() {
        match fs::read_to_string(trunk_toml_path) {
            Ok(content) => {
                // Parse the TOML to extract public_url
                match toml::from_str::<toml::Value>(&content) {
                    Ok(value) => {
                        value
                            .get("build")
                            .and_then(|build| build.get("public_url"))
                            .and_then(|url| url.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "/zzhack".to_string())
                    }
                    Err(_) => "/zzhack".to_string(), // fallback if parsing fails
                }
            }
            Err(_) => "/zzhack".to_string(), // fallback if reading fails
        }
    } else {
        "/zzhack".to_string() // fallback if file doesn't exist
    };

    // Set environment variable for the config module
    println!("cargo:rustc-env=TRUNK_PUBLIC_URL={}", public_url);
}