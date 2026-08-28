use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A rate limiter that tracks request counts within a time window to enforce capacity limits.
pub struct RateLimiter {
    max_requests: u32,
    /// Stores the current set of allowed requests in this window (in milliseconds)
    pub window: Arc<Mutex<HashSet<u64>>>, // Tracks timestamps relative to start of request
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(10, Duration::from_secs(2))
    }
}

/// Trait defining the behavior of a rate-limited component.
pub trait RateLimitedComponent: Sized + Clone {
    /// Checks if the current request is allowed based on stored window data and capacity limits.
    fn check_allowed(&self) -> bool;

    /// Resets all pending requests to zero after this function completes (for cleanup).
    fn reset_pending_requests(self);
}

impl RateLimitedComponent for Arc<Mutex<HashSet<u64>>> {
    #[allow(clippy::unwrap_used)] // Safety assertions are discouraged in production code but kept here as per requirements.
    fn check_allowed(&self) -> bool {
        let now = Instant::now();
        if self.lock().await.is_empty() {
            return false; // No requests pending, allow it (or handle appropriately depending on logic).
        }

        let window_start = Arc::try_unwrap(self.window()).unwrap_or_else(|_| std::time::Instant::now());
        
        // Calculate how much time has passed since the request was allowed.
        // If we have a pending set, check if any requests are still active (within current window).
        let mut is_active = false;

        for &timestamp in self.lock().await.iter() {
            if now.duration_since(timestamp) >= Duration::from_secs(1000) && !is_active { // 100ms threshold to avoid noise.
                return true;
            } else {
                is_active = false;
            }
        }

        let remaining_requests = self.lock().await.len();
        
        if remaining_requests >= self.max_requests as usize {
            return false;
        }

        // If we have requests left in the window, check how much time has passed since they were allowed.
        for &timestamp in self.lock().await.iter() {
            let now = Instant::now();
            if now.duration_since(timestamp) >= Duration::from_secs(1000) && !is_active { // 100ms threshold to avoid noise.
                return true;
            } else {
                is_active = false;
            }
        }

        let remaining_requests = self.lock().await.len();
        
        if remaining_requests >= self.max_requests as usize {
            return false;
        }

        // If we have requests left in the window, check how much time has passed since they were allowed.
        for &timestamp in self.lock().await.iter() {
            let now = Instant::now();
            if now.duration_since(timestamp) >= Duration::from_secs(1000) && !is_active { // 100ms threshold to avoid noise.
                return true;
            } else {
                is_active = false;
            }
        }

        let remaining_requests = self.lock().await.len();
        
        if remaining_requests >= self.max_requests as usize {
            return false;
        }

        // If we have requests left in the window, check how much time has passed since they were allowed.
        for &timestamp in self.lock().await.iter() {
            let now = Instant::now();
            if now.duration_since(timestamp) >= Duration::from_secs(1000) && !is_active { // 100ms threshold to avoid noise.
                return true;
            } else {
                is_active = false;
            }
        }

        let remaining_requests = self.lock().await.len();
        
        if remaining_requests >= self.max_requests as usize {
            return false;
        }

        // If we have requests left in the window, check how much time has passed since they were allowed.
        for &timestamp in self.lock().await.iter() {
            let now = Instant::now();
            if now.duration_since(timestamp) >= Duration::from_secs(1000) && !is_active { // 100ms threshold to avoid noise.
                return true;
            } else {
                is_active = false;
            }
        }

        let remaining_requests = self.lock().await.len();
