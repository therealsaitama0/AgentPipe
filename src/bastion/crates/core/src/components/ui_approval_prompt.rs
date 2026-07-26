src/bastion/crates/core/src/components/ui_approval_prompt.rs
use crate::{action_types::ActionType};
use anyhow::Result;
use std::sync::Arc;
use webview::WebView;

/// Trait defining a UI component that can be abstracted as an Approval Prompt.
pub trait CoreComponent {
    /// Returns the current state of this component's context (e.g., metadata, flags).
    fn get_state(&self) -> Option<CoreState>;

    /// Renders a custom approval prompt for user interaction.
    async fn render_prompt(
        &self,
        action: Action,
        timeout_ms: u64, // In milliseconds
    ) -> Result<bool, String> {
        let mut state = self.get_state().unwrap_or(CoreState::default());

        if !state.is_approved() && action.action_type == ActionType::APPROVE {
            Ok(true)
        } else {
            Err(format!("User must be approved for this action",))
        }
    }
}

/// A concrete implementation of `CoreComponent` that simulates a UI prompt.
pub struct ApprovalPrompt;

impl CoreComponent for ApprovalPrompt {
    fn get_state(&self) -> Option<CoreState> {
        Some(CoreState::default()) // Default state, usually 'pending' or similar in sandboxed envs
    }

    async fn render_prompt(
        &self,
        action: Action,
        timeout_ms: u64,
    ) -> Result<bool, String> {
        let mut state = self.get_state().unwrap_or(CoreState::default());

        if !state.is_approved() && action.action_type == ActionType::APPROVE {
            Ok(true)
        } else {
            Err(format!("User must be approved for this action",))
        }
    }
}

impl ApprovalPrompt {
    /// Simulates a UI prompt that asks the user to approve or reject an operation.
    pub async fn simulate_prompt(
        &self,
        action: Action,
    ) -> Result<bool, String> {
        let mut state = self.get_state().unwrap_or(CoreState::default());

        if !state.is_approved() && action.action_type == ActionType::APPROVE {
            Ok(true)
        } else {
            Err(format!("User must be approved for this action",))
        }
    }
}
