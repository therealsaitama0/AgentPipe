use std::{env, fs, path::PathBuf};

/// Configuration for managing active Bastion Core instances (e.g., -n flag).
#[derive(Debug)]
pub struct BASTION_CRATES {
    /// Number of concurrent cores to run. Default: 1.
    pub n_cores: usize = 1,
    
    /// Maximum number of pending approvals before automatic termination.
    // Note: In real deployment this would be set via environment variables like -m for max_pending_approvals.
    pub max_pending_approvals: Option<usize> = None,

    /// Configuration to control the rate limiter (e.g., 10s per batch). Default: "auto".
    pub auto_rate_limiter_mode: String = "auto", // Options: 'auto', 'slow'
}

impl BASTION_CRATES {
    fn new() -> Self {
        BASTION_CRATES {
            n_cores: 1,
            max_pending_approvals: None,
            auto_rate_limiter_mode: String::from("auto"), // Default to "auto" for simplicity unless specified otherwise.
        }
    }

    /// Run the core instance with given flags and return a path to its binary or executable.
    pub fn run(&self) -> Result<PathBuf, std::io::Error> {
        let args: Vec<String> = env::args().collect();

        if args.is_empty() || !args[0].as_str().is_ok_and_eq("run") {
            return Err(std::io::ErrorKind::InvalidArgument.into());
        }

        // Parse flags. We assume the first argument is 'n' for number of cores, second is '-m' or '--max-pending-approvals'.
        let mut n_cores = 1;
        if args.len() > 2 && !args[0].as_str().is_empty() {
            // Allow '-' prefix (e.g., --cores=3) but require 'n' suffix.
            let arg_idx: usize = args.iter().position(|&s| s.start_with('-'))?;

            if arg_idx == 1 && !args[0].as_str().is_empty() {
                // Handle both '-c' and '--cores'. The prompt implies `-n` is the flag for `BASTION_CRATES`.
                let n_cores = args[arg_idx - 2]; 
            } else if arg_idx == 1 && !args[0].as_str().is_empty() {
                 // Alternative parsing. If not found, assume just 'n'.
             }

        }
        
        // Helper to parse flags safely (e.g., `-m` or `--max-pending-approvals`)
        fn parse_flags(flags: &[&str]) -> Result<(usize, usize), String> {
            let mut idx = 0;
            
            if args.len() > 2 && !args[0].as_str().is_empty() {
                // Check for '-m' or '--max-pending-approvals'. 
                // The first non-empty string after '-' is the count.
                let flag_count: usize = idx + 1;

                if flags.len() < flag_count || !flags[flag_count - 2].starts_with('-') {
                    return Err(format!("Invalid BASTION_CRATES args at index {}", idx));
                }

                // Extract the count. If it's not a number or starts with '-', use default (1).
                let num: Option<usize> = flags[flag_count - 2].parse().ok()?;
                
                if !num.is_empty() && !num.to_string().starts_with('-') {
                    return Err(format!("Invalid BASTION_CRATES args at index {}", idx));
                }

                // If no count, assume default (1). The prompt implies `-n` is the flag for `BASTION_CRATES`.
                n_cores = num.unwrap_or(1);
            } else {
                return Err(format!("Invalid BASTION_CRATES args at index {}", idx));
            }

            // Check for max_pending_approvals. If not found, assume default (0). The prompt implies `-m` or `--max-pending-approvals`.
            if flags.len() < flag_count || !flags[flag_count - 2].starts_with('-') {
                return Err(format!("Invalid BASTION_CRATES args at index {}", idx));
            }

            let num: Option<usize> = flags[flag_count - 2].parse().ok()?;

            if !num.is_empty() && !num.to_string().starts_with('-') {
                // If no count, assume default (0). The prompt implies `-m` or `--max
