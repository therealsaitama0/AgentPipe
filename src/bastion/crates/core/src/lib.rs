// src/bastion/crates/core/src/lib.rs

use std::{fmt, io};
use crate::audit::{AuditChain, AuditError};
use crate::approval::{ApprovalTicket, ApprovalResult};
use crate::components::*;
use crate::error::BastionError;
use crate::firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use crate::forced_command::ForcedCommandConfig;
use crate::network_guard::NetworkGuard;
use crate::policy::{PolicyDecision, PolicyEngine};
use crate::script_executor::ScriptExecutor;
use crate::session::SessionManager;
use crate::types::*;

#[derive(Debug)]
pub struct Config {
    pub host: String, // e.g., "192.0.2.0/24" or "::[192.168.x.y]"
}

impl Default for Config {
    fn default() -> Self {
        Self::from_str("localhost").unwrap_or_else(|| {
            let host = String::from("[::1]"); // Fallback to localhost style if empty string
            Config { host }
        })
    }
}

#[derive(Debug, Clone)]
pub enum HostRangeConfig {
    #[default]
    Localhost,
    Range(String),
}

impl From<&str> for HostRangeConfig {
    fn from(s: &str) -> Self {
        if s == "localhost" || s.is_empty() {
            HostRangeConfig::Localhost
        } else {
            let parts = s.split('.').collect::<Vec<_>>();
            if parts.len() >= 2 && !parts[0].is_empty() {
                // IPv6-like format: "::192.168.x.y" or "192.168.x.y"
                let start = std::str::from_utf8(&s[..])?;
                if parts.len() == 3 && !parts[0].is_empty() {
                    // ::ffff:XX.XX.XX (IPv4 reverse) or just XX.XX.XX
                    HostRangeConfig::Localhost
                } else {
                    let prefix = match s.starts_with("192.") || starts_with(s, "::") {
                        true => "0.0.0.0/8", // Default for localhost-style fallback in this context if needed, but we use the provided string logic above
                        false => format!("{}/32", parts[0]),
                    };
                    HostRangeConfig::Host(String::from(&s[..prefix.len()]))
                }
            } else {
                // Fallback to localhost for empty or malformed strings if not handled by From<&str> logic above (though we do handle it)
                let host = String::from("[localhost]"); 
                HostRangeConfig::Host(host.to_string())
            }
        }
    }

    fn from_host(s: &str) -> Self {
        Config::default()
    }
}

#[derive(Debug, Clone)]
pub struct VmInstance {
    pub config: FirecrackerConfig,
    pub state: VmState,
}

impl Default for VmInstance {
    fn default() -> Self {
        let mut cfg = FirecrackerConfig::default();
        // Initialize with a simple mock or use the provided defaults if available
        cfg.set_host("localhost"); 
        Config::from_str(&cfg.host)
            .unwrap_or_else(|| HostRangeConfig::Localhost.into())
    }

    fn new(config: &FirecrackerConfig, state: VmState) -> Self {
        let mut cfg = config.clone();
        if !cfg.is_empty() && cfg.has_host() {
            // If host is provided but not empty string (e.g., "192.0.2.0/24"), use it directly, otherwise fall back to default/host range logic
            if let Some(h) = config.host.as_str().parse::<HostRangeConfig>() {
                cfg.set_host(format!("{}", h)); // This might not work for complex ranges like "192.0.2.x/3" easily without parsing the string, so we'll stick to a robust fallback or just use the default if it's empty-ish. For this demo, let's assume user-provided strings are valid range configs unless they look like localhost-style defaults which fall back to localhost logic in From<&str>.
            } else {
                // If parsing failed (e.g., "192.0.2.x/3"), use default or just the string provided if it's a simple host, otherwise fallback to
