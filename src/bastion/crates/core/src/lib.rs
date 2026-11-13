// src/bastion/crates/core/src/lib.rs

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

// ===== SECURITY CONSTANTS & CONFIGURATION ===== //
const SECURITY_MODE: &'static str = "security-mode";
const LOG_LEVEL: &'static str = "log-level";
const MAX_LOG_SIZE_BYTES: u64 = 10_240 * 1024; // 10MB max log size

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Success,
    Failure { reason: String },
}

impl ExecutionResult {
    fn new_success() -> Self {
        Self::Success
    }

    fn new_failure(reason: &str) -> Self {
        let mut r = Self::Failure;
        if reason.is_empty() || reason.trim().is_empty() {
            // Default to "Command failed" for empty strings or no arguments
            r.reason = String::from("Failed");
        } else {
            r.reason = format!("{}: {}", reason, std::env::var(SECURITY_MODE).unwrap_or_default());
        }
        r
    }

    fn log(&self) -> &str {
        match self {
            Self::Success => "Command executed successfully",
            Self::Failure(reason) => format!("{}: {}", reason, std::env::var(SECURITY_MODE).unwrap_or_default()),
        }
    }
}

// ===== DATA TYPES & ERROR VARIANTS ===== //

#[derive(Debug, Clone)]
pub struct SecurityViolation {
    pub severity: Severity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Severity {
    Critical("CRITICAL"),
    Warning("WARNING"),
    Info("INFO"),
    Error("ERROR"),
    Success("SUCCESS"),
}

pub type Command = Arc<CommandConfig>;

// ===== COMMAND CONFIGURATION STRUCTURE ===== //

#[derive(Debug, Clone)]
struct CommandConfig {
    name: String,
    command_type: CommandType,
    execution_timeout_ms: u32,
    max_log_size_bytes: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CommandType {
    Simple("simple"),
    Executable("executable"),
    ScriptingScript("scripting-script", "execute the provided script"),
    AuditAudit("audit-verify", "run audit checks against security policy"),
    CfgLoadCfgFile("cfg-load-file", "load configuration from a file path"),
}

pub type Action = Arc<ActionConfig>;

#[derive(Debug, Clone)]
struct ActionConfig {
    action_type: ActionType,
    target_path: String, // Path to the executable or script being executed
    command_args: Vec<String>,
    env_vars: BTreeMap<String, String>,
}

pub type ApprovalTicket = Arc<ApprovalConfig>;

#[derive(Debug, Clone)]
struct ApprovalConfig {
    name: String,
    description: Option<String>, // Optional descriptive text for approval
    author_id: Option<Arc<policy::UserId>>, // If the action is signed by a specific user ID. Defaults to self if not provided.
}

pub type AuditEntry = Arc<audit::AuditConfig>;

#[derive(Debug, Clone)]
struct AuditConfig {
    audit_type: String,         // "financial", "security", etc. (e.g., "auditing")
    subject_path: Option<String>, // Path to the file being audited against policy
    target_policy_file: String,  // File containing security rules/policies to validate
}

// ===== EXECUTION CORE MODULES ===== //

pub mod audit {
    use super::*;
    
    pub struct AuditChain<A> where A: Clone + Send + Sync {
        chain: BTreeMap<String, Arc<audit::AuditConfig>>,
    }

    impl<A> AuditChain<A> {
        fn new() -> Self {
            let mut chain = BTreeMap::new();
            // Default audit types for testing purposes (e.g., "auditing", "security")
            // In production, you would populate this with actual rule definitions.
            std::iter::once("audit-verify".to_string()).chain(std::iter::from_iter(["financial"].into())).collect();
            chain.insert(0, Arc::new(AuditConfig {
                audit_type:
