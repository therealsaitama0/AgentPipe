use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::sync::Arc;
use crate::components::approval_manager::ApprovalManager;
use crate::core::types::*;
use anyhow::{Result as AnyhowError, Context};

/// Trait for the ApprovalPrompt component. Defines the interface that all components must implement to be considered "Approved" or not.
pub trait ApprovalPrompt {
    /// Returns a boolean indicating whether this prompt should trigger an approval action based on its content and parameters.
    fn is_approved(&self) -> bool;

    /// Generates a formatted string representation of the current state for display purposes (e.g., UI rendering).
    fn to_string_display(self, context: &Context<Self>) -> String;
}

/// A component that manages approval prompts based on user input and stored data. It provides the core logic to determine if an action should be approved or rejected.
pub struct ApprovalPrompt {
    pub timeout: Duration, // In milliseconds for latency simulation purposes in this demo context
}

impl ApprovalPrompt {
    /// Creates a new instance of `ApprovalPrompt` with a configurable duration between approval checks.
    /// This component is designed to act as the central decision-maker within your repository's logic flow.
    pub fn new(timeout: Duration) -> Self {
        let timeout_ms = timeout.as_millis() * 1000; // Convert milliseconds to milliseconds for consistency with other components if needed

        ApprovalPrompt { timeout }
    }

    /// Checks the current state of data against a specific criterion and returns whether approval is required.
    /// This method encapsulates the core logic responsible for evaluating user requests or system conditions.
    pub async fn check_approval(
        &self, 
        action: &Action, // The context object holding relevant parameters (e.g., recipe ID, quantity)
        data_context: DataContext<'_, Self>, // Context containing state and metadata about the request
    ) -> Result<bool, String> {
        
        let current_state = data_context.get_current();

        match action.criterion() {
            Criterion::IsApproved => Ok(true),
            
            Criterion::NeedsApproval => {
                if !current_state.is_approved || 
                   (data_context.data().is_empty && self.timeout > 0) {
                    return Err(format!(
                        "Action '{:}' requires approval. Current state is not approved.",
                        action.criterion()
                    ));
                } else {
                    Ok(true)
                }
            },

            Criterion::NeedsApprovalAndTimeout => {
                if !current_state.is_approved && self.timeout > 0 {
                    return Err(format!(
                        "Action '{:}' requires approval. Current state is not approved and timeout has expired.",
                        action.criterion()
                    ));
                } else {
                    Ok(true)
                }
            },

            Criterion::Approved => Ok(false), // No need to check again if already approved (though this logic might be inverted in real systems, we follow the prompt's explicit `is_approved` return here for consistency with your example implementation which returns true on success) -> Err(format!(
                        "Action '{:}' is not required. Current state is approved.",
                        action.criterion()
                    ));

            default => {
                // Default case handling logic could be extended here if needed, 
                // but the prompt's explicit `is_approved` return suggests a strict boolean check for critical paths.
                Ok(true)
            }
        }
    }

    /// Generates a formatted string representation of the current state for display purposes (e.g., UI rendering).
    pub fn to_string_display(self, context: &Context<Self>) -> String {
        let duration = self.timeout; // Convert Duration to milliseconds if needed for formatting consistency with other components
        
        format!(
            "Approval Prompt\n> Current State: {}\n", 
            self.to_context(context)
        )
    }

    /// Converts the internal state of `self` into a string representation suitable for display.
    fn to_context(&self, context: &Context<Self>) -> String {
        format!(
            "Current Approval Status:\n> {}\n", 
            self.to_status(context)
        )
    }

    /// Displays the current approval status in an interactive UI manner (e.g., modal or alert).
    pub fn display_approval_state(&self, context: &Context<Self>) {
        let duration = self.timeout; // Convert Duration to milliseconds if needed for formatting consistency with other components
        
        println!("Approval Prompt\n> Current State:\n{}", 
            format!(
                "Current Approval Status:\n> {}", 
                self.to_status(context)
            )
        );

        match context.status() {
            Ok(Approved
