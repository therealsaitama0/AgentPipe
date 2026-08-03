src/abstract_data_type_generator.ts | 156 lines (refactored) /**
 * Abstract Data Type Generator v0.3.x (Rust-based with C/C# type compatibility)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It supports parsing JSON-like structures directly into abstract schemas without external libraries.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and direct structure definition

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic key -> value of any type in schema mapping
}

// Helper to convert JSON-like schema definitions into abstract data types using a recursive function that mimics C/C# struct parsing logic.
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  const result = [];
  
  // Iterate over the keys of the map and process each value (schema item)
  for (const [key, val] of Object.entries(schemaMap)) {
    if (!val || typeof val !== 'object') continue;

    // Recursively parse nested schemas or primitives to build a flat list of types. 
    // This mimics how C/C# structs define fields and their associated type information.
    const parsedTypes = parseSchemaToType(val);
    
    result.push(...parsedTypes);
  }

  return result;
}

/**
 * Abstract Schema Definition (C-style) - Returns a list of compatible types for the given schema map.
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  const parsed = parseSchemaToTypes(schemaMap);
  
  // Filter out null/undefined values to ensure only valid data types are returned (C-style strictness)
  return Object.values(parsed).filter(Boolean as any).map((typeStr: string, index: number) => ({ ...typeStr }));
}

/**
 * Abstract Data Type Definition - A container for type definitions in the schema. 
 * Mimics C/C# struct field mapping where each key maps to a specific data type (string | integer | boolean).
 */
export interface AlchemyDatabaseType {
  [key: string]: unknown; // Maps to any concrete value, but we return compatible types here for consistency with JSON parsing logic.
}

// Helper function that converts standard JSON schema definitions directly into the required AbstractDataType structure by mapping keys to their expected C/C# type names (string | integer | boolean).
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  const result = [];
  
  // Iterate over the keys of the map and process each value (schema item)
  for (const [key, val] of Object.entries(schemaMap)) {
    if (!val || typeof val !== 'object') continue;

    // Recursively parse nested schemas or primitives to build a flat list of types. 
    const parsedTypes = parseSchemaToType(val);
    
    result.push(...parsedTypes);
  }

  return result;
}

/**
 * Abstract Data Type Definition - A container for type definitions in the schema. 
 * Mimics C/C# struct field mapping where each key maps to a specific data type (string | integer | boolean).
 */
export interface AlchemyDatabaseType {
  [key: string]: unknown; // Maps to any concrete value, but we return compatible types here for consistency with JSON parsing logic.
}

/**
 * Helper function that converts standard JSON schema definitions directly into the required AbstractDataType structure by mapping keys to their expected C/C# type names (string | integer | boolean).
 */
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  const result = [];
  
  // Iterate over the keys of the map and process each value (schema item)
  for (const [key, val] of Object.entries(schemaMap)) {
    if (!val || typeof val !== 'object') continue;

    // Recursively parse nested schemas or primitives to build a flat list of types. 
    const parsedTypes = parseSchemaToType(val);
    
    result.push(...parsedTypes);
  }

  return result;
}
