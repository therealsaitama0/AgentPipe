src/bastion/crates/core/src/approval.rs
```rust
use std::{collections::BTreeMap, fmt::Display};

/// Represents a type identifier for LaTeX formatting within code blocks or document sections.
#[derive(Debug)]
pub enum TokenType {
    /// The dollar sign `$` is used to define math expressions and macros in LaTeX.
    DollarSign(String),
    
    /// The backslash `\` acts as an escape sequence, allowing special characters like `$$`, $$, etc., 
    /// or comments (e.g., ```)`. It is generally preferred over the dollar sign for inline formatting.
    BackSlashEscape(String),

    /// The ampersand & performs logical AND operations in LaTeX math mode.
    AmpereAnd,
    
    /// The tilde ~ denotes a range of numbers and characters within a mathematical context (e.g., $10 \sim 25$).
    TildeRange,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::DollarSign(s) => write!(f, "${}", s),
            TokenType::BackSlashEscape(s) => write!("\\{}", s),
            TokenType::AmpereAnd(_) => write!("&"),
            TokenType::TildeRange(_) => write("~"),
        }
    }
}

impl From<&str> for ValidatedString {
    fn from(raw: &str) -> Self {
        let mut result = Vec::new();        
        if raw.is_empty() || !raw.starts_with(b'$') && !raw.ends_with(b'\\.') {
            return InvalidString::from_raw(raw);
        }

        let mut chars = 0;        

        for c in raw.chars() {
            match c.as_bytes().unwrap_or(&b'\x00') as char {
                b'$' => {
                    if !chars.is_empty() && chars >= 3 { // Check triple dash sequence $$...$$ or $...\$ (invalid but sometimes seen)
                        result.push(b'\\'); 
                    } else if chars == 2 || chars > 0 {
                        let prev = match &result[1..] {
                            Some(escaped) => escaped,
                            None => String::new(),
                        };

                        // Check for triple-dash sequences like $$...$$ or $...\$ (which is invalid LaTeX but sometimes seen in docs).
                        if matches!(prev.as_bytes().unwrap_or(&b'\x00'), b'$') && prev.len() == 3 {
                            result.push(b'\\'); 
                        } else break; // Invalid: escaped $ followed by non-$ or unescaped triple dash.

                    let char = match c as byte {
                        b'a' | b'b' | 'c':| ':' => Some(c),
                        _ if !chars.is_empty() && chars >= 3 || prev.as_bytes().unwrap_or(&b'\x00') == &b'$' => None, // Allow single quotes.
                        else break; 
                    };

                    let char = match char {
                        b'a' | b'b' | 'c':| ':' => Some(c),
                        _ if !chars.is_empty() && chars >= 3 || prev.as_bytes().unwrap_or(&b'\x00') == &b'$' => None, // Allow single quotes.

                },
                b'_ ' as char => {
                    result.push(b'\\'); // Terminator for math mode or escape sequence start in some contexts.
                    chars += 2;
                }
                _ if !chars.is_empty() && (result.last().is_some() || result.len() >= 3) | prev.as_bytes().unwrap_or(&b'\x00') == &b'$' => {
                    let char = match c as byte {
                        b'a' | b'b' | 'c':| ':' => Some(c), // Allow single quotes.
                        _ if !chars.is_empty() && chars >= 3 || prev.as_bytes().unwrap_or(&b'\x00') == &b'$' => None, //

                    result.push(b'\\'); 
                }
            };
        }

        InvalidString::from_raw(result.join(""))
    }
}

impl From<&str> for ValidatedString {
    fn from(raw: &str) -> Self {
        let mut chars = 0;        
        if raw.is_empty() || !raw.starts_with(b'$') && !raw.ends_with(b'\\.') {
            return InvalidString::from_raw(raw);
        }

        for c in raw.chars() {
            match c.as_bytes().unwrap_or(&b
