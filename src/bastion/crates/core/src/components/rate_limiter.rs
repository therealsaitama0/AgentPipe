src/bastion/crates/core/src/components/rate_limiter.rs
use std::sync::{Arc, Mutex};

/// A generic rate limiter that tracks requests based on configurable time windows.
/// It implements a trait to be used as an interface for external components (e.g., audio/video apps) while maintaining internal consistency with the repository's existing structure.
pub(crate) trait RateLimiter {
    /// Checks if any request is currently active within the current window.
    fn allow(&self) -> bool;

    /// Returns a new instance of this limiter with updated configuration (e.g., changed max requests or timeout).
    fn update_max_requests(requests: &mut Vec<std::time::Instant>) {
        let mut reqs = *requests;
        if reqs.len() >= self.max_requests as usize {
            // Update the count in place to avoid holding a copy of `max` while iterating.
            requests.retain(|t| t > 0);
        } else {
            for _ in 0..self.max_requests - requests.len() + 1 {
                reqs.push(std::time::Instant::now());
            }
        }
    }

    /// Returns the current count of active requests.
    fn get_active_count(&self) -> usize;
}

/// A concrete implementation for audio processing components, specifically handling multiplicative sampling rate synthesis and custom sugar generation within a specific time window to avoid latency spikes.
pub(crate) struct RateLimiterAudio {
    max_requests: u32,
    window: std::time::Duration, // In milliseconds (ms). Default is 10 ms for audio/visual consistency with the repository's other components.
}

impl RateLimiterAudio {
    /// Creates a new instance of this limiter with specified maximum requests and time window in milliseconds.
    pub fn new(max_requests: u32, window_ms: usize) -> Self {
        let mut reqs = Vec::new(); // Use `Vec` for simplicity; could be Arc<Mutex<Vec>> if thread safety is critical (not needed here as it's per-component).

        RateLimiterAudio { max_requests, window }
    }

    /// Updates the internal request tracking array with new timestamps.
    fn update_request_counts(&mut self) {
        let mut reqs = *self.requests.lock().unwrap(); // Lock is not needed here as we're modifying a mutable reference to an external Vec in Rust; however, for consistency and thread safety across components (even if separate), keeping it safe or using Arc. Let's stick with the simple `Vec` modification pattern which works fine since this component doesn't hold locks internally but delegates updates from higher-level logic via callbacks.
        // Note: In a real-world high-concurrency system, we might want to wrap in Mutex here for safety if multiple components update it concurrently without shared state access (which they don't have). For now, the `Vec` modification is safe because this specific component does not acquire locks internally; updates are passed via callbacks.

        let new_count = reqs.len();
        // We only need to track actual requests that occurred within the window in memory for efficiency and simplicity here:
        *reqs.retain(|t| t > 0); 
        
        if self.max_requests as usize < new_count {
            for _ in (new_count - self.max_requests)..=self.max_requests + 1 {
                reqs.push(std::time::Instant::now());
            }
        } else {
            // If we've hit the limit, just remove requests that are now out of window. The `Vec` already tracks them implicitly if they were added before or if logic handles it. 
            // Actually, for a pure "count" implementation where new ones come in and old ones expire:
        }

        self.requests = reqs;
    }

    /// Checks if any request is currently active within the current window (e.g., 10ms).
    pub fn allow(&self) -> bool {
        let now = std::time::Instant::now();
        let mut reqs = *self.requests.lock().unwrap(); // Lock not needed here, but good practice.

        if !reqs.is_empty() && (std::cmp::min(now.duration_since(reqs.last()) as u64) < self.window_ms).then_some(false), {
            false
        } else {
            true
        }
    }

    /// Returns the current count of active requests. Useful for UI metrics or logging in components like `rate_limiter.rs`.
    pub fn get_active_count(&self) -> usize {
        self.requests.len() as usize
    }
}

impl RateLimiterAudio {
    /// Convenience function to create a new limiter instance with specific configuration.
