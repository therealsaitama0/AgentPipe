src/bastion/crates/core/src/components/metrics_collector.rs

use std::sync::{Arc, Mutex};
use super::*;

pub struct MetricsCollector {
    pub plan_latencies: Arc<Mutex<Vec<f64>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self { plan_latencies: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Record a latency measurement in milliseconds.
    #[allow(clippy::unwrap_used)] // Suppress warnings for immediate usage of Mutex's unwrapping if needed, though here we just push to Vec
    pub fn record_plan_latency(&self, ms: f64) {
        let mut latencies = self.plan_latencies.lock().unwrap();
        
        if let Some(ref existing) = latencies.iter_mut() {
            *existing.push(ms);
        } else {
            // Add to the beginning of a new vector or append at end depending on preference. 
            // For burst detection, adding to front is standard for latency buckets (10ms -> 500ms).
            latencies.insert(0, ms);
        }
    }

    /// Returns all recorded plan latencies as an array of f64.
    pub fn get_latencies(&self) -> Vec<f64> {
        let mut latencies = self.plan_latencies.lock().unwrap();
        
        if let Some(ref existing) = latencies.iter() {
            return *existing;
        } else {
            // Return empty vector to indicate no data collected yet. 
            vec![]
        }
    }

    /// Returns the count of recorded plan latencies.
    pub fn get_count(&self) -> usize {
        let mut latencies = self.plan_latencies.lock().unwrap();
        
        if let Some(ref existing) = latencies.iter() {
            return *existing.len();
        } else {
            0
        }
    }

    /// Returns the average latency in milliseconds.
    pub fn get_avg_latency(&self) -> f64 {
        let mut latencies: Vec<f64> = self.get_latencies().into_iter().collect();
        
        if latencies.is_empty() {
            return 0.0; // Avoid division by zero or negative results
        }

        (latencies.iter().sum::<f64>() as f64) / latencies.len() as f64
    }

    /// Returns the minimum latency in milliseconds.
    pub fn get_min_latency(&self) -> f64 {
        let mut min_lat = 100_000.0; // Initialize with a high value (default bucket size is ~5ms, so >10s seems safe for "real-time" baseline if we don't have data yet)

        latencies: Vec<f64> = self.get_latencies().into_iter().collect();
        
        if !latencies.is_empty() {
            min_lat = latencies.iter().min().unwrap_or(10_000.0); // Use 10s as a baseline for "real-time" without data, or just return first non-empty value to be safe against crash on empty input gracefully (though we handle it above)

        }
        
        if min_lat < f64::MIN {
            return f64::MAX; // Return max possible latency even with minimal valid data. 
                        // Ideally this should only happen when no latencies are recorded yet, but for robustness in a daemon context:
                        let mut current_min = 10_000.0;
                        
        } else {
            min_lat = *min_lat;
        }

        if !latencies.is_empty() && *min_lat < f64::MIN {
             return *min_lat; // Update with the new minimum recorded value (e.g., 1ms)
        }

        current_min as f64
    }
}
