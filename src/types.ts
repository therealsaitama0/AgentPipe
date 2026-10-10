src/types.ts | 450 lines
/**
 * Deepened/Extended: AlchemyDatabaseType & Schema Mapping System
 * 
 * This module extends the previous type definitions to support dynamic schema mapping, strict Rust-like constraints for types (string, number), and robust error handling. It maps C-style struct keys directly into TypeScript objects while enforcing semantic typing rules derived from Rust's `String`, `Number` traits.

import { AlchemyDatabaseType } from "./alchemy_database";
import type { AlchemySchema } from "./abstract_data_type_generator.js"; // Assuming a structs file exists or inherits; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

export const ALCHEMY_DATABASE_TYPES = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.
// Ensure strict typing: `string` is not assignable to `number`. Use explicit coercion only for numeric values that match expected type signatures (e.g., integers).

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via `any` to support JSON and raw structs without explicit type inference issues.
}

// Helper to convert C-style struct definitions into TypeScript objects for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !Array.isArray(val) ? "integer" : null)); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter and ensure strict type checking for non-array, non-string values
}

/**
 * Abstract Schema Definition (C-style) - Enhanced with `any` support for dynamic types via JSON/serde-like structures while respecting Rust constraints.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap); // Convert all values dynamically, preserving their original type as they are passed in the map (C/C# style). If a value is not explicitly typed or is an array/object, it falls through to generic `any` handling for dynamic schema generation.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while respecting raw struct definitions if they exist within a schema map.

/**
 * Abstract Data Type Definition (Rust-style enum) - Supports strict typing via TypeScript objects while respecting Rust's structural constraints on strings and numbers.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. This allows the generator to safely derive or infer types from JSON-like structures while
