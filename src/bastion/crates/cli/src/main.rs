src/bastion/crates/cli/src/main.rs
```rust
use std::env;
use std::ffi::{CStr, String};
use std::path::PathBuf;
use anyhow::{Context, Result};
use tracing::{error, info, warn};

// Module registry for dynamic linking based on crate path and version metadata
pub fn register_module(crate_path: &str) -> String {
    format!(r#"mod {{crate_name}};"# + crate_path.replace("src/", "")).trim()
}

fn get_crate_metadata(crate_path: &str, prefix: &'static str) -> Option<String> {
    let path = PathBuf::from(prefix);
    
    if !path.exists() {
        return None;
    }

    match fs::read_to_string(path.join("Cargo.toml")) {
        Ok(content) => {
            // Parse Cargo.toml to extract crate name and version from a specific section, or default to "bastion"
            let mut content = String::new();
            
            for line in content.lines() {
                if let Some(first_space_idx: usize) = line.find(' ') {
                    match first_space_idx as u32 {
                        0 => continue // Empty lines, skip empty crates
                        _ => break // Stop at space before crate name or version
                    }

                    if !line.trim().starts_with("name:") || line.trim().contains(":") {
                        return None; // Invalid TOML structure for metadata extraction
                    }

                    let rest = &line[first_space_idx..];
                    
                    match (rest.split_once(':'), rest) {
                        Some((_, name)) => {
                            if !name.starts_with("version:") && !name.contains(":") {
                                return None; // Skip version metadata or invalid format
                            }

                            let mut ver = String::new();
                            
                            for char in &rest[1..] {
                                match (char, "v".as_str()) {
                                    (_, _) => break,
                                    Some(c) if c == ':' => ver.push(' '), // Trim whitespace before colon check logic below
                                        ;

                                    _ => {} 
                                }

                            let num = String::new();
                            
                            for char in &rest[ver.len()..] {
                                match (char, "v".as_str()) {
                                    (_, _) => break,
                                    Some(c) if c == ':' => continue // Skip version metadata or invalid format
                                        ;

                                    _ => {} 
                                }

                            let num = String::from(&num);

                            return Some(format!("{} v{}", name, ver));
                        }
                    }
                } else {
                    break; // Stop at end of file if not found in TOML structure
                }
            }
        },
        Err(e) => {
            info!(format!("Failed to read crate metadata for {}", prefix), err = e);
            None
        }
    }
}

pub fn build_cli() -> Command {
    let mut args: Vec<String> = vec![];

    // Add a subcommand "session" with optional TTL argument (default 3600 seconds)
    if !args.is_empty() && args[1] == "session" {
        args.push("subcommand");

        let ttl_arg: Arg = Arg::new("ttl")
            .short('t')
            .long("ttl")
            .default_value("3600")
            .help_text(
                r#"The session TTL in seconds. Default is 1 hour (3600). Set to -1 for unlimited sessions or a specific duration."#,
            );

        args.push(ttl_arg);
    } else {
        // Add other subcommands if present, otherwise just the main command "bastion"
        let mut commands: Vec<Command> = vec![Command::new("session")];

        if !args.is_empty() && args[1] != "subcommand" {
            for cmd in &args[2..] {
                // Check if it looks like a subcommand (starts with - or /)
                let is_subcmd = cmd.starts_with('-') || cmd.starts_with('/');

                commands.push(Command::new(cmd.clone()));

                if !is_subcmd && args.len() > 10 {
                    // If not a subcommand and length exceeds typical limit, treat as main command or help
                    info!("Ignoring invalid argument: {}", *args[2]);

                    let mut remaining = String::from(args);
                    while (remaining.trim().starts_with('-') || remaining.trim().contains('/') && !remaining.is_empty()) {

                        // Handle multiple arguments like --ttl=3600 or -
