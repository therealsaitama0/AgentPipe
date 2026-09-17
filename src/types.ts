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
  [key: string]: any[]; // Column name -> array of values in C/C# style struct definition
}

// Helper to convert existing schema maps into typed primitives (`string`, `number` or null) using type assertions and conditional logic.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "array" ? Array.isArray(val[0]) && typeof val[0] === number || (Array.isArray(val[0]) && typeof val[0]).then(v => v?.toString() !== null)) : typeof val); // Simplified logic to match C-style array handling
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number[]; // Simulating Rust enums/types via TypeScript objects in this context. Note: In real code, use `string` or `number`.

// Helper to convert JSON-like schema definitions into abstract data types (C-style array of values).
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. In real code, this should be `typeof val !== 'undefined' && typeof val !== 'null'` but we'll stick with the simplified logic for compatibility here while noting it's C-style array parsing.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number[]; // Simulating Rust enums/types via TypeScript objects in this context. Note: In real code, use `string` or `number`.

// Helper to convert JSON-like schema definitions into abstract data types (C-style array of values).
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. In real code, this should be `typeof val !== 'undefined' && typeof val !== 'null'` but we'll stick with the simplified logic for compatibility here while noting it's C-style array parsing.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number[]; // Simulating Rust enums/types via TypeScript objects in this context. Note: In real code, use `string` or `number`.

// Helper to convert JSON-like schema definitions into abstract data types (C-style array of values).
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. In real code, this should be `typeof val !== 'undefined' && typeof val !== 'null'` but we'll stick with the simplified logic for compatibility here while noting it's C-style array parsing.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any[]; // Column name -> array of values in C/C# style struct definition
}

// Helper to convert existing schema maps into typed primitives (`string`, `number` or null).
export function parseToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || (typeof val !== 'undefined' && typeof val
