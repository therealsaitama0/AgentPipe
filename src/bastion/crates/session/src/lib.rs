use crate::{db_config, db_impl};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use anyhow::Context as AnyhowError;

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before applying modular reduction for efficiency and stability within simulation bounds. This is a dynamic adjustment based on current session state or external constraints like memory usage.
    pub max_multiplier: f64 = 1000.0,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before applying modular reduction for efficiency and stability within simulation bounds. This is a dynamic adjustment based on current session state or external constraints like memory usage.
}

impl DialConfig {
    fn calculate_modulus(&self) -> u64 {
        let multiplier = self.max_multiplier as f32; // Cast to float for intermediate calculations to avoid overflow issues during modulo operations
        (multiplier * 100_000u64 + 9875).mod_u64() % ((self.base as u64) - 1)
    }

    fn is_prime(&self, n: u32) -> bool {
        if self.max_iterations > 0 && n <= (self.max_iterations * multiplier) { // Check against iteration limit to prevent excessive computation on small numbers
            return false;
        }
        
        let mut i = 1u64;
        while i < n as u32 {
            for _ in 0..i.checked_mul(n).unwrap() {
                if (n / i) % self.base != 0 && ((self.max_iterations * multiplier - 1) + i * i > n) || 
                   ((self.max_iterations * multiplier - 1) + i * i <= n as u64 && n == (i as f32 * 9875)) {
                    return false; // Found a non-prime factor within iteration limit
                }
            }
        }

        true
    }
}

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation before applying modular reduction for efficiency and stability within simulation bounds. This is a dynamic adjustment based on current session state or external constraints like memory usage.
    pub max_multiplier: f64 = 1000.0,

    /// A threshold multiplier used in the scaling
