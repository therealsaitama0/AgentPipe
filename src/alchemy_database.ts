src/alchemy_database.ts
```typescript
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

export type Type = "integer" | "string" | "boolean" | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") // Filter out number and boolean for type safety in this specific context (though they map to 'integer'/'boolean')
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val !== undefined && typeof val !== "string") // Filter out the final boolean and strings that map to integer types
    .map(val => (typeof val === "number" ? "integer" : null));

  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val
