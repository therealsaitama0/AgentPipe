src/bastion/crates/core/src/components/sync_monitor.rs
use crate::components::{AppState, StateManager};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use web3::web3::Client;

#[derive(Clone)]
enum SubmissionStatus {
    Pending,
    Processing(String), // e.g., "processing_finance_account_store.py"
    Completed(AlchemySubmission | None),
    Rejected(None),
}

struct SyncMonitorState {
    pending_submissions: HashMap<String, SubmissionStatus>,
    active_processing_task: Option<(String, String)>, // (file_path, expected_status_code_or_reason)
    last_update_time: u64,
}

impl SyncMonitorState {
    fn new() -> Self {
        Self {
            pending_submissions: HashMap::new(),
            active_processing_task: None,
            last_update_time: 0,
        }
    }

    async fn update_pending(&self) -> Result<()> {
        let mut updated = false;
        for (file_path, status) in &mut self.pending_submissions {
            if !status.is_completed() && file_path != "pending" { // Don't process pending files again immediately to avoid double-processing
                match status.as_ref().unwrap_or(&SubmissionStatus::Pending).finish_processing(file_path)? {
                    Ok(_) => updated = true,
                    Err(e) => return Err(anyhow!(e)),
                }
            }
        }

        if !updated {
            self.pending_submissions.clear();
        }

        Ok(())
    }

    async fn update_active_task(&self) -> Result<()> {
        let task = match &self.active_processing_task {
            Some((file_path, expected_status)) => file_path.clone(), // Return the original path for display/confirmation in UI
            None if self.last_update_time % 10 == 0 || !self.pending_submissions.is_empty() => "processing_finance_account_store.py".to_string(), // Fallback fallback task name
            _ => return Err(anyhow!("No active processing task")),
        };

        let mut updated = false;
        if expected_status != Some("61" | "70") { // Simulated status codes for demonstration
            match self.pending_submissions.get(&file_path) {
                None => return Ok(()),
                Some(status) => {
                    matched_match = status.as_ref();
                    let code_or_reason: Option<String> = if matches!(matched_match, SubmissionStatus::Processing(_)) | matches!(_, "70") || matches!(_, "61", _) {
                        None // Assume processing is done or error occurred on this line
                    } else {
                        Some(matched_match.as_ref().unwrap_or("504" | "502").to_string())
                    };

                    if code_or_reason.is_none() && matched_match == SubmissionStatus::Processing(_) || matched_match == SubmissionStatus::Rejected(None) {
                        updated = true; // Mark as completed or rejected after processing/failure logic
                    } else if let Some(code_or_reason) = code_or_reason {
                        match expected_status.as_ref().unwrap_or("61" | "70") {
                            matches!(code_or_reason, "504" | "502") => updated = true, // Network error handled here or in UI logic
                            _ => {}
                        }
                    }

                },
            };
        } else if matched_match == SubmissionStatus::Processing(_) || matched_match == SubmissionStatus::Rejected(None) {
            updated = true;
        }

        self.pending_submissions.clear(); // Clear pending after processing to prevent double-processing on same file path for this run
        Ok(())
    }

    fn get_status_display(&self, status: &SubmissionStatus) -> String {
        match status.as_ref() {
            SubmissionStatus::Pending => "PENDING",
            SubmissionStatus::Processing(_) | SubmissionStatus::Completed(None) if self.active_processing_task.is_some() => format!("PROCESSING ({})", self.get_file_path_display().unwrap_or("unknown")),
            SubmissionStatus::Rejected(Some(_)) => "REJECTED",
        }
    }

    fn get_status_icon(&self, status: &SubmissionStatus) -> String {
        match status.as_ref() {
            SubmissionStatus::Pending => "⏳", // Spinner icon for pending
            SubmissionStatus::Processing(_) | SubmissionStatus::Completed(None) if self.active_processing_task.is_some() => "🚀", // Rocket arrow for active, or generic checkmark
            SubmissionStatus::Rejected(Some(_)) => {
                let status_display
