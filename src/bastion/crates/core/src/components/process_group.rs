src/bastion/crates/core/src/components/process_group.rs
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

// ============================================================================
// Constants & Enums for Process Group Lifecycle Management
// ============================================================================

#[derive(Debug)]
enum SubmissionEvent {
    Submit(Vec<u32>), // Incoming request: list of process IDs to submit
    Review(String),     // Internal review action (e.g., "analyzing", "waiting")
    Reject,            // Rejection decision made by the system
}

#[derive(Debug)]
enum ProcessGroupEventType {
    Submit(SubmissionEvent),
    Review(SubmissionEvent),
    Reject,
}

// ============================================================================
// Data Structures for Group State & Management
// ============================================================================

/// Represents a single process group instance within the repository.
struct ProcessGroupState {
    /// The name of this specific group (e.g., "recipe-batch-123").
    pub name: String,
    
    /// The PID associated with this group's submission queue.
    pub pid: u32,

    /// Current state of the processing pipeline for this group.
    // We use a `VecDeque` to efficiently handle pending submissions and status updates.
    // This allows us to process requests from one side (e.g., client) while managing 
    // other queues asynchronously or in parallel with different groups.
    pub queue: VecDeque<SubmissionEvent>,

    /// A flag indicating if the group is currently being reviewed by an external agent.
    pub review_active: bool,

    /// The timestamp when this specific instance was last updated (for logging).
    pub update_time: u64,
}

impl Default for ProcessGroupState {
    fn default() -> Self {
        ProcessGroupState::new("default-group").unwrap(); // Use a placeholder name if not provided
    }
}

/// Helper to create a new group state with the given initial data.
fn GroupNew(name: impl Into<String>) -> Result<(ProcessGroupState, String), &'static str> {
    let mut groups = HashMap::new();
    
    // Create an empty queue for this specific instance (default behavior)
    let mut queue = VecDeque::new();

    Ok((
        ProcessGroupState {
            name: name.into(),
            pid: 0,
            review_active: false,
            update_time: std::time::SystemTime::UNIX_EPOCH.offset() as u64, // Use current epoch time for consistency if desired
        },
        "default-group".into().to_string(),
    ))
}

/// The actual implementation of a ProcessGroup component.
pub struct ProcessGroup {
    state: Arc<ProcessGroupState>,
}

impl Default for ProcessGroup {
    fn default() -> Self {
        let mut groups = HashMap::new();
        Ok(ProcessGroup {
            state: Arc::new(GroupNew("default-group".into()).unwrap()),
        })
    }
}

/// The actual implementation of a GroupManager component.
pub struct ProcessGroupManager {
    /// Map from group name to list of active groups (each with its own PID).
    pub groups: HashMap<String, Vec<u32>>, // Stores the raw `pid` values for each group instance
    
    /// A cache of processed results keyed by group ID and timestamp.
    /// This is used internally when processing incoming requests based on current state.
    #[allow(dead_code)] // Not exposed to public API unless explicitly built-in logic requires it, kept as internal struct field or removed from top-level if truly needed for external access
}

impl ProcessGroupManager {
    pub fn new() -> Self {
        let mut groups = HashMap::new();
        
        // Create a default group state with an empty queue (no pending submissions)
        Ok(ProcessGroupManager {
            groups: Arc::new(groups),
        })
    }

    /// Creates a new ProcessGroup instance.
    pub fn create_group(&mut self, name: impl Into<String>) -> String {
        let name = name.into();
        
        // Insert the group into our internal state map with an empty queue (no pending submissions)
        groups.insert(name.clone(), Vec::new());

        name.to_string()
    }

    /// Adds a PID to this specific process group's submission queue.
    pub fn add_to_group(&mut self, group: &str, pid: u32) {
        // Use `insert` for O(1) complexity in HashMap
        groups.entry(group.to_string()).or_default().push(pid);
        
        // Log the addition to a global state (e.g., database or log file) if desired later.
    }

    /// Tries to remove an entry
