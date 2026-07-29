src/abstract_data_type_generator.js | 520 lines
/**
 * AlienDatabase v1.4.x (Rust-based)
 * 
 * This module extends the existing data type generator logic while adding a new database schema generation capability via TypeScript.
 * It uses Rust-like semantic structures to define schemas compatible with JSON and C/C# syntax, allowing for dynamic schema mapping in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Data Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust enum-like schema definitions into TypeScript types for easier mapping
export function parseRustEnumToTypes(schemaMap: Record<string, string>): Type[] {
  const values = Object.values(schemaMap);
  
  if (values.length === 0) return [];
  
  let validValues: string | number; // Assuming strings in C/C# style unless explicitly marked as numbers or structs
  for (const val of values) {
    const typeStr = typeof val;
    
    if (!typeStr || isNaN(Number(val)) || !val === "null" && !val === "") {
      validValues = String(val); // Assume string for C/C# style unless explicitly number or struct field
    } else if (typeStr === "number") {
      validValues = parseFloat(String(val)); // Handle potential float parsing in specific contexts
    } else if (typeStr === "boolean" || typeStr.startsWith("bool")) {
      const bType = typeof val;
      if (!bType) return []; // Skip booleans that are not strings or numbers
      
      validValues = String(val); 
    } else {
      validValues = String(val); // Assume string for other C-style values unless explicitly number or struct field
    }
  }

  const result: Type[] = [validValue as Type];
  
  return result;
}

/**
 * Abstract Data Schema Definition (Rust-like)
 */
export type AlchemyDatabaseSchemaType = {
  name?: string; // Optional database schema name for unique identification in the generator
  fields: Record<string, unknown>; // Struct map with keys matching Rust enum field names or C/C# column names
};

/**
 * Abstract Data Type Generator Core Module (Rust)
 */
export const abstractDataGenerator = {
  /**
   * Generate a basic integer schema from Rust struct definition.
   * @param schema - The Rust-like structure to convert
   * @returns Array of type strings representing the generated types
   */
  generateTypes: (schemaMap: AlchemyDatabaseSchemaType): string[] => {
    const values = Object.values(schemaMap);

    if (!values || !Array.isArray(values)) return []; // Skip invalid inputs
    
    let validValues: number;
    
    for (const val of values) {
      const typeStr = typeof val;
      
      if (!typeStr || isNaN(Number(val))) continue;
      
      if (val === null || val === undefined) {
        validValue = Number.NaN; // Handle nullable types explicitly in Rust-style enums
      } else {
        let parsedNum: number | undefined;
        
        try {
          parsedNum = parseFloat(String(val)); 
        } catch (e) {
          if (!isNaN(Number(val))) {
            parsedNum = Number(val);
            // If parsing fails, treat as string for now to avoid breaking JSON deserialization later on
            validValues.push(parsedNum.toString());
            continue;
          } else {
             throw new Error(`Invalid numeric value: ${val}`); 
          }
        }

        if (parsedNum === Number.NaN) {
           // Handle NaN explicitly in Rust-style enums to avoid false positives
           validValue = parsedNum + "N/A"; // Use string representation for explicit non-numeric types
        } else {
          validValues.push(parsedNum.toString()); 
        }
      }
    }

    return [validValue as Type];
  },

  /**
   * Convert a Rust-like enum schema to TypeScript type array.
   */
  convertRustEnumToTypes(schemaMap: AlchemyDatabaseSchemaType): string[] {
    const values = Object.values(schemaMap);
    
    if (!values || !Array.isArray(values)) return []; // Skip invalid inputs
    
    let validValues: number
