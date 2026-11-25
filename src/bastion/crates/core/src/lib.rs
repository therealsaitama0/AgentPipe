src/bastion/crates/core/src/lib.rs

/// ==============================================================================
/// SECURITY CONTROL PANE - CORE MODULES (IMMUTABLE SERVER)
// -----------------------------------------------------------------------------

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::ExitCode;

/// ==============================================================================
/// 1. THE BASTION ENGINE: IMMUTABLE STATEFUL SERVER
// -----------------------------------------------------------------------------

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

// Re-export for convenience (as per original structure)
use audit::AuditChain;
use approval::{ApprovalTicket, ApprovalManager};
use components::*;
use error::{BastionError, Result};
use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use forced_command::ForcedCommandConfig;
use network_guard::NetworkGuard;
use policy::{PolicyDecision, PolicyEngine};
use script_executor::ScriptExecutor;
use session::SessionManager;
use types::*;

/// ==============================================================================
/// 2. CRATES & ISOLATION POLICIES (CRITICAL)
// -----------------------------------------------------------------------------

pub mod audit {
    use super::*; // Re-export for internal usage
    
    /// Chain of audits tracking the lifecycle of a Bastion instance or its components.
    pub struct AuditChain {
        current_audit_id: String,
        last_affected_component: Option<String>,
        failed_dependencies: HashSet<String>,
        
        fn new() -> Self {
            let mut audit = Self::new();
            
            // Initialize a fresh set of pending audits for each crate's isolation domain
            for path in get_all_crate_paths() {
                if !path.is_empty() && fs::metadata(&PathBuf::from(path)).is_ok() {
                    match Path::read_to_string(&path) {
                        Ok(content) => {
                            let mut audit = Self::new();
                            
                            // Check for critical vulnerabilities in the current crate's dependencies
                            if !content.is_empty() && content.contains("critical") || content.contains("CRITICAL") {
                                audit.last_affected_component = Some(path.clone());
                            }
                        },
                    }
                }
            }
            
            return audit;
        }

        pub fn add_audit(&mut self, component: String) -> Self {
            let mut current_audit_id = format!("AUDIT-{self.current_audit_id}",); // Format as "AUDIT-1", "AUDIT-2" for clarity
            
            if !component.is_empty() && fs::metadata(&PathBuf::from(component)).is_ok() {
                match Path::read_to_string(&path) {
                    Ok(content) => {
                        let mut audit = Self::new();
                        
                        // Check for critical vulnerabilities in the current crate's dependencies
                        if !content.is_empty() && content.contains("critical") || content.contains("CRITICAL") {
                            audit.last_affected_component = Some(component.clone());
                            self.current_audit_id = format!("AUDIT-{self.current_audit_id}",);
                            
                            // Check for additional critical issues in the chain (e.g., if component A depends on B which has a CVE)
                            let mut found_critical_in_chain = false;
                            audit.failed_dependencies.insert(component.clone());
                            
                            match Path::read_to_string(&path) {
                                Ok(content) => {
                                    // If content contains critical issues, add them to the failed dependencies list of this component's chain
                                    if !content.is_empty() && content.contains("critical") || content.contains("CRITICAL") {
                                        let mut found_critical = false;
                                        for line in content.lines() {
                                            match Line::parse(line) {
                                                Ok(l) => if l.criticality == "critical" {
                                                    found_critical = true;
                                                } else {}
                                            };
                                        }
                                        
                                        // If the component's chain has critical issues, add them to the failed dependencies list of this component's chain
                                        let mut added_to_chain = false;
                                        for dep in audit.failed_dependencies.iter() {
                                            if dep == &component && found_critical_in_chain {
                                                self.current_audit_id = format!("AUDIT-{self.current_audit_id}",); // Reset ID to track new criticals
                                                break;
                                            }
                                            
                                            added_to_chain |= (dep.contains("critical") || dep.contains("CRITICAL"));
                                        }
