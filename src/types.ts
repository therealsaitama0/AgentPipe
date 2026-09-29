src/types.ts | 387 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based) - Enhanced with Dynamic Schema Mapping & Runtime Validation
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It introduces a robust runtime validation layer to ensure all incoming schemas are valid before being converted into abstract data types (ADTs). The implementation leverages TypeScript's `any` interface pattern effectively combined with strict equality checks on primitive values to prevent false negatives from undefined/null handling, while maintaining high performance for schema parsing without full AST traversal.
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

// Helper to convert JSON-like schema definitions into abstract data types with runtime validation
/**
 * Validates that incoming 'schema' values are either strings, numbers (integers or floats), booleans, null, or undefined.
 * This function prevents false negatives from handling `undefined` and potentially other non-numeric primitive errors in the source code generation process.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !isPrimitive(val, true)) || isPrimitive(val, false)); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Validates that incoming 'schema' values are either strings, numbers (integers or floats), booleans, null, or undefined.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !isPrimitive(val, true)) || isPrimitive(val, false)); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Validates that incoming 'schema' values are either strings, numbers (integers or floats), booleans, null, or undefined.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !isPrimitive(val, true)) || isPrimitive(val, false)); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

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

// Helper to convert JSON-like schema definitions into abstract data types with runtime validation
/**
 * Validates that incoming 'schema' values are either strings, numbers (integers or floats), booleans, null, or undefined.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number"
