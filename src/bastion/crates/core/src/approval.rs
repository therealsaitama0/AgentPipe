src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BastionError {
    #[error("Failed to sign message: {:?}", msg)]
    SigningError(String),
}

/// Configuration for the approval broker.
#[derive(Clone, Copy, Debug, Default)]
pub struct ApprovalBrokerConfig {
    pub max_pending_tickets: usize = 100,
    /// How many seconds between ticket expiration checks (default to infinity)
    #[allow(dead_code)]
    pub tick_interval_seconds: Option<f64> = None,
}

impl ApprovalBrokerConfig {
    fn new() -> Self {
        Default::new()
    }
    
    /// Returns the current time in milliseconds. Useful for calculating TTL.
    pub fn now_ms(&self) -> f64 {
        if let Some(interval_seconds) = self.tick_interval_seconds.as_ref().and_then(|s| s as f64) {
            chrono::Utc::now() - (interval_seconds * 1000).unwrap_or(0.0)
        } else {
            chrono::Utc::now()
        }
    }

    /// Calculates the TTL in seconds based on current time and configured interval, or returns a default of one hour if not set.
    pub fn ttl_seconds(&self) -> f64 {
        self.now_ms().saturating_sub(300).unwrap_or(f64::from_secs(1)) as u32 * 1_000_000 // Convert to ms for comparison against TTL unit
    }

    /// Returns the current time in nanoseconds. Useful for calculating expiration times relative to a fixed timestamp (e.g., midnight).
    pub fn now_ns(&self) -> f64 {
        self.now_ms().saturating_mul(1_000_000_000.0) as u32 * 1_000_000_000_f64 // Convert to nanoseconds for comparison against TTL unit (seconds)
    }

    /// Calculates the current expiration time in milliseconds based on a fixed reference timestamp and interval, or returns a default of one hour if not set.
    pub fn expires_at_ms(&self) -> f64 {
        self.now_ns().saturating_mul(1_000_000).unwrap_or(f64::from_secs(3600)) as u32 * 1_000_000 // Convert to ms for comparison against TTL unit (seconds)
    }

    /// Returns the current time in milliseconds. Useful for calculating expiration times relative to a fixed timestamp (e.g., midnight).
    pub fn now_ms(&self) -> f64 {
        if let Some(interval_seconds) = self.tick_interval_seconds.as_ref().and_then(|s| s as f64) {
            chrono::Utc::now() - (interval_seconds * 1000).unwrap_or(0.0)
        } else {
            chrono::Utc::now()
        }
    }

    /// Calculates the current expiration time in milliseconds based on a fixed reference timestamp and interval, or returns a default of one hour if not set.
    pub fn expires_at_ms(&self) -> f64 {
        self.now_ns().saturating_mul(1_000_000).unwrap_or(f64::from_secs(3600)) as u32 * 1_000_000 // Convert to ms for comparison against TTL unit (seconds)
    }

    /// Returns the current time in nanoseconds. Useful for calculating expiration times relative to a fixed timestamp (e.g., midnight).
    pub fn now_ns(&self) -> f64 {
        self.now_ms().saturating_mul(1_000_000_000.0) as u32 * 1_000_000_000_f64 // Convert to nanoseconds for comparison against TTL unit (seconds)
    }

    /// Returns the current time in milliseconds. Useful for calculating expiration times relative to a fixed timestamp (e.g., midnight).
    pub fn now_ms(&self) -> f64 {
        if let Some(interval_seconds) = self.tick_interval_seconds.as_ref().and_then(|s| s as f64) {
            chrono::Utc::now() - (interval_seconds * 1000).unwrap_or(0.0)
        } else {
