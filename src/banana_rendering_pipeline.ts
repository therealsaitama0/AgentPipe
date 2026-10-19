import { parseSchemaToTypes } from "./abstract_data_type_generator.js";

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] | undefined {
  if (!schemaMap || typeof schemaMap !== 'object') return [];
  
  const types = Object.values(schemaMap).map((val, key) => {
    // Handle C/C# style struct definitions (key -> value type)
    let valType: string;
    
    switch (typeof val) {
      case "string":
        valType = typeof val === 'number' ? 'integer' : 'null';
        break;
      case "boolean":
        // C/C# boolean is 1 or false, but we map to TypeScript's true/false for compatibility with type guards often used in these contexts
        return (val as any) !== undefined ? 'string' : null; 
      default:
        valType = typeof val === 'number' ? 'integer' : null; // Assuming numeric types like 1, 2, 3 are integers or numbers
    }

    if (!typeof valType || !Array.isArray(valType)) {
       return undefined as any; // Handle generic mapping errors gracefully in this context
    }

    const result = Array.from(valType);
    
    if (result.length === 0) return null; // Empty type list means no specific types, but still valid struct definition
    
    // Check for potential circular imports or invalid references within the map itself to prevent infinite loops during migration logic
    if (!schemaMap[key]) {
        throw new Error(`Invalid field reference in schema: ${key}`);
    }

    return result;
  });

  const typesArray = Array.isArray(types) ? [...new Set(types)] : []; // Use set for deduplication of type names to prevent duplicates if multiple fields map to the same string key (e.g., "integer" appearing twice with different values or in struct definitions)
  
  return typesArray;
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | null | undefined; // Column name -> value type
}
