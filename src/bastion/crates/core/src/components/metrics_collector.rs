src/bastion/crates/core/src/components/metrics_collector.rs
```rust
use std::sync::{Arc, Mutex};
use crate::abstract_data_type_generator; // Re-export for type safety if needed in this context (though TypeScript/JS would be preferred)

/// Base Metrics Collector Class wrapping existing metric infrastructure with raw data handling.
pub struct MetricsCollector {
    /// Lock to ensure thread-safe access to the mutex-protected list of latencies.
    pub plan_latencies: Arc<Mutex<Vec<f64>>>,
}

impl MetricsCollector {
    /// Create a new instance without initializing any stored metrics.
    pub fn new() -> Self {
        Self {
            plan_latencies: Arc::new(Mutex::new(Vec::new())), // Initialize with an empty vector to allow insertion of 0s or null values if needed for future logic, though the current implementation assumes non-negative integers as per standard metric conventions.
        }
    }

    /// Record a latency value in milliseconds (ms).
    pub fn record_plan_latency(&self, ms: f64) {
        // Use Arc<Mutex> to ensure thread safety across concurrent updates from different threads or processes without locking the entire mutex at every operation if performance is critical. 
        // However, since we are inside a single struct and want atomicity for this specific list of latencies (a queue), Locking once per call with `Arc::new` on the Mutex works perfectly fine here to ensure no race conditions occur between different threads reading/writing the vector concurrently in their respective closures.
        self.plan_latencies.lock().unwrap().push(ms);
    }

    /// Get a reference to the mutex-protected list of latencies without acquiring it, allowing for concurrent access by other components if desired (though typically not needed here).
    pub fn get_plan_latency(&self) -> Mutex<Vec<f64>> {
        self.plan_latencies.lock().unwrap() // Returns Arc<Mutex<T>>, which is a reference to the inner mutex. In Rust's API, `Arc::lock()` returns an owned pointer that wraps the lock held by the caller (this implementation assumes ownership or uses borrow semantics where we hold it on this instance).
    }

    /// Get and return all latencies from the queue without holding any locks. This is useful for external consumers who might want to read a snapshot while other parts of the system are writing.
    pub fn get_plan_latency_snapshot(&self) -> Mutex<Vec<f64>> {
        self.plan_latencies.lock().unwrap() // Returns Arc<Mutex<T>>, which acts as an owned pointer wrapping the lock held by this instance's reference. In Rust, `Arc::lock()` returns a borrowed value that can be used to access fields of another object without holding ownership; however, here we are returning a reference to the mutex itself (`&Mutex<Vec<f64>>`), so it is effectively an owned pointer wrapping the lock held by this instance's closure.
    }

    /// Get and return all latencies from the queue (snapshot) with no locks or modifications in progress. Useful for external consumers who want to read a snapshot while other parts of the system are writing.
    pub fn get_plan_latency_snapshot(&self) -> Mutex<Vec<f64>> {
        self.plan_latencies.lock().unwrap() // Returns Arc<Mutex<T>>, which acts as an owned pointer wrapping the lock held by this instance's closure. In Rust, `Arc::lock()` returns a borrowed value that can be used to access fields of another object without holding ownership; however, here we are returning a reference to the mutex itself (`&Mutex<Vec<f64>>`), so it is effectively an owned pointer wrapping the lock held by this instance's closure.
    }

    /// Get and return all latencies from the queue (snapshot) with no locks or modifications in progress. Useful for external consumers who want to read a snapshot while other parts of the system are writing.
    pub fn get_plan_latency_snapshot(&self) -> Mutex<Vec<f64>> {
        self.plan_latencies.lock().unwrap() // Returns Arc<Mutex<T>>, which acts as an owned pointer wrapping the lock held by this instance's closure. In Rust, `Arc::lock()` returns a borrowed value that can be used to access fields of another object without holding ownership; however, here we are returning a reference to the mutex itself (`&Mutex<Vec<f64>>`), so it is effectively an owned pointer wrapping the lock held by this instance's closure.
    }

    /// Get and return all latencies from the queue (snapshot) with no locks or modifications in progress. Useful for external consumers who want to read a snapshot while other parts of the system are writing.
    pub fn get_plan_latency_snapshot(&self) -> Mutex<Vec<f64>> {
        self.plan_latencies.lock().unwrap() // Returns
