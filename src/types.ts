/**
 * Abstract Data Type Generator v0.5.x (Rust-based) - Enhanced Edition
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
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types at runtime
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is any => typeof val !== 'undefined' && (typeof val === "string" || typeof val === "number" || typeof val === "boolean")) // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
    .map(val => {
      if (typeof val === "string") return "string";
      if (typeof val === "number") return "integer";
      if (typeof val === "boolean") return null;
      // Default fallback for any other type to ensure compatibility with the original logic flow
      return typeof val !== 'undefined' ? undefined : null;
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context
// Note: This is now a union of primitives. In practice with dynamic schemas, you would map these to specific runtime values or use generics if true types are needed for compilation-time checking only (not execution).

/**
 * Abstract Data Type Definition - Refined Implementation
 */
export type AlchemyDatabaseType = string | number | boolean; // Explicitly removed 'null' and 'undefined' as they don't fit the core schema, but kept in mind for validation logic. In a real dynamic system, these would be mapped to specific runtime values or used as placeholders when types are unknown (e.g., `unknown_type`).
// For this version of the generator, we treat them as generic primitives that can be resolved by context if needed, but structurally they align with C-style unions/types.

/**
 * Abstract Schema Definition - Enhanced Version for Dynamic Mapping
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types at runtime (e.g., strings can be objects, numbers are scalars)
}

// Helper to convert JSON-like schema definitions into abstract data types for the database generator.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap).map((val): val is string | number => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number')) as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Note: This returns a generic array of primitive types, which can be mapped dynamically by the application context if true type assertions are required for specific runtime behaviors not covered here (e.g., `string[]` vs `number[]`).
}

/**
 * Abstract Data Type Definition - Refined Implementation with Dynamic Mapping Support
 */
export type AlchemyDatabaseType = string | number; // Simplified from the original to focus on primitive types that are common in C/C# style databases. 'boolean' and 'null' were removed as they don't fit well into a standard schema, but kept for completeness if specific boolean flags or null values need separate handling. In practice with dynamic schemas, you would map these primitives to runtime values using the `schemaToType` function which handles type conversion based on actual data types found in the JSON-like structure at execution time.
// For this version of the generator, we treat them as generic primitives that can be resolved by context if needed for validation logic (e.g., checking schema integrity).

/**
 * Abstract Schema Definition - Enhanced Version with Dynamic Mapping Support
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types at runtime. This allows the generator to handle schemas where values might be objects (strings) or scalars (numbers).
}

// Helper to convert JSON-like
