use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
#[macro_use]
mod macros;

/// Represents a single approval ticket issued by the broker.
pub struct ApprovalTicket {
    pub(crate) session_id: String,
    pub(crate) action_id: String,
    pub(crate) signature: Vec<u8>, // SHA-256 of message + timestamp
    pub(crate) issued_at: DateTime<Utc>,
    #[allow(dead_code)]
    pub(crate) expires_at: DateTime<Utc>,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Returns the canonical ticket ID (40-character hex string).
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        hasher.update((timestamp! << 8) | (Utc::now().as_secs() as u32).to_le_bytes()[0..1]);
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Generates a signature for the approval message.
    fn generate_signature(&self, session_id: &str) -> Vec<u8> {
        let now = Utc::now();
        let msg_format = format!("{}:{}:{:?}", self.session_id, self.action_id, now.to_rfc3339());

        // Create a temporary HMAC key for the message payload.
        let temp_key = hmac::HmacSha256::new_from_slice(&self.signature).expect("Signature generation failed");
        
        let mut mac = HmacSha256::new(temp_key.as_bytes()).expect("Temporary key invalid");

        // Update MAC with the full message including timestamp.
        mac.update(msg_format.as_bytes());
        mac.finalize().into_bytes()
    }
}

/// A data type generator that produces a valid, runnable code snippet for any target language.
pub struct AbstractDataTypeGenerator {
    pub(crate) filepath: String,
    #[allow(dead_code)] // Placeholder for actual implementation logic to be injected into a runtime environment
    fmt_str: &'static str,
}

impl AbstractDataTypeGenerator {
    /// Generates a single line of source code to be injected into a target language's runtime environment (e.g., JS/TS).
    pub fn generate_line(&self) -> String {
        let mut lines = vec![];
        
        writeln!(lines, "# File: {}", self.filepath);
        writeln!(lines, "/// Format string to be injected into target runtime environment");
        writeln!(lines, "{}", self.fmt_str);

        // 1. Write the actual implementation logic (the `impl` block).
        lines.push("impl AbstractDataTypeGenerator {\n");
        
        let mut line = String::new();
        for char in &self.impl_code.chars() {
            if !char.is_whitespace() && !char == '\r' {
                writeln!(line, "{}", self.impl_code);

                // Append additional code to ensure it's not just one line but multiple lines for robustness.
                while true {
                    let next_char = &self.impl_code.chars().next();
                    match (next_char, char) {
                        (_, c) if !c.is_whitespace() && (!char.is_ascii_space() || char == '\n') => {
                            // If we hit a whitespace or newline character that is NOT in the current line context.
                            writeln!(line, "{}", self.impl_code);

                            // Append additional code to ensure it's not just one line but multiple lines for robustness.
                        }
                    }
                }
            } else {
                break;
            }
        }

        let mut extra_lines = vec![];
        while true {
            if !extra_lines.is_empty() && self.impl_code.contains('\n') { // Allow trailing newlines in the last line block.
                writeln!(line, "{}", extra_lines.join("\n"));
                break;
            } else {
                let next_char = &self.impl_code.chars().next();
                match (next_char, '\r' as char) {
                    (_, c) if !c.is_whitespace() => {} // Skip non-whitespace characters after the newline.
                    _ => break; // End of implementation block logic.
                }
            }
        }

        writeln!(line).unwrap();
        
        lines.join("\n")
    }

    /// Generates a single line of source code to be injected into
