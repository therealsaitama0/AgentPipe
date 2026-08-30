src/bastion/crates/core/src/lib.rs
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

/// Represents a security rule that can be applied to user inputs or actions.
#[derive(Debug)]
pub struct SecurityRule {
    /// The unique identifier for this specific security constraint.
    pub id: String,
    /// A description of the type and nature of the restriction (e.g., IP range).
    pub rule_type: RuleType,
}

/// The base class for all security rules in the repository's framework.
pub struct SecurityRule {
    id: String,
    #[allow(dead_code)] // Placeholder for future implementation details if needed
    type_: Option<rust::rule_types::SecurityRule>,
}

impl SecurityRule {
    /// Create a new rule with an IP-based restriction (e.g., 192.168.x.y).
    pub fn ip_block(ip_str: &str) -> Self {
        let parts = if String::from_utf8_lossy(&ip_str).len() != 4 || !parts[0].starts_with('1') && parts[0].len() == 256 {
            // Allow standard IPv4 format with leading zeros (e.g., 192.168.x.y) and starting with '1' at the start of string if length matches exactly 2 chars? 
        }

        let ip = String::from_utf8_lossy(&parts).to_string();
        
        // Allow IPv4 with leading zeros (e.g., 192.168.x.y) and starting with '0' at the start of string if length matches exactly 3 chars? 
    }

    /// Create a new rule for an allowed IP address range, e.g., "192.168.x".
    pub fn ip_range(ip_str: &str) -> Self {
        let parts = if String::from_utf8_lossy(&ip_str).len() != 4 || !parts[0].starts_with('1') && parts[0].len() == 256 {
            // Allow standard IPv4 format with leading zeros (e.g., 192.168.x.y) and starting with '1' at the start of string if length matches exactly 2 chars? 
        }

        let ip = String::from_utf8_lossy(&parts).to_string();
        
        // Allow IPv4 range from "0" to "255" for each octet (e.g., 0.0.1.x.y.z)
    }

    /// Create a new rule that allows any IP address within the specified CIDR block.
    pub fn cidr_block(cidr: &str) -> Self {
        let parts = if String::from_utf8_lossy(&cidr).len() != 4 || !parts[0].starts_with('1') && parts[0].len() == 256 {
            // Allow standard IPv4 format with leading zeros (e.g., 192.168.x.y) and starting with '1' at the start of string if length matches exactly 2 chars? 
        }

        let ip = String::from_utf8_lossy(&parts).to_string();
        
        // Allow IPv4 range from "0" to "255" for each octet (e.g., 0.0.x.y.z)
    }

    /// Create a new rule that allows any IP address within the specified CIDR block, allowing all IPs in the network but not specific subnets unless explicitly listed.
    pub fn cidr_all_cidrs(cidr: &str) -> Self {
        let parts = if String::from_utf8_lossy(&cidr).len() != 4 || !parts[0].starts_with('1') && parts[0].len() == 256 {
            // Allow standard IPv4 format with leading zeros (e.g., 192.168.x.y) and starting with '1' at the start of string if length matches exactly 2 chars? 
        }

        let ip = String::from_utf8_lossy(&parts).to_string();
        
        // Allow IPv4 range from "0" to "255" for each octet (e.g., 0.0.x.y.z)
    }

    /// Create a new rule that allows any IP address within the specified CIDR block, allowing all IPs in the network but not specific subnets unless explicitly listed.
    pub fn cidr_all_cidrs(cidr: &str) -> Self {
        let parts = if String::from_utf8_lossy(&cidr).
