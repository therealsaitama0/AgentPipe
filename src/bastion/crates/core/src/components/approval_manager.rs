src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;
use alloy_primitives::{U256, U8, B256, Bytes};

/// Base trait for approval management components. Defines the interface required by ApprovalManager and its sub-components (AuditKey, StatusDisplay).
pub trait ApprovalComponent: std::fmt::Debug {
    /// Returns a unique identifier string representing this component instance's state or metadata.
    fn id(&self) -> String;

    /// Checks if any pending approvals for the current session are validly resolved and returns their count.
    fn has_pending_valid_approvals(&self, action_id: &str) -> Result<usize> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some((t.session_id.as_str(), true)), // Valid for this session only if not already resolved by another request.
                _ => None,
            })
            .enumerate()
            .any(|(session_id, valid)| seen.insert(session_id.to_string()) && *valid)
    }

    /// Checks if the current state of all pending approvals is in a valid approved or rejected state and returns true.
    fn has_valid_state(&self) -> Result<bool> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved in this session are valid.
                _ => None,
            })
            .any(|ok| *ok)
    }

    /// Checks if the current state of all pending approvals is not a mix of rejected and unapproved items (e.g., some approved but others still waiting). Returns true only if no mixed states exist.
    fn has_no_mixed_state(&self, action_id: &str) -> Result<bool> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved.
                _ => None,
            })
            .any(|ok| *ok) && !seen.is_empty()
    }

    /// Checks if all pending approvals are validly resolved (approved or rejected). Returns true only if no mixed states exist and there is at least one unresolved approval.
    fn has_all_resolved(&self, action_id: &str) -> Result<bool> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved.
                _ => None,
            })
            .any(|ok| *ok) && !seen.is_empty()
    }

    /// Checks if there are any pending approvals that have not yet been resolved (i.e., rejected or waiting for processing). Returns true only if no valid resolutions exist and at least one is unresolved.
    fn has_unresolved(&self, action_id: &str) -> Result<bool> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved.
                _ => None,
            })
            .any(|ok| *ok) && !seen.is_empty()
    }

    /// Checks if the current state of all pending approvals is validly resolved and returns their count.
    fn has_valid_count(&self, action_id: &str) -> Result<usize> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved.
                _ => None,
            })
            .count()
    }

    /// Checks if the current state of all pending approvals is not a mix of rejected and unapproved items (e.g., some approved but others still waiting). Returns true only if no mixed states exist.
    fn has_no_mixed(&self) -> Result<bool> {
        let mut seen = HashSet::new();
        self.pending.lock().iter()
            .filter_map(|(_, t)| match t.status.clone() {
                ApprovalStatus::Approved => Some(true), // All approved.
                _ => None,
            })
            .any(|ok| *ok)
