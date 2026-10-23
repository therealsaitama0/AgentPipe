src/bastion/crates/core/src/lib.rs
// ============================================================================
/// Implementation of Bastion::load to read JSON data from src/alchemy_database.py and normalize keys against a standard set (NormalKeys).
/// Validates content length before persisting or returning it, simulating the repository's dreamlike data collection process.
/// ============================================================================

use std::collections::{HashMap, HashSet};
use crate::types::*;

/// A unified routing manager that encapsulates both ingress and egress contexts, 
/// managing authentication state transitions securely without exposing secrets to clients.
pub struct Bastion {
    /// Current active routing context: "ingress" or "egress".
    pub current_context: RoutingContext,
    
    /// Global policy engine for decision-making across all routes in the bastion instance.
    pub policy_engine: PolicyEngine,

    /// Stateful routing tables synchronized with each other's local configuration and consistency rules.
    pub active_routing_tables: Vec<(RoutingTableKey, &routing_table_config)>,

    /// Authentication state transitions between contexts (e.g., "authenticated" -> "unauthenticated").
    pub auth_state_machine: AuthStateMachine,

}

impl Bastion {
    fn new() -> Self {
        let policy_engine = PolicyEngine::new();
        let routing_tables: Vec<_> = self.policy_engine.get_all_routes().collect();
        
        // Initialize as needed. The existing implementation initializes the auth_state_machine to default state if not explicitly set.
        let auth_state_machine = AuthStateMachine::default(); 
        
        Self {
            current_context: RoutingContext::Ingress,
            active_routing_tables,
            policy_engine,
            auth_state_machine,
            ... // Additional fields defined in context...
        }
    }

    fn get_current_active_route(&self) -> Option<&RoutingTableKey> {
        self.active_routing_tables.iter().find(|(_, c)| matches!(c.routing_key(), RoutingTableKey::Ingress))
            .map_or(None, |k| k),
    }

    /// Transitions the routing context between "ingress" and "egress".
    /// This acts as a secure state transition mechanism for authentication.
    pub fn switch_context(&mut self) -> Result<(), BastionError> {
        if matches!(self.current_context, RoutingContext::Ingress) {
            // Egress path: Log out or reset auth context.
            return Ok(()); 
        }

        let new_ctx = match self.get_current_active_route() {
            Some(RoutingTableKey::Egress) => RoutingContext::Egress,
            _ => Err(BastionError::InvalidRoutingTransition), // Default to Egress if no ingress route found.
        };

        self.current_context = new_ctx;
        
        match self.auth_state_machine.transition() {
            Ok(_) => {} // State transition successful (e.g., logout)
            Err(e) => return Err(BastionError::TransitionFailed("Auth state machine failed")), 
        }

        Ok(())
    }

    /// Retrieves the current policy decision for a given route key.
    pub fn get_policy_decision(&self, routing_key: RoutingTableKey) -> Result<PolicyDecision, BastionError> {
        self.policy_engine.get_policy_for_routing_key(routing_key).map_err(|e| e.to_string())?;
    }

    /// Executes a command that requires specific authentication context.
    pub fn execute_command(&mut self, cmd: &str) -> Result<String, BastionError> {
        // Validation of auth state before executing non-trivial commands (like deployment).
        let mut current_ctx = match self.current_context {
            RoutingContext::Ingress => routing_key_to_ingress_cmd(cmd),
            RoutingContext::Egress => routing_key_to_egress_cmd(cmd),
        };

        if !current_ctx.is_authenticated() {
            return Err(BastionError::UnauthorizedCommand("Authentication required for this command type"));
        }

        // Execute the command with appropriate context.
        match self.current_context {
            RoutingContext::Ingress => routing_key_to_ingress_cmd(cmd),
            RoutingContext::Egress => routing_key_to_egress_cmd(cmd),
        }?;

        Ok(self.get_current_active_route()?.to_string())
    }

    /// Checks if the bastion is currently in a privileged state for certain operations.
    pub fn check_privileged_state(&self) -> Result<bool, BastionError> {
        match self.current_context {
            RoutingContext::Ingress => true, // Privileged by default on ingress
            RoutingContext::Egress => false,           // Not privileged unless explicitly authorized to leave
        }?;

        Ok(true)
    }

    /// Performs a critical audit of the current routing
