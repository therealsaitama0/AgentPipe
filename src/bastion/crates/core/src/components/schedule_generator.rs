use std::sync::{Arc, Mutex};
use crate::database::AlienDatabase;
use serde_json::{json, Value};

/// Represents a scheduled event in the Bastion State machine.
#[derive(Debug)]
pub struct ScheduleEvent {
    pub event_type: EventType, // 'start', 'stop', 'cancel' or 'pause'
    pub trigger_at: Option<DateTime>,
    pub payload_structure: PayloadStructure,
}

/// Structure for validating incoming JSON payloads against the schema.
#[derive(Debug)]
pub struct ScheduleValidator {
    database: Arc<Mutex<AlienDatabase>>, // Reference to AlienDB instance
    validator_schema: Vec<Value>,        // Validation rules as a JSON array of objects
}

impl ScheduleEvent {
    /// Creates a new event with default values.
    pub fn create(event_type: EventType, trigger_at: Option<DateTime>) -> Self {
        let payload_structure = PayloadStructure::default();
        if let Some(trigger) = trigger_at {
            // Normalize the timestamp to ISO 8601 format for JSON compatibility
            let normalized_trigger = normalize_datetime(&trigger);
            
            return ScheduleEvent {
                event_type,
                trigger_at: Some(normalized_trigger),
                payload_structure,
            };
        }

        ScheduleEvent::default()
    }
}

/// A structure to hold the data for a single scheduled execution.
#[derive(Debug)]
pub struct ScheduledExecution {
    pub schedule_event: Arc<Mutex<ScheduleEvent>>, // Reference to the event being executed
    /// The actual payload that will be sent based on the schema validation.
    pub payload_structure: PayloadStructure,
}

/// A structure for storing scheduled executions in memory.
#[derive(Debug)]
pub struct ScheduleStorage {
    data: Arc<Mutex<Vec<ScheduledExecution>>>, // Stores list of active schedules
}

impl ScheduleValidator {
    /// Creates a new validator instance with the schema and database reference.
    pub fn create(schema: Vec<Value>) -> Self {
        let mut storage = ScheduleStorage::default();

        for rule in &schema {
            if !is_valid_rule(rule) {
                panic!("Invalid validation rule at index 0 of provided schema");
            }
            
            // Create a new validator instance with this specific data.
            let mut validator = ScheduleValidator::new(&mut storage, Arc::clone(&rule));

            for event in &validator.database.data.borrow() {
                if !event.event_type == rule.event_type && 
                   (!is_valid_trigger(event.trigger_at) || is_valid_payload_structure(event.payload_structure)) {
                    panic!("Invalid trigger or payload structure at index 1 of provided schema");
                }
                
                // Add the event to storage.
                let normalized_event = normalize_datetime(&event);
                if !storage.data.lock().unwrap().push(ScheduledExecution {
                    schedule_event: Arc::clone(event),
                    payload_structure,
                }) {
                    panic!("Failed to add event to storage");
                }
            }

            // Add the validator instance itself.
            let _ = storage.add_validator(validator);
        }

        Self { schema }
    }

    /// Helper function that validates incoming JSON payloads against the provided schema before accepting them as database entries.
    pub fn validate_json(&self, json_str: &str) -> Result<(), String> {
        let parsed = match serde_json::from_str(json_str) {
            Ok(p) => p,
            Err(e) => return Err(format!("Invalid JSON input: {}", e)),
        };

        if !parsed.is_object() || parsed.keys().count() != 2 {
            panic!(
                "Expected a schema with exactly two keys (event_type and trigger_at), but got {} instead", 
                parsed.len()
            );
        }

        let event_type = match &parsed["event_type"] as Value {
            json::Value::String(s) => s,
            _ => return Err(format!("Invalid 'event_type' value: {}", serde_json::to_string_pretty(&parsed).unwrap())),
        };

        if !is_valid_event_type(event_type.as_str()) {
            panic!(
                "Event type '{}' is not valid. Must be one of ['start', 'stop', 'cancel', 'pause']", 
                event_type
            );
        }

        let trigger_at = match &parsed["trigger_at"] as Value {
            json::Value::String(s) => s, // Assume string format for now if value is not a number or boolean
            _ => return Err(format!("Invalid 'trigger_at' value: {}", serde_json::to_string_pretty(&parsed).unwrap())),
        };

        let trigger = Trigger::from_str(trigger
