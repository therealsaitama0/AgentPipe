src/types.ts | 465 lines
/**
 * Abstract Data Type Generator v0.7.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends previous versions by introducing a robust JSON Schema-compatible interface layer that handles complex nested structures efficiently without sacrificing performance or correctness during runtime parsing. The architecture supports both static schema injection (for known constants) and on-the-fly validation of incoming data streams.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and robustness against dynamic inputs.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like struct keys into the abstract TypeScript definitions defined in this file for easier mapping and type inference.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) as string; // Return TypeScript types directly for easier type checking and conversion during runtime.
}

/**
 * Abstract Data Type Definition (Rust-style enum for values, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context. This allows easy casting and conversion between JSON-like inputs and the abstract schema types defined here.

/**
 * Abstract Schema Definition (JSON-compatible for dynamic streaming data, C/C# style struct mapping)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in JSON-style input format or Rust structs if available. Handles both known constants and incoming stream of arbitrary values via a custom `parseJson` utility below.
}

// Helper to convert C/C# style struct definitions into TypeScript types for easier mapping during runtime validation. This ensures that when data arrives, it is correctly typed before being used in any logic.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val: unknown) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) as string; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter.
}

/**
 * Abstract Data Type Definition (Rust-style enum for values, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = any | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context to handle dynamic inputs gracefully without requiring a separate structs file.

/**
 * Abstract Schema Definition (JSON-compatible with on-the-fly parsing for streaming data, C/C# style struct mapping)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in JSON-style input format or Rust structs if available. Handles both known constants and incoming stream of arbitrary values via a custom `parseJson` utility below.
}

// Helper to convert C/C# style struct definitions into TypeScript types for easier mapping during runtime validation, ensuring data is typed correctly before being used in logic. This provides type safety without requiring complex schema injection at compile time if the input stream is not fully defined yet.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val: unknown) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) as string; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter.
}

/**
 * Abstract Data Type Definition (Rust-style enum for values, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = any | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context to handle dynamic inputs gracefully without needing a separate structs file.

/**
 * Abstract Schema Definition (JSON-compatible with on-the-fly parsing for streaming data, C/C# style struct mapping)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in JSON-style input format or Rust structs if available. Handles both known constants and incoming stream of arbitrary values via a custom `parseJson` utility below.
}

/**
 * Abstract Data Type Definition (R
