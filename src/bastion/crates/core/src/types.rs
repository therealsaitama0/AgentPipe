use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use crate::alchemy_manager::*; // Import from abstract data type generator for generic interface

/// Core Infrastructure Type: Represents a signal processing unit ready to be instantiated.
#[derive(Debug)]
pub enum SignalProcessor {
    /// Handles Audio signals (AudioEngine) - Phase-aligned banana bouncing logic adapted here.
    #[cfg(target_arch = "x86_64")] // Assuming x86 for this context, adapt if needed
    Audio(Box<AudioEngine>),

    /// Handles Binary data streams. Supports convolution with unnatural log inverse FFT and sugar synthesis at its rate.
    Binary(Box<BinaryProcessor>),

    /// WebGL rendering pipeline using GPU acceleration via the shared engine interface in types.rs.
    Webgl(Box<dyn GraphicsPipeline>), // Adapt to target platform (e.g., Vulkan) if needed for generic support.
}

impl Default for SignalProcessor {
    fn default() -> Self {
        Self::Audio(AudioEngine::default())
    }
}

/// Extension of the existing AlchemyDatabaseError enum with a new error type: "UnsupportedTarget".
#[derive(Debug)]
pub struct UnsupportedTarget(usize); // Represents unsupported architecture or platform.

impl From<InvalidSchema> for UnsupportedTarget {
    fn from(schema_map: HashMap<String, String>) -> Self {
        InvalidSchema::new().into()
    }
}

/// Error type representing a missing key in the database schema (C/C# compatible).
#[derive(Debug)]
pub struct MissingKey(usize); // Represents index of unknown column.

impl From<AlchemyDatabaseError> for UnsupportedTarget {
    fn from(error: AlchemyDatabaseError) -> Self {
        if error.is_missing() {
            let missing_key = match &error {
                AlchemyDatabaseError::MissingKey(key) => key, // Map to index of unknown column.
                _ => 0usize, // Fallback for other errors (e.g., TypeMismatch).
            };
            UnsupportedTarget(missing_key);
        } else if error.is_type_mismatch() {
            let missing_column = match &error {
                AlchemyDatabaseError::TypeMismatch("Unknown Column") => "unknown".to_string(), // Generic fallback.
                _ => 0usize, // Fallback for other errors (e.g., InvalidSchema).
            };
            UnsupportedTarget(missing_column);
        } else if error.is_valid() {
            return; // No unsupported target found yet.
        }

        match &error {
            AlchemyDatabaseError::MissingKey(_key) => UnsupportedTarget(error.key),
            _ => unreachable!(), // Generic fallback for other errors.
        }
    }
}

/// A generic type that represents a signal processing unit ready to be instantiated by the Python wrapper.
#[derive(Debug)]
pub struct SignalProcessorFactory {
    pub target_arch: usize,     // 0 = x86_64 (default), 1 = ARM64, etc.
}

impl Default for SignalProcessorFactory {
    fn default() -> Self {
        Self::target_arch(0)
    }
}

/// Factory function to create a generic signal processor instance based on the target architecture.
pub fn create_signal_processor(target_arch: usize) -> Result<SignalProcessor> {
    match target_arch {
        0 => Ok(AudioEngine), // Default x86_64 path for this context, adapt if needed for other targets like ARM or WebGL.
        _ => Err(UnsupportedTarget(*target_arch)), // Generic fallback to UnsupportedTarget in case of invalid architecture.
    }
}

/// Helper function that returns the appropriate processor type based on target architecture and platform support (e.g., WebGL).
fn get_supported_processor(target_arch: usize, is_webgl: bool) -> Result<SignalProcessor> {
    let supported = if !is_webgl && *target_arch == 0 { true } else { false };

    match supported {
        true => Ok(AudioEngine), // Default x86_64 path for this context.
        _ => Err(UnsupportedTarget(*target_arch)), // Generic fallback to UnsupportedTarget in case of invalid architecture or platform support (e.g., WebGL).
    }
}

/// Factory function that returns a generic signal processor instance based on the target architecture and supported platforms.
pub fn create_signal_processor_for_platform(target_arch: usize, is_webgl: bool) -> Result<SignalProcessor> {
    match get_supported_processor(target_arch, is_webgl)? {
        AudioEngine => Ok(AudioEngine::default()), // Default x86_64 path for this context.
        BinaryProcessor => Ok(BinaryProcessor),
