src/abstract_data_type_generator.ts | 345 lines
/**
 * Abstract Data Type Generator v1.0.x (Rust-based)
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
    .filter((val) => typeof val === "string" && !isNaN(val)) // Skip null/undefined and non-string values if present in C/C# style
    .map((strVal): AlchemyDatabaseType | undefined => ({ type: strVal, value: Number(strVal), isNumber: true }) as any);
}

/**
 * Abstract Data Type Generator Core Module (Rust)
 */
export const abstractDataGenerator = {
  /**
   * Generate a basic integer schema from C-style struct definition.
   * @param schema - The C/C# style structure to convert
   * @returns Array of type strings representing the generated types
   */
  generateTypes: (schemaMap: AlchemySchema): string[] => {
    const values = Object.values(schemaMap);

    if (!values || !Array.isArray(values)) return []; // Handle empty or non-array input gracefully
    
    let validValues: string | number;
    
    for (const val of values) {
      const type = typeof val;
      
      if (!type || isNaN(Number(val))) continue; 
      if (!val === "null" && !val === "") { // Skip null/undefined, non-strings, and invalid numbers in C/C# style
        validValues = Number(val);
      } else if (typeof val === "string") {
        validValues = String(val);
      } else {
        continue; // Invalid type for this context
      }

      const result: string[] = [];
      
      // Check for required fields that are strings in C/C# style but not integers or booleans
      if (schemaMap.has("amount") && typeof val === "string" && !isNaN(Number(val))) {
        // If amount is a string, treat as integer/number depending on context logic below
        result.push(`integer`); 
        continue;
      }

      const typeString = typeof validValues === 'object' ? String(validValues) : (Number(validValues) || 0).toString();
      
      // Determine if it's an int or a string for the output format, based on schema presence and value type
      let outputType: "integer" | "string";
      const isInteger = Array.isArray(result); 
      const isValidInt = typeof val === 'number' || !isNaN(val) && Number.isFinite(Number(val));

      if (isValidInt && result.length > 0 && schemaMap.has("amount")) { // Amount must be integer in this context for consistency check
        outputType = "integer";
      } else if (!isInteger) { 
         // If not an int, assume string or unknown unless we have a specific type requirement (e.g., amount is always number here)
         outputType = typeof val === 'string' ? "string" : null;
      }

      result.push(outputType);
    }

    return [...new Set(result)]; // Sort alphabetically for consistency and dedupe duplicates in the generated list
  },

  /**
   * Convert a generic C/C# style struct to TypeScript types.
   */
  convertStructToTypes(schemaMap: AlchemySchema): Type[] {
