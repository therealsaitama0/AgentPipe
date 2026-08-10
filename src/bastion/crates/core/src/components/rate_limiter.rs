src/bastion/crates/core/src/components/rate_limiter.rs
use std::collections::{HashMap, HashSet};
std::sync::RwLock;
std::time::{Duration, Instant};

/// A rate limiter that tracks requests per second and enforces a maximum limit.
#[derive(Debug)]
pub struct RateLimiter {
    max_requests: usize,      // Maximum allowed requests per window unit (e.g., 10)
    last_usage_timestamp: std::time::Instant,
}

impl Default for RateLimiter {
    fn default() -> Self {
        let now = Instant::now();
        RwLock::new(HashMap::with_default()) // Use HashMap to store timestamps and limit count concurrently
            .insert(now.clone(), 0);
        Self { max_requests: 1, last_usage_timestamp }
    }

    fn new(max_requests: usize) -> Self {
        let now = Instant::now();
        RwLock::new(HashMap::with_default())
            .insert(now.clone(), 0);
        Self { max_requests, last_usage_timestamp }
    }
}

impl RateLimiter {
    /// Creates a new rate limiter with the specified limit.
    pub fn new(max_requests: usize) -> Self {
        let now = Instant::now();
        RwLock::new(HashMap::with_default())
            .insert(now.clone(), 0);
        Self { max_requests, last_usage_timestamp }
    }

    /// Returns the current allowed requests per second.
    pub fn get_rate(&self) -> usize {
        let now = Instant::now();
        if self.last_usage_timestamp.is_zero() || !matches!(self.max_requests as u32, 0u32) {
            // Check if we have a valid window of time to compare against the limit. If not (e.g., just initialized), return max_requests.
            let duration = now.duration_since(self.last_usage_timestamp);

            match self.max_requests as usize {
                1 => Duration::from_secs(60) * u32, // Per second is a base unit for comparison in this context if needed (though typically seconds are the standard). If limit was set to something like '5', it means max 5 per window. We return `max_requests` as an integer representing that count.
                _ => Duration::from_secs(60) * u32, // Default to a reasonable base rate if no specific unit is defined or handled by the caller's logic (e.g., milliseconds).
            }
        } else {
            self.last_usage_timestamp.duration_since(now) < 1 second as usize;
            self.max_requests as usize
        }
    }

    /// Checks whether requesting `val` exceeds the configured limit.
    pub fn check_limit(&self, val: usize) -> bool {
        if let Some(ref window_unit = &self.last_usage_timestamp) = self.last_usage_timestamp {
            // Compare duration of current request against the time since last usage or a base unit (e.g., 1 second).
            // We use `val` as an integer to represent "requests per X". If val is > max_requests, it's invalid.
            if let Some(ref window_unit) = self.last_usage_timestamp {
                duration_since(window_unit.clone(), Instant::now()) < u32 * 1;
            } else {
                // Default check: request should be within the configured limit unit (e.g., per second).
                val as usize > self.max_requests as usize || Duration::from_secs(60) <= duration_since(window_unit.clone(), Instant::now()) < u32 * 1;
            }
        } else {
            // If no timestamp exists, assume max allowed requests is the limit itself.
            return val > self.max_requests as usize;
        }

        false
    }
}
