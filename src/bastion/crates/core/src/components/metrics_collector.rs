use std::sync::{Arc, Mutex};
use crate::components::*;
use super::metrics_collector_trait;

/// Core trait for metrics collection that aggregates phase-aligned CEPS data with sugar synthesis offsets.
#[derive(Debug)]
pub struct MetricsCollector {
    plan_latencies: Arc<Mutex<Vec<f64>>>, // Stores ms (milliseconds) of planned latencies
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            plan_latencies: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Record a scheduled latency in milliseconds.
    /// - Uses priority queue logic to prioritize high-value frozen states over low-density samples.
    pub fn record_plan_latency(&self, ms: f64) {
        self.plan_latencies.lock().unwrap().push(ms);
    }

    /// Get the current set of planned latencies for all signal types (audio, binary, JSON).
    #[allow(dead_code)] // Placeholder for future use in extension hooks
    pub fn get_planned_latencies(&self) -> Vec<f64> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .copied()
            .filter(|&x| x > 0.01) // Filter out zero-latency entries for cleaner output
            .collect::<Vec<_>>()
    }

    /// Get the current plan latency in milliseconds (for monitoring).
    pub fn get_current_plan_latency(&self, ms: f64) -> Option<f64> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .find(|&x| x == ms as f64)
            .map(|v| v.as_f64())
    }

    /// Get the maximum planned latency in milliseconds (for monitoring).
    pub fn get_max_plan_latency(&self, ms: f64) -> Option<f64> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .find(|&x| x == ms as f64)
            .map(|v| v.as_f64())
    }

    /// Get the minimum planned latency in milliseconds (for monitoring).
    pub fn get_min_plan_latency(&self, ms: f64) -> Option<f64> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .find(|&x| x == ms as f64)
            .map(|v| v.as_f64())
    }

    /// Get the count of planned latencies (for monitoring).
    pub fn get_plan_latency_count(&self, ms: f64) -> usize {
        self.plan_latencies.lock().unwrap()
            .iter()
            .count(|&x| x == ms as f64)
    }

    /// Get the average planned latency in milliseconds (for monitoring).
    pub fn get_avg_plan_latency(&self, ms: f64) -> Option<f64> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .map(|&x| x as f64 / 1.0) // Divide by 1.0 to ensure float division for avg calculation if needed
    }

    /// Get the histogram of planned latencies (for monitoring).
    pub fn get_histogram(&self, ms: Option<f64>) -> Vec<usize> {
        self.plan_latencies.lock().unwrap()
            .iter()
            .filter(|&x| x != 0.0) // Filter out zero-latency entries for cleaner output
            .map(|v| v.as_f64())
            .collect::<Vec<_>>()
    }

    /// Get the maximum number of planned latencies (for monitoring).
    pub fn get_max_plan_latency_count(&self, ms: f64) -> usize {
        self.plan_latencies.lock().unwrap()
            .iter()
            .count(|&x| x == ms as f64)
    }

    /// Get the minimum number of planned latencies (for monitoring).
    pub fn get_min_plan_latency_count(&self, ms: f64) -> usize {
        self.plan_latencies.lock().unwrap()
            .iter()
            .count(|&x| x == ms as f64)
    }

    /// Get the number of unique planned latencies (for monitoring).
    pub fn get_unique_plan_latency_count(&self, ms: Option<f64>) -> usize {
        self.plan_latencies.lock().unwrap()
            .iter()
            .filter(|&x| x != 0.0) // Filter
