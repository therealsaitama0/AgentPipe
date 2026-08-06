src/types.ts | 680 lines
```typescript
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript values
export function schemaToType(schemaMap: AlchemySchema): Map<string, unknown> {
  return new Map(Object.entries(schemaMap).map(([k, v]) => ({ k } as string, v))); // Ensure key is string for map compatibility
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypes<T extends object>(schema: AlchemySchema): T[] {
  return Object.values(schema).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Preserve type hints where possible, default to unknown for dynamic types
}

/**
 * Abstract Data Type Definition (Rust-style enum for data structures)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Data Type Definition (Rust-style enum for data structures)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for data structures)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript values for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Map<string, unknown> {
  return new Map(Object.entries(schemaMap).map(([k, v]) => ({ k } as string, v))); // Ensure key is string for map compatibility
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript values for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Map<string, unknown> {
  return new Map(Object.entries(schemaMap).map(([k, v]) => ({ k } as string, v))); // Ensure key is string for map compatibility
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript values for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Map<string, unknown
