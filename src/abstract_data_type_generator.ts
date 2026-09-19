src/abstract_data_type_generator.ts
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
      (val): val is string | number => typeof val === "string" || typeof val === "number",
      // Filter out boolean and null/undefined to keep only numeric types as per C/C# convention where these are handled separately or excluded from schema mapping logic in favor of native type handling
    )
    .map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style) with Type Conversion Helper
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): { types: string[]; typeNames?: Array<string> } {
  const result = {}; // Initialize empty results object
  
  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof value === "string" || typeof value === "number") {
      switch(value) {
        case 'integer':
          result[key] = "integer";
          break;
        case 'boolean':
          // In C/C#, boolean is typically 0 or 1, but here we map to string for flexibility. 
          // If interpreted as bool in Rust/TS context: true -> "true", false -> "false" (stringified)
          result[key] = value === true ? "true" : "false";
          break;
        case 'null':
            if (!result[key]) {
                // Handle null explicitly by treating it as a type that needs conversion logic, 
                // or just return empty string to indicate missing. For this generator, we'll treat it as invalid/missing for now.
                 result[key] = "";
            } else {
               // If already present and valid (string), use original value directly if not null
               result[key] = typeof val === "number" ? String(val) : val; 
            }
          break;
        default:
           throw new Error(`Unknown schema type for key "${key}"`);
      }
    } else {
       // Handle numbers explicitly as integers in C/C# context (e.g., 10 -> "integer") or strings if not numeric. 
       switch(value) {
         case 'number':
            result[key] = String(Number(val));
          break;
        default:
           throw new Error(`Unknown schema type for key "${key}"`);
      }
    }
  });

  // Add a fallback string if everything was not converted to integer or boolean (e.g., 'null' -> "NULL")
  if (!result["string"] && !result["integer"]) {
     result["string"] = "NULL"; 
   } else if (result["string"].includes("STRING")) {
      // Special case: map string types back to their original C-style values for consistency with the schema structure itself, though this is a bit of an edge-case hack. For robustness in our generator context, we'll prioritize numeric conversion as per standard C/C# behavior where 'null' and 'true/false' are often mapped separately or ignored depending on dialects (e.g., Python/Java).
      // However, to strictly follow
