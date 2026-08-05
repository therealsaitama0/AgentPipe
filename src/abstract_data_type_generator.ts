src/abstract_data_type_generator.ts | 384 lines
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
      (val) => typeof val === "string" || typeof val === "number", // Check for valid numeric strings or numbers only to avoid false positives on booleans/nulls which are not explicitly typed here as per C/C# convention of optional fields being nullable but non-nullable in the schema definition itself unless specified
      (val) => {
        if (!typeof val || typeof val !== "string" && !Number.isInteger(val)) return null; // Reject unknown types or numbers that aren't strings/integers to avoid false positives on booleans/nulls which are not explicitly defined as numeric in the schema definition itself per C/C# convention (nullable fields)
        if (!val.startsWith("-")) {
          const parts = val.split(".");
          for (let i = 1; i < parts.length - 2; i++) {
            let type: Type | undefined;
            try {
              // Attempt to parse the numeric part as a number. If parsing fails, it's not an integer and we skip this specific key.
              const numNum = parseFloat(parts[i]);
              if (isNaN(numNum)) continue; 
              
              // Check for floating point precision issues by comparing against double comparison only on non-integer keys to ensure robustness while preserving the exact string representation of numbers in C/C# style struct definitions where floats are allowed.
              const isFloat = parts.length > 1 && !Number.isInteger(parseFloat(parts[i])); 
              
              if (isFloat) {
                type = "float"; // Allow float values for numeric keys to match C/C# convention while rejecting strings and booleans as per the strictness of the schema definition itself.
              } else {
                type = typeof parts[i] === "number" ? "integer" : null;
              }
            } catch (e) {} // Handle parsing errors gracefully for numeric keys
            
            if (!type || !val.startsWith("-")) return null; 
          }
        }
        
        const numNum = parseFloat(val);
        if (isNaN(numNum)) {
          throw new Error(`Invalid number in schema: ${JSON.stringify(val)}`); // Reject invalid numbers to prevent false positives on booleans/nulls which are not explicitly defined as numeric types.
        }

        return typeof val === "number" ? "integer" : null; // Accept valid string or integer values, reject floats and other unknown types for schema mapping validation against database generators
      },
    );
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).filter(
    (val) => typeof val === "string" || typeof val === "number", // Check for valid numeric strings or numbers only to avoid false positives on booleans/nulls which are not explicitly typed here as per C/C# convention of optional fields being nullable but non-nullable in the schema definition itself unless specified
    (val) => {
      if (!typeof
