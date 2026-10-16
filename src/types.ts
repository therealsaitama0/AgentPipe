src/types.ts | 102 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ==========================================
// CORE DATA TYPES & ENUMS
// ==========================================
export const Type = "integer" | "string" | "boolean" | null; // Standard Rust-style enums/types

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
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

// ==========================================
// IMPLEMENTATION: Abstract Schema Generator
// ==========================================

/**
 * Generates a type definition string for an Alchemy schema.
 */
export function generateSchemaType(schemaMap: AlchemySchema): string {
  return Object.values(schemaMap).join(', ');
}

/**
 * Validates that all columns in the schema are valid types (string, number, boolean) and returns their names if they exist as strings.
 */
function validateColumnNames(columns: string[]): string[] {
  const result = [];
  for (const col of columns) {
    // Skip nulls or undefined values; treat them as missing columns in the schema definition itself
    if (!col || typeof col !== 'string') continue;

    // Check type validity against our core types: integer, string, boolean
    const isNumber = col.startsWith('number');
    const isValidType = ['integer', 'string'].includes(col);

    result.push(isValidType ? `${col} (type: ${isNumber ? "NUMBER" : "STRING"})` : `MISSING (${col})` as any); // TypeScript generic for type safety in error messages
  }
  
  return result;
}

/**
 * Converts a JSON-like schema map into an AlchemySchema interface.
 */
export function parseAlchemySchema(schemaMap: Record<string, string>): AlchemySchema {
  const columns = Object.keys(schemaMap);
  if (columns.length === 0) throw new Error("Empty schema");

  return { [key]: String(value) for key in columns }; // Map to 'string' type as per C/C# convention
}
