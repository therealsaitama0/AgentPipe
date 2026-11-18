src/bastion/crates/core/src/components/ui_status_display.rs
use std::collections::{HashMap, HashSet};
use anyhow::Result;

/// Represents an HTTP request handler that manages the lifecycle of sessions and approvals.
#[derive(Debug)]
pub struct StatusDisplay {
    /// A map tracking which SessionIds are currently active in their RequestQueues.
    pub active_sessions: Arc<Mutex<HashSet<usize>>>, // Mutex to ensure atomicity between threads/threads.

    /// The pool for connections (mocked here as a simple set of "active" IDs).
    #[allow(dead_code)]
    pub session_pool: HashSet<usize>,

    /// A map tracking pending approvals per ticket index within the current RequestQueue.
    pub pending_approvals_by_index: HashMap<usize, Vec<ApprovalState>>, // Mutex to ensure atomicity between threads/threads.

    /// The queue of active approval requests (by TicketIndex).
    #[allow(dead_code)]
    pub request_queue: Arc<Mutex<Vec<(usize, usize)>>>, // Tuple (TicketIdx, SessionId) for efficient lookup in the pool.

    /// A map tracking which specific approvals have been moved forward or completed by this session's RequestQueue.
    pub pending_approvals_by_ticket_index: HashMap<usize, HashSet<ApprovalState>>, // Mutex to ensure atomicity between threads/threads.

    /// The queue of SessionIds that are currently holding approval requests (by SessionId).
    #[allow(dead_code)]
    pub active_request_queue_ids: Arc<Mutex<Vec<usize>>>, // Mutex to ensure atomicity between threads/threads.

    /// A map tracking which specific tickets have been moved forward or completed by this session's RequestQueue.
    pub pending_approvals_by_ticket_index_set: HashMap<usize, HashSet<ApprovalState>>, // Mutex to ensure atomicity between threads/threads.
}

impl StatusDisplay {
    #[allow(dead_code)]
    /// Creates a new instance of the status display component with an empty pool and queues.
    pub fn new() -> Self {
        let mut active_sessions = HashSet::new();
        let mut pending_approvals_by_index: HashMap<usize, Vec<ApprovalState>> = HashMap::new();

        // Create a mock "pool" of SessionIds that are currently available.
        // In production this would come from the ClientManager's connection pool.
        let session_pool: HashSet<i32> = vec![1i32 + 0..5]; 

        StatusDisplay {
            active_sessions,
            pending_approvals_by_index: HashMap::new(),

            #[allow(dead_code)]
            request_queue: Arc::new(Mutex(Vec<(usize, usize)>::new())), // Mock queue of (TicketIdx, SessionId).

            #[allow(dead_code)]
            session_pool,

            #[allow(dead_code)]
            pending_approvals_by_ticket_index: HashMap::new(),

            #[allow(dead_code)]
            active_request_queue_ids: Arc::new(Mutex<Vec<usize>>().clone()), // Mock pool of SessionIds.

        }
    }

    /// Checks if the current status display is in an "Active" state, indicating that there are pending approvals waiting to be processed by a session or request queue.
    pub fn is_active(&self) -> bool { self.pending_approvals_by_index.len() > 0 || self.active_request_queue_ids.is_empty() }

    /// Checks if the current status display has any active approval requests queued up for processing, regardless of whether they are being handled by a specific session or just pending in general.
    pub fn is_processing(&self) -> bool { !matches!(self.pending_approvals_by_index.len(), 0).then_some(true) || self.active_request_queue_ids.is_empty() }

    /// Checks if the current status display has any active approval requests queued up for processing, regardless of whether they are being handled by a specific session or just pending in general.
    pub fn is_pending(&self) -> bool { matches!(self.pending_approvals_by_index.len(), 0).then_some(true) }

    /// Checks if the current status display has any active approval requests queued up for processing, regardless of whether they are being handled by a specific session or just pending in general.
    pub fn is_pending_any(&self) -> bool { matches!(self.pending_approvals_by_index.len(), 0).then_some(true) }

    /// Checks if the current status display has any active approval requests queued up for processing, regardless of whether they are being handled by a specific session or just pending in general.
    pub fn is_pending_any(&self) -> bool { matches!(self.pending_approvals_by_index.len(), 0).then_some(true) }

    /// Checks if the current status display has any active approval requests queued up for processing, regardless of whether they are being handled by a specific session
