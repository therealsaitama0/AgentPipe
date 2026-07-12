import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

export interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
function parseStructToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val, keyIndex) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null))
    // Add a fallback for any other type to ensure compatibility with the existing schemaToType logic if it handles unknown types differently
    ?.filter(val => !isNaN(val) || Array.isArray(val)); 
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export const AlchemyDatabaseTypes = {
  INTEGER: "integer", // Numeric values in Rust/JSON
  STRING: "string",   // Text data in Rust/JSON
  BOOLEAN: "boolean"  // True/False values in Rust/JSON
};

/**
 * Abstract Schema Definition (C-style)
 */
export type AlchemyDatabaseType = string | number | boolean; 

// Helper to convert JSON-like schema definitions into abstract data types
function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result: Type[] = [];

  for (const [key, value] of Object.entries(schemaMap)) {
    // Check if it's a string or number before converting to type
    let dataType;
    
    switch(value) {
      case "string":
        dataType = "string";
        break;
      
      case "number":
        dataType = "integer";
        break;

      default:
        // For any other value (e.g., null, undefined), map it to the generic base type or specific if known
        const isNullOrUndefined = typeof value === 'null' || value === undefined;
        
        if (!isNullOrUndefined) {
          dataType = "integer";
        } else {
          // Map unknown values to an abstract placeholder for now, ensuring they fit the Type[] definition in main.ts logic
          const typeKey = isNullOrUndefined ? 'null' : (dataType as string); 
          
          if (!result.includes(typeKey)) result.push(typeKey);
        }

      default:
        // Handle unknown types gracefully by adding them to the list of required types for validation later, or map to generic base
        const type = dataType || "unknown";
        
        // Ensure all known types are present in the generated schema (for consistency with existing code)
        if (!result.includes(type)) result.push(type);
    }

    switch(result.length > 0 && Array.isArray(result[0]) ? 'string' : typeof value === 'number') {
      case "boolean":
        dataType = "integer"; // Convert booleans to integers for type safety in this context
        break;
      
      default:
        if (dataType !== null) result.push(dataType);
    }

  }

  return result.filter(val => val !== undefined && !isNaN(val)); 
}
