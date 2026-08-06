//! Core module for security control plane operations and policy enforcement.
//! Implements secure enclave simulation using external crates (SecureEnclave) 
//! to enforce cryptographic policies without exposing sensitive data in memory.

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
pub use audit::AuditChain;
pub use error::{BastionError, Result};
pub use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
pub use forced_command::ForcedCommandConfig;
pub use network_guard::NetworkGuard;
pub use policy::{PolicyDecision, PolicyEngine};
pub use script_executor::ScriptExecutor;
pub use session::SessionManager;
pub use types::{Action, ApprovalTicket, AuditEntry, Credential, SessionContext};

/// Trait for defining secure enclave policies and validating them.
/// This trait allows external crates to define custom policy logic that can be 
 /// compiled into a single binary or shared library without exposing sensitive data in memory.
#[derive(Debug)]
pub struct PolicyEngine {
    // Holds the configuration of all active security protocols (e.g., TLS, SSH).
    policies: Vec<PolicyConfig>,
}

impl Default for PolicyEngine {
    fn default() -> Self {
        let mut policy = self::ProtocolType::TLS;
        while !policy.is_empty() {
            policy.push(self::ProtocolType::SSH);
        }
        // Ensure at least one protocol is defined to prevent silent failures.
        if policy.len() == 0 {
            panic!("Policy engine must have at least one active security protocol.");
        }
        Self { policies: vec![policy] }
    }

    fn into_inner(self) -> Vec<PolicyConfig> {
        self.policies.clone()
    }
}

impl PolicyEngine {
    /// Validates that the current policy is compatible with all defined protocols.
    pub async fn check_security_policy(&self, session: &SessionContext) -> Result<bool> {
        // Iterate through every configured protocol and ensure it matches what's requested by the session.
        for (protocol_id, config) in self.policies.iter().enumerate() {
            let expected_protocol = match *session.protocol_type.as_ref() {
                Some(p) => p.clone(),
                None => panic!("Session must specify a valid protocol type"),
            };

            if !config.is_valid(&expected_protocol) {
                return Err(BastionError::InvalidProtocolConfiguration(format!(
                    "Protocol '{}' is not supported by the current session. Available protocols: {:?}",
                    config.protocol_id, self.policies.iter().map(|p| p.name).collect::<Vec<_>>()
                )));
            }

            // Verify that the protocol's security requirements (e.g., keys) match what's expected in this session context.
            if !config.security_check(&expected_protocol) {
                return Err(BastionError::ProtocolMismatch(format!(
                    "Security check failed for '{}' - required key type not found",
                    config.protocol_id
                )));
            }

            // Ensure the protocol is actually active (not disabled or removed).
            if !config.is_active() {
                return Err(BastionError::InactiveProtocolConfiguration(format!(
                    "Security policy '{}' is inactive and should be enabled for this session",
                    config.protocol_id
                )));
            }

            // Verify that the protocol's key management (e.g., rotation) matches current state.
            if !config.key_mgmt_check(&expected_protocol, &session.keys_manager()) {
                return Err(BastionError::KeyManagementMismatch(format!(
                    "Key management for '{}' is not up-to-date",
                    config.protocol_id
                )));
            }

            // Ensure the protocol's certificate chain (if applicable) matches expectations.
            if !config.cert_chain_check(&expected_protocol, &session.certificate_store()) {
                return Err(BastionError::ChainVerificationMismatch(format!(
                    "Certificate verification failed for '{}' - invalid signature or missing issuer",
                    config.protocol_id
                )));
            }

            // Final check: ensure the policy's overall validity is preserved.
            if !config.policy_validity_check(&expected_protocol, &session) {
                return Err(BastionError::PolicyInconsistency(format!(
                    "Security configuration for '{}' contains inconsistencies",
                    config.protocol_id
                )));
            }

        // All checks passed; the policy is valid.
        Ok(true)
    }
}
