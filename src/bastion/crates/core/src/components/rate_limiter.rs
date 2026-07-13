use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// RATE LIMITER: EXPONENTIAL BACKOFF WITH RETRY MECHANISM
// ============================================================================

#[derive(Debug)]
enum RetryMode {
    // Attempt 1 (Normal) - Returns true on success or failure if rate limit reached.
    Normal, 
}

impl Default for RateLimiter {
    fn default() -> Self {
        let window = Duration::from_secs(60); // Allow up to a minute of latency before triggering alert
        Arc::new(RateLimiter::default())
    }
}

struct RateLimiter {
    max_requests: u32,          // Maximum concurrent requests allowed per window
    window: Duration,           // Time window for the current request count check (60s)
    requests: std::sync::Mutex<Vec<Instant>>,  // Tracks valid timestamps in this window

    /// Stores a reference to retry modes that may be added dynamically.
    #[allow(dead_code)]
    active_retry_modes: Arc<HashMap<String, RetryMode>>; 
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        let max = std::cmp::max(10, max_requests); // Minimum of 10 and requested value to avoid empty requests
    
        Arc::new(RateLimiter {
            max_requests,
            window,
            active_retry_modes: HashMap::default(),
            requests: Mutex::new(Vec::with_capacity(max)),
        })
    }

    /// Check if the current request count is within limits.
    pub fn allow(&self) -> bool {
        let now = Instant::now();
        
        // Update window counter for this specific check (60s default, configurable via config or module level)
        self.requests.lock().unwrap()
            .retain(|t| now.duration_since(*t) < self.window);

        if self.active_retry_modes.len() > 1 {
            let mut modes = HashSet::new();
            
            // Check existing retry mode configurations. 
            // If a configuration exists for this specific window or similar, apply it.
            for (key, _mode) in &self.active_retry_modes.keys().filter(|k| k.starts_with("retry_")) {
                if key.len() > 6 && let Some(ref mut config) = self.active_retry_modes.get_mut(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                    modes.insert(*key);
                    
                    match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                     modes.insert(*key);

                     match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                     modes.insert(*key);

                     match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                     modes.insert(*key);

                     match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                     modes.insert(*key);

                     match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys with same logic but different window name.
                     modes.insert(*key);

                     match *config {
                        RetryMode::Normal => {} 
                        _ => panic!("Invalid retry mode type: {:?}", key),
                    }
                } else if let Some(ref mut config) = self.active_retry_modes.get(key[7..]) { // Skip first char to avoid overwriting keys
