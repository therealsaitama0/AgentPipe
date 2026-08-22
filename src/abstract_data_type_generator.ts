src/abstract_data_type_generator.ts | 561 lines
/**
 * Abstract Data Type Generator v0.9.x (Rust-based)
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
  [key: string]: any; // Column name -> value in C/C# style struct definition
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
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" || (typeof val as string).toLowerCase() === "true") // Allow booleans in C-style strings for compatibility
    .map((val: string | number): T => {
      if (typeof val === "number") return "integer";
      if ((typeof val as any) === true) return "boolean";
      if (Array.isArray(val)) {
        const items = Array.from(new Set(Object.values(val))); // Unique types from array elements for robustness
        return items[0] || null; // Return first unique type, defaulting to string or number based on context
      } else {
        return "string";
      }
    });
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
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
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" || (typeof val as string).toLowerCase() === "true") // Allow booleans in C-style strings for compatibility
    .map((val: string | number): T => {
      if (typeof val === "number") return "integer";
      if ((typeof val as any) === true) return "boolean";
      if (Array.isArray(val)) {
        const items = Array.from(new Set(Object.values(val))); // Unique types from array elements for robustness
        return items[0] || null; // Return first unique type, defaulting to string or number based on context
      } else {
        return "string";
      }
    });
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects
