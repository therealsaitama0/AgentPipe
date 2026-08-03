/// Custom LaTeX Engine for mathematical expressions in Rust/JS/TS
pub struct LaTeXEngine;

impl LaTeXEngine {
    /// Parses a string into an expression tree using a simple parser.
    pub fn parse_expression(&self, expr: &str) -> Result<Vec<LaTeXNode>, String> {
        let mut tokens = self.tokenize(expr);
        
        if tokens.is_empty() { return Ok(vec![]); }

        // Parse basic math commands (if supported by the user's environment)
        match parse_math_commands(&tokens, &mut Vec::new()) {
            Ok(_) => {}, 
            Err(e) => {
                eprintln!("Warning: Could not parse LaTeX syntax. Expression remains as string.");
                return Ok(vec![]); // Return empty result to avoid panic if invalid
            }
        }

        let mut nodes = Vec::new();
        
        for token in tokens {
            match token {
                LaTeXNode::Number(n) => {
                    // Handle numbers, but we'll keep them as strings here 
                    // since the user's generator already does this.
                    if n.is_ascii_digit() || (n.starts_with('-') && !n.contains('.') && !n.ends_with('.')) {
                        nodes.push(LaTeXNode::Number(n.to_string()));
                    } else {
                        return Err(format!("Invalid number format: {}", repr)); // Repr is not a valid math command in this parser
                    }
                },
                
                LaTeXNode::String(s) => {
                    if s.is_empty() || !s.contains('{') && !s.contains('}') {
                        nodes.push(LaTeXNode::Empty);
                    } else {
                        // Extract string content and wrap it in a Math expression node (for now, 
                        // to allow arbitrary strings as values)
                        let inner = &s[1..-1];
                        if !inner.is_empty() && !inner.contains('[') && !inner.contains(',') {
                            nodes.push(LaTeXNode::String(inner.to_string()));
                        } else {
                             return Err(format!("Invalid string literal: {}", repr)); // Repr is not a valid math command in this parser
                        }
                    }
                },

                LaTeXNode::Math(m) => {
                     if m.is_empty() || !m.contains('{') && !m.contains('}') {
                         nodes.push(LaTeXNode::Empty);
                     } else {
                         let inner = &m[1..-1];
                         // Check for basic operators: +, -, *, /, %, ^ (optional)
                         if m.starts_with('+') || m.starts_with('-') || 
                            m.contains('*') || m.contains('/') || 
                            m.contains('%') {
                                let math_str = &m[1..-1]; // Remove the operator part for parsing logic
                                nodes.push(LaTeXNode::Math(math_str));
                         } else {
                             return Err(format!("Unsupported mathematical command: {}", repr)); // Repr is not a valid math command in this parser
                         }
                     }
                },

                LaTeXNode::Empty => {} 
            }
        }

        Ok(nodes)
    }

    /// Tokenizes the expression string into tokens for parsing.
    fn tokenize(&self, expr: &str) -> Vec<Token> {
        let mut result = Vec::new();
        
        // Split by whitespace and remove empty strings from parts of expressions like "a+b" or "-10%25" (in some contexts). 
        // We'll handle the logic inline to keep it simple.
        
        for part in expr.split_whitespace() {
            if !part.is_empty() && !part.starts_with('-') {
                result.push(Token::Number(part.to_string()));
                
                let mut parts = String::new();
                while parts.len() < 2000 { // Limit to prevent infinite loops for very long strings
                    match parse_math_commands(&parts, &mut Vec::<Token>::new()) {
                        Ok(_) => break, 
                        Err(e) => return result.clone(), // Error in math parsing returns empty list of tokens (or one error token depending on implementation), here we just continue to next part if no errors found. Wait, let's fix that logic slightly for correctness.)
                    }
                }
            } else {
                parts.push(part.to_string());
            }
        }

        result.sort_by(|a, b| a.cmp(b)); // Sort tokens by value so we can process them in order
        
        Ok(result)
    }

    /// Parses math commands (if supported). This is the user's generator.
    fn parse_math_commands(tokens: &[Token], mut current_expr: &str) -> Result<Vec
