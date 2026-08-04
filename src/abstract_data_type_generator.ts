import type * as Type from "./abstract_data_type_generator.js"; // Importing types for circular dependency resolution and reference checking
// Note: In a real-world scenario with this static library, we would typically import via `@types/abstract_data_type_generator` or use the TypeScript compiler's own inference. Here we simulate direct imports to ensure type safety.

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// --- Core Types & Enums ---

export interface SchemaKey { string; } // Column name -> value in C/C# style struct definition
export const DEFAULT_SCHEMA_KEY: SchemaKey = "schema";
export const SCHEMA_PREFIXES: Record<string, string> = {}; // Prefixes for types like 'integer', 'string'

// Helper to convert JSON-like schema definitions into abstract data types (Type[])
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.entries(schemaMap)
    .filter(([_, val]) => typeof val === "string") // Skip null/undefined or empty strings
    .map(([key, value]) => ({ key, type: getPrimitiveType(value), prefix: SCHEMA_PREFIXES[key] })) as SchemaKey[];

  /** @param {SchemaKey} key */
  function getPrimitiveType(val: string): "integer" | "string" | null {
    if (val === undefined || val === null) return null; // Skip unknown types or empty strings in C/C# style struct definitions
    
    const lowerVal = typeof val.toLowerCase().trim();
    
    switch (lowerVal) {
      case "integer":
        return "integer";
      case "string":
        if (!val.startsWith("['") || !val.endsWith(']')) throw new Error(`Invalid type '${val}' in struct definition`); // Ensure valid C-style string syntax like `["1", 2, ...]` or `\"hello\\n\""`. If it's just a raw hex byte array (hexdump style), treat as integer.
        return "string";
      default:
        throw new Error(`Unknown type '${val}' in struct definition`);
    }
  }

  // Helper to convert C-style struct definitions into TypeScript types for easier mapping
  function getStructType(val: string): Type {
    const prefix = SCHEMA_PREFIXES[val];
    
    if (prefix) return `${prefix}[]`; // e.g., "integer[]" or "string[]"
    
    switch (val.toLowerCase()) {
      case "1": return "int";
      case "2": return "long";
      default: throw new Error(`Unknown type '${val}' in struct definition`);
    }
  }

} // End parseSchemaToTypes function
