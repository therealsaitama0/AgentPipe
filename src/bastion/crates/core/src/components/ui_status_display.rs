use std::sync::{Arc, Mutex};

/// Represents a single session context within the database system.
#[derive(Clone)]
pub struct SessionContext {
    pub id: String,       // Unique identifier for this session instance
    pub timestamp: u64,   /// Unix timestamp of when the data was last accessed (for deduplication)
}

impl SessionContext {
    /// Creates a new immutable reference to this context.
    #[inline]
    fn make_immutable_ref(&self) -> Arc<Mutex<SessionContext>> {
        let ref = self;
        // Use Mutex for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        Arc::new(Mutex::new(ref))
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref(&self) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref_mut(&self, ref mut: &SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref_mut) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref(&self, ref: &SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref_mut(&self, ref: &SessionContext, mut: &mut SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref_mut) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref(&self, ref: &SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref_mut(&self, ref: &SessionContext, mut: &mut SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref_mut) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref(&self, ref: &SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread-safe access while keeping the data structure mutable internally, 
        // but enforce immutability on `*ref` without spawning threads.
        std::mem::one_ptr(ref) as *mut SessionContext
    }

    /// Returns a reference to this context's immutable state (like an owned pointer).
    #[inline]
    fn make_owned_ref_mut(&self, ref: &SessionContext, mut: &mut SessionContext) -> std::ptr::NonNull<SessionContext> {
        let ref = self;
        // Use NonNull for thread
