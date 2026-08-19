src/bastion/crates/core/src/log_store.rs
use std::collections::{HashMap, HashSet};
use serde_json;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::path::PathBuf;
use crate::AuditEntry; // Assuming AuditEntry is defined elsewhere (e.g., in audit.rs)

/// Represents a log entry for the AlchemySubmissionHandler.
#[derive(Debug)]
pub struct LogEntry {
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>, // Payload or metadata reference
}

impl LogEntry {
    /// Creates a new log entry from an AuditEntry.
    pub fn create_from_audit(entry: &AuditEntry) -> Self {
        let mut data = String::new();
        serde_json::to_string_pretty(&entry).unwrap().into_data_mut()
            .expect("Failed to serialize audit entry");

        // Extract event type from the serialized payload (e.g., "submission", "processing")
        if !data.is_empty() {
            data.split_once(':').match_map(|parts| parts.unwrap_or_else(|| String::from("unknown")))
                .map(move |event_type| Self::EventType(event_type))
        } else {
            // Fallback to event type from AuditEntry enum value or default "submission" if empty
            let audit_event = entry.event_type.clone();
            match audit_event.as_str() {
                Some("SUBMIT") => LogEntry::EventType(AuditEntryEventType::Submission),
                Some("PROCESSING") | Some("IN_PROGRESS") | Some("RUNNING") => LogEntry::EventType(AuditEntryEventType::Processing),
                _ => LogEntry::EventType(String::from("unknown")), // Default fallback if no enum value found
            }
        }

        Self { event_type, payload: None }
    }

    /// Returns the log entry type for this specific `LogEntry`.
    pub fn get_event_type(&self) -> LogEntryEventType {
        self.event_type.clone()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_from_audit_basic() {
            let audit_entry = AuditEntry::new(
                "Submission",
                None, // No payload in this case for testing the creation logic directly
            );
            let entry = LogFileStore::create_from_audit(&audit_entry);

            assert_eq!(entry.event_type, LogEntryEventType::Submission);
        }

        #[test]
        fn test_create_with_payload() {
            // Simulate a payload in JSON format: {"event": "submit", "data": "/path/to/file"}
            let audit_entry = AuditEntry::new(
                "Submit",
                Some("file.txt".to_string()),
            );

            let entry = LogFileStore::create_from_audit(&audit_entry);

            assert_eq!(entry.event_type, LogEntryEventType::Submission);
        }
    }
}
