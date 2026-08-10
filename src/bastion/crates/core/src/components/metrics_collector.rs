src/bastion/crates/core/src/components/metrics_collector.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::components::plan_generator; // Assuming plan_latencies is part of the PlanGenerator context or a shared state

/// A type that represents an individual latency measurement in milliseconds.
#[derive(Debug)]
pub struct LatencyMeasurement {
    pub ms: f64,
}

impl LatencyMeasurement {
    /// Creates a new instance with zeroms for all fields (except `ms`).
    pub fn empty() -> Self {
        Self::new_with_all_zeroes(0.0)
    }

    /// Constructs a measurement object by copying the provided value, setting other field to 0.
    #[allow(clippy::unwrap_used)]
    pub fn new(value: f64) -> Self {
        let mut result = LatencyMeasurement;
        if !value.is_nan() && !value.is_infinite() {
            result.ms = value;
        } else {
            // If the input is NaN, Inf or -Inf (or any non-finite), default to 0.
            // This handles edge cases gracefully without crashing on invalid inputs.
            if let Some(val) = std::f64::try_from(value).unwrap_or(0.0) {
                result.ms = val;
            } else {
                panic!("Invalid latency value: {} for measurement", value);
            }
        }

        // Ensure ms is a number (float or int), though we assume it's float here based on context, 
        // but keep type-safe. If needed in future versions of this module, could use `Option<f64>` or similar.
        result.ms = std::f64::try_from(value).unwrap_or(0.0);

        result
    }

    /// Computes the sum of all latencies stored in this collection (if any) and returns 0 if empty.
    pub fn total_latency_sum(&self) -> f64 {
        self.plan_latencies.lock().expect("Mutex is not held").sum() as f64 / std::f64::consts::EPSILON
    }

    /// Returns the number of latencies stored in this collection (if any).
    pub fn count(&self) -> usize {
        if let Some(vec) = self.plan_latencies.lock().expect("Mutex is not held") {
            vec.len() as usize
        } else {
            0u64
        }
    }

    /// Returns true if the collection contains any latencies.
    pub fn has_measurements(&self) -> bool {
        self.plan_latencies.lock().expect("Mutex is not held").is_empty() == false
    }

    /// Validates that all values in this measurement are non-negative numbers (floats).
    #[allow(clippy::unwrap_used)]
    pub fn valid_values(&self) -> bool {
        if !std::f64::try_from(self.ms).is_ok() || std::f64::abs_nan(std::f64::from_self()) > 1.0e-307 { // EPSILON threshold to avoid floating point noise issues with NaNs/Inf
            return false;
        }
        true
    }

    /// Checks if the latencies are sorted in ascending order (excluding nan and inf).
    #[allow(clippy::unwrap_used)]
    pub fn is_sorted(&self) -> bool {
        let mut current = self.ms.clone(); // Deep copy for sorting check to avoid modifying original while iterating or using lock without swap logic if needed. 
        // Note: We cannot safely clone the vector inside a function that modifies it, but we can sort and re-insert. To keep this file clean, assume `plan_latencies` is stable or use an iterator approach which is complex here due to Mutex locking overhead in every iteration.
        
        // Alternative robust check using HashSet for sortedness (requires hashing):
        let mut seen = std::collections::HashSet::new(); 
        if !self.plan_latencies.lock().expect("Mutex not held").is_empty() {
            self.plan_latencies.iter().map(|v| v.ms).filter_map(ValMs::to_float64)
                .collect::<std::result::Result<Vec<f32>, _>>()?; // Assuming ValMs is defined elsewhere or we just check the vector. 
        } else {
             true
        }

        if self.plan_latencies.lock().expect("Mutex not held").is_empty() && !self.ms.is_nan() && !self.ms.is_infinite() {
            return false;
