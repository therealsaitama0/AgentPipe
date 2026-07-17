src/types.ts | 432 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
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
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaWithNulls {
  [key: string]: string | number; // Add nullable support for strings and numbers in C/C# style structs
}

// Helper to convert JSON-like schema definitions into abstract data types with null handling
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as (typeof inferType<infer T> extends infer U ? T : never)[0] // Simplified type inference to extract valid types from JSON keys; in real code, use proper generic mapping
    .filter((val): val is string | number => typeof val !== 'undefined' && typeof val !== null) as any;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaWithNulls {
  [key: string]: string | number; // Add nullable support for strings and numbers in C/C# style structs
}

// Helper to convert JSON-like schema definitions into abstract data types with null handling
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as (typeof inferType<infer T> extends infer U ? T : never)[0] // Simplified type inference to extract valid types from JSON keys; in real code, use proper generic mapping
    .filter((val): val is string | number => typeof val !== 'undefined' && typeof val !== null) as any;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaWithNulls {
  [key: string]: string | number; // Add nullable support for strings and numbers in C/C# style structs
}

// Helper to convert JSON-like schema definitions into abstract data types with null handling
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as (typeof inferType<infer T> extends infer U ? T : never)[0] // Simplified type inference to extract valid types from JSON keys; in real code, use proper generic mapping
    .filter((val): val is string | number => typeof val !== 'undefined' && typeof val !== null) as any;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaWithNulls {
  [key: string]: string | number; // Add nullable support for strings and numbers in C/C# style structs
}

// Helper to convert JSON-like schema definitions into abstract data types with null handling
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number") as (typeof inferType<infer T> extends infer U ? T : never)[0] // Simplified type inference to extract valid types from JSON keys; in real code, use proper generic mapping
    .filter((val): val is string | number => typeof val !== 'undefined' && typeof val !== null) as any;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchemaWithNulls {
  [key: string]:
