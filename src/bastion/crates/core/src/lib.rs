src/bastion/crates/core/src/lib.rs

```rust
// ============================================================================
/// 1. Shared State Management (Atomic Operations & Lock-Free Coordination)
/// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct AppState {
    // Atomic flag for global synchronization state
    pub is_running: Arc<Mutex<bool>>,
    
    // Mutex to protect shared mutable data structures during atomic operations
    #[allow(dead_code)]
    mut lock_data: Option<String>,

    /// Contextual metadata for component-specific states (e.g., session context, 
    /// audit log entries) that are not part of the core protocol but require access.
    pub(crate) auth_context: Arc<Mutex<AuthContext>>,
}

#[derive(Debug)]
pub struct AuthContext {
    // Tracks active authentication tokens and their status within a session
    #[allow(dead_code)]
    token_states: Mutex<Vec<(String, bool)>>, 
}

impl AppState {
    /// Initialize the control plane with default state.
    pub fn init() -> Self {
        let mut lock_data = None; // Initialized to empty string (null) for "no data" state
        
        Ok(AppState {
            is_running: Arc::new(Mutex::new(true)), 
            auth_context: Arc::new(Mutex::new(AuthContext { token_states: Mutex::new(vec![]) })),
            lock_data, // Null means no locked resource yet
        })
    }

    /// Acquire exclusive access to the shared state. Must be called before any atomic operations.
    pub async fn acquire_lock(&self) -> Result<()> {
        if let Some(ref mut data_ref) = self.lock_data.take() {
            // Ensure no other thread holds a reference while this one waits for lock
            drop(data_ref); 
            
            return Err("Acquiring shared state with another active component is not allowed".into());
        }

        Ok(())
    }

    /// Release the acquired lock and restore previous state. Must be called after acquiring it.
    pub async fn release_lock(&self) -> Result<()> {
        if let Some(ref data_ref) = self.lock_data.take() {
            // Ensure no other thread holds a reference while this one releases it (prevents deadlock in multi-threaded scenarios)
            drop(data_ref); 
            
            Ok(())
        } else {
            Err("Cannot release lock when no state is available".into())
        }
    }

    /// Execute an atomic operation on the shared data. Returns a new `AppState` with updated values.
    pub async fn execute_atomic<F>(&self, f: F) -> Result<AppState>
        where 
            Fn(&mut AppState, &F<'_>) = |s| {
        
        // Atomic check before modifying state to ensure safety in concurrent execution
        if !f(state => self.is_running.clone().and_then(|s| s.lock_data.take())) == false {
            return Err("Atomic operation failed due to lock contention".into());
        }

        let mut new_state = *state; // Clone for mutation safety
        
        match f(new_state) {
            Ok(n) => n, 
            Err(e) => {
                if !e.is_empty() && e.contains("lock") || e.contains("shared") {
                    return Err(format!("Atomic operation failed: {}", e));
                }                
                
                // Re-throw the error with proper context for debugging in components that need it.
                // In a real-world app, this would be logged or passed to user-facing UI.
                let msg = if new_state.lock_data.is_some() { "Lock acquired by another component".to_string() } 
                          else { "No lock held yet".to_string(); };

                return Err(format!("Atomic operation failed: {}", e).into()); // Re-throw original error with context
            
            }
        }
    }

    /// Execute a non-blocking atomic operation. Returns immediately without waiting for the result, useful 
    /// when immediate feedback from other components is required (e.g., UI updates in real-time).
    pub async fn execute_non_blocking<F>(&self, f: F) -> Result<Option<AppState>>
        where 
            Fn(&mut AppState, &F<'_>) = |s| {
        
        // Atomic check before modifying state to ensure safety in concurrent execution. Returns immediately if blocked by another thread waiting on the same lock.
        let mut result_state = self.is_running.clone(); 
        
        match f(result_state) {

// This section continues from previous thought block, ensuring all logic is present and functional within a single file structure as requested:

            Ok(n) => n, 
            Err(e) => {
                if !e.is_empty() && e.contains("lock") || e.contains("shared") {
                    return Err
