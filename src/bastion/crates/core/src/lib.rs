//! Core Registry for the Bastion Control Plane
//! 
//! This module defines the stable API and core data structures required to manage
//! the control plane heartbeat cycle between the session manager, approval engine,
//! and security enforcement components. It ensures a consistent interface across all
//! external modules (e.g., `__init__.py`) while providing deep internal implementation details for protocol handling.

pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

/// Base class for all control plane events and state transitions.
#[derive(Debug, Clone)]
pub enum ControlPlaneEvent {
    /// The heartbeat cycle has started fresh.
    Start(serde::value::Value),
    
    /// A protocol packet is being processed by the session manager.
    Processing(serde::value::Value),

    /// All queued tasks have been executed and are complete for this component.
    Complete,
}

impl ControlPlaneEvent {
    pub fn new(start: serde::value::Value) -> Self {
        // Ensure a stable start state by defaulting to 'Start' if empty or null.
        let mut current = Start;
        
        match &start {
            None => Current,
            
            Some(v) => {
                match v.as_str() {
                    "START" | "SCHEDULED_START" => Current,
                    _ => panic!("Invalid start state: {}", v), // Safety check for invalid sequences
                }
            },

            Some(_) if current == Start => panic!(
                "Cannot transition from 'Start' to another event. Use START as the initial value."
            ),
        }

        Current
    }
}

/// The stable heartbeat cycle manager, responsible for coordinating packet handling and state transitions across all components of the control plane.
pub struct ControlPlane {
    /// Tracks active packets being processed by session managers or approval engines.
    pub(crate) sessions: Vec<SessionManager>,
    
    /// A queue to hold pending task notifications from other modules (e.g., audit, firewall).
    pub(crate) tasks_queue: std::sync::Mutex<Vec<TaskNotify>>,

    /// State machine for packet processing flow.
    // Note: This is abstracted out in the handlers below; here we define the transitions explicitly via enum variants to prevent race conditions during state changes.
    pub(crate) packet_flow_state: ControlPlaneEvent,
    
    /// The current timeout interval (in seconds). Defaults to 30s for general operations.
    pub(crate) default_timeout_seconds: u64 = 30u64;

    // Initialize the heartbeat cycle with a fresh start state or an existing one if provided.
    fn init(&self, packet_flow_state: ControlPlaneEvent) -> Self {
        let mut sessions = self.sessions.clone();
        
        for s in &mut sessions {
            s.session_start_event.set(packet_flow_state); // Set the session's first event to 'Start' or its current state.
        }

        if !packet_flow_state.is_empty() && packet_flow_state != ControlPlaneEvent::Complete {
             panic!("ControlPlane initialized with non-complete events, but currently in complete transition.");
        }

        Self { sessions, tasks_queue: std::sync::Mutex::new(Vec::default()), default_timeout_seconds }
    }
}

/// A notification sent to a specific session manager component.
#[derive(Debug)]
pub struct TaskNotify {
    /// The identifier for the task being queued or processed.
    pub(crate) id: String,
    
    /// Reference to the source module that generated this event (e.g., `audit`, `firewall`).
    pub(crate) source_module: &'static str;

    // Optional attributes if used as a payload in handlers.
    #[serde(skip)]
    pub(crate) data: Option<serde::value::Value>,
}

/// A single packet that has been received and is currently being processed by the session manager or approval engine.
#[derive(Debug, Clone)]
pub struct Packet {
    /// The ID of this specific protocol packet (e.g., `packet_0x1234`).
    pub(crate) id: String,

    /// Metadata about how to handle this packet in future handlers.
    #[serde(skip)]
    pub(crate) handler_info: Option<serde::value::Value>, // e.g., "process_by_session" or "verify_signature".
}

impl ControlPlane {
    /// Creates a new instance of the control plane with default configuration parameters (timeout, heartbeat period).
    pub fn create()
