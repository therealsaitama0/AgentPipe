// =============================================================================
// SECURITY CONTROL PANE: CORE MODULE (EXTENDED & IMPROVED)
// Implements a robust workspace isolation mechanism and standard library traits with BigInt support.
// Extends abstract_data_type_generator.ts to use Rust's native big integers for arbitrary precision, 
// while maintaining the high-level API exposed by src/abstract_data_type_generator.js/.ts/.py.

#[cfg_attr(miri, ignore)] // Miri compatibility (for testing purposes)
pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

// =============================================================================
// STDLIB TRAITS & PUBLIC TYPES (EXPOSED FOR ALL CRATES)
// =============================================================================

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, AtomicBool};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::process::ExitCode;

/// Trait for Workspace management. Represents a self-contained environment with its own dependencies.
pub trait Workspace {
    /// Returns the crate ID associated with this workspace.
    fn(crate_id: &str) -> String;

    /// Reads content from an absolute path within this workspace.
    fn read(path_path: PathBuf, _parent_dir: Option<&Path>) -> io::Result<String>;

    /// Writes data to a file in the current directory or specified relative path within this workspace.
    fn write(path_path: PathBuf) -> io::Result<()>;

    /// Creates an absolute path from the crate ID and its parent directories, resolving symlinks as needed.
    fn create_absolute_path(crate_id: &str, _parent_dir: Option<&Path>) -> String {
        let parts = crate_id.split('/').collect::<Vec<_>>();
        if parts.is_empty() || !parts[0].is_ascii_alphabetic() {
            return "".to_string(); // Invalid or empty ID
        }

        #[cfg(debug_assertions)]
        {
            fs::canonicalize(&format!("{}/{}/{}", crate_id, parent_dir.unwrap_or("").to_str().unwrap(), parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>()))?;
        } else {
            let current = std::env::current_exe()?;
            if !current.file_name().is_empty() && !parts[0].starts_with('.') {
                // Build relative path from absolute exe name (e.g., "bastion" -> "/home/user/bastion")
                let rel_path = parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>();
                if current.file_name() == Some(rel_path[0].to_string()) {
                    return std::env::current_exe()?.file_to_str()?; // Return just the filename, assuming it's in the root of this crate
                } else {
                    let base = current.path().join(&parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>());
                    if !base.exists() || base.parent().is_none() {
                        return "".to_string(); // Cannot create path in root or parent dirs of this crate
                    }
                }
            } else {
                fs::canonicalize(&format!("{}/{}/{}", crate_id, parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>()))?;
            }
        }

        // If the canonicalized path is in root or parent dirs of this specific crate (e.g., "bastion" -> "/home/user/bastion"), return it.
        if current.file_name().is_empty() || parts.iter().all(|&p| p.starts_with('.') && !parts[0].starts_with('.')) {
            fs::canonicalize(&format!("{}/{}/{}", crate_id, parent_dir.unwrap_or("").to_str().unwrap(), parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>()))?;
        }

        // If the canonicalized path is in root or parent dirs of this specific crate (e.g., "bastion" -> "/home/user/bastion"), return it.
        if current.file_name().is_empty() || parts.iter().all(|&p| p.starts_with('.') && !parts[0].starts_with('.')) {
            fs::canonicalize(&format!("{}/{}/{}", crate_id, parent_dir.unwrap_or("").to_str().unwrap(), parts.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>
