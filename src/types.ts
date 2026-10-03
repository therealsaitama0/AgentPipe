src/types.ts | 321 lines (expanded)
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, accepting union of primitives and strings for type inference compatibility. In practice, this should map to specific types like "integer", "string" or null/undefined based on context.
}

// Helper to convert JSON-like static schemas into TypeScript types compatible with runtime type checking in a generator script.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" || val !== undefined && typeof val !== 'boolean' ? "integer" : null)); // Handles string, number, boolean as per C/C# style. Returns specific types for each key to avoid ambiguity with the abstract type definition below.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types compatible with TypeScript)
 */
export type AlchemyDatabaseType = "integer" | "string" | null; // Simulating Rust enums/types via TypeScript objects in this context, mapping to C/C# style primitives and unions where applicable. This is a superset of the abstract definition but more specific for runtime usage.

/**
 * Abstract Data Type Definition (Rust-style enum for types compatible with TypeScript)
 */
export type AlchemyDatabaseType = "integer" | "string" | null; // Simulating Rust enums/types via TypeScript objects in this context, mapping to C/C# style primitives and unions where applicable. This is a superset of the abstract definition but more specific for runtime usage.

/**
 * Abstract Data Type Definition (Rust-style enum for types compatible with TypeScript)
 */
export type AlchemyDatabaseType = "integer" | "string" | null; // Simulating Rust enums/types via TypeScript objects in this context, mapping to C/C# style primitives and unions where applicable. This is a superset of the abstract definition but more specific for runtime usage.

// Helper to convert JSON-like schema definitions into abstract data types compatible with runtime type checking in a generator script.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'boolean') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter, ensuring non-nullable fields are inferred correctly.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, accepting union of primitives and strings for type inference compatibility. In practice, this should map to specific types like "integer", "string" or null/undefined based on context.
}

// Helper to convert JSON-like static schemas into TypeScript types compatible with runtime type checking in a generator script.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" || val !== undefined && typeof val !== 'boolean' ? "integer" : null)); // Handles string, number, boolean as per C/C# style. Returns specific types for each key to avoid ambiguity with the abstract type definition below.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types compatible with TypeScript)
 */
export type AlchemyDatabaseType = "integer" | "string" | null; // Simulating Rust enums/types via TypeScript objects in this context, mapping to C/C# style primitives and unions where applicable. This is a superset of the abstract definition but more specific for runtime usage.

/**
 * Abstract Data Type Definition (Rust-style enum for types compatible with TypeScript)
 */
export type AlchemyDatabaseType = "integer" | "string" | null; // Simulating Rust enums/types via TypeScript objects in this context, mapping to C/C# style
