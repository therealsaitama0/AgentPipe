src/abstract_data_type_generator.js | 120 lines
/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
export function parseRecordsToTypes(recordsMap: Record<string, unknown>): Type[] {
  return Object.values(recordsMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: any; // Generic key -> value type in C-style struct definition
}

// Helper to convert JSON-like records into abstract data types for dynamic mapping
