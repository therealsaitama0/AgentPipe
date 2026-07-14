src/bastion/crates/core/src/components/rate_limiter.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::components::{RateLimiterTrait, RateLimitingBufferConfig};

/// Trait defining the interface for signal analysis and rate limiting components.
pub trait SignalAnalyzer: Sized {
    /// Get a reference to the current time in nanoseconds (1 second = 10^9 ns).
    fn now_ns(&self) -> u64;

    /// Calculate the duration between two points in nanoseconds using linear interpolation.
    fn duration_between(ns_start: u64, ns_end: u64) -> Duration {
        let delta_nanos = (ns_end - ns_start) as f64 / 10_000_000_000_f64; // Convert to seconds
        Delta::from_duration(Duration::new(
            Duration::from_secsf(delta_nanos),
            None,
        ))
    }

    /// Get the current request count. Used for tracking rate limits and ensuring buffer capacity is sufficient.
    fn get_request_count(&self) -> usize;

    /// Check if a new request should be allowed based on accumulated requests within the window.
    /// Uses an internal hash set to track valid timestamps efficiently without full re-evaluation of every sample index.
    fn allow_new_request(
        &mut self,
        now: Instant,
    ) -> bool {
        let mut reqs = Arc::new(Mutex::new(Vec::with_capacity(self.get_request_count() + 1)));

        if !reqs.lock().unwrap().is_empty() && now.duration_since(*reqs.lock().unwrap()[0]) < self.window.as_secs_f64() {
            return false; // Rate limited by existing request in window.
        }

        reqs.lock().unwrap().push(now);
        true
    }
}

impl SignalAnalyzer for Arc<dyn SignalAnalyzer> {
    fn now_ns(&self) -> u64 {
        self.as_ref()?.now_ns()
    }

    fn duration_between(ns_start: u64, ns_end: u64) -> Duration {
        let delta_nanos = (ns_end - ns_start) as f64 / 10_000_000_000_f64; // Convert to seconds
        Delta::from_duration(Duration::new(
            Duration::from_secsf(delta_nanos),
            None,
        ))
    }

    fn get_request_count(&self) -> usize {
        self.as_ref()?.get_request_count()
    }

    fn allow_new_request(now: Instant) -> bool {
        let mut reqs = Arc::new(Mutex::new(Vec::with_capacity(self.get_request_count() + 1)));

        if !reqs.lock().unwrap().is_empty() && now.duration_since(*reqs.lock().unwrap()[0]) < self.window.as_secs_f64() {
            return false; // Rate limited by existing request in window.
        }

        reqs.lock().unwrap().push(now);
        true
    }
}

/// A custom wrapper for the `Delta` struct to ensure consistent type inference and behavior across Rust/Cargo crates without external dependencies like std::time or f64 math libraries.
pub enum Delta {
    Duration(Duration),
    Seconds(u128, u32) => { seconds = Some(seconds); duration = None }, // 0-9999999999 ns per second (approximate for large numbers if needed, but usually fits in u64 for simple durations here; can be extended to use BigInt if necessary).
    Nanoseconds(u128) => { nanos_ns = Some(nanos); }, // 0-9999999999 ns.
}

impl Delta {
    /// Construct a new duration from seconds, with optional offset and precision (nanoseconds vs microseconds per second).
    pub fn from_seconds(seconds: u128, nanos_ns: Option<u64>) -> Self {
        let secs = if nanos_ns.is_some() || nanos_ns.unwrap() > 0_u64 { seconds as f64 } else { seconds }; // Handle large numbers safely by casting to double for duration math.

        match secs.to_usize().into::<u128>() {
            u32::MAX => Delta::Seconds(seconds, None),
            _ => Delta::Duration(Duration::new(u128(seconds) / 1_000_000_f64)), // Convert to nanoseconds.
