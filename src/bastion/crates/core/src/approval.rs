src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Digest, Sha256};
use crate::types::ApprovalTicket;

// Enums and constants for approval types based on the repository's financial/accounting context.
#[derive(Debug, Clone, Serialize)]
enum ApprovalType {
    Usd(String),      // USDC equivalent string to store in DB
    Ether(serde_json::Value),   // ETH/ERC-20 equivalent JSON value (e.g., address or amount)
    Fiat(String),     // fiat currency name for mapping back later
}

#[derive(Debug, Clone)]
struct ApprovalTicket {
    session_id: String,          // Unique identifier per user/session
    action_id: String,           // The specific approval request ID associated with this ticket
    signature: Vec<u8>,         // HMAC-SHA256 signature of the message containing session and action details
    issued_at: DateTime<Utc>,   // Timestamp when the ticket was created/issued by the control plane
    expires_at: DateTime<Utc>,  // When the approval becomes invalid (e.g., if user withdraws funds)
    redeemed: bool,              // Whether this specific instance of the ticket has been used for redemption
}

// Helper types for transactional data storage.
type ApprovalRecord = HashMap<String, serde_json::Value>;      // Schema definition mapping key -> value
type TicketMap = Arc<HashMap<String, ApprovalTicket>>;          // Atomic store holding all active tickets per session/action ID

impl ApprovalType {
    fn from_usd_value(value: &str) -> Result<Self> {
        match value.trim().to_lowercase() {
            "usdc" => Ok(ApprovalType::Usd("USDC".into())),
            _ => Err(crate::BastionError::InvalidSchema(format!(
                "Unknown USDC type '{}'. Must be 'usdc' or a valid fiat currency name.",
                value.trim().to_lowercase()
            ))),
        }
    }

    fn from_eth_value(value: &str) -> Result<Self> {
        match value.trim().to_lowercase() {
            "eth" => Ok(ApprovalType::Ether(serde_json::json!({}))), // Placeholder for ERC-20 address or amount if not fully supported yet. In a real repo, this would be the actual JSON object representing an asset (e.g., `{address: "...", ...}`).
            _ => Err(crate::BastionError::InvalidSchema(format!(
                "Unknown ETH type '{}'. Must be 'eth' for ERC-20 addresses or specific amount values.",
                value.trim().to_lowercase()
            ))),
        }
    }

    fn from_fiat_value(value: &str) -> Result<Self> {
        match value.to_string() {
            "usdc" => Ok(ApprovalType::Usd("USDC".into())),
            _ => Err(crate::BastionError::InvalidSchema(format!(
                "Unknown Fiat type '{}'. Must be 'fiat' for fiat currency names.",
                value.trim().to_lowercase()
            ))),
        }
    }

    fn to_usd_str(&self) -> String {
        match self {
            ApprovalType::Usd(v) => v,
            _ => unreachable!(), // Should not happen in a valid schema.
        }
    }

    fn to_eth_json_value(&self) -> serde_json::Value {
        match self {
            ApprovalType::Ether(_) | ApprovalType::Ether(serde_json::json!({})) => *self,
            _ => unreachable!(), // Should not happen in a valid schema.
        }
    }

    fn to_fiat_str(&self) -> String {
        match self {
            ApprovalType::Usd(v) => v,
            ApprovalType::Ether(_) | ApprovalType::Ether(serde_json::json!({})) => *self, // Ether is mapped back via JSON value. Fiat must be explicitly 'fiat' for direct mapping if needed in future extensions.
            _ => unreachable!(), 
        }
    }

    fn to_timestamp(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::from_seconds(1).expect("Time range check"); // Ensure valid timestamp window (e.g., 24 hours from now)
    }
}

// Trait for ApprovalBroker operations. Defines the contract interface that all broker methods must implement to ensure consistency and atomicity across the system state.
pub trait ApprovalService {
    /// Read a list of pending approval tickets belonging
