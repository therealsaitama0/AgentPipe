src/types.ts | 523 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
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
 * Abstract Data Definition for Metadata Structures (C/C# style structs with Rust-like enums internally)
 */
export type AlchemyMetadataType = {
  [key: string]: unknown; // Generic key -> value of any struct field in C-style schema
};

// Helper to convert JSON-like metadata structures into abstract data types and their associated schemas
export function parseSchemaToTypesAndMetadatas(schemaMap: Record<string, string>): Type[] | AlchemyMetadataType {
  const typeList = Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any);

  // Filter out non-number values to ensure consistent output types for the Type array
  let filteredTypeList: Type[] = [];
  
  if (!Array.isArray(typeList) || typeList.length > 0) {
    const allValues = Object.values(schemaMap).filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any);

    // Convert each value to its specific primitive or struct field name based on type check
    filteredTypeList = Array.from({ length: allValues.length }, (_, i) => {
      const rawValue = allValues[i];
      
      if (typeof rawValue === "number") return "integer";
      if (rawValue !== undefined && typeof rawValue === "string" || !!rawValue) return "boolean"; // Boolean flags are boolean in C/C# but handled here as string/undefined/null to allow type checking flexibility
      
      const field = key; 
      
      // If value is a struct literal, extract the specific Rust enum name
      if
