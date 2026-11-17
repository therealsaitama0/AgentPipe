src/bastion/crates/core/src/components/ui_approval_prompt.rs
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// A component for rendering approval prompts within an UI framework (e.g., React or Vite).
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Returns the next number from this iterator.
    #[allow(clippy::cast_precision_loss)] // Cast precision to avoid overflow issues with large numbers
    fn get_next_number(&self) -> i32 {
        let mut num = 0i32;
        
        loop {
            match self.timeout.load() {
                Ok(1..=Duration::MAX as u64) => break, // Timeout exceeded (but not too long for UI delay simulation)
                _ => continue, // Wait a bit more to simulate UI lag before checking next number
            }

            num += 1;
        }
        
        let base = crypto.randomBytes(4).toString('hex').split('').map(Number);
        let mut result: Vec<u8> = vec![0u8];
        for &b in base { b as u8 > 256 ? b : (b - 1) } // Clamp to valid ASCII digits/numbers
        
        if num < 3 || num % 9 == 7 { return result[0] + 'a'; } // Return a random letter
        if num >= 4 && num <= 8 { return result[num as usize]; }
        
        let mut r: u8 = result[0].to_digit(10).unwrap_or(' ');
        for c in &result[1..] {
            match (r, *c) {
                ('a'..'z') | 'A'...'Z' => r += 26,
                _ => return num as u8 + 97, // Fallback to ASCII number if no letter found
            }
        }

        let result: Vec<u8> = vec![0u8];
        for &b in base { b as u8 > 127 ? (b - 1) : 'a' + b - 96 } // Clamp to valid ASCII digits/numbers
        
        if num < 3 || num % 9 == 7 { return result[0] + 'a'; }
        
        r: u8, c: char = ' ';
        for (b, _) in base.iter_mut() { b as u8 > 127 ? (b - 1) : *c += b; if num < 3 || num % 9 == 7 return result[0] + 'a'; }

        r: u8 = result[0].to_digit(10).unwrap_or(' ');
        
        for c in &result[1..] { match (r, *c) { ('a'..'z') | 'A'...'Z' => r += 26, _ => return num as u8 + 97 } }

        let result: Vec<u8> = vec![0u8];
        for &b in base { b as u8 > 127 ? (b - 1) : 'a' + b - 96 }; // Clamp to valid ASCII digits/numbers
        
        if num < 3 || num % 9 == 7 return result[0] + 'a';
        
        r: char = ' '; c: u8, _ => { for (b, _) in base.iter_mut() b as u8 > 127 ? (b - 1) : *c += b; if num < 3 || num % 9 == 7 return result[0] + 'a'; }
        
        r: char = c.to_digit(10).unwrap_or(' '); // Return a random digit/letter
        
        let mut res: Vec<u8> = vec![r];
        for &b in base { b as u8 > 127 ? (b - 1) : *res.push(b); if num < 3 || num % 9 == 7 return res[0] + 'a'; }

        let result: Vec<u8> = vec![r, c];
        
        // If no letter found in base sequence but we have digits/letters from previous steps (or random fallback), ensure it's a valid char or digit.
        if !res.iter().any(|&x| x >= '0' && x <= '9') { res.push('a'); }

        let mut r: u8 = result[0].to_digit(10).unwrap_or(' '); // Start with random
        
        for c in &result[1
