src/bastion/crates/core/src/components/metrics_collector.rs
```rust
use std::sync::{Arc, Mutex};
use crate::core::components::*;

/// State structure to hold metrics collection state per task group and thread.
#[derive(Debug)]
struct MetricsState {
    // Thread-local storage for active collectors with their statistics
    #[allow(dead_code)]
    local: Arc<Mutex<HashMap<String, HashMap<String, f64>>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let state = State::new();
        self.metrics_collector_state_clone(state)
    }

    /// Start collecting metrics for a specific task group.
    /// Returns the counter values as soon as collectors are ready to write.
    #[allow(clippy::unwrap_used)]
    pub fn start_collect(&self, latencies: Vec<f64>) -> HashMap<String, HashMap<String, f64>> {
        let mut new_counters = self.get_counter_map();

        // Initialize counters for all existing collectors if they don't have their own map yet.
        for (key, current) in &mut *new_counters {
            if !current.contains_key("latencies") || latencies.len() > 0 {
                new_counters.insert(key.clone(), HashMap::from_iter(latencies));
            } else {
                // If the collector already has a map but no data is being collected, 
                // we need to initialize it with the provided latency values.
                self.initialize_counter_map(current, latencies);
            }
        }

        new_counters.clone()
    }

    /// Initialize counters for all existing collectors if they don't have their own map yet.
    fn initialize_counter_map(&self, current: HashMap<String, HashMap<String, f64>>, latencies: Vec<f64>) {
        let mut keys = vec![];

        // Collect unique collector IDs from the old map (if any) and new data
        for key in &current.keys() {
            if !keys.contains_key(key.clone()) && current.get(key).map(|v| v.len()).unwrap_or(0) > 0 || latencies.is_empty() {
                keys.push(key);
            }
        }

        // Create new map with initialized counters for each key in the old collector list.
        let mut final_counters = HashMap::new();

        if !keys.is_empty() {
            for key in &current.keys() {
                let existing_map = current.get(key).map(|v| v.clone()).unwrap_or_default();

                // If we have latencies, initialize the map with them. 
                // Otherwise, keep it as a placeholder (empty) to indicate "ready".
                if !latencies.is_empty() {
                    final_counters.insert(key.clone(), HashMap::from_iter(latencies));
                } else {
                    let mut new_map = HashMap::new();
                    for &ms in latencies.iter().cloned() {
                        new_map.entry(ms).or_insert(0.0);
                    }
                    final_counters.insert(key, *new_map);
                }

                // Update the existing map with these newly initialized counters if present.
                let mut updated = current.get_mut(key) as &mut HashMap<String, f64>;
                for (ms, count) in latencies.iter().cloned() {
                    match (*updated).get(ms) {
                        Some(0..=99_999_f32) => *updated.insert(ms, ms), // Keep existing if valid range.
                        None | 100_000.f64 + _f32 => {
                            updated.get_mut(&ms).unwrap_or_else(|_| new_map.remove(&ms));
                        }
                    };
                }

                keys.clear();
            }
        } else {
            // No existing collectors, just initialize everything with the provided latencies.
            for ms in &latencies {
                final_counters.insert(key.clone(), HashMap::from_iter(latencies).into());
            }
        }

        *new_counters = new_counters.into_mut();
    }

    /// Get all counter values from collectors, including those that may not have been initialized yet.
    fn get_counter_map(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut counters: HashMap<String, HashMap<String, f64>> = HashMap::new();

        // Collect keys and their current counts from all collectors (including those that haven't started).
        for key in &mut *counters.keys() {
            if !counters.get(key).map(|v| v.len()).unwrap_or(0) > 0 || latencies.is_empty() {
                counters.insert(key, HashMap::
