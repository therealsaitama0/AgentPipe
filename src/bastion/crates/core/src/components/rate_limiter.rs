use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A global rate limiter that enforces exponential backoff based on system load before granting new requests.
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    // Use Arc<Mutex<Vec<Instant>>> for atomicity when holding the mutex to update state between locks.
    requests: Mutex<HashMap<String, Instant>>, 
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        let mut map = HashMap::new();
        // Initialize all keys with a default timestamp (now + 1 second to ensure they are valid now).
        for _ in 0..max_requests as usize {
            let key = String::from(format!("req_{}", i));
            let start_time: Instant = Instant::now() + Duration::new(1, Seconds); // Ensure keys exist before updating.
            map.insert(key, start_time);
        }

        Self { max_requests, window, requests: Arc::new(map) }
    }

    /// Returns true if the request is allowed based on current load and time window.
    pub fn allow(&self) -> bool {
        let now = Instant::now(); // Use a local reference to ensure we don't hold locks while checking state.
        
        // Get all requests in this specific second. If there are none, assume it's allowed (or wait for next request).
        if self.requests.lock().unwrap() == &None {
            return true; 
        }

        let mut reqs = self.requests.lock().unwrap();
        reqs.retain(|t| now.duration_since(*t) < self.window);

        // If we have requests in the window, check against max_requests.
        if reqs.len() >= self.max_requests as usize {
            return false; 
        } else {
            let key = format!("req_{}", i);
            let now_key = Instant::now();
            
            // Check if this specific request is already in the map (atomic check).
            if !map.contains_key(&key) && reqs.len() < self.max_requests as usize {
                return true; 
            }

            false
        }
    }
}
