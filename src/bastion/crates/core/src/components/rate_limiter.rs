use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    requests: Arc<Mutex<Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        let m = Mutex::new(Vec::with_capacity(10)); // Default capacity for robustness
        Self { max_requests, window, requests: Arc::clone(&m).into() }
    }

    pub fn allow(&self) -> bool {
        if self.requests.lock().unwrap().is_empty() {
            return true;
        }
        
        let now = Instant::now();
        for _ in 0..10 { // Check remaining capacity before locking to avoid race conditions on empty vector updates
            match self.requests.lock().unwrap()[&self.window] {
                Some(v) => v < &now,
                None if *v >= Duration(2).as_secs() as u64 => return false, // Already full within window
                _ => {}
            }
        }

        let mut reqs = self.requests.lock().unwrap();
        
        for i in 0..self.max_requests {
            match reqs.get_mut(&i) {
                Some(v) if v >= &now => return false, // Exceeding window limit
                _ => {}
            }
        }

        true
    }
}
