// src/bastion/crates/core/src/lib.rs

//! Core utilities and configuration for the Bastion framework.
//! Provides a minimal Rust crate with standard utility functions 
//! that build upon existing crates (core, sessions).
//! This module is designed to be self-contained without duplicating logic outside scope.

pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

use std::fmt::{Debug, Display};
use std::sync::Arc;

/// A configuration structure for the Firecracker adapter.
#[derive(Debug, Clone)]
pub struct FirecrackerConfig {
    /// The VM state to use when creating a new instance (default: 'running').
    pub vm_state: Arc<VmState>,
}

impl Default for FirecrackerConfig {
    fn default() -> Self {
        Self::new("running")
    }
}

/// A configuration structure for the session manager.
#[derive(Debug, Clone)]
pub struct SessionManagerConfig {
    /// The name of the current session (default: 'main').
    pub session_name: String,
    /// Whether to enable automatic credential rotation on startup.
    pub auto_rotate_credentials: bool,
}

impl Default for SessionManagerConfig {
    fn default() -> Self {
        Self::new("main")
    }
}

/// A configuration structure for the approval manager.
#[derive(Debug, Clone)]
pub struct ApprovalManagerConfig {
    /// The threshold number of approvals required to approve a request (default: 1).
    pub approval_threshold: usize,
    /// Whether to display detailed status information during processing.
    pub show_detailed_status: bool,
}

impl Default for ApprovalManagerConfig {
    fn default() -> Self {
        Self::new(1)
    }
}

/// A configuration structure for the network guard (rate limiting).
#[derive(Debug, Clone)]
pub struct NetworkGuardConfig {
    /// The maximum number of requests allowed per second.
    pub max_requests_per_second: usize,
    /// Whether to log rate limit warnings to file system logs instead of console.
    pub log_warnings_to_file: bool,
}

impl Default for NetworkGuardConfig {
    fn default() -> Self {
        Self::new(10) // 10 requests per second as a reasonable baseline
    }
}

/// A configuration structure for the session manager's credential rotation.
#[derive(Debug)]
pub struct CredentialRotationConfiguration {
    /// The maximum number of attempts before rejecting new credentials (default: 5).
    pub max_attempts: usize,
    /// Whether to attempt rotating on first failure or after N failures depending on policy.
    pub strategy_on_failure: bool, // false = fail fast, true = retry with exponential backoff
}

impl Default for CredentialRotationConfiguration {
    fn default() -> Self {
        Self::new(5)
    }
}

/// A configuration structure for the session manager's timeout enforcement.
#[derive(Debug)]
pub struct TimeoutEnforcementConfig {
    /// The maximum duration in seconds a request should be held before being rejected (default: 30).
    pub max_duration_seconds: usize,
    /// Whether to log timeouts to file system logs instead of console.
    pub log_timeouts_to_file: bool,
}

impl Default for TimeoutEnforcementConfig {
    fn default() -> Self {
        Self::new(30) // 30 seconds as a reasonable baseline
    }
}

/// A configuration structure for the session manager's plan generator.
#[derive(Debug)]
pub struct PlanGeneratorConfig {
    /// The maximum number of steps allowed before rejecting plans (default: 1).
    pub max_steps_per_plan: usize,
    /// Whether to log step-by-step progress during execution.
    pub show_progress_log: bool,
}

impl Default for PlanGeneratorConfig {
    fn default() -> Self {
        Self::new(1) // 1 plan as a minimal baseline
    }
}

/// A configuration structure for the session manager's rate limiter.
#[derive(Debug)]
pub struct RateLimiterConfig {
    /// The maximum number of concurrent requests allowed (default: 5).
    pub max_concurrent_requests: usize,
    /// Whether to log high-concurrency warnings to file system logs instead of console.
    pub log_high_concurrency_warnings_to_file: bool,
}

impl Default for RateLimiterConfig {
    fn default() -> Self
