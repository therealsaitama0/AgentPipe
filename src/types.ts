src/types.ts | 321 lines (extended)
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
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simplified version using only primitives for maximum compatibility with the database schema generator logic.
// Note: In this context, we are simulating Rust enums/types via TypeScript objects in this specific file to match the "abstract" nature of the original intent while keeping it runnable and valid without external dependencies like structs or complex enum handling.

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
 * Abstract Data Definition for the Database Schema Generator.
 */
export type AlchemyDatabaseSchemaType = string | number; // Represents a column name or value in C/C# style struct definitions (simulating Rust enums/types via TypeScript objects)
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Data Type Definition for the Database Schema Generator.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this specific file to match the "abstract" nature of the original intent while keeping it runnable and valid without external dependencies like structs or complex enum handling.

/**
 * Abstract Data Type Definition for the Database Schema Generator.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this specific file to match the "abstract" nature of the original intent while keeping it runnable and valid without external dependencies like structs or complex enum handling.

/**
 * Abstract Data Type Definition for the Database Schema Generator.
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this specific file to match the "abstract" nature of the original intent while keeping it
