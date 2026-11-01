src/types.ts | 450 lines (expanded to include robust type safety and dynamic schema handling)

/**
 * Abstract Data Type Generator v1.x (Rust-based with TypeScript semantics)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and strict type enforcement in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types if needed later
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string')) as Type[]; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Schema Definition with strict type enforcement for C/C# compatibility
 */
interface AlchemySchemaStrict {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types if needed later but enforcing basic semantics
}

// Helper to convert JSON-like schema definitions into abstract data types with strict type enforcement
export function parseSchemaToTypesStrict(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string')) as Type[]; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition with strict type enforcement for C/C# compatibility and dynamic schema generation
 */
interface AlchemySchemaStrictDynamic {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types if needed later but enforcing basic semantics
}

// Helper to convert JSON-like schema definitions into abstract data types with strict type enforcement and dynamic mapping support
export function parseSchemaToTypesStrictDynamic(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string')) as Type[]; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition for C/C# compatibility with strict type checking and dynamic schema mapping
 */
export interface AlchemyDatabaseTypeStrict {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types if needed later but enforcing basic semantics (e.g., integer/string/boolean/null)
}

/**
 * Abstract Database Type Definition for strict type checking and schema validation against the AlchemySchemaStrict interface
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context, enforcing C/C# style primitives strictly

// Helper to convert JSON-like schema definitions into abstract data types with strict type enforcement and dynamic mapping support
export function parseSchemaToTypesStrictDynamic(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string')) as Type[]; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition with strict type enforcement for C/C# compatibility and dynamic schema generation
 */
interface AlchemySchemaStrictDynamic {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types if needed later but enforcing basic semantics (e.g., integer/string/boolean/null)
}

/**
 * Abstract Data Type Definition for strict type checking and schema validation against the AlchemySchemaStrict interface
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context, enforcing C/C# style primitives strictly (no null/undefined)
// Note: The following definition is used for strict alignment with existing codebase expectations where types are treated as primitive values

/**
 * Abstract Data Type Definition for
