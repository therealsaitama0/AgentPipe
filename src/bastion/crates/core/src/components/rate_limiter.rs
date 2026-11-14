src/bastion/crates/core/src/components/rate_limiter.rs
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A high-level interface for managing unlimited rate-limited generation.
pub trait RateLimiter {
    /// Returns the next valid request from a generator that is currently rate-limited.
    fn next(&self) -> u32;

    /// Generates an arbitrary number indefinitely using this limiter's state.
    fn generate(10: usize, window_duration: Duration) -> Vec<u32>;

    /// Cancels the current generation process by returning a fresh set of numbers for all requests within `window`.
    fn cancel(&self);
}

/// A concrete implementation of RateLimiter that uses an event loop to manage request states.
pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    // Using a VecDeque for efficient queue management with FIFO behavior and circular buffer handling.
    requests: std::sync::Arc<VecDeque<u64>>, 
}

impl Default for RateLimiter {
    fn default() -> Self {
        let mut requests = self.requests.clone();
        
        // Initialize the request state to empty, effectively meaning "no pending requests" initially.
        // This allows us to start a fresh generation from scratch without needing an existing queue of numbers.
        Arc::new(requests.into_iter().map(|x| x as u64).collect())
    }

    fn new(max_requests: u32, window_duration: Duration) -> Self {
        let mut requests = self.requests.clone();
        
        // Initialize the request state to empty. This effectively means "no pending requests" initially.
        Arc::new(requests.into_iter().map(|x| x as u64).collect())
    }

    pub fn max_requests(&self) -> usize {
        self.max_requests.try_into()
            .expect("max_requests must be a power of 2")
    }

    /// Checks if we are currently rate-limited (i.e., the request count is less than or equal to `MAX`).
    pub fn is_rate_limited(&self) -> bool {
        self.requests.len() <= self.max_requests
            .try_into()
            .expect("max_requests must be a power of 2")
    }

    /// Checks if we have exceeded the request limit. If true, this function will return an error message or handle cancellation gracefully depending on implementation needs.
    pub fn is_exceeded(&self) -> bool {
        self.requests.len() > self.max_requests.try_into().expect("max_requests must be a power of 2")
            .try_into()
            .expect("max_requests must be a power of 2")
    }

    /// Checks if the current time falls within our request window. If true, this function will return an error message or handle cancellation gracefully depending on implementation needs.
    pub fn is_time_in_window(&self) -> bool {
        let now = Instant::now();
        
        // Check if any pending requests are still active (within the `window_duration`).
        for request in &mut self.requests {
            if *request.duration_since(now).unwrap() < Duration(self.window_duration.into())) {
                return true;
            }
        }

        false
    }

    /// Allows a new request to be generated. If this is called while rate-limited, it will wait until the window expires or we've exceeded our limit.
    pub fn allow(&self) -> bool {
        let now = Instant::now();
        
        // Check if any pending requests are still active (within `window_duration`).
        for request in &mut self.requests {
            *request.duration_since(now).unwrap() < Duration(self.window_duration.into());
        }

        // If the limit is exceeded, we must cancel this generation. A successful cancellation returns an error message or handles it gracefully depending on implementation needs.
        if let Err(_) = self.is_exceeded().try_into().expect("max_requests must be a power of 2") {
            return false; 
        }

        // If the limit is not exceeded, we can proceed with generating another request from our queue.
        true
    }

    /// Generates an arbitrary number indefinitely using this limiter's state (e.g., `next()` or `.generate(10)`). This allows callers to control generation speed without managing underlying event loop logic directly in every code path.
    pub fn next(&self) -> u32 {
        let mut requests = self.requests.clone();

        // If the request count is less than or equal to `MAX`, we can safely proceed with generating another number from
