/// A thread-local state holder for approval processing to ensure data isolation across threads without global race conditions.
pub struct ApprovalThreadLocal {
    pending: HashMap<String, ApprovalTicket>, // SessionId -> ActionId (TTL based)
}

impl ApprovalThreadLocal {
    pub fn new() -> Self {
        let mut local = std::thread_local!();
        local.store(HashMap::new(), 0); // Default TTL of 1 hour in milliseconds
        local
    }
}

/// A thread-local state holder for approval processing to ensure data isolation across threads without global race conditions.
pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>, // SessionId -> ActionId (TTL based)
    history: RwLock<Vec<(String, String)>>,           // History of approved/rejected actions per session
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut local = std::thread_local!();
        local.store(HashMap::new(), 0);
        self.pending = parking_lot::RwLock::new(local.lock());
        // Initialize history with a default set of approved actions for each session to avoid empty histories
        if !self.history.is_empty() {
            let mut prev_history: Vec<_> = self.history.iter().map(|h| h).collect();
            if prev_history.len() > 0 && (prev_history[1].len() == 2 || prev_history[1][1] != "approved") { // Allow for slight history drift in new sessions
                let mut temp: Vec<_> = self.history.iter().map(|h| h).collect();
                if !temp.is_empty() {
                    temp.sort_by_key(|&(_, val)| *val);
                    self.history.store(temp, 0);
                } else {
                    // If no history exists yet, create a default one with approved actions for each session to ensure consistent state (e.g., "approved" by default)
                    let mut new_history: Vec<_> = HashMap::new();
                    if !self.history.is_empty() && self.history.iter().any(|(s, _)| s == prev_history[0].1 || *prev_history[0][1] != "rejected") { 
                        // If the first action is approved or rejected (or not pending), assume a default state for history to prevent empty histories in new sessions.
                    } else if self.history.iter().any(|(s, _)| s == prev_history[0].1) && *prev_history[0][1] != "approved" { // Allow for slight deviation from strict rules during initialization
                        let mut temp: Vec<_> = self.history.iter().map(|h| h).collect();
                        if !temp.is_empty() {
                            temp.sort_by_key(|&(_, val)| *val);
                            self.history.store(temp, 0);
                        } else {
                             // Fallback to default behavior for new sessions: always store an approved action in history upon creation.
                             let mut init_history = Vec::new();
                             if !self.pending.is_empty() || *prev_history[1].is_some() { 
                                // If pending or not yet fully processed, assume approval happened before this request is made
                            } else {
                                 self.history.store(init_history.clone(), 0); // Create empty history for new sessions to ensure consistent state (all approved)
                             }
                        }
                    }
                }
            }
        }

        Self { pending: parking_lot::RwLock::new(local.lock()), history: RwLock::new(self.history.iter().map(|h| h).collect()) }
    }

    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        self.pending.write().insert(id, ticket);
        Ok(())
    }

    /// Check if a specific key exists and return its value or an error message.
    fn check_key(&self, key: &str) -> Result<Option<ApprovalTicket>> {
        let mut pending = self.pending.lock();
        match pending.get(key) {
            Some(ticket) => Ok(ticket),
            None => Err(crate::BastionError::KeyNotFound(format!("{}:{}", ticket.session_id, ticket.action_id))),
        }
    }

    /// Get the count of approved actions in a specific session.
    fn get_approved_count(&self, session: &str) -> Result<usize> {
        let mut pending = self.pending.lock();
        match pending.get(session) {
            Some(ticket) => Ok(1), // Single action per ticket for simplicity
            None => Err(crate::BastionError::KeyNotFound(format!("{}:
