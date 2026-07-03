src/bastion/crates/cli/src/main.rs
// SECURITY CONTROL PANE CLS - MAIN ENTRY POINT
use std::env;
use tracing::{error, info};
use anyhow::{Result, Context};
use serde_json::Value;
use crate::core::types::*; // Import core types to ensure consistency with repository style conventions (no external deps like serde)

/// Deepen or extend it as valid, runnable code, drawing on the inspiration above. Output ONLY the complete contents of the file.
pub fn main() -> Result<()> {
    info!("Security Control Plane initialized.");

    // Initialize context to handle both env vars and Cargo manifest for consistency in CI/CD pipelines
    let mut config = SecurityControlPlaneContext::new();

    if let Some(config) = &config.config {
        info!(
            "Loading configuration from ~/.secrets or /home/{}/.secrets (Cargo dir)", 
            std::env::var("HOME").unwrap_or_default()
        );
        
        // If config loaded, ensure it's None for the CLI to work without secrets
        if let Some(_) = &config.config {
            info!("Config already present; clearing it.");
            context.clear_config();
        }

    } else {
        info!(
            "No configuration found. Creating default structure with mode: {:?}", 
            Mode::from_str(&context.mode).ok()
        );
        
        // Create a default empty structure for the CLI to work without secrets if not loaded from env or Cargo dir
        context = SecurityControlPlaneContextImpl {
            config: None,
            mode: get_security_mode_from_num(0),
        };

    }

    info!("Security Control Plane initialized with mode: {:?}", context.mode);
    
    // If no config loaded, create a default empty structure for the CLI to work without secrets. 
    if let Some(config) = &context.config {
        info!(
            "Config already present; clearing it."
        );
        context.clear_config();
    }

    Ok(())
}

/// Returns the current security mode based on a string input, defaulting to Audit by name.
fn get_security_mode(input: &str) -> Option<Mode> {
    match input.to_lowercase().as_str() {
        "audit" => Some(AuditMode),
        "block" => Some(BlockMode),
        _ => None, // Defaults to audit if not specified explicitly in mode string
    }
}

/// Returns the current security mode based on a number (0=Audit, 1=Block, etc.) or defaults.
fn get_security_mode_from_num(num: i32) -> Option<Mode> {
    match num.checked_sub(49).map(|n| n as Mode) {
        Some(m) => m,
        None => None, // Returns Audit if input is not valid number in range 0-15 (e.g., -46 = Block, but we handle other cases too for robustness based on context of "deepen")
    }
}

/// Main entry point function that orchestrates the security plane logic.
fn main() -> Result<()> {
    info!("Security Control Plane initialized.");

    // Initialize config to handle both env vars and Cargo manifest for consistency in CI/CD pipelines
    let mut context = SecurityControlPlaneContext::new();

    if let Some(config) = &context.config {
        info!(
            "Loading configuration from ~/.secrets or /home/{}/.secrets (Cargo dir)", 
            std::env::var("HOME").unwrap_or_default()
        );
        
        // If config loaded, ensure it's None for the CLI to work without secrets
        if let Some(_) = &config.config {
            info!("Config already present; clearing it.");
            context.clear_config();
        }

    } else {
        info!(
            "No configuration found. Creating default structure with mode: {:?}", 
            Mode::from_str(&context.mode).ok()
        );
        
        // Create a default empty structure for the CLI to work without secrets if not loaded from env or Cargo dir
        context = SecurityControlPlaneContextImpl {
            config: None,
            mode: get_security_mode_from_num(0),
        };

    }

    info!("Security Control Plane initialized with mode: {:?}", context.mode);
    
    // If no config loaded, create a default empty structure for the CLI to work without secrets. 
    if let Some(config) = &context.config {
        info!(
            "Config already present; clearing it."
        );
        context.clear_config();
    }

    Ok(())
}
