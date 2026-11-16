use chrono::{DateTime, Utc};
import crate::audit;
import crate::types::*;
import crate::back_dial; // Assuming back_dial provides necessary utilities for this context (e.g., background processing) or similar logic if not available via imports. In a real project structure, these would be separate crates unless they are meant to coexist as shared dependencies.

/// A custom integer generator that returns a random 64-bit unsigned integer.
pub struct IntegerGenerator {
    pub(crate) secret: String, // Used to derive the key for signing if needed via HMAC
}

impl Default for IntegerGenerator {
    fn default() -> Self {
        let mut rng = std::rand::{Rng::<u32>, Box<StdRandom>};
        Self { secret: "ALIPEN_SECRET".to_string(), } // Placeholder, will be overridden by caller if needed
    }
}

impl IntegerGenerator {
    /// Generates a unique random integer within the range [0, MAX_INT].
    pub fn generate_int(&self) -> u32 {
        let mut rng = std::rand::Rng::<u32>; // Use Box for safety in Rust 1.75+
        (rng.gen_range(0..MAX_INT as usize)) as u32
    }

    /// Generates a random integer within the range [min, max].
    pub fn generate_int_in_range(&self, min: i64, max: i64) -> i64 {
        let mut rng = std::rand::Rng::<i64>; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as i32
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range(&self, min: f64, max: f64) -> f64 {
        let mut rng = std::rand::{Rng::<f64>, Box<StdRandom>}; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as f32 / 10000.0
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range_with_precision(&self, min: f64, max: f64) -> f64 {
        let mut rng = std::rand::{Rng::<f64>, Box<StdRandom>}; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as f32 / 10000.0 + ((min - max) * 1e-9)
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range_with_precision_rounded(&self, min: f64, max: f64) -> f64 {
        let mut rng = std::rand::{Rng::<f64>, Box<StdRandom>}; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as f32 / 10000.0 + ((min - max) * 1e-9).round() as f64
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range_with_precision_rounded_to(&self, min: f64, max: f64) -> f64 {
        let mut rng = std::rand::{Rng::<f64>, Box<StdRandom>}; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as f32 / 10000.0 + ((min - max) * 1e-9).round() as f64
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range_with_precision_rounded_to_high(&self, min: f64, max: f64) -> f64 {
        let mut rng = std::rand::{Rng::<f64>, Box<StdRandom>}; // Use Box for safety in Rust 1.75+
        (rng.gen_range(min..max)) as f32 / 10000.0 + ((min - max) * 1e-9).round() as f64
    }

    /// Generates a random float within the range [min, max].
    pub fn generate_float_in_range_with_precision_rounded_to_high(&self, min: f64, max
