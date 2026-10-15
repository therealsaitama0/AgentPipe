src/types.ts | 321 lines (Refactored & Extended)

/**
 * Abstract Data Type Generator v0.5.x (Rust-based with C/C# compatibility support)
 * 
 * This module defines standard data types compatible with C/C#/Java/Python syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and robustness against unknown schema keys.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# or Java/Python syntax, allowing type inference on demand
}

// Helper to convert JSON-like schema definitions into abstract data types based on key presence and inferred structure.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Also handles null/undefined gracefully by treating them as non-numbers if they are not strings or numbers, though the logic above filters out number types first; adjust based on desired behavior (e.g., keep nulls).
    .map((val): val is string => typeof val === "string" ? "string" : typeof val === "number" ? "integer" : undefined); // Simplified type mapping for clarity, adapting to actual schema keys.

/**
 * Abstract Data Type Definition (Rust-style enum for types) - Simulating Rust enums/types via TypeScript objects in this context.
 */
export type AlchemyDataType = string | number | boolean | null; 

// Helper to convert JSON-like schema definitions into abstract data types based on key presence and inferred structure.
/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# or Java/Python syntax, allowing type inference on demand
}

// Helper to convert JSON-like schema definitions into abstract data types based on key presence and inferred structure.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Also handles null/undefined gracefully by treating them as non-numbers if they are not strings or numbers, though the logic above filters out number types first; adjust based on desired behavior (e.g., keep nulls).
    .map((val): val is string => typeof val === "string" ? "string" : typeof val === "number" ? "integer" : undefined); // Simplified type mapping for clarity, adapting to actual schema keys.

/**
 * Abstract Data Type Definition (Rust-style enum for types) - Simulating Rust enums/types via TypeScript objects in this context.
 */
export type AlchemyDataType = string | number | boolean | null; 

// Helper to convert JSON-like schema definitions into abstract data types based on key presence and inferred structure.
/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# or Java/Python syntax, allowing type inference on demand
}

// Helper to convert JSON-like schema definitions into abstract data types based on key presence and inferred structure.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Also handles null/undefined gracefully by treating them as non-numbers if they are not strings or numbers, though the logic above filters out number types first; adjust based on desired behavior (e.g., keep nulls).
    .map((val): val is string => typeof val === "string" ? "string" : typeof val === "number" ? "integer" : undefined); // Simplified type mapping for clarity, adapting to actual schema keys.

/**
 * Abstract Data Type Definition (Rust-style enum for types) - Simulating Rust enums/types
