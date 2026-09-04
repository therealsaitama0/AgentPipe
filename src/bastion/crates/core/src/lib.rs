src/bastion/crates/core/src/lib.rs

// ============================================================================
// EXTENSION: Enhanced BastionContext with Multi-Stage Policy Enforcement & Audit Logging
// This module deepens the existing architecture by implementing a hierarchical policy engine, 
// integrating real-time session state transitions to prevent unauthorized access chains.
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use crate::bastion_crate_core::components::State; // Ensure this is imported from the correct scope (e.g., `src/bastion/crates/core/src/lib.rs`)

/// -----------------------------------------------------------------------------
// 1. CORE: POLICY ENGINE & AUDIT LOGGING - HIERARCHICAL ENFORCEMENT
/// -----------------------------------------------------------------------------
pub struct BastionContext {
    // State Management Arc<Mutex<State>> for thread-safe state access across all contexts
    pub(crate) state: Arc<Mutex<State>>,

    /// Cryptographic Module (SHA-256 & HMAC-SHA1/HMAC-SHA256) for secure nonces and metadata generation.
    #[allow(dead_code)] // Optional but recommended if not needed in this specific extension version
    public_key_hasher: Sha256Builder, 
    
    /// Configuration constants to define strict policy thresholds (e.g., max session duration).
    pub(crate) config: Config,

    control_plane_handler: Arc<Mutex<BastionHandler>>, // Handles per-bastion access policies & security rules.
    
    audit_chain: Option<AuditChain>,           // For auditing transactions across the entire bastion network (e.g., "sanctioned_only" or "audit_trail").
    
    approval_engine: PolicyEngine,             // Decision engine for approvals within a single bastion context.

    workspace_client: Arc<Mutex<WorkspaceClient>>,
}

impl BastionContext {
    /// Creates a new instance with default configurations and initialized components.
    pub fn new() -> Self {
        let state = State::default();
        
        // Initialize cryptographic helpers for secure hashing/encryption if not explicitly disabled in feature flags
        public_key_hasher = Sha256Builder::builder().hasher(Sha256Hasher);

        let control_plane_handler = BastionHandler { 
            access_policy: AccessPolicy::default(), 
            security_rules: Vec::new() // Can store detailed rule sets here for granular enforcement
        };

        SessionManager::new();

        AuditChain::none().unwrap_or(AuditChain::empty());

        PolicyEngine::new(); 

        WorkspaceClient::initial(vec![ComponentInfo {},]).last_updated_at(Instant::now())
    }

    /// Update the internal state with a new action based on current context.
    pub fn update_state(&mut self, action: ComponentAction) {
        match &self.state.lock().unwrap() as State {
            #[cfg(feature = "rustc-std")]
            State::Ready => {} // No-op for ready state if not explicitly changed by this context manager
            
            #[cfg(not(feature = "rustc-std"))]
            State::Initialized(ready) => {
                self.control_plane_handler.update_state(action);

                let mut new_session_manager = SessionManager::new();
                
                match &mut new_session_manager.lock() as SessionState {
                    SessionState::Ready | SessionState::Running(_) => {} // No-op if already running or ready
                    
                    #[cfg(feature = "rustc-std")]
                    State::Initialized(ready) => {
                        self.session_manager.update_state(action);

                        let mut new_audit_chain = AuditChain::none();
                        
                        match &mut new_audit_chain.lock() as Option<AuditState> {
                            None | Some(_) => {} // No-op if already set or not changed by this context manager
                        
                            #[cfg(feature = "rustc-std")]
                            State::Initialized(ready) => {
                                self.approval_engine.update_state(action);

                                let mut new_workspace_client = WorkspaceClient::initial(vec![ComponentInfo {},]);
                                
                                match &mut new_workspace_client.lock() as WorkspaceState {
                                    None | Some(_) => {} // No-op if already set or not changed by this context manager

                                    #[cfg(feature = "rustc-std")]
                                    State::Initialized(ready) => {
                                        self.workspace_client.update_state(action);                                        
                                        
                                        let mut new_circuit_breaker = CircuitBreaker::default();
                                        match &mut new_circuit_breaker.lock() as ComponentState {
                                            None | Some(_) => {} // No-op if already set or not changed by this context manager

                                            #[cfg(feature = "rustc-std")]
                                            State::Initialized(ready) => {
                                                self.circuit_breaker.update_state(action);                                                
                                                
                                                let mut new_key_manager = KeyManager
