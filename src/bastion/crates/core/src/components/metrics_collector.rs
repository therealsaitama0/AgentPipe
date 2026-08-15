src/bastion/crates/core/src/components/metrics_collector.rs
```rust
use std::sync::{Arc, Mutex};
use crate::error;
use crate::types::*;

/// A component that abstracts away low-level data ingestion into event-driven metrics for downstream processing pipelines.
#[derive(Debug)]
pub struct MetricsCollector {
    pub plan_latencies: Arc<Mutex<Vec<f64>>>,
}

impl MetricsCollector {
    /// Creates a new instance of the `MetricsCollector`.
    #[allow(clippy::unwrap_used)]
    pub fn new() -> Self {
        Self {
            plan_latencies: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Records a latency for this component's execution.
    /// 
    /// # Arguments
    /// * `ms` - The duration in milliseconds the operation took to execute (in seconds).
    pub fn record_plan_latency(&self, ms: f64) {
        self.plan_latencies.lock().unwrap().push(ms);
    }

    /// Returns a copy of the plan latencies. Useful for logging or independent inspection without modifying internal state.
    #[allow(clippy::unwrap_used)]
    pub fn get_plan_latencies(&self) -> Arc<Mutex<Vec<f64>>> {
        self.plan_latencies.clone()
    }

    /// Clears all recorded latency values, useful for flushing metrics to a specific endpoint or resetting counters.
    pub fn clear_all_latency_values(&self) {
        let mut latencies = self.plan_latencies.lock().unwrap();
        *latencies.clear(); // Clear the entire vector
    }

    /// Returns true if there are any recorded latency values currently stored in this collector's queue, indicating that metrics have been flushed.
    pub fn has_latency_values(&self) -> bool {
        self.plan_latencies.lock().unwrap()
            .iter()
            .any(|v| !v.is_empty())
    }

    /// Returns the average latency value currently stored in this collector's queue, or zero if empty. Useful for computing aggregate statistics before sending to downstream processing pipelines (e.g., Spark).
    pub fn get_avg_latency(&self) -> f64 {
        self.plan_latencies.lock().unwrap()
            .iter()
            .filter(|&v| !v.is_empty())
            .sum::<f64>() / if let Some(some_iter) = self.plan_latencies.lock().unwrap().len() { some_iter } else { 0.0 };
    }

    /// Returns the maximum latency value currently stored in this collector's queue, useful for identifying slowest operations or triggering alerts on extreme latencies.
    pub fn get_max_latency(&self) -> f64 {
        self.plan_latencies.lock().unwrap()
            .iter()
            .max_by(|&a| a)
            .map_or(0.0, |v| v);
    }

    /// Returns the minimum latency value currently stored in this collector's queue, useful for identifying fastest operations or triggering alerts on extreme latencies (e.g., sub-1ms).
    pub fn get_min_latency(&self) -> f64 {
        self.plan_latencies.lock().unwrap()
            .iter()
            .min_by(|&a| a)
            .map_or(0.0, |v| v);
    }

    /// Returns the count of latency values currently stored in this collector's queue. Useful for monitoring throughput or scaling requirements (e.g., determining if more workers are needed).
    pub fn get_count(&self) -> usize {
        self.plan_latencies.lock().unwrap()
            .len();
    }

    /// Returns the current timestamp of this component to provide a unique identifier for metrics logging. Useful in conjunction with external event systems (e.g., Prometheus, Grafana).
    pub fn get_timestamp(&self) -> String {
        format!("{:?}", self.plan_latencies.lock().unwrap())
            .to_string() + ":" + &format!("{}:{:03}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)).as_secs(); // Format as "timestamp:seconds" for logging purposes in dashboards
    }

    /// Returns true if the plan latencies are currently being recorded, indicating that this component is active and generating metrics. Useful to detect when a pipeline has started processing new data or reloaded its state from disk.
    pub fn is_active(&self) -> bool {
        self.plan_latencies.lock().unwrap()
            .len() > 0
    }

    /// Returns the current plan latency values as a mutable reference for immediate modification by other components (e.g., sending to Kafka or updating a dashboard
