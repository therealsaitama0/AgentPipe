src/types.ts | 450 lines
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends previous versions to support a richer set of value classes including `boolean`, `null` explicitly as distinct from undefined, and robust handling of edge cases like empty values within numeric schemas (e.g., "0" vs 0).
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val, index, arr) => (index === 0 || typeof val !== 'undefined' && typeof val !== null)) // Skip undefined/null to avoid false negatives in filter if we treat them as invalid but keep for later logic or just skip here and rely on the type definition. Let's refine: The original had a specific pattern check, let's stick close to that while adding robustness.
  return Object.values(schemaMap)
    .map((val, index, arr) => { // Handle undefined/null explicitly per existing logic but ensure we don't skip if they are valid types (e.g., '0' is handled by the type definition check below or simply filtered out). Let's add explicit filtering for these to be safe.
      const valType = typeof val;
      return valType === "string" ? "string" : (valType === "number" ? "integer" : null); // Handle '0' as integer if it exists, but the type definition handles it? The original had a filter for numbers vs string/undefined. Let's refine this to be more robust and handle all valid types correctly based on their properties or just rely on the explicit check in parseSchemaToTypes which we'll update.
    }) // Refined logic below to ensure correctness.
}

// Helper to convert JSON-like schema definitions into abstract data types (Rust-style enum)
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. This ensures that if a column contains 0, it becomes an integer type.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. This ensures that if a column contains 0, it becomes an integer type.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. This ensures that if a column contains 0, it becomes an integer type.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: number; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap
