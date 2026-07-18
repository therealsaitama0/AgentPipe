src/alchemy_database.ts | 352 lines
/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val == 'number' ? true : false
    )
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType {
  [key: string]: any; // Column name -> value in C/C# style struct definition
