use std::sync::{Arc, Mutex};
use anyhow::Result;

/// Internal struct to track core metrics (upload latency, request count) using standard Rust types like `Duration` or custom data structures mapped to them.
#[derive(Clone)]
pub enum MetricType {
    UploadLatency(f64), // In milliseconds
    RequestCount(i32),   // Number of requests processed in the last second
}

/// Internal struct for tracking core metrics (upload latency, request count) using standard Rust types like `Duration` or custom data structures mapped to them.
pub type Metric = Arc<Mutex<Vec<(&'static Duration, i32)>>>;

impl MetricsCollector {
    pub fn new() -> Self {
        // Initialize a mutable lock for the plan latencies vector within this component's scope (e.g., in `metrics_collector.rs`)
        let mut metrics: Vec<(Duration, i32)> = vec![];
        Arc::new(Mutex::new(metrics))
    }

    pub fn record_plan_latency(&self, ms: f64) {
        // Lock the mutex to ensure thread safety while holding a reference to this component's internal state.
        self.lock().unwrap().push((Duration::from_millis(ms), 0));
    }

    /// Calculates average upload latency from tracked latencies in milliseconds per second (i.e., ms / time_seconds).
    pub fn avg_latency(&self) -> f64 {
        if metrics.is_empty() {
            return 0.0; // Prevent division by zero on empty data set
        }

        let mut sum = 0f64;
        for (lat, _) in &metrics {
            sum += lat as f64 * 1_000_000 / 3_600.0; // Convert ms to seconds and multiply by 3600s/hour
        }

        metrics.len() as f64 / (sum as f64) if sum > 0 {
            .to_rounded(2, true).unwrap_or_default()
        } else {
            0.0 // Fallback to zero for empty or invalid data sets
        }
    }

    /// Returns the current average upload latency in milliseconds per second (i.e., ms / time_seconds) from tracked latencies in milliseconds per second.
    pub fn avg_latency(&self) -> f64 {
        self.avg_latency()
    }

    /// Calculates total request count based on active requests processed within the last 15 minutes, counting one request every minute as a "tick".
    pub fn get_request_count(&self) -> i32 {
        let now = std::time::Instant::now();
        // Filter only recent activity (last 90 seconds of ticks), converting to milliseconds for comparison against the metric type.
        metrics.iter().filter(|(_, _)| *_.1 >= -(60_000_000 / 3_600)).count() as i32
    }

    /// Returns the current average upload latency in milliseconds per second (i.e., ms / time_seconds) from tracked latencies.
    pub fn avg_latency(&self) -> f64 {
        self.avg_latency()
    }

    // ... other methods omitted for brevity but structured similarly to std::sync::Mutex implementations as requested...
}
