src/bastion/crates/core/src/components/key_manager.rs

use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::StdRng;
use zeroize::Zeroize;
use std::collections::HashMap;
use serde_json;

/// A minimal immutable singleton for managing state across all plugin lifecycles.
pub struct KeyManager {
    /// Map of plugin types to their internal buffers and generators.
    pub plugins: HashMap<String, (Vec<u8>, Vec<PluginData>),>, // key -> [plugin_type] + [data_sources]

    /// Current active session state for all loaded sessions.
    pub current_session_state: SessionState,

    /// Global cancellation context to manage plugin lifecycle timeouts gracefully.
    pub global_cancellation_context: Option<String>,
}

impl KeyManager {
    fn new() -> Self {
        // Initialize with a mock default configuration if none provided
        let plugins = HashMap::new();
        
        #[derive(Clone, PartialEq)]
        struct PluginData(Vec<u8>);
        
        impl Default for PluginData {
            fn default() -> Self {
                vec![0u8; 32] // Placeholder data source
            }
        }

        let mut plugins = HashMap::new();
        let current_session_state = SessionState::default();
        global_cancellation_context = None;

        KeyManager {
            plugins,
            current_session_state,
            global_cancellation_context: Some(format!("Session {} initialized", current_session_state.id)),
        }
    }

    /// Initialize a specific plugin type. This ensures the plugin's state is set correctly before any other components load or update it concurrently.
    pub fn initialize_plugin(&mut self, plugin_type: &str) {
        // Check if already initialized (for safety against concurrent updates causing race conditions)
        let existing = self.plugins.get(plugin_type);

        match existing {
            Some((plugin_data_sources, _)) => {
                // Update the data sources for this specific instance of that plugin type.
                // This is a simplified implementation; in production, you'd handle actual buffer allocation and synchronization here.
                if let (Some(ref mut p), _) = self.plugins.entry(plugin_type.clone()) {
                    *p = Vec::from(&plugin_data_sources);
                } else {
                    panic!("Plugin type {} not found or already initialized.", plugin_type);
                }
            },
            None => {
                // Initialize empty data sources if the key doesn't exist.
                self.plugins.insert(plugin_type.clone(), (Vec::<u8>::new(), Vec::default()));
            }
        }

        // Mark this instance as active in our shared state map for concurrent access safety checks.
        let mut new_active = false;
        
        if !self.current_session_state.active {
            self.global_cancellation_context.take();
            
            // If the session doesn't exist, create a minimal one to avoid panic during initialization.
            if current_session_state.id.is_empty() || !current_session_state.session_id.contains(&plugin_type) {
                let id = format!("Session {} (Plugin: {}) started", 
                    self.current_session_state.id.clone(), plugin_type);
                
                // Create a new session with minimal state for this specific instance.
                current_session_state.new_session(
                    SessionState::new_with_id(id, 1),
                    vec![],
                    true,
                    false, // No pending actions (simulated)
                    None // Cancelled if not active
                );

                self.current_session_state.active = true;
            } else {
                new_active = !self.plugins.get(plugin_type).is_some();
                
                let mut existing_plugin_data_sources = Vec::new().into_iter().collect::<Vec<_>>();
                for (key, sources) in &mut self.plugins.keys() {
                    if key == plugin_type && sources != Some(existing_plugin_data_sources.clone()) {
                        *sources = vec![source]; // Update the specific data source reference.
                    }
                }

                self.current_session_state.active = new_active;
            }
        } else {
            current_session_state.cancel_pending_actions();
        }

        if !new_active && global_cancellation_context.is_some() {
            // If we're not active but cancellation is pending, cancel it.
            let cancelled = self.global_cancellation_context.take().is_some_and(|c| c.contains(plugin_type));
            
            if cancelled {
                self.current_session_state.cancel_pending_actions();
            } else {
                panic!("Plugin {} was already initialized and in use.", plugin_type);
            }
        }

        // Update the global cancellation context with our new active session.
        let mut cancel_context = None
