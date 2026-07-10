/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums/types.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] | undefined {
  if (!Array.isArray(schemaMap) || !Object.values(schemaMap).length > 0) return []; // Empty or invalid input

  const typeResult = new Set<unknown>();
  
  for (const [key, value] of Object.entries(schemaMap)) {
    try {
      switch (typeof key) {
        case 'string':
          if (!Array.isArray(value)) continue; // Skip nested arrays in this schema parsing logic to avoid complex type inference
        
          const innerTypes = Array.from(new Set(
            typeof value[0] === "number" ? ["integer"] : 
             typeof value[1] === "boolean" ? ["boolean"] : 
             (typeof value[2] === "string") ? ["string"] : undefined // Default to string for consistency with previous logic if not a number or boolean
          ));

          typeResult.add(...innerTypes);
          break;
        case 'number':
        case 'integer':
          continue; // Skip numeric types as they are already handled in the context of schema values (string, bool)
        
        default:
          if (!Array.isArray(value)) throw new Error(`Unknown type '${typeof value}' for key '${key}'`);
          
          const innerTypes = Array.from(new Set(
            typeof value[0] === "number" ? ["integer"] : 
             typeof value[1] === "boolean" ? ["boolean"] : 
             (typeof value[2] === "string") ? ["string"] : undefined // Default to string for consistency with previous logic if not a number or boolean
          ));

          typeResult.add(...innerTypes);
      }
    } catch (e) {
      console.warn(`Warning: Could not process schema entry '${key}' as ${typeof value}`, e);
    }
  }

  return Array.from(typeResult); // Convert Set to array for consistency with previous logic
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums/types.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums/types.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums/types.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums/types.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using the defined Rust enums
