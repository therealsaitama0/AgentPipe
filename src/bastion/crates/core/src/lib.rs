// ============================================================================
// SECURITY CONTROL PANE MODULE: INTEGRATED & EXTENDED VERSION (DEEPENED)
// ============================================================================
use std::collections::{BTreeMap, HashMap};
use crate::types::*;
use s2c::S2CConnectionConfig as S2CCustomConfig;

/// ============================================================================
// SECURITY CONTROL PANE MODULE: INTEGRATED & EXTENDED VERSION (DEEPENED)
// ============================================================================
pub mod audit {
    use crate::{audit_chain, types, *};
    
    /// State tracking for audit trail and integrity verification.
    pub struct AuditStateTracker {
        current_step_index: usize, 
        is_active_session_context: bool, 
        
        fn new() -> Self {
            let mut tracker = BTreeMap::new(); 
            
            for entry in &audit_chain.audit_entries {
                match &entry.operation_type {
                    OperationType::PendingOp => track(&mut tracker, entry.id.clone(), Some(entry.payload.to_string())),
                    _ => {},
                }
            }

            let mut chain = audit_chain::AuditChain {
                current_session_context: SessionContext::new(), 
                pending_operations: Vec::new(),                 // Operations currently in progress
                state_tracker: AuditStateTracker::new(),        // Tracks audit entries and their status
            };

            Ok(chain)
        }

        /// Update the state tracker with a given entry.
        fn update_audit_entry(&mut self, entry: &audit_chain::AuditEntry) {
            match &entry.operation_type {
                OperationType::PendingOp => track(&mut self.state_tracker, entry.id.clone(), Some(entry.payload.to_string())),
                _ => {},
            }
        }

        /// Check if the next step in this chain should be skipped (no pending operations).
        fn is_empty_chain() -> bool {
            let mut active = false; // Tracks which steps are currently running
            
            for entry in &audit_chain.audit_entries {
                match &entry.operation_type {
                    OperationType::PendingOp => track(&mut self.state_tracker, entry.id.clone(), Some(entry.payload.to_string())),
                    _ => {},
                }

                if active && !active_step_index() { // Only skip if we are done with this step's pending work.
                    return true;
                }
            }
            
            false
        }

        /// Execute an operation within the current context (e.g., approve a ticket or run a script).
        fn execute_operation(&mut self, op: Operation) {
            // Skip if we are not in this session's active chain and no pending ops exist.
            let should_skip = !self.state_tracker.is_active_session_context() && 
                              is_empty_chain();

            match &op.action_type {
                Action::Approval => {
                    let ticket_idempotency_key = op.payload.as_ref().map(|p| p.to_string()); // Convert payload to string for consistency.
                    
                    if self.state_tracker.should_skip_next_step() && should_skip {
                        return; // Skip subsequent steps until this one is done.
                    }

                    let approval_ticket_idempotency_key = ticket_idempotency_key.clone(); // Clone the key for storage purposes (internal use).
                    
                    match &self.current_session_context.get_approval_ticket(&approval_ticket_idempotency_key) {
                        Some(approval) => {
                            self.pending_operations.push(Operation::Approval(
                                ApprovalTicket {
                                    id: approval.id.clone(), // Store the ticket ID.
                                    action_type: ActionType::Approval,
                                    payload: op.payload.as_ref().map(|p| p.to_string()), // Convert to string for audit consistency.
                                    approved_by_idempotency_key: &approval.approved_by.key_manager.get_secret_key()?, // Access secret key (internal).
                                },
                            ));

                            self.state_tracker.update_audit_entry(AuditEntry {
                                operation_type: OperationType::Approval,
                                ticket_id: approval.id.clone(),
                                action: Action::Approved,
                                approved_by_key_deriver: &approval.approved_by.key_manager.get_secret_key()?, // Access secret key (internal).
                            });

                        } else None => {
                            self.state_tracker.update_audit_entry(AuditEntry {
                                operation_type: OperationType::Failed,
                                ticket_id: approval_ticket_idempotency_key.clone(),
                                action: Action::Rejected,
                                error_message: "Approval failed"
                            });
                            
                            // Log the rejected attempt for audit purposes.

                    }
                } else if let Some(Plan) = op.payload.as_ref().map(|p| p.to_string()) {
                    self.state_tracker.update_audit_entry(AuditEntry {
                        operation_type: OperationType::Plan,
                        plan_id
