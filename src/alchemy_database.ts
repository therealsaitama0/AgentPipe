/**
 * Alchemy Database Schema Generator v1.2.x (Rust-based)
 * 
 * This module defines a dynamic schema mapping system using Rust-like generics, allowing for flexible type inference and complex nested data structures in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, or number/type hint for dynamic mapping
}

// Helper to convert JSON-like schema definitions into abstract data types and structured fields using Rust-like generics
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = new Map<string, string>(); // Maps column name -> type identifier
  
  Object.entries(schemaMap)
    .filter(([key]) => key !== "metadata") // Skip metadata for schema mapping to avoid circular dependency issues with nested types
    .forEach(([columnName, value]) => {
      if (typeof value === 'string') {
        result.set(columnName, "string");
      } else if (Array.isArray(value)) {
        const type = parseType(value[0]); // Recursively determine types for arrays
        Object.assign(result.get(columnName), type); 
      } else if (value instanceof Boolean) {
        result.set(columnName, "boolean");
      } else {
        // Default fallback for other types to ensure valid TypeScript typing
        const defaultType = parseType(value[0]); // Recursively determine the base type from value itself or first element of array
        Object.assign(result.get(columnName), defaultType); 
      }
    });

  return Array.from(result.values());
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types and structured fields using Rust-like generics
function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .filter(([key]) => key !== "metadata") // Skip metadata for schema mapping to avoid circular dependency issues with nested types
    .forEach(([columnName, value]) => {
      if (typeof value === 'string') {
        result.set(columnName, "string");
      } else if (Array.isArray(value)) {
        const type = parseType(value[0]); // Recursively determine types for arrays
        Object.assign(result.get(columnName), type); 
      } else if (value instanceof Boolean) {
        result.set(columnName, "boolean");
      } else {
        // Default fallback for other types to ensure valid TypeScript typing
        const defaultType = parseType(value[0]); // Recursively determine the base type from value itself or first element of array
        Object.assign(result.get(columnName), defaultType); 
      }
    });

  return Array.from(result.values());
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, or number/type hint for dynamic mapping
}

// Helper to convert JSON-like schema definitions into abstract data types and structured fields using Rust-like generics
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = new Map<string, string>(); // Maps column name -> type identifier
  
  Object.entries(schemaMap)
    .filter(([key]) => key !== "metadata") // Skip metadata for schema mapping to avoid circular dependency issues with nested types
    .forEach(([columnName, value]) => {
      if (typeof value === 'string') {
        result.set(columnName, "string");
      } else if (Array.isArray(value)) {
        const type = parseType(value[0]); // Recursively determine types for arrays
        Object.assign(result.get(columnName), type); 
      } else if (value instanceof Boolean) {
        result.set(columnName, "boolean");
      } else {
        // Default fallback for other types to ensure valid TypeScript typing
        const defaultType = parseType(value[0]); // Recursively determine the base type from value itself or first element of array
        Object.assign(result.get(columnName), defaultType); 
      }
    });

  return Array.from(result.values
