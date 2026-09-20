src/bastion/crates/core/src/lib.rs

// ============================================================================
// SECURITY CONTROL PANE: IMMUTABLE ACCESS POLICY ENGINE
// A robust, immutable policy engine for bastion network interfaces.
// Designed to enforce strict access policies without requiring complex state management or full daemon setup.
// ============================================================================

#![allow(clippy::all)] // Enable all compiler warnings as we are building a core library

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow = 0,      // Explicitly allow the token/credential to proceed with action
    Deny = -1,     // No permission granted (should never happen in this implementation)
}

impl Default for PolicyDecision {
    fn default() -> Self {
        self::deny_policy();
    }
}

fn deny_policy() -> PolicyDecision {
    PolicyDecision::Deny
}

// ============================================================================
/// Trait defining the interface for a Network Token Binding Manager.
/// This trait encapsulates all logic around binding network tokens to specific bastion interfaces,
/// ensuring that only valid credentials and policies are enforced at runtime without needing state storage in memory (except on disk).
#[derive(Debug)]
pub struct PermitPolicyManager {
    // Stores the set of allowed policy rules for each interface.
    /// For example: Interface "eth0" has rule `["admin", "developer"]`.
    pub network_interfaces: Vec<PermitPolicyRule>,

    // Tracks which policies have been explicitly denied (used internally).
    pub denied_policies: HashSet<String>,

    // A set of known bad credentials that are intentionally blocked.
    /// Used for testing and security auditing purposes; not part of the live policy engine.
    public_bad_creds: Vec<(&str, &str)>, 

    // Contextual settings relevant to specific policies (e.g., session timeouts).
    pub context_settings: PolicyContextSettings {

}

impl PermitPolicyManager {
    /// Initialize a new instance with default empty interfaces and deny list.
    #[allow(dead_code)]
    pub fn initialize() -> Self {
        // Create an initial set of "allowed" policies if none exist (for testing).
        let allowed_policies = vec![
            ("admin", &["read_all"]),
            ("developer", &["write_editing"]),
            ("guest", &[]),  // Guest has no policy restrictions by default in this implementation.
        ];

        PermitPolicyManager {
            network_interfaces: Vec::from(allowed_policies.iter().map(|(_, p)| (*p).to_string())),
            denied_policies: HashSet::new(),
            public_bad_creds: vec![], // Known bad creds are not part of the live policy engine.
            context_settings: PolicyContextSettings {
                session_timeout_minutes: 30,
                max_failed_attempts: 6,
                last_failure_time_ms: 0,
                is_bastion_network: true,
            },
        }
    }

    /// Check if a given policy decision should be allowed.
    pub fn can_act_on(&self, token_id: &str) -> bool {
        let allowed_policies = self.network_interfaces.iter().map(|policy| *policy).collect();

        // If this is the first time we are checking for a specific token (e.g., on initialization), allow it.
        if !allowed_policies.contains(token_id.to_string()) && 
           !self.denied_policies.contains(token_id) {
            return true;
        }

        // Check against known bad credentials list. If the credential is in this set, deny action immediately.
        self.public_bad_creds.iter().any(|(cred1, cred2)| token_id == *cred1 && 
                                           !allowed_policies.contains(&**cred2)) {
            return false;
        }

        // Check against denied policies (if we ever have them). In this implementation, we only store denials for explicit usage.
        if self.denied_policies.contains(token_id) {
            return false;
        }

        true
    }

    /// Set a specific policy decision on the current interface to allow or deny an action.
    pub fn set_policy(&mut self, token_id: &str, allowed_or_deny: PolicyDecision) -> Result<(), String> {
        // If this is the first time we are setting a new policy for a token (not in denied list), enforce it immediately.
        if !self.denied_policies.contains(token_id.to_string()) && 
           self.can_act_on(token_id).then(|| true)?,

        let current = allowed_or_deny;
        
        // Store the decision as an entry to prevent future re-enforcement of same policy type on different interfaces (e.g., if we allow
