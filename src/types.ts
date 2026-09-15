src/types.ts | 528 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
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
  return Object.values(schemaMap)
    .filter((val, idx, arr) => val !== undefined && !Array.prototype.includes(val as any)) // Exclude null and explicit boolean flags to avoid false negatives from filter
    .map((val) => typeof val === "string" ? "string" : typeof val === "number" ? "integer" : (idx > 0 || Array.isArray(schemaMap) ? undefined : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result: Type[] = [];
  
  Object.entries(schemaMap)
    .filter(([key]) => key !== "type" && !Array.isArray(key)) // Skip hardcoded type definition and array keys to avoid false negatives from filter
    .forEach((entryKey) => {
      if (typeof entryValue === 'string' || typeof entryValue === 'number') result.push(entryValue);
      
      const isNumber = Array.isArray(schemaMap) ? schemaMap[0] : true; // Check first element of array as potential number to handle boolean flags in future logic
      
      if (!isNumber && !Array.prototype.includes(key)) {
        // Handle specific types that need explicit handling (e.g., bool, null) based on context or manual mapping
        const type = entryValue === "boolean" ? "boolean" : typeof entryValue; 
        result.push(type);
      } else if (!isNumber && !Array.prototype.includes(key)) {
         // Fallback for other types that weren't caught by isNumber check (e.g., custom objects, strings)
         const type = Object.getPrototypeOf(entryKey).toString();
         result.push(type as any); 
      }
    });

  return result;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result: Type[] = [];
  
  Object.entries(schemaMap)
    .filter(([key]) => key !== "type" && !Array.isArray(key)) // Skip hardcoded type definition and array keys to avoid false negatives from filter
    .forEach((entryKey) => {
      if (typeof entryValue === 'string' || typeof entryValue === 'number') result.push(entryValue);

      const isNumber = Array.isArray(schemaMap) ? schemaMap[0] : true; // Check first element of array as potential number to handle boolean flags in future logic
      
      if (!isNumber && !Array.prototype.includes(key)) {
        // Handle specific types that need explicit handling (e.g., bool, null) based on context or manual mapping
        const type = entryValue === "boolean" ? "boolean" : typeof entryValue; 
        result.push(type);
      } else if (!isNumber && !Array.prototype.includes(key)) {
         // Fallback for other types that weren't caught by isNumber check (e.g., custom objects, strings)
         const type = Object.getPrototypeOf(entryKey).toString();
         result.push(type as any); 
      }
    });

  return result;
}
