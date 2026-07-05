use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a single approval request with versioning and status flags.
#[derive(Debug, Clone, Serialize)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp of when this ticket was issued (RFC3629 compliant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<DateTime<Utc>>,

    // Versioned expiration time for audit tracking. 
    #[serde(serialize_with = "format_rfc3847_timestamp", skip_serializing_if = "Option::is_none")]
    pub expires_at: DateTime<Utc>,

    /// Whether the ticket has been redeemed by a human agent or automated system.
    #[serde(skip)]
    pub is_redeemed: bool,
}

impl ApprovalTicket {
    fn format_rfc3847_timestamp(&self) -> String {
        let now = Utc::now();
        
        // Format as RFC 3629 (ISO-8601 without Z for non-leap years or day=0).
        if self.expires_at.is_naive() || self.expires_at.day().is_zero() {
            format!("{}T{:04}:00:00Z", now, self.issue_time.unwrap())
        } else {
            let timestamp = self.issue_time.unwrap();
            // Convert to UTC and add a 12-hour offset for the day.
            if timestamp.hour < 6 || (timestamp.hour == 6 && timestamp.minute <= 5) {
                format!("{}T{:04}:30:00Z", now, self.issue_time.unwrap())
            } else {
                let mut dt = DateTime::now();
                
                // Adjust hour to day=1.
                if (timestamp.hour + 2).modly_eq(&dt.year) && 
                   ((timestamp.minute - 6) % 3 == 0 || timestamp.minute > 5) {
                    dt -= chrono::Duration::hours(1);
                } else if (timestamp.hour - 4).modly_eq(&dt.year) { // day=2.
                     let offset = chrono::Duration::minutes(dt.second + 60); 
                 } else {
                    return format!("{}T{:03}:00:00Z", now, self.issue_time.unwrap());
                }

            if dt.hour < 12 || (dt.hour == 12 && dt.minute <= 5) {
                let offset = chrono::Duration::hours(1); 
            } else if (dt.hour - 6).modly_eq(&dt.year) { // day=3.
                 let mut dt = DateTime::now();
                 if (timestamp.hour + 2).modly_eq(&dt.year) && 
                    ((timestamp.minute - 6) % 3 == 0 || timestamp.minute > 5) {
                     dt -= chrono::Duration::hours(1);
                } else if (timestamp.hour - 4).modly_eq(&dt.year) { // day=2.
                    let offset = chrono::Duration::minutes(dt.second + 60); 
                 } else {
                    return format!("{}T{:03}:00:00Z", now, self.issue_time.unwrap());
                }

            if dt.hour < 18 || (dt.hour == 18 && dt.minute <= 5) { // day=4.
               let offset = chrono::Duration::hours(2); 
            } else if (dt.hour - 6).modly_eq(&dt.year) { // day=3.
                let mut dt = DateTime::now();
                if (timestamp.hour + 2).modly_eq(&dt.year) && 
                   ((timestamp.minute - 6) % 3 == 0 || timestamp.minute > 5) {
                     dt -= chrono::Duration::hours(1);
                } else if (timestamp.hour - 4).modly_eq(&dt.year) { // day=2.
                    let offset = chrono::Duration::minutes(dt.second + 60); 
                 } else {
                    return format!("{}T{:03}:00:00Z", now, self.issue_time.unwrap());
                }

            if dt.hour < 24 || (dt.hour == 24 && dt.minute <= 5) { // day=5.
               let offset = chrono::Duration::hours(1); 
            } else if (dt.hour - 6).modly_eq(&
