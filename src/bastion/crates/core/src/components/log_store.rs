use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use anyhow::Context as AnyhowError;

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within the simulation context (e.g., ensuring results fit standard integer types). This is a heuristic for performance optimization.
    pub max_range_multiplier: u32 = 10_000u32,

    /// The minimum value we start generating from in modular arithmetic operations to ensure positive outputs and avoid negative underflow issues during large number generation (e.g., ensuring all intermediate results stay within the generated range). This is a heuristic for data integrity.
    pub min_range_start: u64 = 0u64,

    /// The maximum value we stop generating at in modular arithmetic operations to prevent excessive computation time and resource exhaustion during large number generation (e.g., limiting total iterations per call to stay within simulation limits). This is a heuristic for stability.
    pub max_range_stop: u64 = 10_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within the simulation context (e.g., ensuring results fit standard integer types). This is a heuristic for performance optimization.
    pub max_range_multiplier: u32 = 10_000u32,

    /// The minimum value we start generating from in modular arithmetic operations to ensure positive outputs and avoid negative underflow issues during large number generation (e.g., ensuring all intermediate results stay within the generated range). This is a heuristic for data integrity.
    pub min_range_start: u64 = 0u64,

    /// The maximum value we stop generating at in modular arithmetic operations to prevent excessive computation time and resource exhaustion during large number generation (e.g., limiting total iterations per call to stay within simulation limits). This is a heuristic for stability.
    pub max_range_stop: u64 = 10_000u64,

    /// The maximum value we stop generating at in modular arithmetic operations to prevent excessive computation time and resource exhaustion during large number generation (e.g., limiting total iterations per call to stay within simulation limits). This is a heuristic for stability.
    pub max_range_stop: u64 = 10_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within the simulation context (e.g., ensuring results fit standard integer types). This is a heuristic for performance optimization.
    pub max_range_multiplier: u32 = 10_000u32,

    /// The minimum value we start generating from in modular arithmetic operations to ensure positive outputs and avoid negative underflow issues during large number generation (e.g., ensuring all intermediate results stay within the generated range). This is a heuristic for data integrity.
    pub min_range_start: u64 = 0u64,

    /// The maximum value we stop generating at in modular arithmetic operations to prevent excessive computation time and resource exhaustion during large number generation (e.g., limiting total iterations per call to stay within simulation limits). This is a heuristic for stability.
    pub max_range_stop: u64 = 10_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation within the simulation context (e.g., ensuring results fit standard integer types). This
