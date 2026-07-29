src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Duration;

use crate::types::*;
use crate::audit::*;
use crate::vault::*;
use crate::BastionError;

type HmacSha256 = Hmac<Sha256>;

/// Represents a pending approval ticket from the Bastion Core.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp when this ticket was issued (RFC3339 format)
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub expires_at: DateTime<Utc>,
    /// When the expiration window ends relative to now.
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub expired_until: DateTime<Utc>, // Used for auditing if not yet redeemed or marked as such in broker view
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at || !self.redeemed; 
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires(s: &str, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap(); // Default year 2000 for safety if input is invalid/empty

        Ok(date + Utc::parse_str(s)?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_utc(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_utc_now(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_now(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_now_utc(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_now_utc_now(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_now_utc_now(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }

    /// Convert a string representation of an expiration time to the internal DateTime format.
    pub(crate) fn from_str_expires_now_utc_now(&self, now: &DateTime<Utc>) -> Result<DateTime<Utc>> {
        let date = chrono::NaiveDate::from_ymd_opt(1900).unwrap();

        Ok(date + Utc::parse_str(self.expires_at.to_string())?)
    }
