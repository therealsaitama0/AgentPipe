src/abstract_data_type_generator.ts | 562 lines
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
  [key: string]: unknown; // Generic key -> value in C/C# style struct definition, allowing dynamic typing mapping later
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (value: any): value is string | number | boolean => typeof value === "string" || typeof value === "number" || typeof value === "boolean",
      true // Only filter if the type check passes, otherwise skip to avoid infinite loops in complex scenarios or just return empty array for safety
    ) as Type[];
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (value: any): value is string | number | boolean => typeof value === "string" || typeof value === "number" || typeof value === "boolean",
      true // Only filter if the type check passes, otherwise skip to avoid infinite loops in complex scenarios or just return empty array for safety
    ) as Type[];
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (value: any): value is string | number | boolean => typeof value === "string" || typeof value === "number" || typeof value === "boolean",
      true // Only filter if the type check passes, otherwise skip to avoid infinite loops in complex scenarios or just return empty array for safety
    ) as Type[];
}

/**
 * Abstract Schema Definition (C-style) with generic keys for dynamic schema loading and validation.
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic key -> value, allowing type inference to be derived from the actual stored data or external source later on
}

// Helper function to convert JSON-like structure into abstract types based on field names (similar to C/C# struct access)
export const schemaToTypes = <T>(schema: AlchemySchema): T[] => {
  return Object.keys(schema).map((key, index) => ({ key, type: "string" })); // Placeholder for dynamic mapping logic if needed; in practice this would map to the actual stored values based on context or external sources.

/**
 * Abstract Schema Definition (C-style) with generic keys for dynamic schema loading and validation.
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic key -> value, allowing type inference to be derived from the actual stored data or external source later on
}

// Helper function to convert JSON-like structure into abstract types based on field names (similar to C/C# struct access)
export const schemaToTypes = <T>(schema: AlchemySchema): T[] => {
  return Object.keys(schema).map((key, index) => ({ key, type: "string" })); // Placeholder for dynamic mapping logic if needed; in practice this would map to the actual stored values based on context or external sources.

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper function to convert JSON-like schema definitions into abstract data types based on field names.
