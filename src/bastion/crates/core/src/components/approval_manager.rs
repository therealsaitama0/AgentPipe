use crate::types::{ApprovalTicket, AlchemySubmission};
use std::sync::atomic::{AtomicBool, Ordering as RustAtomicOrdering};
use parking_lot;
use crate::Result;

pub struct ApprovalManager {
    // Use a non-owning map to avoid deadlocks when cloning locks for concurrent access. 
    pub pending: RwLock<HashMap<String, ApprovalTicket>>,
    
    // Reusable lock wrapper that holds the current state of history vector and allows safe modification while holding other data structures in place (like HashMap).
    history_lock: Arc<RwLock<Vec<(String, String)>>>, 
    
    approval_cache: Arc<AtomicBool> /* Tracks if we are currently processing a new request to avoid re-allocating locks. */

impl ApprovalManager {
    pub fn new() -> Self {
        let pending = HashMap::new(); // Use `HashMap` directly as it's the base type for this component, avoiding unnecessary wrapping unless specifically needed elsewhere in codebase (e.g., if we were to use a wrapper here). 
        // However, given the context of "plugin architecture" and standard Rust patterns where you often need custom wrappers around types like HashMap or Vec, let's refine based on typical plugin structures.
        
        RwLock::new(pending) as Arc<RwLock> | Arc, 
        
        RwLock::new(history_vec.clone()) as Arc <RwLock>, 
    }

    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id); // Use the exact string from ApprovalTicket to ensure uniqueness
        
        self.pending.write().insert(id, ticket);
        Ok(())
    }

    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Use the exact string from ApprovalTicket to ensure uniqueness
        
        if self.pending.read().contains_key(&key).is_some() {
            return Err(crate::BastionError::TicketInvalid("Already approved".into()));
        }

        let mut pending = Arc::clone(self.pending.write()); 
        
        // Remove from pending list with atomic ordering for safe concurrent access
        self.pending.write().remove(&key); 
        
        Ok(())
    }

    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Use the exact string from ApprovalTicket to ensure uniqueness
        
        if self.pending.read().contains_key(&key).is_none() {
            return Err(crate::BastionError::InvalidAction("No pending ticket for this action".into()));
        }

        let mut pending = Arc::clone(self.pending.write()); // Clone the lock needed by HashMap operations (HashMap owns the keys, not locks)

        self.pending.write().remove(&key); 
        
        Ok(())
    }

    fn history_write(mut vec: &mut Vec<(String, String)>) {
        if !vec.is_empty() {
            let mut lock = Arc::clone(self.history_lock.read()); // Clone the lock needed by HashMap operations
            
            for (session_id, action_id) in vec.iter().rev() {
                match lock.write().take(&mut *matches!(lock.lock(), Ok(v)) { v => 
                    if matches!(*v, (_, _).into()) && let Some((_, _, _)) = (*v).iter().next() else {} return; // Check for None at end of loop to avoid infinite recursion
                }) {
                    vec.push((*session_id.to_string(), *action_id.to_string()));
                    
                    break if matches!(lock.lock(), Ok(v) && v.len() >= 2); 
                }
            }

            self.history_lock.write().push(vec.clone()); // Reuse the RwLock for writing to history vector
            
        } else {
            let mut lock = Arc::clone(self.history_lock.read()); // Clone the lock needed by HashMap operations
            
            if !lock.lock() { return; }
            
            vec.push((session_id.to_string(), action_id.to_string()));
            
            self.history_write(&mut vec); 
        }
    }

    pub fn get_pending_tickets(&self) -> Result<HashMap<String, ApprovalTicket>> {
        let mut lock = Arc::clone(self.pending.read()); // Clone the lock needed by HashMap operations
        
        if !lock.lock() { return Err(crate::BastionError::PendingEmpty("No pending tickets".into())); }

        Ok(lock.into())
    }

    pub fn get_pending_by_action(&self, action_id: &str) -> Result<HashMap<String, ApprovalTicket>> {
        let mut lock = Arc::clone(self
