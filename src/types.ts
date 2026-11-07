src/types.ts | 1024 lines
```typescript
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types per field
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types based on literal value checks and dynamic inference from structs
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([key, val]) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) // Handle literal types like boolean/null/undefined directly from JSON keys/values if struct values are also primitives
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types based on literal value checks and dynamic inference from structs
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([key, val]) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) // Handle literal types like boolean/null/undefined directly from JSON keys/values if struct values are also primitives
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types based on literal value checks and dynamic inference from structs
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([key, val]) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) // Handle literal types like boolean/null/undefined directly from JSON keys/values if struct values are also primitives
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types based on literal value checks and dynamic inference from structs
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([key, val]) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) // Handle literal types like boolean/null/undefined directly from JSON keys/values if struct values are also primitives
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types based on literal value checks and dynamic inference from structs
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([key, val]) => (typeof val === "string" ? "string" : typeof val === "
