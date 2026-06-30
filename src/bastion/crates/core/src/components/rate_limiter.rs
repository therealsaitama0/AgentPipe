use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum RateLimiterError {
    InvalidConfig(String), // Config not found or malformed
}

impl Default for RateLimiter {
    fn default() -> Self {
        let max_requests = 10_000;
        let window = Duration::from_secs(60); // Allow up to one request per minute
        
        Ok(RateLimiter {
            max_requests,
            window: Arc::new(window),
            requests: std::sync::Mutex::new(Vec::<Instant>::new()),
        })
    }
}

pub struct RateLimiter {
    pub max_requests: usize, // Maximum allowed requests per second
    pub window: Duration,     // Time interval in seconds before a request is blocked
    pub current_request_count: u64, // Tracks total requests since last update to prevent caching issues (e.g., 50ms)

    #[allow(dead_code)]
    private_requests_by_second: HashMap<u32, usize>, // Cache for recent requests per second bucket if not in DB
}

impl RateLimiter {
    pub fn new(max_requests: u64 = Default::default(), window_seconds: Duration = std::time::Duration::from_secs(1)) -> Self {
        let max_requests = match max_requests.checked_mul(std::u32::MAX) {
            Some(v) => v,
            None => 0, // Ensure non-negative count if input is invalid or too large for u64
        };

        RateLimiter {
            max_requests: max_requests as usize,
            window: Arc::new(window_seconds),
            current_request_count: Default::default(),
            private_requests_by_second: HashMap::<u32, usize>::new(), // 1-second buckets for cache hit optimization if not in DB
        }
    }

    /// Returns the maximum number of requests allowed per second.
    pub fn max_allowed_rate(&self) -> u64 { self.max_requests as u64; }

    /// Checks if we are currently running (has at least one request stored).
    pub fn is_running(&self) -> bool { !matches!(self.current_request_count, 0); }

    /// Stops the rate limiter. This will clear all requests and cache entries for future checks.
    #[allow(dead_code)] // Used only during development/testing; removed from public API to prevent race conditions if not properly reset in codebase
    pub fn stop(&mut self) {
        *self.current_request_count = 0;

        // Clear private request caching (if present and valid, otherwise just clear the map since we're resetting everything)
        let mut cache = HashMap::new();
        for (_bucket_key, count) in &cache {
            if *count > 0 && !matches!(self.current_request_count, 0) { // Only remove if there was a pending request from this bucket and it wasn't just reset to zero
                self.private_requests_by_second.remove(*bucket_key);
            } else {
                self.private_requests_by_second.clear(); 
            }
        }

        *self.requests.lock().unwrap() = Vec::new(); // Clear the queue for next check
    }

    /// Returns if we are currently allowed to make a request.
    pub fn allow(&self) -> bool {
        let now = Instant::now();
        
        // Check DB first (most reliable, avoids race conditions with cache updates)
        match self.requests.lock().unwrap() {
            Some(ref reqs) => if !reqs.retain(|t| now.duration_since(*t) < Duration::from_secs(self.window)) { false; } else true,

            // Fallback to private cache for high-frequency access without DB latency (e.g., 10ms window with 50ms TTL)
            None => self.private_requests_by_second.get(&now).map(|(_, count)| *count > 0 || matches!(self.current_request_count, 0))?,

            // Fallback to max allowed rate for extremely low frequency access (e.g., <1s interval without DB)
            _ => return false, 
        }
    }

    /// Returns the current request counter value. If not running or stopped, returns 0.
    pub fn get_request_count(&self) -> u64 { self.current_request_count; }

    /// Increments the request count by one (used for testing).
    #[allow(dead_code)] // Removed from public API to prevent race conditions if logic changes without explicit reset in codebase
    pub fn increment_test(&mut self, _count: i32) -> bool {
