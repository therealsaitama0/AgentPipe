// src/bastion/crates/cli/src/main.rs
use clap::{Arg, Command};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use tracing::info;

pub fn build_cli() -> Command {
    let mut cmd = Command::new("bastion")
        .about("Security Control Plane CLI")
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Session Management
    cmd.arg("session").long("ttl".to_string()).short('t').default_value("3600");
    
    // Health Check (System Status)
    let health_cmd = Command::new("health")
        .about("Display control-plane status and metrics")
        .arg(Arg::new("metrics"))
            .long("metrics".to_string())
            .short('m')
            .default_value(true);

    // Audit Export (Log Generation)
    let audit_cmd = Command::new("audit-export").about("Export current audit log to file")
        .arg(Arg::new("output"))
            .long("output".to_string())
            .short('o')
            .default_value("audit.log");

    // Audit Verification (Chain Integrity)
    let verify_cmd = Command::new("audit-verify").about("Validate audit chain integrity")
        .arg(Arg::new("path"))
            .long("path".to_string())
            .short('p')
            .default_value("src/bastion/crates/core/src/tests/audit_tests.rs");

    cmd.subcommands(health_cmd, health_cmd.clone(), verify_cmd)
}

pub fn run() {
    let cli = build_cli();
    
    match cli.get_matches() {
        Ok(matches) => {
            if matches.is_present("ttl") && !matches.value_of::<u32>("ttl").unwrap_or(&0).is_zero() {
                info!("Creating new session with TTL: {}", matches.value_of::<String>("ttl"));
                
                // Create a fresh, isolated environment for this specific run
                let temp_dir = tempfile::TempDir::new().expect("Failed to create temporary directory");
                
                if !temp_dir.exists() {
                    eprintln!("Error: Failed to create temporary directory '{}'. Make sure 'cargo install' is available.", &temp_dir.path());
                    return;
                }

                let data = temp_dir.join("bastion_data.json").create_file().unwrap();
                info!("[BASTION] Session created successfully with TTL {}", matches.value_of::<String>("ttl"));
            } else {
                if !matches.is_present("metrics") || matches.value_of::<bool>("metrics").unwrap_or(false) && matches.is_present("health") {
                    // Show basic system status using the health command logic (simplified for this demo)
                    info!("System Status: Control Plane Ready");
                    
                    let output = match cli.get_matches() {
                        Ok(m) => m.value_of::<String>("output"),
                        Err(_) => "audit.log".to_string(), // Fallback if no path specified
                    };

                    std::fs::write(&data, &format!("=== {} ===\n", env!["HOME"]).unwrap_or_default())
                        .expect("Failed to write output file");
                    
                    info!("{}", data);
                } else {
                    eprintln!("Error: Invalid command or arguments provided.");
                }
            }
        },
        Err(_) => {
            if matches.is_present("ttl") && !matches.value_of::<u32>("ttl").unwrap_or(&0).is_zero() {
                info!("[BASTION] Session created successfully with TTL {}", matches.value_of::<String>("ttl"));
                
                let temp_dir = tempfile::TempDir::new().expect("Failed to create temporary directory");

                if !temp_dir.exists() {
                    eprintln!("Error: Failed to create temporary directory '{}'. Make sure 'cargo install' is available.", &temp_dir.path());
                    return;
                }

                let data = temp_dir.join("bastion_data.json").create_file().unwrap();
                
                info!("[BASTION] Session created successfully with TTL {}", matches.value_of::<String>("ttl"));
            } else {
                if !matches.is_present("metrics") || matches.value_of::<bool>("metrics").unwrap_or(false) && matches.is_present("health") {
                    // Show basic system status using the health command logic (simplified for this demo)
                    info!("System Status: Control Plane Ready");

                    let output = match cli.get_matches() {
                        Ok(m) => m.value_of::<String>("output"),
                        Err(_) => "audit.log".to
