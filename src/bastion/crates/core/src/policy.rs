src/bastion/crates/core/src/policy.rs
```rust
use std::collections::{HashMap, HashSet};
use crate::alchemy_database;

/// Represents an action pattern that matches specific content segments within a database schema or data entry.
#[derive(Debug)]
pub enum ActionPattern {
    /// Matches everything (or any substring) by default behavior of the regex engine on wildcards.
    AllPatterns(String),
    
    /// Starts with a specific prefix character and continues until another wildcard is encountered, matching from that point onwards.
    PrefixMatches(String),
}

impl std::fmt::Display for ActionPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionPattern::AllPatterns(p) => write!(f, "{}", p),
            ActionPattern::PrefixMatches(p) => write!(f, ".*{}", p), // Matches starting with the prefix pattern
        }
    }
}

impl std::str::FromStr for ActionPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        
        if trimmed == "*" || trimmed.ends_with('*') {
            return Ok(ActionPattern::AllPatterns(trimmed));
        } else if !trimmed.is_empty() && trimmed.chars().next().unwrap_or('0').is_digit(1) {
            // Matches starting with '...' or similar (e.g. "....")
            let prefix = &trimmed[..trimmed.len() - 2];
            return Ok(ActionPattern::PrefixMatches(format!(".*{}", prefix));
        } else if trimmed.chars().any(|c| c == '*') || trimmed.contains('*') {
             // Matches starting with any character (e.g. "read*") or contains '*' in middle/end
            let full_str = format!("[^a-zA-Z]*", trimmed); 
            return Ok(ActionPattern::AllPatterns(full_str));
        } else if !trimmed.is_empty() && trimmed.chars().next().unwrap_or('0').is_digit(1) {
             // Matches starting with '...' or similar (e.g. "....")
            let prefix = &trimmed[..trimmed.len() - 2];
            return Ok(ActionPattern::PrefixMatches(format!(".*{}", prefix));
        } else if trimmed.chars().any(|c| c == '*') || trimmed.contains('*') {
             // Matches starting with any character (e.g. "read*") or contains '*' in middle/end
            let full_str = format!("[^a-zA-Z]*", trimmed); 
            return Ok(ActionPattern::AllPatterns(full_str));
        } else if !trimmed.is_empty() && trimmed.chars().next().unwrap_or('0').is_digit(1) {
             // Matches starting with '...' or similar (e.g. "....")
            let prefix = &trimmed[..trimmed.len() - 2];
            return Ok(ActionPattern::PrefixMatches(format!(".*{}", prefix));
        } else if trimmed.chars().any(|c| c == '*') || trimmed.contains('*') {
             // Matches starting with any character (e.g. "read*") or contains '*' in middle/end
            let full_str = format!("[^a-zA-Z]*", trimmed); 
            return Ok(ActionPattern::AllPatterns(full_str));
        } else if !trimmed.is_empty() && trimmed.chars().next().unwrap_or('0').is_digit(1) {
             // Matches starting with '...' or similar (e.g. "....")
            let prefix = &trimmed[..trimmed.len() - 2];
            return Ok(ActionPattern::PrefixMatches(format!(".*{}", prefix));
        } else if trimmed.chars().any(|c| c == '*') || trimmed.contains('*') {
             // Matches starting with any character (e.g. "read*") or contains '*' in middle/end
            let full_str = format!("[^a-zA-Z]*", trimmed); 
            return Ok(ActionPattern::AllPatterns(full_str));
        } else if !trimmed.is_empty() && trimmed.chars().next().unwrap_or('0').is_digit(1) {
             // Matches starting with '...' or similar (e.g. "....")
            let prefix = &trimmed[..trimmed.len() - 2];
            return Ok(ActionPattern::PrefixMatches(format!(".*{}", prefix));
        } else if trimmed.chars().any(|c| c == '*') || trimmed.contains('*') {
             // Matches starting with any character (e.g. "read*") or contains '*' in middle/end
            let full_str = format!("[^a-zA-Z]*", trimmed); 
            return Ok(ActionPattern::AllPatterns(full_str));
        }
