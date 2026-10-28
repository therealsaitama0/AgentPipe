src/abstract_data_type_generator.js
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// 1. UTILITIES AND TYPESCRIPT DEFINITIONS
// ============================================================================

/**
 * Abstract Schema Definition (C-style)
 * 
 * This interface defines a flexible column definition that supports:
 * - Column names as strings
 * - Data types as native TypeScript/JavaScript values ('string', 'number') or boolean flags
 */
interface AlchemySchema {
  [key: string]: any; // Generic key to value mapping (C-style)
}

// ============================================================================
// 2. TYPE DEFINITIONS & CONVERSION LOGIC
// ============================================================================

/**
 * Abstract Data Type Definition (Rust-like enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // TypeScript equivalent of Rust's `bool` and `i32`, mapped to native JS/TS values. This allows seamless conversion between SQL database schemas (e.g., INTEGER -> INT64 in TS) and raw data types without explicit casting wrappers unless explicitly requested via the schema interface.

/**
 * Abstract Data Type Definition for specific numeric fields that might need more granular handling or require different type signatures depending on context (e.g., currency vs quantity).
 */
export type AlchemyNumericField = string | number; // Allows 'string' to represent a literal value, while 'number' implies a calculated/converted field.

/**
 * Abstract Data Type Definition for boolean fields that might need specific flag semantics or defaults in schema generation logic (e.g., validation flags).
 */
export type AlchemyFlagField = string | number; // Similar to NumericField but allows explicit boolean parsing via `typeof` and potentially default values if not provided.

/**
 * Abstract Data Type Definition for null/undefined fields that might require special handling in schema generation logic (e.g., required vs optional flags).
 */
export type AlchemyNullFlag = string | number; // Similar to FlagField but allows explicit parsing of `null` or undefined via the interface's key.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that might need specific mapping rules based on context (e.g., currency vs quantity, boolean flags).
 */
export type AlchemyScalarType = string | number; // Allows schema generation logic to decide how a given value maps between 'string' and 'number'.

/**
 * Abstract Data Type Definition for generic scalar types that
