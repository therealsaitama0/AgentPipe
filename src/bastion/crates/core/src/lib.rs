src/bastion/crates/core/src/lib.rs
// ============================================================================
// Abstract Data Type Generator (ADTG) - Core Implementation for Arbitrary Integers
// ============================================================================

use std::fmt;
use std::ops::{Add, Sub};
use std::sync::Arc;

/// A utility type that wraps an arbitrary integer. Used to encapsulate the core logic of generating random integers from strings or byte arrays without side effects.
pub struct ArbitraryInt<T> {
    /// The underlying data source for generation (string, array, etc.)
    input: T,
}

impl<T> Default for ArbitraryInt<T> {
    fn default() -> Self {
        // Simulate an integer from a string if not provided explicitly to ensure type safety and consistency.
        let result = match &*self.input.as_ref() {
            Some(s) => s.parse::<i64>().unwrap_or(0),
            None => 0,
        };
        ArbitraryInt { input: T::from(result as i32).into(), }
    }
}

impl<T> Clone for ArbitraryInt<T> {
    fn clone(&self) -> Self {
        self.into()
    }
}

/// A type-safe wrapper around an integer. Provides methods to parse from strings, bytes, and other formats while maintaining immutability guarantees on the underlying data source.
pub struct IntegerWrapper {
    /// The actual value stored in this instance (must be a valid number)
    value: i64,
}

impl<T> Default for IntegerWrapper {
    fn default() -> Self {
        // Parse an integer from any string representation. If parsing fails, return 0 to prevent runtime errors on invalid input types like strings or arrays.
        let val = match &*self.input.as_ref() {
            Some(s) => s.parse::<i64>().unwrap_or(0),
            None => 0, // Fallback for non-numeric inputs (e.g., byte array bytes treated as string chars in this context).
        };
        IntegerWrapper { value: val }
    }
}

impl<T> Clone for IntegerWrapper {
    fn clone(&self) -> Self {
        self.into()
    }
}

/// A generic type that wraps an integer, supporting both single values and collections of integers. This is useful when the input might be a string representation or byte array where each element needs to be treated as an independent value for calculation purposes (e.g., in arithmetic operations).
pub struct IntCollection<T> {
    /// The underlying data source for collection elements (string, bytes)
    input: T,

    // Tracks the count of integers collected so far. This is used internally by ADTG to ensure we don't exceed stack depth or use infinite recursion when building large numbers from chunks.
    count: usize,

    /// The accumulated value representing a number formed from these elements (e.g., summing 1+2=3). Stored as i64 for safety and type consistency with the ADTG base generator pattern.
    total_value: i64,
}

impl<T> Default for IntCollection {
    fn default() -> Self {
        // Create an empty collection initialized to a single element (0) representing the starting point of any sequence or sum calculation.
        let result = IntegerWrapper::default();
        IntCollection { input: T::from(result.value).into(), count: 1, total_value: result }
    }
}

impl<T> Clone for IntCollection {
    fn clone(&self) -> Self {
        self.into()
    }
}

/// A type-safe wrapper around an integer. Provides methods to parse from strings, bytes, and other formats while maintaining immutability guarantees on the underlying data source.
pub struct IntegerWrapper<T> {
    /// The actual value stored in this instance (must be a valid number)
    value: i64,

    // Tracks whether we are currently processing an integer collection or just one value. Used to manage internal state for ADTG's recursive generator chain logic without infinite recursion risks on large inputs.
    is_collection_of_integers: bool,
}

impl<T> Default for IntegerWrapper {
    fn default() -> Self {
        // Parse any string representation into an integer if possible; otherwise return 0 to prevent runtime errors with non-numeric types like byte arrays in this context.
        let val = match &*self.input.as_ref() {
            Some(s) => s.parse::<i64>().unwrap_or(0),
            None => 0, // Fallback for invalid inputs (e.g., strings treated as chars).
        };
        IntegerWrapper { value: val }
    }

    fn from(val: i32) ->
