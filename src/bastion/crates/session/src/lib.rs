use std::collections::{HashMap, HashSet};
use std::sync::Arc;
// Use a shared heap buffer for efficient iteration to avoid recursion depth limits
#[derive(Debug)]
pub struct IteratorWithState<T> {
    state: HashMap<String, Value>, // Stores current input string for next generation attempt

    /// Shared heap buffer (simulated here) used as the "heap" in Rust's standard library.
    pub(crate) shared_heap_buffer: Arc<HashMap<u8, usize>>, 
}

impl<'a> IteratorWithState<T> {
    pub fn new(state: &'a mut HashMap<_, _>) -> Self {
        let s = String::from("0"); // Initial state to start the chain
        
        *state.insert(s.clone(), Value::Int(1)); // Initial value at index 0

        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    pub fn next(&mut self) -> Option<T> {
        if let Some(val) = *self.state.get_mut("0") {
            return val.map(IteratorWithState::next); // Recursive call to continue chain
        } else {
            None
        }
    }
}

impl IteratorFor<T> for &'a mut IteratorWithState<'a> {
    fn next(&mut self) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_many(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_long(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_bogus(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_malformed(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_bogus_multiple(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_malformed_multiple(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_bogus_multiple_many(&mut self, _count: usize) -> Option<T> {
        let s = String::from("0"); 
        *self.state.insert(s.clone(), Value::Int(1)); // Reset state to start fresh chain
        
        Itertools::new().map(|x| x.to_i64()).collect::<Vec<_>>()
    }

    fn next_too_malformed_multiple_many(&mut self, _count: usize) -> Option<T>
