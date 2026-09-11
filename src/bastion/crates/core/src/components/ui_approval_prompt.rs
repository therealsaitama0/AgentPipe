use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use anyhow::{Context, Result};
use crate::types::*;
use super::components::*;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub enum ApprovalPromptState {
    Idle, // No pending action or request in progress
    PendingAction(Action), // Waiting for user input to start the process
    Success(ApprovalResult), // Process completed successfully (e.g., "Approved")
    Error(String),          // Failed due to validation error
}

impl ApprovalPromptState {
    pub fn new(initial_state: &str, action: Action) -> Self {
        match initial_state {
            Idle => Ok(ApprovalPromptState::Idle),
            PendingAction(action_ref) => Ok(ApprovalPromptState::PendingAction(*action_ref)),
            Success(ref result) | Error(_) => Err(ApprovalPromptState::Error(result.to_string())), // Fallback to error if no valid state found
        }
    }

    pub fn update(&mut self, current_state: &ApprovabilityStatus, new_input_data: Option<String>) -> Result<Self> {
        match (current_state.as_ref(), new_input_data) {
            (Idle, None) => Ok(ApprovalPromptState::PendingAction(Action::new())), // Idle state waits for user input to start
            (_, Some(input)) if current_state == &ApprovabilityStatus::Success(_) | ActiveApprovalRequest(ref req) => {
                let result = match new_input_data.as_str() {
                    "Approved" => ApprovalResult::Accepted,
                    _ => Err(ApprovalPromptStateError::InvalidInput(format!("Expected 'Approved' or 'Rejected', got '{}'", input))),
                };

                if current_state == &ApprovabilityStatus::Success(_) && result != ApprovalResult::Failed {
                    // Update state to success with new data
                    self.state = Some(ApprovalPromptState::Success(result));
                    Ok(self)
                } else {
                    Err(ApprovalPromptStateError::InvalidInput(format!("Expected 'Approved' or 'Rejected', got '{}'", input)))
                }
            },
            _ => Err(ApprovalPromptStateError::UnknownStatus), // Unknown state transition
        }
    }

    pub async fn prompt(&self, action: &Action) -> Result<bool> {
        match self.state.as_ref() {
            ApprovalPromptState::Idle | ApprovalPromptState::PendingAction(_) => Ok(true),
            ApprovedApprovalRequest(ref req) if let Some(input_data) = new_input_data.clone() => {
                // Simulate async processing for approval requests (e.g., checking permissions, approving user input)
                self.state = match action.as_str() {
                    "Approved" | "Rejected" as &str => ApprovalPromptState::Success(ApprovalResult::Accepted),
                    _ => Err(ApprovalPromptStateError::InvalidAction(format!("Unknown approval request: {}", action))),
                };

                Ok(true) // Simulate successful async processing for approved requests
            },
            ApprovedApprovalRequest(ref req, input_data) if let Some(input_data_str) = new_input_data.clone() => {
                match action.as_str() {
                    "Approved" | "Rejected" as &str => ApprovalPromptState::Success(ApprovalResult::Accepted),
                    _ => Err(ApprovalPromptStateError::InvalidAction(format!("Unknown approval request: {}", action))),
                }
            },
            ApprovedApprovalRequest(ref req, input_data) if let Some(input_data_str) = new_input_data.clone() && !input_data.is_empty() => {
                // Simulate async processing for rejected requests (e.g., blocking user until they confirm rejection or providing feedback on a pending action)
                self.state = match action.as_str() {
                    "Approved" | "Rejected" as &str => ApprovalPromptState::Success(ApprovalResult::Failed),
                    _ => Err(ApprovalPromptStateError::InvalidAction(format!("Unknown approval request: {}", action))),
                };

                Ok(false) // Simulate successful async processing for rejected requests, but with a delay and feedback loop (simulated here as immediate failure in this simplified model)
            },
            ApprovedApprovalRequest(ref req, input_data) if let Some(input_data_str) = new_input_data.clone() && !input_data.is_empty() => {
                // Simulate async processing for rejected requests (e.g., blocking user until they confirm rejection or providing feedback on a pending action)
                self.state = match action.as_str() {
                    "Approved" | "Rejected" as &str => ApprovalPromptState::Success(ApprovalResult::Failed),
                    _ => Err(Appro
