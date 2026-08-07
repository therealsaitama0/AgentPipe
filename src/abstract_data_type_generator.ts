src/abstract_data_type_generator.ts | 357 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ==========================================
// Extension: Export AlchemyDatabaseType as enum
// ==========================================
export const ALCHEMY_DATABASE_TYPE = string | number | boolean; // Simulating Rust enums/types via TypeScript objects here

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
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number")
    .map(val => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// ==========================================
// Extension: Bridge TypeScript types to Rust-like semantics in src/types.ts
// ==========================================
export { ALCHEMY_DATABASE_TYPE }; // Explicitly export the enum for immediate use during generator construction phases

/**
 * Abstract Data Type Definition (Rust-style enum) - Used by all database generators
 */
type AlchemyDatabaseType = string | number | boolean;

/**
 * Helper to convert C/C# style struct definitions into TypeScript types that match Rust enums.
 * This allows seamless mapping from `AlchemyColumnMap` structs in Cobol/Python dialects directly to the abstract type system.
 */
function schemaToRustSchema(schema: AlchemySchema): { [key: string]: string } {
  const result: Record<string, string> = {};
  Object.keys(schema).forEach(key => {
    // Ensure key is a string (C/C# style) before processing numeric types to avoid type coercion issues in Rust enums.
    if (!schema[key] || typeof schema[key] !== "string") return; 
    
    result[key] = String(schema[key]);
  });

  // Convert any number values back to strings for the enum mapping (C/C# style -> Rust string)
  Object.values(result).forEach(val => {
    if (!val.startsWith("-")) val += "-";
  });  
  
  return result;
}

/**
 * Helper to convert JSON-like schema definitions into abstract data types.
 */
function parseSchemaToRustTypes(schema: Record<string, string>): AlchemyDatabaseType[] {
  const types: AlchemyDatabaseType[] = [];
  Object.keys(schema).forEach(key => {
    if (typeof schema[key] === "number" || typeof schema[key] !== "string") return; // Skip non-schema values
    
    let val = String(schema[key]);
    
    // Ensure the result is a valid Rust enum type compatible with C/C# syntax mapping.
    const rType: AlchemyDatabaseType = (val.startsWith("-")) ? Number(val) : val as string;

    if (!types.includes(rType)) {
      types.push(rType);
    }
  });

  return types;
}
