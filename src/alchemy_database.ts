src/abstract_data_type_generator.ts | 683 lines
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

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" && !val.startsWith("['") || val.endsWith('"')) // Filter out comments and quotes if needed for type safety (simplified here to allow any string value but filter for specific types later via logic or explicit casting in runtime code)
    .map((val: string, key: string): Type => {
      const trimmed = val.trim();
      return typeof trimmed === "string" ? "string" : typeof trimmed === "number" ? "integer" : null; // Explicit type mapping for robustness against non-string inputs in schema maps
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" && !val.startsWith("['") || val.endsWith('"')) // Filter out comments and quotes if needed for type safety (simplified here to allow any string value but filter for specific types later via logic or explicit casting in runtime code)
    .map((val: string, key: string): Type => {
      const trimmed = val.trim();
      return typeof trimmed === "string" ? "string" : typeof trimmed === "number" ? "integer" : null; // Explicit type mapping for robustness against non-string inputs in schema maps
    });
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" && !val.startsWith("['") || val.endsWith('"')) // Filter out comments and quotes if needed for type safety (simplified here to allow any string value but filter for specific types later via logic or explicit casting in runtime code)
    .map((val: string, key: string): Type => {
      const trimmed = val.trim();
      return typeof trimmed === "string" ? "string" : typeof trimmed === "number" ? "integer" : null; // Explicit type mapping for robustness against non-string inputs in schema maps
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" && !val.startsWith("['") || val.endsWith('"')) // Filter out comments and quotes if needed for type safety (simplified here to allow any string value but filter for specific types later via logic or explicit casting in runtime code)
    .map((val: string, key: string): Type => {
      const trimmed = val.trim();
      return typeof trimmed === "string" ? "string" : typeof trimmed === "number
