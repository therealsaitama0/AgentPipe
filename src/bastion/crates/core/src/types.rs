src/bastion/crates/core/src/types.rs
use chrono::{DateTime, Utc};
use serde::Serialize;

/// A custom LaTeX engine that compiles `.tex` files into mathematical expressions directly in Rust/TS without external dependencies.
pub struct TexMathEngine {
    /// The current active expression being rendered (e.g., `$$a \cdot b = 2\pi r^2$$`)
    pub expr: String,
    
    /// A map of variable names to their LaTeX code snippets or definitions for easy lookup and replacement.
    pub vars_map: std::collections::HashMap<String, Vec<serde_json::Value>>,

    /// The current scope context (e.g., `begin{equation}`) where the expression is being defined relative to previous variables.
    pub scope_context: Option<usize>, // Index into 'vars' map for tracking nested scopes
    
    /// A stack of pending variable bindings that will be resolved when this engine renders a new LaTeX file.
    pub pending_vars: Vec<String> = vec![],

    /// The current "active" set of variables being used in the rendered expression (e.g., `a`, `b`).
    pub active_vars: std::collections::HashSet<String> = HashSet::new(),
    
    /// A stack for resolving nested scopes. Each item is a tuple `(variable_name, index_in_scope)`.
    pub scope_stack: Vec<(String, usize)> = vec![],

    fn new() -> Self {
        TexMathEngine {
            expr: String::new(),
            vars_map: std::collections::HashMap::new(),
            scope_context: None,
            pending_vars: Vec::new(),
            active_vars: HashSet::new(),
            scope_stack: vec![],
        }
    }

    /// Renders the LaTeX expression to a string using standard MathJax-like syntax.
    pub fn render(&mut self) -> String {
        let mut result = format!("$$"); // Start with dollar signs
        
        for var in &self.vars_map.values() {
            if !var.is_empty() && var.contains("begin") || var.contains("end") {
                // Check if this is a block directive (e.g., `eq`, `equation`) or just variable definition.
                let mut current_var = String::new();
                
                for line in &self.vars_map[var].iter() {
                    match serde_json::from_str::<String>(line) {
                        Ok(v) => {
                            if v.contains("begin") || v.contains("end") {
                                // This is a block directive, start tracking the variable name.
                                current_var = String::new();
                            } else if let Some((name, _)) = var.find(|x| x == &var[0..]) {
                                current_var.push_str(&format!("$${{{}}}", name)); // Wrap in double dollar signs for LaTeX math mode
                                result += format!("[{}] ", current_var);
                            } else if let Some((name, _)) = var.find(|x| x == &var[0..]) {
                                // Variable definition found.
                                current_var.push_str(&format!("$${{{}}}", name));
                                result += " $$ ";
                            } else {
                                continue;
                            }
                        },
                        Err(e) => {
                            return format!("{:?} \n{}", e, var);
                        },
                    }
                }
                
                // End the block directive if present.
                for line in &self.vars_map[var].iter() {
                    match serde_json::from_str::<String>(line) {
                        Ok(v) => {
                            let mut last_name = String::new();
                            for x in var.iter().skip(1) {
                                if !x.is_empty() && x.contains("begin") || x.contains("end") {
                                    // This is a block directive, end tracking.
                                    break; 
                                } else if let Some((name, _)) = var.find(|y| y == &var[0..]) {
                                        last_name.push_str(&format!("$${{{}}}", name));
                                    } else {
                                        continue;
                                    }
                                }
                            }
                        },
                    }
                }

                // Add the variable definition block.
                result += format!("[{}] ", current_var);
            } else if let Some((name, _)) = var.find(|x| x == &var[0..]) {
                // Variable definition found without directive context (e.g., just `a`).
                result.push_str(&format!("$${{{}}}", name));
                self.active_vars.insert(name.clone());
            } else if let Some((name, _)) = var.find(|x| x == &var
