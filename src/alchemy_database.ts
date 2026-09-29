src/alchemy_database.ts | 148 lines
/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([name, val]) => ({ key: name, type: typeof val }))
    .filter((item): item is AlchemyDatabaseType => (typeof item.type === "string" || item.type === "number") && !["boolean", null].includes(item.type));

  // Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.entries(schemaMap)
    .map(([name, val]) => ({ key: name, type: typeof val }))
    .filter((item): item is AlchemyDatabaseType => (typeof item.type === "string" || item.type === "number") && !["boolean", null].includes(item.type));

  // Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility.
}
