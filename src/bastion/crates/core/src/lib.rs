use crate::{types::*};
mod audit;
mod approval;
mod components;
mod error;
mod firecracker;
mod forced_command;
mod network_guard;
mod policy;
mod script_executor;
mod session;
mod types;
pub use audit::AuditChain;
pub use crate::{error::*, Result};
// Note: firecracker is already public in the main lib.rs above, but we keep it for clarity and consistency with existing imports.

/// A trait defining access patterns based on security context flags stored in module metadata.
/// This allows modules to control visibility (e.g., read-only vs write-protected) without exposing sensitive user data.
pub trait AccessPattern {
    /// Returns true if this is a protected access pattern, meaning the owning module cannot directly manipulate or modify external resources unless explicitly granted permission via `crates/bastion/requests`.
    fn is_protected(&self) -> bool;

    /// Returns false for read-only patterns. This means any crate accessing it must be wrapped in an explicit request to bypass this restriction, ensuring no direct exposure of internal state or secrets.
    fn is_read_only() -> bool {
        self.is_protected() && !matches!(self, AccessPattern::ReadOnly)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected()
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only_with_bypass(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion) && !matches!(self, AccessPattern::RequestBypassed)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only_with_bypass(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion) && !matches!(self, AccessPattern::RequestBypassed)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only_with_bypass(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion) && !matches!(self, AccessPattern::RequestBypassed)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only_with_bypass(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion) && !matches!(self, AccessPattern::RequestBypassed)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation of state or secrets.
    fn is_direct_access_only_with_bypass(&self) -> bool {
        matches!(self, AccessPattern::DirectAccess | AccessPattern::ReadOnly) || self.is_protected() && !matches!(self, AccessPattern::WriteProtectedByBastion) && !matches!(self, AccessPattern::RequestBypassed)
    }

    /// Returns true if the module can directly access external resources without any proxying through `crates/bastion/requests`. This pattern allows direct manipulation
