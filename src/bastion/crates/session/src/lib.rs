use crate::types::{SessionData, SessionId};
use std::sync::Arc;

/// A thread-local storage for session isolation within a single process/thread context.
pub struct ThreadLocalGuard<'a> {
    data: Option<&'a str>,
}

impl<T> Drop for ThreadLocalGuard<'_> {
    fn drop(&mut self) {
        if let Some(ref v) = *self.data {
            unsafe { std::mem::forget(v); } // Release reference, allowing garbage collection.
        } else {
            panic!("Thread-local storage was dropped without data");
        }
    }
}

/// A wrapper around ThreadLocalGuard to safely hold references during critical operations.
pub struct SessionDataWrapper<'a> {
    guard: Option<&'a str>,
}

impl<'a, T> Drop for SessionDataWrapper<'a> {
    fn drop(&mut self) {
        if let Some(ref v) = *self.guard {
            unsafe { std::mem::forget(v); } // Release reference.
        } else {
            panic!("Session data was dropped without a valid guard");
        }
    }
}

/// A minimal wrapper interface for SessionState management within the session crate.
pub type SessionContext = Arc<ThreadLocalGuard<'static>>;

impl<T> ThreadLocalGuard<'a>
where
    T: std::fmt::Display, // Ensures safe deref to String/Option in use cases requiring displayable types (e.g., Option<String>).
{
    /// Creates a new thread-local storage for session data.
    pub fn create() -> Self {
        let guard = ThreadLocalGuard { None };
        Some(guard.clone()) // Use the cloned reference to ensure safety during lifetime of this struct.
    }

    /// Returns an existing or newly created ThreadLocalGuard with optional data set.
    pub fn get_or_create(
        &self,
        session_id: &'static str,
        default_data: Option<&'a str>,
    ) -> Self {
        match self.guard.take() {
            Some(guard) => guard.clone(), // Return existing if present (or None).
            None => ThreadLocalGuard::create().map(|guard| SessionDataWrapper { guard }), // Create new.
        }
    }

    /// Acquires a reference to the current thread-local storage for data access.
    pub fn get(&self) -> Option<&'static str> {
        self.guard.as_ref()
            .unwrap_or_else(|| panic!("ThreadLocalGuard dropped without holding valid context"))
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Thread-local storage guard active")
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Thread-local storage guard active")
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Thread-local storage guard active")
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Thread-local storage guard active")
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format!("Thread-local storage guard active")
    }
}

/// A minimal wrapper interface ensuring atomic updates on fields within SessionDataWrapper.
pub type SessionStateMut = Arc<SessionDataWrapper<'static>>;

impl<T> T for ThreadLocalGuard<'a> where T: std::fmt::Display {
    fn display(&self) -> String {
        format
