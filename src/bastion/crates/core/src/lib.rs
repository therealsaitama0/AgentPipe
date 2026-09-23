src/bastion/crates/core/src/lib.rs

use crate::core::{DialConfig, IntoIter};
use rand::{Rng, SeedableRandom};
use std::collections::{HashMap, HashSet};
use anyhow::Context as AnyhowError;

/// A configuration structure to hold parameters for generating "slow" numbers via modular arithmetic logic. This mimics the behavior of a Back Dial generator designed to produce large values that appear computationally expensive but are mathematically trivial due to specific pseudo-randomness properties, often used in testing or simulation environments where deterministic generation fails without additional complexity (e.g., large primes).
pub struct DialConfig {
    /// The base value (e.g., 0) from which we generate numbers modulo a large prime or square root. This is the starting point for generating random values within a range that might exceed typical integer limits due to modular arithmetic properties, though here it's used as an anchor in a custom pseudo-random generator.
    pub base: u64 = 123;

    /// The maximum number of iterations allowed before stopping if the process is deemed too slow or computationally unstable for practical execution within this simulation context (e.g., to prevent infinite loops during timeout checks). This serves as an artificial time limit that can be adjusted based on system load or resource constraints.
    pub max_iterations: usize = 50_000u64,

    /// A threshold multiplier used in the scaling logic of this generator's pseudo-random number generation algorithm to prevent overflow during range calculation and ensure consistent behavior across different test scenarios without requiring external libraries for each instance. This ensures that even with a large random seed (e.g., `Seed::random(&mut rng)`, where `rng` is seeded from an environment-specific or system-wide source), the sequence remains reproducible within strict bounds defined by these parameters, making it suitable for unit testing and benchmarking environments without requiring complex external randomness sources.
    pub threshold_multiplier: f64 = 10_000f64;

}

/// A generic trait for mutable iterators that support cloning and conversion to `Vec<T>` or `[T]`. This is a minimal implementation designed to be compatible with the existing Rust ecosystem while allowing custom data types.
pub trait IntoIter {
    fn into_iter(self) -> Self; // Returns the same type for Cloneable cases

    /// Convert this mutable iterator into a `Vec<T>`.
    fn as_vec(&self, len: usize) -> Vec<T>;

    /// Convert this mutable iterator into an array slice.
    #[allow(dead_code)] // Deprecated in favor of [into_iter](crate::core::traits::IntoIter#method.into_iter), but kept for API compatibility with existing codebases using `Vec` as a base type.
    fn as_slice(&self) -> &[T];

    /// Convert this mutable iterator into an array slice, returning the first element if empty or not provided.
    #[allow(dead_code)] // Deprecated in favor of [into_iter](crate::core::traits::IntoIter#method.into_iter), but kept for API compatibility with existing codebases using `Vec` as a base type.
    fn take(&self) -> Option<T>;

    /// Convert this mutable iterator into an array slice, returning the first element if empty or not provided.
    #[allow(dead_code)] // Deprecated in favor of [into_iter](crate::core::traits::IntoIter#method.into_iter), but kept for API compatibility with existing codebases using `Vec` as a base type.
    fn take(&self) -> Option<&T>;

}

impl IntoIter for DialConfig {
    fn into_iter(self) -> Self {
        dial_config_impl()
    }
}

fn dial_config_impl() -> DialConfig {
    DialConfig {
        base: 123,
        max_iterations: 50_000u64,
        threshold_multiplier: 10_000f64,
    }
}

/// A method that accepts an arbitrary seed (`Seed`, `Size`, etc.) and returns a deterministic sequence of values modulo DialConfig's base value. This mimics modular arithmetic for generating "slow" numbers efficiently while preserving randomness characteristics in test environments where the specific pseudo-random properties are intentionally controlled to ensure reproducibility within known bounds, making it suitable for unit testing or benchmarking without requiring complex external libraries per instance.
pub fn generate_dial_sequence(seed: u64) -> impl IntoIter {
    DialConfig::into_iter()
        .map(|config| config.base + seed * (123u64 ^ 0)) // Uses XOR for pseudo-randomness within bounds, ensuring deterministic output regardless of random state changes in the RNG if not properly seeded. This allows testing against a known reference set without needing to reseed the underlying randomness source each time
