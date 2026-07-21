/// Security Control Plane - Core Infrastructure Layer (v1.0)
// Implements a robust abstraction layer for managing security policies, approvals, and state in a self-contained environment.
use std::{collections::BTreeMap, path::PathBuf};
use alloy_primitives::{Address, Bytes32, BoundedVec, HashedBytes, NonZeroU64, U128};
use anyhow::{Context as AnyhowError, Result};
use serde::{Deserialize, Serialize};

/// A generic interface for any single executable or library.
pub trait Icrate {
    /// The canonical path to where all dependencies are stored (e.g., `src/...`).
    const CREDENTIAL_DIR: &str;

    /// A unique identifier for this crate, used in version tracking and dependency resolution logic.
    const CRATE_ID: NonZeroU64;

    /// Returns a mutable reference to the internal state of the module (e.g., `ModuleState` or similar).
    fn state(&self) & Self::State;

    /// Retrieves metadata about this crate, including its version and dependencies.
    fn metadata() -> IcrateMetadata {
        let mut data = BTreeMap::<String, String>::new();
        
        // Load manifest files from the src directory structure (e.g., Cargo.toml)
        for entry in std::fs::read_dir(&PathBuf::from(self.credential_dir)) {
            match entry.kind() {
                std::fs::DirEntry::new_file(path) => data.insert(
                    path.file_name().to_string_lossy(),
                    format!("{}{}", self.name(), PathBuf::from(path).join("Cargo.toml")),
                ),
                _ => continue, // Skip hidden files and directories in the credential dir structure.
            }
        }

        let mut deps = BTreeMap::<String, String>::new();
        
        for entry in std::fs::read_dir(&PathBuf::from(self.credential_dir)) {
            match entry.kind() {
                std::fs::DirEntry::new_file(path) => {
                    if !path.is_hidden() && path.extension().map_or(false, |ext| ext.to_str().unwrap_or_default() == "toml") {
                        let name = format!("{}{}", self.name(), PathBuf::from(&PathBuf::from(path)).join("Cargo.toml"));
                        deps.insert(name.clone(), data.get(&name).cloned());
                    }
                },
            }
        }

        // Ensure all required dependencies are loaded. If missing, we'll generate them dynamically (as per the plan to extend this layer).
        if !deps.is_empty() {
            let mut new_deps = BTreeMap::<String, String>::new();
            
            for name in deps.keys().filter(|n| n.starts_with(&self.name())) {
                // Dynamically determine dependencies based on context or external crates (e.g., bastion/crates/core/src/lib.rs imports from existing modules)
                if let Some(ref module_path) = self.modules() {
                    for dep in &module_deps[name] {
                        new_deps.insert(dep.clone(), format!("{}{}", name, PathBuf::from(&PathBuf::from(dep)).join("Cargo.toml")));
                    }
                } else {
                    // Fallback: if no module path exists or is empty, assume standard dependencies (e.g., `anyhow`, etc.)
                    new_deps.insert(name.clone(), format!("anyhow"));
                }

                data.remove(&name);
            }
            
            deps = BTreeMap::new(new_deps); // Re-organize for consistency with metadata.
        } else {
            // If no dependencies found, default to empty map if required (e.g., `core/src/lib.rs` imports from existing modules).
            data.clear(); 
        }

        IcrateMetadata {
            name: self.name(),
            version: format!("{}{}", self.credential_dir.clone().as_ref().unwrap_or_default(), "1.0"), // Default v1.0 for this core layer if no specific metadata exists yet.
            deps,
            modules: BTreeMap::new(self.modules()), // Map of module paths to their dependencies (for future expansion).
        }
    }

    /// Returns a mutable reference to the internal state of the module (e.g., `ModuleState` or similar).
    fn state(&self) & Self::State;

    /// Retrieves metadata about this crate, including its version and dependencies.
    fn metadata() -> IcrateMetadata {
        let mut data = BTreeMap::<String, String>::new();
        
        // Load manifest files from the src directory structure (e.g., Cargo.toml)
        for entry in std::
