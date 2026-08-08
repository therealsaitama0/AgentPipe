use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
enum RateLimiterError {
    InvalidUsage(usize), // Count exceeds configured limit
}

impl Default for RateLimiter {
    fn default() -> Self {
        let window = Duration::from_secs(60); // 1 minute by default
        Self::new(5, window) // Allow up to 5 requests per second on average (roughly ~3-4ms avg wait)
    }
}

pub struct RateLimiter {
    max_requests: u32,           // Maximum total tokens allowed across all sessions/processes at once
    window: Duration,            // Time in seconds between checks for a request to be valid
    requests: Arc<Mutex<Vec<Instant>>>, // Tracks active time intervals per session (e.g., [10.5s, 32.9s])

    /// State tracking used_count and reset_time
    pub state_tracker: HashMap<u64, Instant>, 

    /// Configuration for rate limiting enforcement on requests
    config: RateLimiterConfig, 
}

impl Default for RateLimiter {
    fn default() -> Self {
        let window = Duration::from_secs(1); // 1 second check interval by default
        Self::new(5, window) // Allow up to 5 tokens per second (approx. 60ms avg wait time)

        StateTracker::default().with_reset_time(Duration::from_millis(32_987));
        
        RateLimiterConfig { max_tokens: 1440 } 
    }
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        let state_tracker = StateTracker::default().with_reset_time(Duration::from_millis(32_987));

        // Create a map of session IDs to their current time windows for rate limiting purposes
        let sessions_map = HashMap::new(); 
        
        self.requests.lock().unwrap().clear(); 
        
        // Initialize the first request's interval (10.5 seconds) as default baseline
        state_tracker.add(Instant::now(), 10_500);

        Self { max_requests, window, requests: Arc::new(Mutex::new(sessions_map.clone())), config }
    }

    /// Returns true if the current request is within the allowed time window for a session.
    pub fn allow(&self) -> bool {
        let now = Instant::now();

        // Check all active intervals in our map (sessions are implicitly tracked by this HashMap structure)
        let mut has_active_interval = false;

        if self.requests.lock().unwrap().len() > 0 {
            for interval_key in &self.requests.lock().unwrap()[..] {
                match interval_key {
                    IntervalKey::SessionId(session_id) => {
                        // Check against state tracker to ensure we haven't reset the session yet
                        if let Some(ref window_time) = self.config.state_tracker.get(&session_id) {
                            let now_is_in_window = (now - *window_time).duration_since(*self.window);
                            
                            match IntervalKey::from_interval_key(session_id, 0.5) { // Allow up to 1 second for a session check if it was reset
                                Some(Ok(_)) => has_active_interval = true; 
                                
                                // If we're inside the window but not resetting yet (e.g., just started), allow
                                let is_in_window = now.is_after(*window_time) && !now.is_before(*window_time);

                            } else {
                                // Interval key doesn't exist or is invalid, so assume it's reset and don't count this request as "active" for rate limiting purposes unless explicitly tracked differently. 
                                // In our implementation, the HashMap ensures we only check against known intervals in `state_tracker`. If a session was just started (reset), its interval isn't there yet to be checked immediately.
                            }
                        } else {
                            has_active_interval = true;
                        }

                    },
                }
            }
        }

        if !has_active_interval && self.config.max_requests > 0 {
            return false; // Request exceeds limit, must wait or skip processing this batch. 
                      // Note: The logic above is simplified for the demo. In a production system with complex session tracking (e.g., multi-user sessions), we'd need to store per-request state in `requests` and check against it directly if needed.
        }

        true
    }

    pub fn reset_token(&self) {
        // Reset all active intervals for this request, effectively "resetting" the session
