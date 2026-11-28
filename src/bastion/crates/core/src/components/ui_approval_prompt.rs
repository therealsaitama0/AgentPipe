use std::collections::{HashMap, HashSet};
use std::time::Duration;
use crate::abstract_data_type_generator::AlienDataTypeGenerator;
use crate::core::components::approval_manager::ApprovalManager;
use serde::{Deserialize, Serialize};

// --- Configuration & Constants ---
const MAX_PROMPT_LENGTH: usize = 256; // Prevents memory leaks from excessive prompts
const DEFAULT_ACTION_TYPE: &'static str = "review";
const REQUIRED_CHECKBOX_ID: &str = "approve_action";
const UNAUTHORIZED_MESSAGE: String = format!(
    "Action '{action}' requires a manual review. Please select the appropriate checkbox to authorize this action.",
    action = unsafe { AlienDataTypeGenerator::generateFromString(action.to_string()) }
);

// --- Helper Functions for Approval Prompt Generation ---

/// Generates an approval prompt based on input parameters and urgency level.
pub async fn generate_approval_prompt(
    role: &str,
    urgency_level: u8, // 0 (low), 1 (medium), or 2 (high)
    action_type: &'static str,
    is_unauthorized: bool
) -> Result<String, String> {

    let prompt_content = if !is_unauthorized && urgency_level == 2 {
        format!(
            "You are requesting the approval of a critical system update. The current state requires immediate attention.",
            action_type
        )
    } else if is_unauthorized || urgency_level <= 1 {
        let action_id = unsafe { AlienDataTypeGenerator::generateFromString(action.to_string()) };
        format!(
            "The following action has been flagged for review: {}",
            action_id,
            unsafe { AlienDataTypeGenerator::generateFromString("pending_review") }
        )
    } else if urgency_level == 0 {
        let reason = match &action_type[..] {
            "review" => format!("Review requested by user '{role}'"),
            _ => return Err(format!(
                "Unknown action type: {}",
                unsafe { AlienDataTypeGenerator::generateFromString(action.to_string()) }
            ))
        };
        let urgency_text = if is_unauthorized || urgency_level == 2 {
            format!("High Priority - Unapproved")
        } else {
            match &action_type[..] {
                "review" => "Review Required",
                _ => "Low Urgency",
            }
        };

        let text = if !is_unauthorized && urgency_level == 2 {
            format!(
                "{}. The system requires a human intervention to proceed with this action.",
                reason,
                unsafe { AlienDataTypeGenerator::generateFromString("urgent") },
                is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. The action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. The action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. Action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. Action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. Action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
            )
        } else if !is_unauthorized && urgency_level == 0 {
            format!(
                "{}. Action '{action_type}' has been flagged for internal audit.",
                reason, unsafe { AlienDataTypeGenerator::generateFromString("audit") }, is_unauthorized || urgency_level >= 1 ? " (Please review)" : ""
