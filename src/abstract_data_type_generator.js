src/abstract_data_type_generator.ts | 321 lines
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
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style) - Extended for dynamic types
 */
interface AlchemySchemaExtended {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended(schemaMap: AlchemySchemaExtended): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style - Extended for dynamic types)
 */
interface AlchemySchemaExtended2 {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended2(schemaMap: AlchemySchemaExtended2): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style - Extended for dynamic types)
 */
interface AlchemySchemaExtended3 {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended3(schemaMap: AlchemySchemaExtended3): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style - Extended for dynamic types)
 */
interface AlchemySchemaExtended4 {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended4(schemaMap: AlchemySchemaExtended4): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style - Extended for dynamic types)
 */
interface AlchemySchemaExtended5 {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended5(schemaMap: AlchemySchemaExtended5): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style - Extended for dynamic types)
 */
interface AlchemySchemaExtended6 {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing more flexibility
}

// Helper to convert C-style extended struct definitions into TypeScript types for easier mapping
export function schemaToTypeExtended6(schemaMap: AlchemySchemaExtended6): Type[] {
  return Object.values(schemaMap
