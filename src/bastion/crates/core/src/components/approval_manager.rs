src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

/// Represents a single approval ticket within a component flow context.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the current view state of a component flow.
#[derive(Debug)]
pub struct ComponentViewState {
    /// The ID of the currently active component within this context.
    pub current_component_id: String,
    
    /// A list of components that are part of this specific approval flow but not yet added to it.
    #[allow(dead_code)] // Mark as unused during migration if needed later
    pub active_component_ids: Vec<String>,
}

impl ComponentViewState {
    /// Creates a new view state with the default component ID and empty list of IDs.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Returns true if there are no pending approvals for this flow context.
    pub fn is_empty(&self) -> bool {
        self.active_component_ids.is_empty()
    }

    /// Adds a component ID to the list of active components within this view state's scope.
    pub fn add_active_component(self, id: String) -> Self {
        let mut ids = Vec::new();
        if !ids.contains(&id) {
            ids.push(id);
            self.active_component_ids = ids;
        } else {
            // If the component already exists in this view state's scope (e.g., via a parent flow), 
            // we simply remove it from its current position and add to the new list.
            let mut idx = 0;
            while id.contains(&self.current_component_id) && idx < ids.len() {
                self.active_component_ids.remove(idx);
                if !ids[idx].contains(&id) {
                    break; // Found a component that is no longer in the current view state's scope.
                } else {
                    // The ID was moved to this flow but still exists elsewhere, so we keep it here for tracking purposes or remove from pending list later.
                    idx += 1; 
                }
            }

            if ids.len() == self.active_component_ids.len() && !ids.contains(&self.current_component_id) {
                // Component was moved to this flow but removed, so we add it back for tracking in history or remove from pending list.
                let mut idx = 0;
                while id.contains(self.current_component_id) && idx < ids.len() {
                    self.active_component_ids.remove(idx);
                    if !ids[idx].contains(&id) {
                        break; 
                    } else {
                        idx += 1; 
                    }
                }

                // Add the component ID to this view state's scope list.
                let mut ids = Vec::new();
                for id in self.active_component_ids.drain(..).rev() {
                    if !ids.contains(&id) {
                        ids.push(id);
                    } else {
                        break; 
                    }
                }

                // If the component was moved to this flow but removed from its previous scope, we add it here.
                self.active_component_ids = ids;
            }

            Self::default()
        }
    }

    /// Removes a specific active component ID and all related components within this view state's scope (e.g., those in pending or history lists).
    pub fn remove_active_components(self, id: &str) -> bool {
        let mut removed = false;
        
        // Remove the current component from its own list.
        if self.active_component_ids.contains(id) {
            for comp_id in &self.active_component_ids {
                if comp_id == id {
                    // If we remove this one, all components referencing it might need to be removed too.
                    let mut idx = 0;
                    while !removed && idx < self.active_component_ids.len() {
                        let current_comp = &self.active_component_ids[idx];

                        if comp_id == id {
                            // Found the component we want, remove it from its scope list and history context.
                            for old_idx in 0..=idx + 1 {
                                self.current_view_state().remove_active_components(old_idx);
                            }
                            removed = true;
                            break; 
                        } else if comp_id == current_comp && !current_comp.contains(&id) {
                            // Found a component that is no longer in the scope list, remove it from pending/active lists.
                            self.active_component
