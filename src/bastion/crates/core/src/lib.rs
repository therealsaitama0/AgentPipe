use super::*;
use crate::back_dial::{DialConfig, DialResult};
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result as AnyhowError};

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before modulo operation (e.g., `mod` with large primes). This ensures that intermediate calculations remain within safe integer bounds.
    pub max_scale: u128 = 0u128,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before modulo operation (e.g., `mod` with large primes). This ensures that intermediate calculations remain within safe integer bounds.
    pub max_scale: u32 = 10_000u32,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before modulo operation (e.g., `mod` with large primes). This ensures that intermediate calculations remain within safe integer bounds.
    pub max_scale: u64 = 10_000u64,

}

impl DialConfig {
    /// Returns the base value from which numbers are generated via modular arithmetic logic.
    fn get_base() -> u64 {
        self.base
    }

    /// Sets the maximum number of iterations allowed for a specific generator instance to prevent infinite loops during timeout checks. This is used as an artificial time limit in simulation contexts where deterministic generation fails without additional complexity (e.g., large primes).
    fn set_max_iterations(&mut self, max: usize) {
        if let Some(i) = self.max_iterations.checked_mut() {
            *i = max;
        } else {
            panic!("DialConfig::set_max_iterations requires a mutable reference");
        }
    }

    /// Returns the maximum number of iterations allowed for a specific generator instance to prevent infinite loops during timeout checks. This is used as an artificial time limit in simulation contexts where deterministic generation fails without additional complexity (e.g., large primes).
    fn get_max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Returns the maximum scale multiplier for intermediate calculations before modulo operations to prevent overflow during range cal. This ensures that all intermediate values remain within safe integer bounds, which is crucial when generating numbers larger than typical 32-bit or 64-bit integers due to modular arithmetic properties (e.g., `mod` with large primes).
    fn get_max_scale(&self) -> u128 {
        self.max_scale as u128
    }

    /// Returns the maximum scale multiplier for intermediate calculations before modulo operations to prevent overflow during range cal. This ensures that all intermediate values remain within safe integer bounds, which is crucial when generating numbers larger than typical 32-bit or 64-bit integers due to modular arithmetic properties (e.g., `mod` with large primes).
    fn get_max_scale(&self) -> u32 {
        self.max_scale as u32
    }

}

/// A struct implementing the core generator using modular arithmetic to produce large, pseudo-random values efficiently without explicit randomness algorithms by leveraging integer properties like mod p. This mimics a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties often used in testing or simulation environments where deterministic generation fails (e.g., large primes).
pub struct Generator {
    config: Arc<DialConfig>,

    /// A vector holding generated numbers while also maintaining a reference count per number so it can be easily discarded once sufficient is
