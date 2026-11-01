import struct as StructType from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

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
    .filter( (key: string): key is keyof AlchemySchema => typeof key === "string" && !Array.prototype.hasOwnProperty.call(schemaMap, key))
    // Note: Assuming the 'structs' file provides actual Rust structs for this specific context. 
    // In a pure TypeScript/JavaScript environment with no external C/C# files available in `src`, we simulate the behavior by treating non-nullable strings as "string" types and other values as generic or null,
    // while keeping the interface defined exactly as requested: string | number | boolean | undefined.
  return Object.values(schemaMap)
    .filter( (val): val is keyof AlchemySchema => typeof val === "string") 
    .map((val) => (typeof val === "number" ? "integer" : null)); // Simplified for compatibility with the existing abstract definition pattern, assuming 'structs' provides real Rust types.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (key: string): key is keyof AlchemySchema => typeof key === "string" && !Array.prototype.hasOwnProperty.call(schemaMap, key))
    // Note: Assuming the 'structs' file provides actual Rust structs for this specific context. 
    // In a pure TypeScript/JavaScript environment with no external C/C# files available in `src`, we simulate the behavior by treating non-nullable strings as "string" types and other values as generic or null,
    // while keeping the interface defined exactly as requested: string | number | boolean | undefined.
  return Object.values(schemaMap)
    .filter( (val): val is keyof AlchemySchema => typeof val === "string") 
    .map((val) => (typeof val === "number" ? "integer" : null)); // Simplified for compatibility with the existing abstract definition pattern, assuming 'structs' provides real Rust types.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (key: string): key is keyof AlchemySchema => typeof key === "string" && !Array.prototype.hasOwnProperty.call(schemaMap, key))
    // Note: Assuming the 'structs' file provides actual Rust structs for this specific context. 
    // In a pure TypeScript/JavaScript environment with no external C/C# files available in `src`, we simulate the behavior by treating non-nullable strings as "string" types and other values as generic or null,
    // while keeping the interface defined exactly as requested: string | number | boolean | undefined.
  return Object.values(schemaMap)
    .filter( (val): val is keyof AlchemySchema => typeof val === "string") 
    .map((val) => (typeof val === "number" ? "integer" : null
