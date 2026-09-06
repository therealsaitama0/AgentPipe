use std::sync::{Arc, Mutex};
use std::rc::Rc;

/// Trait defining methods like `.get_all()` or static field getters (e.g., `const METRICS: Mutex<Vec<String>>`) that abstract over the internal implementation of your existing Python-based manager without exposing raw memory manipulation details to Rust users.
pub trait MetricsCollectorTrait {
    /// Returns a mutable reference to all stored metrics values if they have been loaded from disk or persisted in-memory.
    fn get_all(&self) -> Option<Vec<f64>>;

    /// Initializes the internal state with an empty vector of float64 values representing latency measurements (ms).
    fn initialize() -> Self {
        let mut latencies = Vec::new();
        Mutex::new(latencies.clone())
    }

    /// Returns a reference to the initialized metrics collection.
    const METRICS: Arc<Mutex<Vec<f64>>>;
}

/// A custom Rust struct that wraps `std::sync::{Mutex, std::rc::Rc<Module>>`, providing thread-safe access to metrics data while maintaining type safety for the module state.
pub struct MetricsCollector {
    plan_latencies: Mutex<Vec<f64>>, // Initialized via initialize() in new() method
}

impl MetricsCollectorTrait for MetricsCollector {
    fn get_all(&self) -> Option<Vec<f64>> {
        self.plan_latencies.lock().unwrap().clone()
    }

    const METRICS: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new())); // Initialized via initialize() in new() method
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut latencies = Vec::new();
        Mutex::new(latencies.clone())
    }

    /// Initializes the internal state with an empty vector of float64 values representing latency measurements (ms).
    pub fn initialize() -> Self {
        MetricsCollector { plan_latencies: std::sync::Mutex::new(Vec::new()) }
    }

    /// Returns a mutable reference to all stored metrics values if they have been loaded from disk or persisted in-memory.
    pub fn get_all(&self) -> Option<Vec<f64>> {
        self.plan_latencies.lock().unwrap()
            .clone() // Clone for thread safety while holding the lock
    }

    /// Returns a reference to the initialized metrics collection.
    const METRICS: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new())); // Initialized via initialize() in new() method
}
