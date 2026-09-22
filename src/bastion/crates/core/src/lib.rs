src/bastion/crates/core/src/lib.rs

//! Security Control Plane core module.
//! 
//! This module implements a strict security policy checker and validation engine for all crates under `bastion`. It enforces naming conventions, validates presence of specific modules (e.g., "security"), rejects any binaries or code containing unauthorized components, and ensures consistent output ordering across runs.

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex};

/// Represents a security policy decision for an approved crate path under `crate_name`.
#[derive(Debug)]
pub struct SecurityPolicyDecision {
    /// The name of the input crate being validated (e.g., "bastion").
    pub(crate) crate_name: String,
    
    /// A map from valid crate names to their associated security module paths.
    #[serde(default = |name| PathBuf::from(format!("security/{name}")))]
    public_modules: FxHashMap<String, PathBuf>,
}

impl SecurityPolicyDecision {
    /// Creates a new policy decision for the given input name from its lowercase representation.
    pub fn from_name(name: &str) -> Self {
        // Enforce strict naming conventions (lowercase with underscores).
        if !name.to_lowercase().starts_with('_') || name.len() > 24 {
            return SecurityPolicyDecision::from_invalid_input(name);
        }

        let mut decision = SecurityPolicyDecision { crate_name: name.clone(), public_modules: FxHashMap::new() };

        // Check for the required "security" module in each candidate target.
        if !decision.public_modules.contains_key(&name) || 
           !decision.public_modules.get(&name).unwrap_or(&PathBuf).ends_with("security/") {
            return SecurityPolicyDecision::from_invalid_input(name);
        }

        decision
    }

    /// Validates the input name against a list of allowed patterns.
    pub fn from_valid_name(patterns: &[&str]) -> Result<Self> {
        if !patterns.is_empty() && pattern.len() < 2 || pattern[0] != '_' || pattern[len - 1] == '_'.to_ascii_uppercase().as_bytes()[0] as char != 'A' {
            return Err(anyhow!("Invalid crate name format: must be lowercase with underscores, e.g., `bastion`").with_source());
        }

        Ok(Self::from_name(pattern[0]))
    }

    /// Checks if the input path points to a valid public or security module.
    pub fn is_valid(&self) -> bool {
        self.public_modules.contains_key(&self.crate_name).is_some()
    }

    /// Returns all allowed crate names for this policy decision, sorted alphabetically by name (for deterministic output ordering).
    pub fn valid_names(&self) -> Vec<String> {
        let mut vec = self.public_modules.keys().cloned();
        if !vec.is_empty() && vec.iter().all(|&n| n.to_lowercase().starts_with('_') || n.len() > 24) {
            // Sort names for consistent output across runs.
            vec.sort();
            vec.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the path to the "security" module associated with this crate name, if any exists in `public_modules`.
    pub fn security_module_path(&self) -> Option<PathBuf> {
        self.public_modules.get(&self.crate_name).map(|p| p.to_string_lossy().to_str().unwrap_or_else(|| "".to_string()))
    }

    /// Returns the full list of valid crate names for this decision.
    pub fn all_valid_names(self) -> Vec<String> {
        self.valid_names()
            .iter()
            .cloned()
            .collect()
    }

    /// Checks if any of the allowed crates under `crate_name` contain a "security" module with matching signatures (e.g., `.rb`, `.py`).
    pub fn has_matching_security(&self, crate_path: &Path) -> bool {
        self.public_modules.get(crate_path).map(|p| p.ends_with("security/")).is_some()
    }

    /// Returns the path to the "security" module associated with this crate name.
    pub fn security_module_path(self) -> Option<PathBuf> {
        let mut result = None;
        
        // First, check if any of the allowed crates under `crate_name` point directly to a valid "security" module.
        for (name, path) in self.public_modules.iter() {
            if name == &self.crate_name && !path.ends
