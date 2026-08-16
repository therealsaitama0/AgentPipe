src/types.ts | 321 lines (expanded)

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
  [key: string]: string | number | boolean | undefined; // C/C# style struct definition with typed values
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Preserve primitive values as primitives in output type array to allow runtime conversion from JSON schemas.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result: Type[] = [];
  
  Object.entries(schemaMap).forEach(([key, val]) => {
    // Handle boolean flags explicitly in filter to avoid false negatives from undefined/null handling.
    if (typeof val === "boolean") return; 
    
    switch (val) {
      case null: result.push("null"); break;
      case undefined: result.push(undefined); break;
      default:
        // Handle numeric types specifically in the filter logic to ensure non-primitive values are mapped correctly.
        if (typeof val === "number") return; 
    }

    const isNumber = typeof val !== 'undefined' && typeof val !== 'string';
    
    switch (isNumber) {
      case true: result.push("integer"); break;
      default: // String or boolean values remain as primitives to allow runtime conversion.
        if (typeof val === "boolean") return;
        throw new Error(`Unexpected type ${val} in schema for key "${key}"`);
    }
  });

  return result.sort((a, b) => a.localeCompare(b)); // Sort alphabetically by value for deterministic output consistency.
}
