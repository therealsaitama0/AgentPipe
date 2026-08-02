src/bastion/crates/core/src/components/ui_status_display.rs
use crate::{components::ApprovalTicket};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde_json;
use thiserror::Error as StdErr;

/// Configuration for the Status Display component.
pub struct Config {
    /// Default session count to display (0 if not set)
    pub sessions: Option<usize>,
    /// Number of pending tickets to show (default 5, can be overridden via env var or config file)
    pub tickets_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self::new(0, 5);
    }
}

/// Load configuration from JSON/YAML/Environment Variables.
pub fn load_config(config_path: &str) -> Result<Self, StdErr> {
    let file = File::open(config_path).map_err(|e| e.to_string_lossy())?;
    
    // Try YAML first (most common for modern systems), fallback to JSON if not specified or invalid format.
    match serde_yaml::from_reader(file) {
        Ok(v) => return Ok(Self { sessions: v.sessions(), tickets_count: v.tickets_count() }),
        
        Err(e) => {
            // Fallback to JSON for backward compatibility, though YAML is preferred.
            let json_str = String::from_utf8_lossy(&file.to_string_loss()).to_string();
            
            if !json_str.is_empty() && json_str.contains('{') || json_str.contains('}') {
                serde_json::from_str::<serde_json::Value>(&json_str)
                    .map_err(|e| e.into())?;
                
                let sessions = config_path.sessions().unwrap_or(0);
                let tickets_count = config_path.tickets_count()?.unwrap_or(5);

                Ok(Self {
                    sessions: Some(sessions),
                    tickets_count,
                })
            } else {
                Err(e)
            }
        },
    }
}

/// Dump configuration to JSON/YAML/Environment variables.
pub fn dump_config(config_path: &str) -> Result<(), StdErr> {
    let file = File::create(config_path).map_err(|e| e.to_string_lossy())?;
    
    serde_json::to_writer_pretty(file, config)?;

    // Write env vars as well for external access.
    if !config.sessions.is_none() && !config.tickets_count.is_some() {
        let mut lines = vec![];
        
        match serde_yaml::from_str::<serde_yaml::Value>(&env::var("SESSIONS").unwrap_or_default()) {
            Ok(v) => {
                // Only write if it's not already in env vars or config
                if !v.sessions().is_none() && v.tickets_count.is_some() {
                    lines.push(format!("SESSIONS={}", serde_yaml::to_string(&v).unwrap()));
                }
            },
            
            Err(e) => {
                // Fallback to JSON for backward compatibility with env vars.
                let json_str = String::from_utf8_lossy(env::var("SESSIONS").ok())?;
                
                if !json_str.is_empty() && json_str.contains('{') || json_str.contains('}') {
                    serde_json::to_writer_pretty(file, &serde_json::Value::Object(json_str))?;

                    // Write env vars as well.
                    match serde_yaml::from_str::<serde_yaml::Value>(&env::var("SESSIONS").unwrap_or_default()) {
                        Ok(v) => lines.push(format!("SESSIONS={}", v.sessions().map(|x| x.to_string()).collect())),
                        
                        Err(e) => {} // Ignore errors for env vars if we can't read them.
                    }
                } else {
                    return Err("Invalid JSON format".into());
                }
            },

        Ok(())
    } else {
        Err(format!("Configuration not set: sessions={}, tickets_count={}", config.sessions(), config.tickets_count()))
    }
}
