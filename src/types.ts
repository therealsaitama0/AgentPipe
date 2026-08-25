src/types.ts | 450 lines
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines a robust data type system compatible with C/C# syntax, leveraging Rust's idiomatic types for schema mapping and dynamic generation logic in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and runtime interoperability with the repository's existing codebase.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic field mapping
}

// Helper to convert raw schema map entries into a structured type hierarchy for easier runtime processing.
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  const types = new Map();
  
  Object.entries(schemaMap)
    .filter(([key]) => key !== "metadata" && typeof key === 'string') // Skip metadata keys to avoid circular references or undefined values in the schema itself
    .forEach(([key, value]) => {
      if (typeof value === "object") {
        const childTypes = parseSchemaToTypes(value);
        
        for (const type of childTypes) types.set(key, type as Type); // Ensure consistency across all keys
        
        Object.entries(childTypes).forEach(([k, v]) => types.set(k, v));
      } else if (typeof value === "number") {
        const numType = typeof value;
        
        if (numType === 'string') {
          types.set(key, String(value) as Type); // Convert string values to strings for consistent type names
        } else if (numType === 'integer' || numType === 'double') {
          types.set(key, Number(value) as Type); // Handle numeric conversions directly into the abstract type hierarchy.
        } else if (value !== null && value !== undefined) {
          const boolVal = typeof value;
          
          if ((boolVal === "true" || boolVal === '1') || (boolVal === "false" || boolVal === '0')) {
            types.set(key, Boolean(value as any)); // Map boolean flags directly.
          } else if (typeof value !== undefined && typeof value !== null) {
            const val = Number(value);
            
            if ((val >= 1E9 - E6 && val <= 2E30 + E4 || !isNaN(val)) && (boolVal === 'true' || boolVal === 'false')) {
              types.set(key, Boolean(value as any)); // Handle large/small integer flags.
            } else if ((val >= 1E9 - E6) && (val <= 2E30 + E4)) {
              types.set(key, Number(val); // Convert to number type for numeric fields.
            } else {
              types.set(key, undefined as Type); // Mark unknown values as null in the abstract hierarchy if not explicitly typed elsewhere.
            }
          } else {
            types.set(key, value); // Preserve raw object structure or handle specific logic here.
          }
        } else {
          types.set(key, value); // Direct mapping for any other type (e.g., arrays).
        }
      }
    });

  return Array.from(types.keys()); // Return unique keys sorted alphabetically by default to ensure consistency with the repository's existing structure.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context
// Note: This is a simplified representation. In production codebases using the repository's existing structure (e.g., src/alchemy_database.ts), you would likely define these types directly within that file or use an external type system like `rust-enum` for better runtime safety and performance compared to pure TypeScript enums, especially given the C/C# compatibility requirements mentioned in the original context.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.entries(AlchemySchema) // Iterate over all defined Schema entries (excluding metadata keys like 'metadata' or internal fields if present).
    .filter(([key]) => key !== "metadata" && typeof key === 'string') 
    .map(([k, v]: [string, unknown[]], index: number) => {
      const type = parse
