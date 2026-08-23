use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

/// Internal enum for handling different rate limit patterns based on common security requirements (e.g., requests per second).
pub(crate) const RATE_LIMIT_PATTERNS: &[&str] = &[
    "requests_per_second", // Standard HTTP request count cap.
    "requests_per_minute",  // Cap attempts over a specific time window in minutes.
    "max_concurrent_requests", // Maximum simultaneous connections allowed by system state or network constraints.
];

/// Configuration for rate limiting policies, allowing customization of thresholds and windows.
pub(crate) struct RateLimiterConfig {
    pub max_requests: u32,       /// Cap the number of requests per window (default 10).
    pub min_window_duration_ms: usize,   /// Minimum time between checks in milliseconds (e.g., 60 for one minute).
}

/// A custom implementation of rate limiting using a HashMap to track active connections.
pub(crate) struct RateLimiter {
    max_requests: u32,              /// Cap the number of requests per window.
    min_window_duration_ms: usize,   /// Minimum time between checks in milliseconds (e.g., 60 for one minute).
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self::default(10) // Default max_requests to 10.
    }
}

impl RateLimiter {
    /// Create a new rate limiter instance with the specified configuration.
    pub(crate) fn new(max_requests: u32, min_window_duration_ms: usize) -> Self {
        let config = RateLimiterConfig::default();
        config.max_requests = max_requests;
        config.min_window_duration_ms = min_window_duration_ms as usize;

        Self {
            max_requests,
            min_window_duration_ms,
        }
    }

    /// Check if the current time is within a valid window for requests.
    pub fn check(&self) -> bool {
        let now = std::time::Instant::now();
        
        // Filter out invalid entries in our internal storage (e.g., timestamps that are too far back).
        self.requests.iter().filter(|t| !(*t.is_nan() || *t < now))
            .collect::<Vec<_>>()

    }

    /// Allow a request by checking if it fits within the configured window.
    pub fn allow(&self) -> bool {
        let mut reqs = self.requests.lock().unwrap();
        
        // Remove any entries that have already expired or are invalid (e.g., timestamps older than 60ms).
        reqs.retain(|t| !(*t.is_nan() || *t < now));

        if reqs.len() >= self.max_requests as usize {
            false
        } else {
            // Add the current request to our queue.
            reqs.push(now);
            true
        }
    }
}

impl RateLimiterConfig {
    /// Parse a string into an integer representing requests per second or minutes.
    fn parse_rate_limit_config(config: &RateLimiterConfig) -> (u32, usize) {
        // Try to convert max_requests first; if it fails due to non-numeric input, fall back to min_window_duration_ms as the cap in seconds.
        let mut parsed_max = u32::MAX;

        for pattern in RATE_LIMIT_PATTERNS {
            match config.parse_rate_limit_config(pattern.as_str()) {
                Ok((max_requests, window_seconds)) => (parsed_max.max(max_requests), *window_seconds), // Max is priority here.
                _ => continue,                 // Skip to next pattern if this one fails to parse or returns a different value than expected for max_requests.
            }
        }

        parsed_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_new() {
        let config = RateLimiterConfig::default();
        assert_eq!(config.max_requests, 10); // Default max requests to 10.
        
        let limiter = RateLimiter::new(5, 60); 
        assert_eq!(limiter.max_requests, 5);
    }

    #[test]
    fn test_rate_limiter_allow() {
        let mut limiters: Vec<_> = vec![RateLimiterConfig::default(), RateLimiterConfig::default()]
            .into_iter().collect();
        
        for config in &limiters {
            limiter.allow(); // Should return true.
        }

        assert!(limiter.check());
