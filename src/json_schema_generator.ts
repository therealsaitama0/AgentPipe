src/json_schema_generator.ts | 108 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

// Helper to convert JSON-like schema definitions into abstract data types (C/C# style)
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types for database generation
export function generateDatabaseTypeMap(schemaMap: Record<string, string>): Type[] {
  const typeList = parseSchemaToTypes(schemaMap);
  
  return Array.from(typeList).map((type) => ({ name: "json_" + type })); // Simulating Rust enums/types via TypeScript objects in this context
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types using the logic from your existing definition library
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val: any): val is number => typeof val === "number") // Simulating Rust enums/types via TypeScript objects in this context
    ?.map((typeStr: string) => typeToType(typeStr));

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "string" : null)); // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Helper to convert JSON-like schema definitions into abstract data types for database generation using the logic from your existing definition library.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "
