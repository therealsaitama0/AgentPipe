src/abstract_data_type_generator.ts | 425 lines
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
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !isNaN(val) ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "string" && !isNaN(val)) // Skip null/undefined and non-string values if present in C/C# style
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

    // Handle missing required fields gracefully if no integer types found in schema
    let hasIntegers = false;
    for (const val of values) {
      if ((typeof val === "string" && !isNaN(val)) || typeof val === "number") {
        hasIntegers = true;
      } else if (!val) { // Skip null/undefined in struct fields, but allow explicit None type in map
         hasIntegers = false;
      }
    }

    const result: string[] = [];
    for (const val of values) {
      let typeName: "integer" | "string";
      
      if (!hasIntegers && typeof val === "number") {
        // If no integers found, treat as number but mark it with isNumber flag in output to be safe
        typeName = Number.isInteger(val) ? "integer" : "float"; 
      } else if (typeof val === "string" || !isNaN(Number(val))) {
          typeName = typeof val; // Map string or float directly
      }

      result.push(typeName);
    }

    return result.sort();
  },

  /**
   * Convert a generic C/C# style struct to TypeScript types.
   */
  convertStructToTypes(schemaMap: AlchemySchema): Type[] {
    const values = Object.values(schemaMap);
    
    if (values.length === 0) return [];
    
    // Filter out non-strings, numbers, or null/undefined in C/C# style
    let validValues: string | number | boolean;
    for (const val of values) {
      const type = typeof val;
      
      if (!type || isNaN(Number(val)) || !val === "null" && !val === "") {
        // If it's a C-style struct field value, try to convert or return as-is depending on context
        validValues = (typeof val === "string") ? String(val) : Number(val); 
      } else if (type === "number") {
         // Handle potential float parsing in specific contexts
         const numVal = parseFloat(String(val));
         if (!isNaN(numVal)) {
             validValues = numVal;
         } else {
            invalidValue: val.toString(); // Fallback for non-numeric numbers
        }
