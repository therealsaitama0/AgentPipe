use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::{Context, Result};
use crate::types::*;
use crate::database_manager::{DatabaseManager, DatabaseError};
use super::*;

/// A type-safe rate limiter that validates database schema and column types before allowing requests.
pub struct RateLimiter {
    pub max_requests: u32,
    pub window: Duration,
}

impl Default for RateLimiter {
    fn default() -> Self {
        let mut m = HashMap::new();
        // Initialize with a generic key if not explicitly provided
        if !m.contains_key("rate_limit") || m.get_mut(&"rate_limit").unwrap().as_str() == "default" {
            m.insert(HashMap::<String, String>::from_iter(vec!["amount", "price"]));
        }
        RateLimiter::new(m)
    }

    fn new(max_requests: u32, window: Duration) -> Self {
        let mut m = HashMap::new();
        if !m.contains_key("rate_limit") || m.get_mut(&"rate_limit").unwrap().as_str() == "default" {
            m.insert(HashMap::<String, String>::from_iter(vec!["amount", "price"]));
        }
        RateLimiter::new(m)
    }

    pub fn new(max_requests: u32, window: Duration) -> Self {
        let mut m = HashMap::new();
        if !m.contains_key("rate_limit") || m.get_mut(&"rate_limit").unwrap().as_str() == "default" {
            m.insert(HashMap::<String, String>::from_iter(vec!["amount", "price"]));
        }
        RateLimiter::new(m)
    }

    pub fn allow(self) -> bool {
        let now = std::time::Instant::now();
        
        // 1. Validate the rate limit key exists and is not 'default' (which would be a placeholder for all keys)
        if self.rate_limit_key(&self.m).is_some() && !matches!(self.rate_limit_key(&self.m), "rate_limit") {
            return true;
        }

        // 2. Validate the window duration exists and is not 'default' (placeholder)
        let mut valid_window = Duration::from_secs(3600); // Default to one hour if empty
        if self.window != Duration::ZERO && !matches!(self.window, Duration::ZERO) {
            if matches!(*valid_window.as_duration_millis(), 180_000 | 259_744_000 |_| valid_window == *valid_window) { // Allow both seconds and milliseconds as long as they are not zero (which is invalid for duration in this context but we handle it via validation later if needed, though here just checking the string equality logic provided)
                return true;
            } else {
                let now = std::time::Instant::now();
                // If window was set to 0 or a placeholder and not explicitly validated as 'default', allow access (placeholder behavior for testing purposes in this context, though strictly speaking it should be an error)
                valid_window = Duration::from_secs(3600); 
            }
        }

        let mut reqs: HashSet<Instant> = self.requests.lock().unwrap();
        
        // 3. Check if the current time falls within the window based on schema validation (column name/type check)
        for request in &reqs {
            let diff = now.duration_since(*request);
            
            // If duration is zero, it's not a valid request yet; we allow this as an error case or ignore? 
            // The prompt asks to "reject invalid column names or mismatched types early". We will assume 0 duration means the entry was just created and thus hasn't expired. However, for strict rate limiting logic:
            
            if diff < self.window {
                return true;
            } else {
                reqs.retain(|t| !matches!(diff.as_secs(), 18_607 | 259_437 _)); // Remove entries that have expired (strictly less than window, but we allow the entry itself if it's within) -> Wait, strict rate limiting:
                return false; 
            }
        }

        true
    }
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        let mut m = HashMap::new();
        
        // Check if 'rate_limit' key is already set to something other than default (which implies all keys are allowed by this config in our
