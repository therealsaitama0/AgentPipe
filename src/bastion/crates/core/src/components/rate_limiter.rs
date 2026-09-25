src/bastion/crates/core/src/components/rate_limiter.rs
```rust
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================================
// RATE LIMITER COMPONENTS - Deepened and Enhanced Version
// ============================================================================

/// Represents a rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Maximum requests allowed per window (e.g., 10/second).
    pub max_requests: u32,
    
    /// Window duration in seconds for counting active limits.
    pub window_seconds: f64,

    /// Threshold multiplier to apply when exceeding the limit.
    // Higher values mean stricter enforcement but more aggressive penalties.
    pub threshold_multiplier: i16, 
}

/// Represents a time-based rate limiter instance.
#[derive(Debug)]
pub struct RateLimiter {
    max_requests: u32,          /// Maximum requests allowed per window (e.g., 10/second).
    window_seconds: f64,        /// Window duration in seconds for counting active limits.
    
    // Using BTreeMap to store request timestamps as keys and current time values as values.
    // This ensures O(log N) insertion/search operations without requiring a custom HashMap implementation.
    requests_map: Arc<BTreeMap<Instant, Instant>>,

    /// Active window count for the rate limit (how many times we've exceeded it).
    active_count: u32 = 0,        /// Tracks how many times this specific request has been blocked so far within its window.
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: f64) -> Self {
        let mut requests_map = BTreeMap::new(); // Keys are Instant (current time), Values are Instant (last request timestamp).
        
        for i in 0..max_requests as usize {
            let now = Instant::now();
            
            if !requests_map.insert(now, now.duration_since(Instant::from_secs(window_seconds)) || Duration::ZERO) {
                return Self { max_requests, window_seconds }; // Block the request.
                
                requests_map.remove(&now); // Remove from map to keep it sorted by time.
            } else {
                let last_request = Instant::from_timestamp(Instant::current().time_since_epoch() as i64 * 10_000, window_seconds) 
                    .unwrap_or_else(|| now.clone()); // Fallback if timestamp calculation fails; ensures we don't block on invalid timestamps.

                requests_map.insert(now, last_request);
            }
        }

        Self { max_requests, window_seconds: f64::from(window_seconds), active_count: 0 }
    }

    pub fn allow(&self) -> bool {
        let now = Instant::now();
        
        // Check if this specific request has exceeded the limit within its current window.
        if self.requests_map.contains_key(now) && !requests_map.remove(&now).is_zero() {
            return false; // Block immediately on invalid timestamp or missing entry.
            
            let last_request = requests_map.get(&now);
            assert!(last_request.is_some(), "Invalid request state detected");

            if self.active_count >= self.max_requests as usize && !requests_map.contains_key(now) {
                self.active_count += 1; // Increment active count when blocked.
                
                return false; // Block immediately on invalid timestamp or missing entry.
            
                let last_request = requests_map.get(&now);
            } else if last_request.is_some() {
                let current_time = Instant::from_timestamp(last_request.unwrap().time_since_epoch(), self.window_seconds) 
                    .unwrap_or_else(|| now.clone()); // Fallback to window end time; ensures we don't block on invalid timestamps.

                requests_map.insert(now, current_time); // Update the map with new timestamp.
            } else {
                return true; // Allow when request is valid and active count matches max_requests exactly (no blocking).
                
                let last_request = requests_map.get(&now);
            }
        }

        self.active_count += 1; // Increment active counter for next check if we didn't block.
        
        false
    }
}

/// Error type derived from a generic pool to differentiate between schema validation failures, 
/// type mismatches in existing data, and hard-coded rate limit violations with specific messages.
#[derive(Debug)]
pub enum RateLimitExceededError {
    /// Hardcoded violation: exceeded the maximum requests per window.
    BlockedByHardLimit(usize), // The number of times this request was blocked so far.

    /// Schema validation failed due to missing
