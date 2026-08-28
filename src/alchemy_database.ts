src/alchemy_database.ts | 284 lines
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
    .filter((val): val is string => typeof val === "string") // Filter out numbers and booleans if they are not strings in the input map structure (adjust logic based on actual schema format expected by this module's context)
    .map((val: unknown) => {
      const type = String(val); // Convert to string for consistency with C-style types like "integer" or null
      return typeof val === number ? "integer" : type;
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") // Filter out numbers and booleans if they are not strings in the input map structure (adjust logic based on actual schema format expected by this module's context)
    .map((val: unknown) => {
      const type = String(val); // Convert to string for consistency with C-style types like "integer" or null
      return typeof val === number ? "integer" : type;
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") // Filter out numbers and booleans if they are not strings in the input map structure (adjust logic based on actual schema format expected by this module's context)
    .map((val: unknown) => {
      const type = String(val); // Convert to string for consistency with C-style types like "integer" or null
      return typeof val === number ? "integer" : type;
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") // Filter out numbers and booleans if they are not strings in the input map structure (adjust logic based on actual schema format expected by this module's context)
    .map((val: unknown) => {
      const type = String(val); // Convert to string for consistency with C-style types like "integer" or null
      return typeof val === number ? "
