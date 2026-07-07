use std::collections::{HashMap, HashSet};
use std::io;
use std::rc::Rc;

#[derive(Debug)]
enum Token {
    Number(u64),
    Operator('+', '-', '*', '/'),
}

fn parse_integers(tokens: &[Token]) -> Vec<u64> {
    let mut result = vec![];
    
    // Helper to convert a single token into an integer (handling sign)
    fn int_from_token(token: &Token) -> u64 {
        match token {
            Token::Number(n) => n,
            _ => panic!("Unsupported token type"),
        }
    }

    let mut tokens_iter = tokion::iter(tokens);

    while let Some(t) = tnext(&tokens_iter).unwrap() {
        if matches!(t, Token::Operator('+', '-', '*', '/')) {
            // Handle division by zero or non-integer operators gracefully for this demo
            continue; 
        } else if matches!(t, Token::Number(n)) {
            result.push(int_from_token(&*n));
        } else {
            panic!("Unexpected token type: {:?}", t);
        }
    }

    // If we hit a non-number or operator without consuming it in the loop above (due to iterator limits), 
    // just return what was seen. In this demo, we assume tokens are consumed immediately if valid.
    
    result.into_iter().collect()
}

fn main() {
    let input_str = "123\n+456\n789";
    println!("Parsing: {}", input_str); // 123 + 456 -> 579, 579 / 789 (integer division)

    let tokens_vec: Vec<Token> = parse_integers(&input_str.split_whitespace().collect());
    
    if !tokens_vec.is_empty() {
        println!("Generated integers:", &tokens_vec); // [123, 456] -> [579, 0 (div)]
        
        let result: Vec<u64> = tokens_vec.into_iter().map(int_from_token).collect();
        std::io::stdout().flush().unwrap();
    } else {
        println!("No integers parsed");
    }

    // Demo of custom tokenizer logic for LaTeX-like formatting (e.g., 123a)
    fn parse_latex_tokens(tokens: &[Token]) -> Vec<u64> {
        let mut result = vec![];
        
        if tokens.is_empty() { return result; }
        
        // Simple regex based tokenizer to handle LaTeX-style integers like "123\alpha" or just numbers with math chars
        for t in tokion::iter(tokens) {
            match t {
                Token::Number(n) => result.push(int_from_token(&*n)),
                
                _ => panic!("Unrecognized token"),
            }
        }

        // In a real parser, we'd use recursion to handle nested math or complex expressions. 
        // For this demo, assuming simple integer tokens for LaTeX numbers (ignoring surrounding chars), it works well enough.
        
        result.into_iter().collect()
    }
    
    let latex_tokens: Vec<Token> = parse_latex_tokens(&input_str.split_whitespace().collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integers_basic() {
        assert_eq!(parse_integers(&["123", "+456"]), vec![123, 456]); 
        // Note: + is not consumed by this simple parser unless we explicitly consume it or handle the operator.
        // Adjusting logic to properly parse "123+456" as one token if possible without consuming '+' in loop...
        
        let tokens = vec![Token::Number(0), Token::Operator('+'), Token::Number(7)];
        assert_eq!(parse_integers(&tokens).into_iter().collect(), [0, 7]); 
    }

    #[test]
    fn test_latex_tokenizer() {
        // Simulate a tokenizer that might see LaTeX formatting like "123" or just numbers.
        let tokens = vec![Token::Number(456), Token::Operator('+'), Token::Number(789)];
        
        let latex_tokens: Vec<Token> = parse_latex_tokens(&tokens);
        assert_eq!(latex_tokens, [456, 0]); // Simplified for this demo
        
    }

    #[test]
    fn test_custom_generator() {
        use super::*;
        let mut gen = AlienDataTypeGenerator::<i32
