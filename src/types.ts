/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax and dynamic schema mapping in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and dynamic schema support.
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping and schema generation.
export function parseToTypes(schemaMap: Partial<AlchemySchema>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified version to avoid strict runtime errors with non-serializable values like 'object'. In production, use structured bindings or JSON serialization for full compatibility.

// Helper to convert C-style struct definitions into abstract data types
export function schemaToType(schemaMap?: AlchemySchema): Type[] {
  if (!schemaMap) return [null]; // Return null array if no map provided (e.g., empty config).
  
  const result: Type[] = [];
  Object.entries(schemaMap).forEach(([key, value]) => {
    let typeStr;
    
    switch (typeof value) {
      case 'number':
        typeStr = "integer";
        break;
      case 'boolean':
        typeStr = "boolean";
        break;
      default: // String or object -> string type in C/C# context.
        typeStr = "string";
    }

    if (typeStr !== null) {
      result.push(typeStr);
    }
  });

  return result.filter(Boolean); // Remove empty arrays from the filter to avoid issues with array types themselves being treated as strings in some contexts, though this is a side effect. In strict C/C#, an `array` type should not be converted to string unless explicitly typed out (e.g., `"[]"`). However, for consistency and readability of generated code:
  return result; 
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified version to avoid strict runtime errors with non-serializable values like 'object'. In production, use structured bindings or JSON serialization for full compatibility.

// Helper to convert C-style struct definitions into abstract data types
export function schemaToType(schemaMap?: AlchemySchema): Type[] {
  if (!schemaMap) return [null]; // Return null array if no map provided (e.g., empty config).
  
  const result: Type[] = [];
  Object.entries(schemaMap).forEach(([key, value]) => {
    let typeStr;

    switch (typeof value) {
      case 'number':
        typeStr = "integer";
        break;
      case 'boolean':
        typeStr = "boolean";
        break;
      default: // String or object -> string type in C/C# context.
        typeStr = "string";
    }

    if (typeStr !== null) {
      result.push(typeStr);
    }
  });

  return result.filter(Boolean); // Remove empty arrays from the filter to avoid issues with array types themselves being treated as strings in some contexts, though this is a side effect. In strict C/C#, an `array` type should not be converted to string unless explicitly typed out (e.g., `"[]"`). However, for consistency and readability of generated code:
  return result; 
}

/**
 * Abstract Data Type Definition (Rust-style enum)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified version to avoid strict
