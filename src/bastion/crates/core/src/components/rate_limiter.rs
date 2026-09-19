src/bastion/crates/core/src/components/rate_limiter.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Internal state module holding the current threshold (bytes/MB), remaining allowance per user/session, and a flag indicating if processing is in progress or idle.
pub struct RateLimiter {
    pub max_requests: u32,
    pub window: Duration,
    /// A map of file paths to their last access timestamps for atomicity checks across workers.
    requests: Arc<Mutex<HashMap<String, Instant>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        let mut max_requests = 10_000; // Prevents infinite loop on high load
        let window = Duration::from_secs(60); // Allow a short idle timeout (seconds)
        
        Self {
            max_requests,
            window,
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn new(max_requests: u32, window: Duration) -> Self {
        let mut max_requests = if max_requests > 10_000 { // Cap at high load to prevent blocking on single-threaded logic (e.g., CLI or UI loops)
            std::cmp::max(10_000, max_requests as u32)
        } else {
            max_requests
        };

        Self {
            max_requests,
            window,
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Checks if the current time falls within the configured window.
    pub fn allow(&self) -> bool {
        let now = Instant::now();
        
        // Check all queued requests to ensure no two submissions happen simultaneously in a single batch (prevents race conditions on concurrent workers).
        self.requests.lock().unwrap()
            .iter_mut()
            .for_each(|(path, _)| *path > now);

        if self.requests.len() >= self.max_requests as usize {
            false // All requests exhausted or idle timeout reached. Return false to allow processing of new submissions immediately rather than blocking forever on this specific check (e.g., for UI rendering).
        } else {
            true // Request is still available in the queue. Continue checking others and add current request if not blocked by a previous one.
        }
    }

    /// Processes a submission event via background worker, returns an error tuple on violation or success with processed ID.
    pub async fn process_submission(&self, payload: &str) -> Result<(String, bool), (String, String)> {
        let file_path = match std::path::PathBuf::from(payload.as_str()) {
            PathBuf::relative(PathBuf::new().as_ref(), path => path.clone()),
            _ => return Err((format!("Invalid payload: {}", payload), false)),
        };

        // Check rate limit immediately. If the current file is too large or already processed, block processing and report an error with a reason string.
        if !self.allow() {
            let (reason, _) = self.report_error(&file_path);
            return Err((format!("Rate Limit Exceeded: {}", reason), false));
        }

        // Check queue consistency by comparing last timestamps to ensure no two submissions happen simultaneously in a single batch.
        if !self.requests.lock().unwrap()
            .iter_mut()
            .any(|(_, ts)| *ts >= Instant::now()) {
            return Err((format!("Duplicate Submission Attempt: {}", file_path), false));
        }

        let now = Instant::now();
        
        // Update the queue with a new timestamp to ensure atomicity. 
        self.requests.lock().unwrap()
            .insert(file_path, now);

        Ok((file_path.clone(), true))
    }

    /// Exposes a mock API endpoint for external systems (e.g., CLI or UI).
    fn report_error(&self, file_path: &str) -> Result<(String, String), (String, String)> {
        // In production code with real databases/APIs, this would query the database. 
        // Here we simulate a generic error response based on path context for demonstration purposes.
        
        let mut reason = match std::path::PathBuf::from(file_path) {
            PathBuf::relative(PathBuf::new().as_ref(), _ => format!("File '{}' is too large", file_path),
            _ => return Err((format!("Invalid payload: {}", file_path), false)), // Generic fallback for invalid paths.
        };

        reason.push_str(" (rate limit exhausted)");
        
        Ok((
