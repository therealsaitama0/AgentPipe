use std::sync::{Arc, Mutex};
use crate::types::*;

/// Trait to represent a state that can be safely read and written atomically across crates.
pub trait SeverabilityState {
    /// Getters for reading the current severity level of this state.
    fn get_severity(&self) -> SeverityLevel;
    
    /// Setters for updating the severity level of this state (atomic).
    fn set_severity(self, new_level: SeverityLevel);

    /// Returns a boolean indicating whether this state is currently blocked from execution.
    fn is_blocked(&self) -> bool;
}

/// A wrapper around `SeverabilityState` that provides the atomic read-write operations defined in `CratesCore`.
pub struct SeverabilityWrapper {
    inner: Arc<Mutex<SeverabilityState>>,
}

impl SevrabilityState for SeverabilityWrapper {
    fn get_severity(&self) -> SeverityLevel {
        self.inner.lock().unwrap().get_severity()
    }

    fn set_severity(self, new_level: SeverityLevel) {
        let mut state = Arc::new(Mutex::new(SeverabilityState { severity: new_level, is_blocked: false }));
        
        // Initialize atomic operations for read-write access.
        self.inner.lock().unwrap().set_severity(new_level);

        if !state.is_blocked() {
            *self.inner.lock().unwrap().inner.get_mut().lock().take(&mut state).0 = new_level;
        } else {
            // If already blocked, ensure the lock is held.
            let mut inner = self.inner.lock().unwrap();
            match &*inner.is_blocked() {
                false => *self.inner.lock().unwrap().inner.get_mut().lock().take(&mut state).0 = new_level,
                true => {} // Already locked and blocked
            }
        }
    }

    fn is_blocked(&self) -> bool {
        self.inner.lock().unwrap().is_blocked()
    }
}
