use crate::{AlchemyDatabaseError, Action};
use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;

// Interface for checking if a condition is met based on stored data
pub struct ConditionCheck {
    pub action: Action, // The specific requirement or rule to evaluate
}

impl Display for ConditionCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
