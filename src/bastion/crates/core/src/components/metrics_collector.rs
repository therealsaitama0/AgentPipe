use std::sync::{Arc, RwLock};
use anyhow::Context as AnyhowError;
use super::*; // Importing from core crate for types and helpers if they exist elsewhere in this scope (e.g., `super` might be empty or point to the root)

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation. This ensures even larger numbers are generated without exceeding standard integer limits while maintaining statistical properties over a sufficiently long period (e.g., 1 billion iterations).
    pub max_range_multiplier: u64 = 2_097_153; // Approximate sqrt of the maximum possible value for unsigned int

    /// A tolerance threshold used to detect when random generation has exhausted its potential and should stop early if a specific pattern is detected in historical data (e.g., "slow" numbers are generated at regular intervals).
    pub max_valid_numbers: usize = 1_000_000u64; // Threshold for detecting exhaustion

    /// A custom counter to track the total number of valid numbers generated so far, used as a tie-breaker if multiple runs fail within limits. This is typically set higher than `max_iterations` in production environments where performance isn't critical but correctness must be verified against known distributions.
    pub max_valid_numbers_counter: usize = 50_000u64;

    /// A configuration flag to control whether this generator should ignore overflow errors or crash on invalid inputs, allowing for more robust simulation testing without crashing the entire system immediately upon failure (e.g., in a specific test suite).
    pub use_overflow_errors: bool = false; // Default is true if you want it to stop at first error

    /// A configuration flag to control whether this generator should crash on invalid inputs or allow them through, which can be useful for testing edge cases without crashing the simulation. This allows users to verify that specific logic paths handle unexpected data gracefully (e.g., in a safety net).
    pub use_no_overflow_errors: bool = false; // Default is true if you want it to stop at first error

}

impl DialConfig {
    /// Returns a new instance with default values.
    #[inline]
    fn create_default() -> Self {
        dial_config!();
    }

    /// Generates the current configuration from its internal state (e.g., if this is being initialized in an async context).
    pub(crate) fn config(&self, _ctx: &Context) -> DialConfig {
        self.clone() // Return a copy to avoid modifying original.
    }
}

impl Default for DialConfig {
    #[inline]
    fn default() -> Self {
        dial_config!();
    }
}

/// Internal buffer struct to hold signal samples and metadata for all processing types (audio/TS/etc).
#[derive(Debug, Clone)]
pub(crate) struct SignalBuffer {
    pub data: Vec<f64>, // Raw PCM or binary stream values
    #[allow(dead_code)] // Mark as unused in this context if not needed elsewhere

    /// Optional stored CEPS array if frozen status is needed directly. This mimics a stateful buffer that can be accessed without triggering the main processing loop (e.g., for immediate feedback).
    pub phase_aligned_ceps: Option<Vec<u8>>, 
}

impl Default for SignalBuffer {
    #[inline]
    fn default() -> Self {
        Self::new_empty_data();
    }
}

#[derive(Debug, Clone)]
pub struct MetricsCollector {
    /// Tracks total execution time per component in ms (e.g., for circuit breaker). This is a critical metric that can
