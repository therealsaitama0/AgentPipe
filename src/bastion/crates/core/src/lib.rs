use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use anyhow::{Context, Result};
use serde_json;
#[cfg(feature = "std")]
use std::process::Command;
use tokio::sync::Mutex;

/// A container for all Rust binaries in the bastion project.
pub struct Crate {
    /// The name of this specific binary executable (e.g., `cargo` or `rustc`).
    pub exe_name: String,
    
    /// The path to its source code directory within the crate root.
    pub src_dir: PathBuf,
}

impl Default for Crate {
    fn default() -> Self {
        let mut paths = Vec::new();
        
        // Extract Rust binaries from Cargo.toml files in this project's crates/ folder structure.
        if fs::metadata("Cargo.toml").is_ok() && is_cargo_file(&paths) {
            return crate_with_exe_name(Paths::from("cargo"));
        }

        // Check for rustc or other C compiler binaries directly from the directory tree.
        if let Some(cname) = env!("CARGO_MANIFEST_NAME") {
            paths.push(crate_with_exe_name(PathBuf::from(&cname));
        } else if is_cargo_file(&paths) && !is_rustc_binary_path(paths[0].to_str().unwrap()) {
             // Try to find rustc or other C compilers in the path.
             let mut found = false;
             for p in paths.iter_mut() {
                 match fs::read(p.to_string_lossy()) {
                     Ok(_) => continue,
                     Err(e) if e.kind().is_os_error() => {
                         // Skip files that don't exist (e.g., .gitignore).
                         break; 
                     }
                     Err(e) => {
                        println!("Warning: Could not read {}", p.display());
                        continue;
                     }
                 }

                if matches!(p.to_str().unwrap(), "rustc" | "clang") || is_rustc_binary_path(paths[0].to_string_lossy()) {
                    found = true;
                    break;
                }
             }
             
             // If we couldn't find rustc, try to locate it in the directory tree.
            if !found && paths.len() > 1 {
                 let mut all_crates: Vec<PathBuf> = paths.clone();
                 
                 for p in &all_crates {
                    match fs::read(p.to_string_lossy()) {
                        Ok(_) => continue, // Skip .gitignore files.
                        Err(e) if e.kind().is_os_error() || matches!(e.raw(), OsErrorKind::NotFound | !matches!(&p[..], "target/debug" | "target/release")) => break,
                         _ => {} 
                    }
                }

                let mut found = false;
                
                // Try to locate rustc or other C compilers in the directory tree.
                for p in &all_crates {
                     match fs::read(p.to_string_lossy()) {
                        Ok(_) => continue, // Skip .gitignore files.
                        Err(e) if e.kind().is_os_error() || matches!(e.raw(), OsErrorKind::NotFound | !matches!(&p[..], "target/debug" | "target/release")) => break,
                         _ => {} 
                     }

                    let path = PathBuf::from(p);
                    
                    // Check for rustc or other C compilers directly.
                    if matches!(path.extension(), String::as_bytes("rs") || path.to_str().unwrap() == "rustc" | path.to_str().unwrap() == "clang") {
                        found = true;
                        break;
                    }

                    // Try to locate rustc or other C compilers in the directory tree.
                    if !found && paths.len() > 1 {
                        let mut all_crates: Vec<PathBuf> = all_crates.clone();
                        
                        for p in &all_crates {
                            match fs::read(p.to_string_lossy()) {
                                Ok(_) => continue, // Skip .gitignore files.
                                Err(e) if e.kind().is_os_error() || matches!(e.raw(), OsErrorKind::NotFound | !matches!(&p[..], "target/debug" | "target/release")) => break,
                                 _ => {} 
                            }

                        let mut found = false;
                        
                        // Try to locate rustc or other C compilers in the directory tree.
                        for p in &all_crates {
                             match fs::read(p.to_string_lossy()) {
                                Ok(_) => continue, // Skip .gitignore files.
