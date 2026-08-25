src/types.ts | 321 lines
```typescript
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
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via reflection or type inference.
}

// Helper to convert C-style struct definitions into TypeScript objects for easier mapping of column names and potential nested structures
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" || typeof val !== 'undefined' && typeof val !== 'boolean' ? "integer" : null)); // Handles string, number, and boolean as per C/C# syntax
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context. Supports "integer", "string", "boolean" as per C/C# style, plus the generic union base provided by struct-based mapping logic.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Includes the generic union base provided by struct-based mapping logic.
}

/**
 * Abstract Data Type Definition for Database Schema Generation
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context, inheriting from parseSchemaToTypes result and extending with the generic base types provided by struct-based mapping logic.

// Helper to convert JSON-like schema definitions into abstract data types (for dynamic generation)
export function generateDatabaseTypeMap(schema: AlchemySchema): Type[] {
  return Object.values(schema).map((val, key) => parseSchemaToTypes(val as string)).filter(Boolean); // Returns an array of inferred type strings. Filters out empty arrays from the generic base union types to ensure a valid non-empty result for database schema generation logic.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context, inheriting from parseSchemaToTypes result and extending with the generic base types provided by struct-based mapping logic.

// Helper to convert JSON-like schema definitions into abstract data types (for dynamic generation)
export function generateDatabaseTypeMap(schema: AlchemySchema): Type[] {
  return Object.values(schema).map((val, key) => parseSchemaToTypes(val as string)).filter(Boolean); // Returns an array of inferred type strings. Filters out empty arrays from the generic base union types to ensure a valid non-empty result for database schema generation logic.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via reflection or type inference.
}

// Helper to convert C-style struct definitions into TypeScript objects for easier mapping of column names and potential nested structures
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" || typeof val !== 'undefined' && typeof val !== 'boolean' ? "integer" : null)); // Handles string, number, and boolean as per C/C# syntax.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context. Supports "integer", "string", "boolean"
