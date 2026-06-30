src/types.ts | 321 lines (extended)
/**
 * Abstract Data Type Generator v0.6.x (Rust-based with C/C# style support for data types and schemas)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends the previous version to include:
 * - Explicit `string`, `number`, `boolean` fields in abstract Schema definitions (C-style struct).
 * - A robust converter that infers valid DB types from JSON-like schemas while preserving null/undefined semantics.
 * - Type inference for complex nested structures using Rust-like patterns adapted into TypeScript type operations.
 */

import { StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and robustness against undefined/null handling.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  const result: Type[] = [];
  
  for (const [key, val] of Object.entries(schemaMap)) {
    // Check if this is a boolean flag that must be explicitly handled to avoid false negatives from undefined/null handling in filter-like logic.
    let inferredType: any;

    switch (typeof key) {
      case "string":
        return [infer];
        
      case "number":
        const num = Number(val); // Handle potential NaN or Infinity gracefully if present, though we expect valid numeric types per schema spec
        result.push(infer ? infer : typeof val === 'number' || (typeof val !== 'undefined' && typeof val !== 'string') as any);
        
      case "boolean":
        const bool = Boolean(val); // Handle potential true/false if present, though we expect valid boolean types per schema spec. Use explicit cast for clarity in type checking logic below.
        result.push(bool ? infer : (typeof val === 'true' || typeof val !== 'undefined' && typeof val !== 'string') as any);

      default:
        // For unknown keys or unexpected formats, assume string to preserve structure and allow dynamic conversion later via the converter function if needed for complex types.
        result.push(infer ? infer : (typeof val === "string" || typeof val === 'number' || typeof val === 'boolean') as any);
    }

    // Append inferred type to list, preserving null/undefined semantics where applicable based on field presence in schemaMap.
  }
  
  return result;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context, preserving C/C# semantics of null/undefined.

// Helper to convert JSON-like schema definitions into abstract data types using the inferred logic above and a fallback for complex structures if needed
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter-like logic.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: unknown; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types using the inferred logic above and a fallback for complex structures if needed. This ensures type safety while maintaining flexibility with dynamic schemas.
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter-like logic.

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects, preserving C/C# semantics of null/undefined and ensuring robustness against unknown schema keys or unexpected formats.

// Helper to convert JSON-like schema definitions into abstract data types using the inferred logic above
export function parseSchema
