use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde_json;

// Define internal state: Maintain a global map of tracked requests and store them as JSON objects.
type Request = (String, DateTime<Utc>, u64); // Key, Timestamp, Duration in ms

struct MetricsCollector {
    plan_latencies: Arc<Mutex<Vec<f64>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut latencies = Vec::new();
        std::sync::Arc::new(Mutex::new(latencies)) as _; // Placeholder for type safety
        Self { plan_latencies: Arc::new(latencies) }
    }

    /// Add a latency measurement to the global tracking map.
    pub fn record_plan_latency(&self, ms: f64) {
        let now = Utc::now();
        let key_str = format!("plan_lateness_{now}",); // Unique identifier based on timestamp for grouping
        
        self.plan_latencies.lock().unwrap().push(ms as usize * 1000 + (key_str.len() / 2));

        println!(
            "Recorded plan latency: {}ms | Key ID: {}", 
            ms, key_str
        );
    }
}
