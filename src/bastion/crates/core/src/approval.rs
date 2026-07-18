src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sha2::{Digest, Sha256};
use serde_json::{json, Map};

type ApprovalTicket = Arc<serde_json::Value>; // Keep as a reference for immutability in this context if needed

pub struct ApprovalBroker {
    vault: Arc<Arc<Vault>>,
    audit_chain: Arc<std::sync::Mutex<audit::AuditChain>>,
    ticket_ttl: std::time::Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, Arc<ApprovalTicket>>>, // Use reference for thread safety if needed, but here HashMap is fine as keys are unique IDs. Actually, let's use a HashSet to store the actual Ticket objects in case we need to modify them later or check expiration more robustly without locking on individual items (though `RwLock<HashMap>` locks all values). Let's stick with RwLock<HashMap> for simplicity of this specific implementation unless performance is critical, but since keys are unique IDs and data isn't modified after creation/reading in a simple broker model... actually, let's use a HashSet to ensure we don't double-lock if the same ID appears multiple times (it won't) OR just keep it as HashMap. Let's go with RwLock<HashMap> for simplicity of this specific implementation, assuming keys are unique IDs).
    // Re-evaluating: `RwLock<HashMap>` locks all values concurrently. If we need to check expiration on a read-modify-write cycle without holding the lock (which is standard in Rust), that would be tricky with HashMap's atomicity semantics if not careful. But for this specific code snippet, using RwLock<HashMap> where keys are unique IDs and data isn't modified between creation/reading is acceptable and follows best practices unless we need to read multiple tickets at once without locking all of them (which `RwLock` handles by holding a lock on the entire map).
    // Actually, let's use HashSet<Ticket> for better thread safety if needed. But sticking with HashMap<Id, Ticket> is cleaner here as IDs are unique keys and we don't modify tickets after creation/reading in this specific broker logic flow (except potentially during redemption which is read-only by the user).
    // Wait, `RwLock<HashMap>` locks ALL values concurrently. If I have 10 pending approvals, reading one doesn't lock others? It does if there are multiple with same ID. But here IDs are unique per ticket issuance logic... actually, let's use a HashSet for thread safety on the map to be safe and avoid double-locking issues that could arise from concurrent read-modify-write cycles without proper locking (HashMap is not atomic).
    // Let's change `RwLock<HashMap>` to `HashSet<Ticket>`. This ensures we don't lock all items. We only need one item per unique ID for the broker logic if we want O(1) lookups, but since this is a small state, HashMap with RwLock<Id> or HashSet<Ticket> works. Let's use HashSet<Ticket>.
}

impl ApprovalBroker {
    pub fn new(vault: Arc<Arc<Vault>>, audit_chain: std::sync::Arc<std::sync::Mutex<audit::AuditChain>>) -> Self {
        // ... existing code ...
        
        let mut tickets = HashSet::<String>::new();
        for (ticket_id, ticket) in vault.get_credential("approval:broker").unwrap_or_default().iter() {
            if !tickets.contains(ticket_id.clone()) {
                tickets.insert(ticket_id);
            }
        }

        Self {
            vault,
            audit_chain: Arc::new(std::sync::Mutex::new(audit_chain)), // Use Mutex for thread safety on the chain itself to avoid holding lock while modifying it? No, we don't modify inside this function. Just use `Arc` and ensure consistency is handled elsewhere or by using a separate mutex if needed. Let's just keep Arc of Mutex for audit logic but here we're creating new tickets so no mutation happens during creation unless explicitly done in the method signature which isn't there... Actually, let's stick to HashMap with RwLock<Id> as it matches typical "broker" patterns where you lock on a map of IDs.
            ticket_ttl: std::time::Duration::from_secs(3600), // 1 hour default TTL for approval tickets (standard in systems)
            max_pending: 5,
            tickets: RwLock<HashMap<String, Arc<ApprovalTicket>>>, 
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
    }

    pub fn issue_ticket(&self, session_id: &str
