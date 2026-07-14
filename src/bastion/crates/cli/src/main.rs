src/bastion/crates/cli/src/main.rs
```rust
use std::env;
use clap::{Arg, Command};
use tracing::{info, error, spanned};

/// The Security Control Plane CLI entry point.
#[derive(Debug)]
struct Cli {
    #[clap(long = "ttl", default_value="3600")]
    session_ttl: u64, // In seconds for sessions
}

pub fn build_cli() -> Command {
    let args = env::args().collect();

    if !args.is_empty() && args.len() > 1 {
        error!("Usage: bastion <command> [options]");
        eprintln!("{}", std::process::exit(1).code());
        return Command::from_args(args);
    }

    let mut matches = Cli::new();
    
    // Parse arguments for 'manage-bastion' command (if provided) or default to session/health modes.
    if args.len() >= 2 && !matches.subcommands().is_empty() {
        println!("Command not recognized: {}", env::args_os()[1]);
        eprintln!("{}", std::process::exit(1).code());
        return Command::from_args(args);
    }

    matches.command = if args.len() == 2 && !matches.subcommands().is_empty() {
        match &env::var("BASTION_MANAGE") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'manage-bastion' or just session/health if no args.
        }
    } else if matches.subcommands().is_empty() && !args.is_empty() {
        match &env::var("BASTION_SESSION") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'session' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 2 {
        match &env::var("BASTION_HEALTH") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'health' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 3 && !matches.subcommands().is_empty() {
        match &env::var("BASTION_AUDIT") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'audit' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 4 {
        match &env::var("BASTION_AUDIT_VERIFY") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'audit' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 5 || !matches.runs.is_empty() {
        println!("Usage: bastion <command> [options]");
        eprintln!("{}", std::process::exit(1).code());
        return Command::from_args(args);
    } else if matches.subcommands().is_empty() && args.len() >= 6 {
        match &env::var("BASTION_MANAGE") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'manage-bastion' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 7 || !matches.runs.is_empty() {
        match &env::var("BASTION_SESSION") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'session' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 8 || !matches.runs.is_empty() {
        match &env::var("BASTION_HEALTH") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'health' or just session/health.
        }
    } else if matches.subcommands().is_empty() && args.len() >= 9 || !matches.runs.is_empty() {
        match &env::var("BASTION_AUDIT") {
            Ok(name) => name.to_string(),
            Err(_) => "bastion".to_string(), // Default to 'audit' or just session/health.
        }
