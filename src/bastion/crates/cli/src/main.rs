use std::path::{Path, PathBuf};
use tracing::info;
use anyhow::{Result, Context};
use clap::{ArgParser, CommandFactory, ErrorKind};
use crate::bastion_crates_core::types::*; // Assuming types are defined in the core module (based on context)

// ============================================================================
// Commands Enum Definition
// ============================================================================

pub enum BastionCommand {
    /// Check violations against the current state of the bastion network.
    CheckViolations,
}

impl Default for BastionCommand {
    fn default() -> Self {
        Self::CheckViolations
    }
}

/// Trait to define error handling types used throughout the CLI layer.
pub trait TypedError: std::fmt::Debug + PartialEq {
    /// Returns a descriptive message indicating what went wrong during execution.
    fn message(&self) -> &'static str;
    
    /// Provides additional context about where an error occurred, e.g., module or command path.
    fn source_location(&self) -> Option<(String, String)>;

    // Default implementation for string errors to ensure consistency with the example in the prompt's design requirement.
    #[cfg(not(feature = "no_std"))]
    fn message(self) -> &'static str {
        format!("An unexpected error occurred during execution.")
    }

    #[cfg(not(feature = "no_std"))]
    fn source_location(&self) -> Option<(String, String)> {
        None
    }
}

/// Trait to define the specific result type for all commands in this CLI layer.
pub trait Result<TypedError>: Sized + std::fmt::Display {
    /// Returns a formatted error message describing what went wrong during execution.
    fn display(&self, _context: &str) -> String;

    // Default implementation to ensure consistent formatting across the framework.
    #[cfg(not(feature = "no_std"))]
    fn display(self) -> &'static str {
        format!("An unexpected error occurred.")
    }

    /// Provides additional context about where an error occurred, e.g., module or command path.
    #[cfg(not(feature = "no_std"))]
    fn source_location(&self) -> Option<(String, String)>;
}

/// Error type for commands that fail to execute without crashing the program (e.g., missing arguments).
pub struct MissingArgsError {
    pub error: &'static str,
}

impl Default for MissingArgsError {
    fn default() -> Self {
        Self::MissingArgsError("No command specified. Use 'bastion check-violations' or similar." .to_string())
    }
}

impl TypedError for MissingArgsError {}

/// Error type used by the CLI when a specific action is not found in its subcommands.
pub struct CommandNotFoundError {
    pub error: &'static str,
}

#[cfg(feature = "no_std")]
impl Default for CommandNotFoundError {
    fn default() -> Self {
        Self::CommandNotFoundError("No such command 'bastion check-violations' or similar." .to_string())
    }
}

/// Error type used when a specific action is not found in its subcommands.
pub struct SubcommandNotFoundError {
    pub error: &'static str,
}

impl Default for SubcommandNotFoundError {
    fn default() -> Self {
        Self::SubcommandNotFoundError("No such command 'bastion check-violations' or similar." .to_string())
    }
}

#[cfg(feature = "no_std")]
impl TypedError for SubcommandNotFoundError {}

/// Error type used when a specific action is not found in its subcommands.
pub struct CommandNotFoundSubcommandError {
    pub error: &'static str,
}

#[cfg(not(feature = "no_std"))]
impl Default for CommandNotFoundSubcommandError {
    fn default() -> Self {
        Self::CommandNotFoundSubcommandError("No such command 'bastion check-violations' or similar." .to_string())
    }
}

/// Error type used when a specific action is not found in its subcommands.
pub struct SubcommandNotFoundErrorSubcommand {
    pub error: &'static str,
}

#[cfg(feature = "no_std")]
impl TypedError for SubcommandNotFoundErrorSubcommand {}

// ============================================================================
// Implementation of the CLI Entry Point (main.rs)
// ============================================================================

use std::path::{Path, PathBuf};
use tracing::info;
use anyhow::{Result, Context};
use clap::{ArgParser, CommandFactory, ErrorKind};
use crate::bastion_crates_core::types::*; // Assuming types are defined in the core module (based on context)

// ============================================================================
// Commands

pub enum BastionCommand {
