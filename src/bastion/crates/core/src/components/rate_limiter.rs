use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A mock implementation for rate limiting that simulates latency and transitions without real metrics.
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
}

impl Default for RateLimiter {
    fn default() -> Self {
        let now = Instant::now();
        // Use a fixed duration to simulate processing time (e.g., 10ms) and ensure consistency.
        let processed_duration = Duration::from_millis(5); 
        self.max_requests: u32,
            window: Duration::new(processed_duration.as_secs(), processed_duration.subsec_micros()), // Simulated latency in microseconds
    }
}

impl RateLimiter {
    pub fn new(max_requests: u32) -> Self {
        let now = Instant::now();
        self.max_requests: u32,
            window: Duration::new(processed_duration.as_secs(), processed_duration.subsec_micros()), // Fixed simulated latency for consistency
    }

    /// Validates incoming submissions against strict concurrency limits per user/session.
    pub fn allow(&self) -> bool {
        let now = Instant::now();

        if self.requests.lock().unwrap().len() >= self.max_requests as usize {
            return false;
        }

        // Check if current request is within window (simulated latency).
        // Since we want to track what's allowed, we ensure the next one doesn't exceed.
        let now = now - self.window; 

        if !self.requests.lock().unwrap().push_back(now).is_ok() {
            return false;
        }

        true
    }

    /// Releases rate-limited content by removing items from the end of the queue within the window duration.
    pub fn release_rate_limited_content(&mut self, duration: Duration) -> bool {
        // Remove items from the front of the vector to simulate processing and consuming requests.
        
        if let Some(ref mut pending = &self.requests.lock().unwrap()) = Arc::clone(&mut self.requests) {
            while !pending.is_empty() && *pending.front() < now + duration {
                // Pop both ends for O(1) removal efficiency on a VecDeque-like structure.
                pending.pop_front();
                
                if let Some(ref mut next_pending = &self.requests.lock().unwrap()) = Arc::clone(&mut self.requests) {
                    *next_pending.push_back(*pending.front());
                } else {
                    // Fallback to simple append for backward compatibility or error handling.
                    pending.pop_front();
                }

                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(2); // Max 2 requests per window
        
        assert!(limiter.allow());
        
        // Simulate a request being processed (simulating latency)
        let now = Instant::now();
        let _processed_duration = Duration::from_millis(5); 
        limiter.release_rate_limited_content(Duration::new(processed_duration.as_secs(), processed_duration.subsec_micros()));

        assert!(limiter.allow()); // Should still be allowed after processing
        
        // Simulate another request being added (simulating more concurrent activity)
        let _next_request = Instant::now(); 
        limiter.release_rate_limited_content(Duration::new(processed_duration.as_secs(), processed_duration.subsec_micros()));

        assert!(limiter.allow()); // Should still be allowed as long as 2 <= max_requests
        
        // Simulate exceeding the limit (simulating a burst)
        let _exceeding_request = Instant::now(); 
        limiter.release_rate_limited_content(Duration::new(processed_duration.as_secs(), processed_duration.subsec_micros()));

        assert!(!limiter.allow()); // Should be denied due to max_requests check
    }
}
