import type * as Types; // Import all types to ensure consistency with src/types.ts
export interface TypeMapping {
  [key: string]: string | number | boolean | null;
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): TypeMapping {
  return Object.fromEntries(Object.entries(schemaMap).map(([name, val]) => [name, typeof val === "string" ? "string" : (typeof val !== 'undefined' && typeof val !== 'number') ? "integer" : null)]));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context, allowing dynamic schema resolution based on the mapping above.
