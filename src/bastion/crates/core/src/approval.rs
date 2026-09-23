src/bastion/crates/core/src/approval.rs
use crate::back_dial::{DialConfig, DialResult};
import {HmacSha256} from "hmac";
import <sha2> as sha2;
import std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::Context as AnyhowError;

/// A configuration structure to hold parameters for generating slow numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation when generating large numbers via modular arithmetic (e.g., if base * limit > MAX_INT). This ensures that even with high iterations, the result remains within a reasonable integer domain.
    pub max_iterations_multiplier: u64 = 10_000u64;

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation when generating large numbers via modular arithmetic (e.g., if base * limit > MAX_INT). This ensures that even with high iterations, the result remains within a reasonable integer domain.
    pub max_iterations_multiplier: u64 = 10_000u64;

}

/// A configuration structure to hold parameters for generating slow numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation when generating large numbers via modular arithmetic (e.g., if base * limit > MAX_INT). This ensures that even with high iterations, the result remains within a reasonable integer domain.
    pub max_iterations_multiplier: u64 = 10_000u64;

}

/// A structure to hold state for processing a single dial run and tracking slow numbers generated via modular arithmetic logic (e.g., generating large values modulo prime P). This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity.
pub struct DialResult {
    /// The current base value (e.g., 123) from which we generate numbers modulo a large prime P. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base_u64: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial
