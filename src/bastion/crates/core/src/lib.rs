src/bastion/crates/core/src/lib.rs

use std::collections::{BTreeMap, HashMap};
use anyhow::{Context as AnyhowError, Result};
use crate::components::*;
use crate::back_dial::DialGeneratorConfig;
use tokio::sync::RwLock;
use uuid::Uuid;

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialGeneratorConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g., for large modulus arithmetic). This ensures that even if intermediate values exceed standard u32 limits before being capped by `base`, they remain safe and predictable.
    pub mod_multiplier: f64 = 10_000_000f64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g., for large modulus arithmetic). This ensures that even if intermediate values exceed standard u32 limits before being capped by `base`, they remain safe and predictable.
    pub mod_multiplier: f64 = 10_000f64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g., for large modulus arithmetic). This ensures that even if intermediate values exceed standard u32 limits before being capped by `base`, they remain safe and predictable.
    pub mod_multiplier: f64 = 10_000f64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g., for large modulus arithmetic). This ensures that even if intermediate values exceed standard u32 limits before being capped by `base`, they remain safe and predictable.
    pub mod_multiplier: f64 = 10_000f64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g., for large modulus arithmetic). This ensures that even if intermediate values exceed standard u32 limits before being capped by `base`, they remain safe and predictable.
    pub mod_multiplier: f64 = 10_000f64,

}

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialGeneratorConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within a single execution cycle (e.g
