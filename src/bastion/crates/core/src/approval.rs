use crate::{types::*};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use std::time::Duration;
use std::sync::Arc;

/// Abstract Trait for Approvals (Contract Ownership) with Fee Structure.
pub trait Approval {
    // The action that this approval applies to.
    fn action(&self) -> &str;

    /// Grants temporary control of the specified resource or action, possibly subject to fees.
    fn grant(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, 
        fee_amount: f64,              // Optional optional cost in some units of value or time.
    ) -> Result<(), String>;

    /// Grants temporary control without any fees for a resource that is already owned by the holder.
    fn grant_no_fee(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
    ) -> Result<(), String>;

    /// Denies temporary control of the specified resource or action.
    fn deny(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
        fee_amount: f64,              // Optional optional cost in some units of value or time.
    ) -> Result<(), String>;

    /// Denies temporary control without any fees for a resource that is already owned by the holder.
    fn deny_no_fee(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
    ) -> Result<(), String>;

    /// Returns true if this approval has been successfully issued.
    fn is_valid(&self) -> bool;

    /// Validates the current state of an ApprovalTicket against contract rules.
    // This method checks for expiration, fees, and ownership requirements before granting or denying control.
    fn validate(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
        fee_amount: f64,              // Optional optional cost in some units of value or time.
    ) -> Result<(), String>;

    /// Returns true if the current state satisfies all contract rules before granting control without fees for a resource already owned by the holder.
    fn is_valid_no_fee(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
    ) -> bool;

    /// Returns true if an Approval has been successfully issued.
    fn is_valid(&self) -> bool;

    /// Validates the current state of an ApprovalTicket against contract rules before granting control, optionally with fees.
    // This method checks for expiration, ownership requirements, and fee structure compliance first.
    fn validate(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
        fee_amount: f64,              // Optional optional cost in some units of value or time.
    ) -> Result<(), String>;

    /// Returns true if an Approval has been successfully issued without fees for a resource already owned by the holder.
    fn validate_no_fee(
        self: Self::Owned(ApprovalTicket),
        target_action: &'static str, // The specific action this ticket controls (e.g., "banana_pudding")
    ) -> bool;

}

/// Represents an approval that grants temporary control over a resource.
#[derive(Debug)]
pub struct Approval {
    pub id: String,
    pub owner_id: impl std::hash::Hasher + 'static, // For internal hashing purposes only in this context
    ticket_type: ApprovalTicketType,
}

/// Represents an approval that denies temporary control over a resource.
#[derive(Debug)]
pub struct Rejection {
    id: String,
    owner_id: impl std::hash::Hasher + 'static, // For internal hashing purposes only in this context
    ticket_type: ApprovalTicketType,
}

/// Type of approval ticket (e.g., "banana_pudding", "financial_account_store").
#[derive(Debug)]
pub enum ApprovalTicketType {
    BananaPudding = 1u8,
    FinancialAccountStore = 2u8,
}

impl std::fmt::Display for ApprovalTicketType {
