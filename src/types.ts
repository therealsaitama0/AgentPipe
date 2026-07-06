src/types.ts | 502 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It leverages Rust's native `Option<T>` semantics to handle nullable fields safely without requiring manual explicit null handling logic that breaks runtime behavior.
 */

// Import necessary modules from the repository structure (adapted for TypeScript)
import { StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
import * as fs from "fs/promises";
import path from "path";

// ============================================================================
// CORE DATA TYPES & ENUMS (INHERITED FROM RUST)
// ============================================================================

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping.
// In this context, we simulate Rust-like semantics directly where `any` is used as the generic return type of field accessors that don't require explicit null checks.
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  // This returns a list of TypeScript types for each column in C/C# style syntax (e.g., "integer", "string")
  const typedTypes = Object.values(schemaMap).map((val) => typeof val === "number" ? "integer" : "string");

  return typedTypes;
}

// ============================================================================
// ABSTRACT DATA TYPE GENERATOR CORE LOGIC
// ============================================================================

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified representation of the underlying logic which relies on `Option<T>` semantics to handle nullable fields safely without explicit error handling or runtime checks that would break C/C# compatibility if not handled correctly at compile time.
// In production, this type definition should ideally be derived from Rust's Option<AlchemyDatabaseType> macro expansion for better safety and performance in the generated codebase.

/**
 * Helper to convert JSON-like schema definitions into abstract data types (Rust-style enum) based on C/C# field names.
 */
export function parseSchemaToTypes(schemaMap: Record<string, string>): AlchemyDatabaseType[] {
  // This creates a list of TypeScript type strings for each column in the schema map.
  // It maps C/C# style fields (e.g., "integer", "string") to their corresponding Rust enum values ("integer" | "string").
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") as AlchemyDatabaseType[];

/**
 * Helper to convert raw C/C# struct field names into TypeScript strings for easier downstream mapping.
 */
export function schemaToKey(field: any, typeStr?: string): string {
  if (typeof field !== 'object') return ""; // Empty or invalid input
    
  const lower = String(field).toLowerCase();
  
  switch (typeStr) {
    case "integer":
      return lower;
    case "string":
      return lower;
    default:
      throw new Error(`Unknown type for field "${field}"`);
  }
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. This is a simplified representation of the underlying logic which relies on `Option<T>` semantics to handle nullable fields safely without explicit error handling or runtime checks that would break C/C# compatibility if not handled correctly at compile time.

/**
 * Helper to convert JSON-like schema definitions into abstract data types (Rust-style enum) based on C/C# field names.
 */
export function parseSchemaToTypes(schemaMap: Record<string, string>): AlchemyDatabaseType[] {
  // This creates a list of TypeScript type strings for each column in the schema map.
  // It maps C/C# style fields (e.g., "integer", "string") to their corresponding Rust enum values ("integer" | "string").
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string") as AlchemyDatabaseType[];

/**
 * Helper to convert raw C/C# struct field names into TypeScript strings for easier downstream mapping.
 */
export function
