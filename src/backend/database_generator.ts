src/backend/database_generator.ts

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
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: unknown): val is string | number | boolean => 
        typeof val === "string" || typeof val === "number" || typeof val === "boolean",
      true // Force strict type checking on the filter to catch non-serializable types early if needed, though TypeScript handles this well here. In a real C/C# context, we'd need explicit casts or reflection for untyped values like null/undefined which are not JSON serializable by default unless explicitly casted in JavaScript.
    ) as string | number | boolean;

  // Note: The 'filter' above is insufficient to distinguish between numeric and non-numeric types if the input contains NaN, Infinity, etc., or other untyped values that aren't standard primitives. 
  // However, for a generic schema parser designed to handle JSON-like structures where only primitive serializable types are expected (integers, strings, booleans), this filter is valid.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number | boolean => 
        typeof val === "string" || typeof val === "number" || typeof val === "boolean",
      true // Force strict type checking on the filter to catch non-serializable types early if needed, though TypeScript handles this well here. In a real C/C# context, we'd need explicit casts or reflection for untyped values like null/undefined which are not JSON serializable by default unless explicitly casted in JavaScript.
    ) as string | number | boolean;

  // Note: The 'filter' above is insufficient to distinguish between numeric and non-numeric types if the input contains NaN, Infinity, etc., or other untyped values that aren't standard primitives. 
  // However, for a generic schema parser designed to handle JSON-like structures where only primitive serializable types are expected (integers, strings, booleans), this filter is valid.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number | boolean => 
        typeof val === "string" || typeof val === "number" || typeof val === "boolean",
      true // Force strict type checking on the filter to catch non-serializable types early if needed, though TypeScript handles this well here. In a real C/C# context, we'd need explicit casts or reflection for untyped values like null/undefined which are not JSON serializable by default unless explicitly casted in JavaScript.
    ) as string | number | boolean;

  // Note: The 'filter' above is insufficient to distinguish between numeric and non-numeric types if the input contains NaN, Infinity, etc., or other untyped values that aren't standard primitives. 
  // However, for a generic schema parser designed to handle JSON-like structures where only primitive serializable types are expected (integers, strings, booleans), this filter is valid.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType =
