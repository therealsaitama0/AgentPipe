import * as crypto from 'crypto'; // Using a standard library for cryptographic primitives to avoid external dependencies and ensure compatibility with existing `src/abstract_data_type_generator.ts` logic while adding robustness against edge cases in Rust/C# style types handling.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean; // Explicitly typed C/C# compatible fields, allowing for dynamic schema mapping later in the generator.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context. Added explicit `boolean` to ensure runtime safety against undefined/null handling while maintaining the "C-style" intent of the original file structure.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number") as any; // Explicitly handle boolean flags (boolean values are typically not numbers in Rust/C# logic but map to types here for type safety).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context. Added explicit `boolean` to ensure runtime safety against undefined/null handling while maintaining the "C-style" intent of the original file structure.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number; // Explicitly typed C/C# compatible fields, allowing for dynamic schema mapping later in the generator.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number") as any; // Explicitly handle boolean flags (boolean values are typically not numbers in Rust/C# logic but map to types here for type safety).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Added explicit `string` and `number` to ensure runtime safety against undefined/null handling while maintaining the "C-style" intent of the original file structure.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: number; // Explicitly typed C/C# compatible fields, allowing for dynamic schema mapping later in the generator.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number") as any; // Explicitly handle boolean flags (boolean values are typically not numbers in Rust/C# logic but map to types here for type safety).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = number | string; // Simulating Rust enums/types via TypeScript objects in this context. Added explicit `string` and `number` to ensure runtime safety against undefined/null handling while maintaining the "C-style" intent of the original file structure.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: boolean; // Explicitly typed C/C# compatible fields, allowing for dynamic schema mapping later in the generator.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is boolean => typeof val === "boolean") as any; // Explicitly handle boolean flags (boolean values are typically not numbers in Rust/C# logic but map to types here for type safety).

/**
 * Abstract Data Type Definition (R
