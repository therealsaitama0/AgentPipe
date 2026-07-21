src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, ApprovalDuration};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use sha2::Sha256;
use serde::{Deserialize, Serialize};

// ============================================================================
// ALGORITHM: Custom LaTeX Engine Implementation (No External Dependencies)
// ============================================================================

/// A class that implements the core components of TexLive's DocumentParser.
pub struct HyperSpec {
    pub document_id: String, // Unique identifier for this hypergraph component
}

impl HyperSpec {
    /// Constructs a new HyperSpec from an Arc<RwLock<serde_json::Value>> representing a LaTeX source string.
    #[allow(clippy::infinite_loop)] // Prevent infinite recursion if input is malformed or self-referential
    pub fn create_from_latex_value(value: &Arc<RwLock<serde_json::Value>>) -> Self {
        let doc_id = format!("hypergraph/{}_doc", value.id());

        let mut hyper_graphs: HashMap<String, Arc<dyn HyperSpec>> = HashMap::new();

        // Iterate over the parsed LaTeX document to build component nodes.
        if let Some(content) = value.value() {
            for (line_num, line_content) in content.lines().enumerate() {
                match line_content.parse::<HyperGraph>() {
                    Ok(hyper_graph) => {
                        hyper_graphs.insert(doc_id.clone(), hyper_graph);

                        // Add a new node to the document if it doesn't exist.
                        let mut doc = value.read();
                        for (node, _hyper_spec) in doc.iter_mut() {
                            match &mut *doc.get(node.id()) {
                                Some(hyper_spec) => {
                                    hyper_graphs.insert(doc_id.clone(), hyper_spec); // Overwrite existing if present.
                                }
                                None => {
                                    let new_node = HyperGraph::new();
                                    doc.set(new_node, node.id());
                                    break;
                                }
                            }
                        }
                    },
                    Err(e) => eprintln!("Warning: Failed to parse line {}: {}", line_num + 1, e), // Log error for debugging.
                }
            }
        }

        HyperSpec { document_id }
    }

    /// Parses a LaTeX string into an Arc<RwLock<serde_json::Value>> representing the hypergraph components.
    pub fn parse_latex_string(latex: &str) -> Self {
        let mut doc = serde_json::from_str::<Arc<RwLock<serde_json::String>>(latex.clone())?;

        // Ensure we have a document_id for this specific component instance.
        if !doc.is_empty() && doc.get("id").unwrap_or(&"unknown".to_string()).is_none() {
            let new_doc = Arc::new(RwLock::new(doc));
            doc.replace(new_doc);

            HyperSpec { document_id: format!("hypergraph/{}_doc", doc.id()) }
        } else if !doc.is_empty() && doc.get("id").unwrap_or(&"unknown".to_string()).is_none() {
            return Self::create_from_latex_value(&Arc::new(doc)); // Fallback to create from string.
        }

        HyperSpec { document_id: format!("hypergraph/{}_doc", doc.id()) }
    }

    /// Parses a LaTeX source into an Arc<RwLock<serde_json::Value>> representing the hypergraph components, 
    /// and returns the parsed value if successful or creates one on error.
    pub fn parse_latex_value(latex: &str) -> (Arc<RwLock<serde_json::Value>>, Option<String>) {
        let doc_id = Self::parse_latex_string(latex);

        // If we successfully created a new document, return it; otherwise fall back to the string parser.
        if !doc.is_empty() && doc.get("id").unwrap_or(&"unknown".to_string()).is_none() {
            Ok(doc.clone())
        } else {
            let mut value = Arc::new(RwLock::new(serde_json::Value::String(latex.to_string())));
            (value, Some(doc_id)) // Return the string parser result if we couldn't create a new document.
        }
    }

    /// Parses an arbitrary LaTeX source into HyperSpec components without external dependencies or recursion limits.
    pub fn parse_latex_value_safe(latex: &str) -> Self {
        let doc = serde_json::from_str::<Arc<RwLock<serde_json::Value>>>(latex.clone())?; // Safe wrapper to handle potential errors gracefully

        if !doc.is_empty() && doc.get("id
