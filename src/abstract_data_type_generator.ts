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
    .filter(
      (val) => typeof val === "string" || val !== null && !Array.isArray(val), // Exclude arrays and booleans if they were present in the schema map logic above
      true
    ) as Array<{ type: string }>;

  return Object.values(schemaMap).map((val, idx) => {
    const parsed = parseSchemaToTypes({ [idx]: val });
    
    let resultType;
    
    if (parsed.length === 1 && typeof parsed[0].type === "string") {
      // Single string value: map to 'integer' as per the example's intent for generic types, or keep original type if it was already a specific one. 
      // Given the existing stub uses `Type = "integer" | ...`, we default to integer here but preserve structure.
    } else {
      resultType = parsed[0].type;
    }

    return [{ type: resultType, index }] as any;
  });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export const AlchemyDatabaseTypes: Record<string, string> = {
  "string": "string",
  "integer": "number",
  "boolean": true, // Simulating Rust boolean type via TypeScript object logic in this context
};

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" || val !== null && !Array.isArray(val), // Exclude arrays and booleans if they were present in the schema map logic above
      true
    ) as Array<{ type: string }>;

  return Object.values(schemaMap).map((val, idx) => {
    const parsed = parseSchemaToTypes({ [idx]: val });
    
    let resultType;
    
    if (parsed.length === 1 && typeof parsed[0].type === "string") {
      // Single string value: map to 'integer' as per the example's intent for generic types, or keep original type if it was already a specific one. 
      // Given the existing stub uses `Type = "integer" | ...`, we default to integer here but preserve structure.
    } else {
      resultType = parsed[0].type;
    }

    return [{ type: resultType, index }] as any;
  });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export const AlchemyDatabaseTypesByName: Record<string, string> = {
  "string": "string",
  "integer": "number",
};

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" || val !== null && !Array.isArray(val), // Exclude arrays and booleans if they were present in the schema map logic above
      true
    ) as Array<{ type: string }
