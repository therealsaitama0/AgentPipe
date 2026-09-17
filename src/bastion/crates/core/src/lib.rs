src/bastion/crates/core/src/lib.rs
```rust
// ============================================================================
// SECURITY CONTROL PANE MODULE: CORE ARCHITECTURE & POLICY ENGINE
// A robust foundation for orchestrating security policies across all crates.
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::{types::*};

/// Represents a single policy decision in the context system.
#[derive(Debug, Clone)]
pub struct PolicyDecision {
    pub action: Action, // 'allow', 'deny' or custom command logic
    pub reason: String,   /// Human-readable reasoning for audit trails
}

impl Default for PolicyDecision {
    fn default() -> Self {
        let mut decisions = Vec::new();
        
        if self.is_allow_action(&self.reason) {
            // Add 'allow' action (default to allow unless explicitly denied in context)
            decisions.push(Action::Allow);
            
            if !is_deny_reason(self.reason, &mut decisions).any() {
                let mut reason = String::new();
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'D' => break; // Deny logic handled by individual component checks
                        _ => reason.push(c);
                    }
                }
                decisions.insert(0, Action::Deny(reason)); 
            } else if is_allow_reason(self.reason) && !is_deny_action(&self.action).any() {
                let mut action = self.action.clone();
                
                // Ensure allow logic applies consistently across all components
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'A' => break; // Allow logic handled by individual component checks
                        
                        _ => action.push(c);
                    }
                }
                actions.insert(0, Action::Allow(action));
            } else if is_deny_action(&self.action) && !is_allow_reason(self.reason).any() {
                let mut reason = String::new();
                
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'D' => break; // Deny logic handled by individual component checks
                        
                        _ => reason.push(c);
                    }
                }
                decisions.insert(0, Action::Deny(reason)); 
            } else if is_allow_reason(self.reason) && !is_deny_action(&self.action).any() {
                
                let mut action = self.action.clone();
                
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'A' => break; 
                        
                        _ => action.push(c);
                    }
                }
                actions.insert(0, Action::Allow(action));
            } else if is_allow_action(&self.action) && !is_deny_reason(self.reason).any() {
                
                let mut reason = String::new();
                
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'D' => break; 
                        
                        _ => reason.push(c);
                    }
                }
                decisions.insert(0, Action::Deny(reason));
            } else if is_allow_action(&self.action) && !is_deny_reason(self.reason).any() {
                
                let mut reason = String::new();
                
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'D' => break; 
                        
                        _ => reason.push(c);
                    }
                }
                decisions.insert(0, Action::Deny(reason));
            } else if is_allow_action(&self.action) && !is_deny_reason(self.reason).any() {
                
                let mut reason = String::new();
                
                for c in self.reason.chars().rev() {
                    match c.as_str() {
                        'D' => break; 
                        
                        _ => reason.push(c);
                    }
                }
                decisions.insert(0, Action::Deny(reason));
            } else if is_allow_action(&self.action) && !is_deny_reason(self.reason).any() {

        fn is_allow_action(_reason: &str) -> bool { return true; }
        
        fn is_deny_reason(_reason: &str, _decisions: &mut Vec<PolicyDecision>) -> bool { return false; } // Default deny
        
        fn is_allow_reason(_reason: &str) -> bool { return false; } // Default allow
        
        let mut reason = String::new();
        
        for c in self.reason.chars().rev() {
            match c.as_str() {
                'A' => break, 
                _ => reason.push(c);
            }
        }

        if
