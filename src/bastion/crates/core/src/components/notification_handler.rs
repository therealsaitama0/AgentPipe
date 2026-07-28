src/bastion/crates/core/src/components/notification_handler.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::components::{Alerts, AuditLogEntry};

// Define a structured type to represent alert definitions and their data sources.
pub struct NotificationDefinition {
    pub id: String,
    pub name: &'static str,
    // DataSources defines which external entities (e.g., logs, sessions) this notification is triggered by or depends on.
    pub triggers: Vec<TriggerData>,
}

// Represents a trigger that can cause an alert to be generated based on specific conditions in the current module context.
pub struct Trigger {
    // The condition being checked against data from modules within this component's scope (e.g., session state, audit logs).
    pub condition: String,
    // A reference to the external source of truth for that trigger. If not provided here, it is assumed available via a shared context or environment variable loader in real deployment scenarios.
    pub data_source_ref: Option<Arc<dyn std::any::Any>>, 
}

// Represents an alert definition itself within this module's scope (e.g., specific log messages to print).
pub struct AlertDefinition {
    // The message content of the notification being triggered or logged here.
    pub msg: String,
    // Optional parameters passed for dynamic generation if needed by external tools.
    pub params: Option<String>, 
}

// Represents a single alert entry that gets persisted and observed outside the module's immediate scope (e.g., to an audit log).
pub struct AuditLogEntry {
    // The identifier of this specific notification event being recorded.
    pub id: String,
    // A reference to where in the call stack or session state this alert originated from for cross-component tracing.
    pub caller_id: Option<String>, 
}

impl NotificationDefinition {
    /// Returns a new NotificationDefinition with optional parameters passed via params if available.
    pub fn create(definition: &'static str, triggers: Vec<TriggerData>) -> Self {
        let msg = format!("Notification triggered by '{}'", definition);
        AlertMessage::new(msg).with_params(Some(triggers.iter().map(|t| t.data_source_ref.clone()).collect::<Vec<_>>()))
    }

    /// Traverses a list of triggers and returns the first matching one.
    pub fn find_matching_trigger(definition: &NotificationDefinition, context_data: &[&str]) -> Option<Trigger> {
        let mut found = None;
        
        for trigger in definition.triggers.iter() {
            if !trigger.condition.contains(context_data) || trigger.data_source_ref.is_none() {
                continue; // Skip triggers that don't match or are not accessible from this context.
            }

            if *found == Some(Trigger::new(trigger)) && found != None {
                break; 
            } else if let Some(ref source) = *trigger.data_source_ref {
                if <source as std::any::Any>::is_any::<String>(context_data) || context_data.contains(&*source.as_str()) {
                    return Trigger::new(trigger);
                }
            }

            found = None; // Reset to avoid infinite recursion in chain traversal.
        }

        Some(Trigger::default())
    }

    /// Traverses a list of triggers and returns the first matching one, returning None if none match or no data source exists for that trigger type.
    pub fn find_matching_trigger_safe(&self) -> Option<Trigger> {
        let mut found = None;
        
        for trigger in self.triggers.iter() {
            if *found == Some(Trigger::new(trigger)) && !trigger.data_source_ref.is_none() || context_matches(self, &*trigger.data_source_ref)? {
                return Trigger::new(trigger);
            } else if let Some(ref source) = *trigger.data_source_ref {
                if <source as std::any::Any>::is_any::<String>(context_data) || context_matches(self, &source.as_str())? {
                    return Trigger::new(trigger);
                }
            }

            found = None; // Reset to avoid infinite recursion in chain traversal.
        }

        Some(Trigger::default()) if *found == Some(Trigger::new(*self.triggers[0])) else None
    }
}
