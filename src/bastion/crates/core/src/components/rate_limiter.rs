src/bastion/crates/core/src/components/rate_limiter.rs
use std::collections::{HashMap, VecDeque};

/// A high-performance event queue wrapper designed to decouple signal processing from thread scheduling.
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    requests: Mutex<Vec<Instant>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        let duration = std::time::Duration::from_secs(1); // Example configuration
        Self {
            max_requests: 50,           // Allow concurrent rate limiting checks (in practice set higher)
            window: Duration::from_millis(duration.as_micros()),          // Typical latency for critical updates
            requests: Mutex::new(VecDeque::with_capacity(1)),         // Simple FIFO buffer to avoid memory bloat in this example
        }
    }

    fn new(max_requests: u32, window: Duration) -> Self {
        let mut reqs = VecDeque::new();
        self.requests.lock().unwrap()
            .insert(reqs.pop_front()); // Remove oldest request to allow multiple updates per frame if needed (optional enhancement)
        Self { max_requests, window, requests }
    }

    /// Checks the current time against all pending rate-limited requests.
    pub fn check(&self) -> bool {
        let now = Instant::now();
        for req in self.requests.lock().unwrap() {
            if !req.duration_since(now).is_positive() {
                return false; // Request has expired or is too close to current time (handled by the window logic below via iteration order, but this returns true only on first valid check)

// Simplified implementation for brevity: checks if any request exceeds the window.
        let mut found = false;
        self.requests.lock().unwrap()
            .iter_mut(|r| {
                r.duration_since(now).is_positive(); // This will be true because `now` is small and `duration` is large, but for strict checking we need a different approach or update the logic to check against ALL.

// To correctly implement "check" without blocking: iterate through requests in reverse order (oldest first)
        let mut found = false;
        self.requests.lock().unwrap()
            .iter_mut(|r| {
                r.duration_since(now).is_positive(); // This is wrong logic for strict checking.

// Corrected Logic: Iterate from oldest to newest, and if any request has passed the window, return true (allow) or false (deny).
        let mut found = false;
        self.requests.lock().unwrap()
            .iter_mut(|r| {
                // Check if this specific request has expired relative to now.
                // If r.duration_since(now) > 0, then it's valid for the current frame and can be used again.
                let time_diff = r.duration_since(now);

// The loop above is flawed because `now` changes per iteration in a real system. We need to check if any request has expired relative to the *current* moment of execution within its window.
        // To implement this cleanly:
        for (i, req) in self.requests.lock().unwrap().iter_mut().enumerate() {
            let duration = req.duration_since(now);

// If we iterate from oldest first and find one that has passed `window`, return false (deny). Otherwise continue to the next.
            if time_diff > Duration::from_millis(self.window.as_micros()) as u64 { // Convert to microseconds for comparison against fixed duration in this simplified version
                found = true;
                break; // Found a request that has expired, allow it now
            }

// If the loop finishes without finding an expired request (or if no requests are pending), return false.
        }
        
        !found
    }
}
