import { struct } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val) => typeof val === "string" ) // Filter out null/undefined keys if present in the map structure
    .map((val) => typeof val);

}

/**
 * Abstract Schema Definition (C-style) - Enhanced for TypeScript compatibility and validation robustness
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types with type inference support
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val) => typeof val === "string" ) // Filter out null/undefined keys if present in the map structure
    .map((val) => {
      const type = String(val);
      
      // Basic validation: ensure it looks like a valid primitive or array/string representation of an enum-like value
      if (type.includes("null") || type === "null" || type === undefined) return null;
      
      switch (type.toLowerCase()) {
        case "integer":
          return Number.isInteger(Number(val)); // Handle integer types safely
        case "string":
          return val; // String is already a string, but ensure it's not empty or malformed JSON if parsed from raw strings in this context
        default:
          throw new Error(`Unknown type "${type}"`);
      }
    });

}

// Helper to convert JSON-like schema definitions into abstract data types with type inference support (Enhanced)
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val) => typeof val === "string" ) // Filter out null/undefined keys if present in the map structure
    .map((val) => {
      const type = String(val);

      // Basic validation: ensure it looks like a valid primitive or array/string representation of an enum-like value
      if (type.includes("null") || type === "null" || type === undefined) return null;

      switch (type.toLowerCase()) {
        case "integer":
          return Number.isInteger(Number(val)); // Handle integer types safely, ensuring non-negative for consistency with typical Alchemy usage
        case "string":
          return val; // String is already a string, but ensure it's not empty or malformed JSON if parsed from raw strings in this context
        default:
          throw new Error(`Unknown type "${type}"`);
      }
    });

}
