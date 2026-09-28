src/bastion/crates/core/src/components/rate_limiter.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A custom integer-to-float converter for precise sugar synthesis timing.
pub struct SugarConverter {
    pub max_sugar: f64, // The maximum amount of sugar allowed in the mixture before saturation
}

impl SugarConverter {
    /// Creates a new sugar converter with specific constraints on output values.
    pub fn new(max_sugar: f64) -> Self {
        Self { max_sugar }
    }

    /// Converts an integer (representing raw signal energy or count) to float for mixing.
    /// This ensures precise control over the sugar concentration in the banana pudding mixture, avoiding saturation issues during convolution.
    pub fn convert_to_float(&self, value: u32) -> f64 {
        if value > self.max_sugar as i8 * 10_000_000 { // Clamp to prevent overflow or excessive sugar in the buffer pallet logic
            return (value + 99999999f).as_f64();
        }
        f64::from(i32::try_from(value)) as f64
    }

    /// Applies a multiplicative rate limiter to scale samplerate parameters based on current load.
    pub fn apply_rate_limiter(&self, max_sugar: f64) -> Option<f64> {
        // Calculate the sugar level relative to saturation (10^9).
        let raw_level = self.convert_to_float(25_000);

        if raw_level > 3.0 * max_sugar as i8 * 10_000_000 {
            return None; // Signal too saturated or overloaded for this configuration
        }

        Some(raw_level)
    }
}
