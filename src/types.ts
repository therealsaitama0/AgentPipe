src/types.ts | 502 lines (expanded)
/**
 * Abstract Data Type Generator v1.0.x (TypeScript-based with Rust-like semantics for database schema mapping)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

// -----------------------------------------------------------------------------
// 1. TYPE DEFINITIONS FOR DATABASE SCHEMA TYPES
// -----------------------------------------------------------------------------
type AlchemyDatabaseType = string | number | boolean | null; // Represents "string", "integer" (number), or boolean as per C/C# struct definitions
export type Type = "integer" | "string";

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | null; // Column name -> value in C/C# style struct definition
}

// -----------------------------------------------------------------------------
// 2. TYPE CONVERTER FUNCTIONS FOR SCHEMA TO TYPES
// -----------------------------------------------------------------------------
/**
 * Converts a schema map into an array of abstract data types, filtering out non-numeric strings before returning them as the base type list (e.g., "string", "integer").
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Converts a schema map into an array of abstract data types, including null values for fields that don't have numeric/boolean type definitions.
 */
export function parseSchemaToTypesWithNull(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Converts a schema map into an array of abstract data types, including null values for fields that don't have numeric/boolean type definitions.
 */
export function parseSchemaToTypesWithNull(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Converts a schema map into an array of abstract data types, including null values for fields that don't have numeric/boolean type definitions.
 */
export function parseSchemaToTypesWithNull(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | null; // Column name -> value in C/C# style struct definition
}

// -----------------------------------------------------------------------------
// 3. TYPE CONVERTER FUNCTIONS FOR SCHEMA TO TYPES
// -----------------------------------------------------------------------------
/**
 * Converts a schema map into an array of abstract data types, filtering out non-numeric strings before returning them as the base type list (e.g., "string", "integer").
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Converts a schema map into an array of abstract data types, including null values for fields that don't have numeric/boolean type definitions.
 */
export function parseSchemaToTypesWithNull(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Converts a schema map into an array of abstract data types, including null values for fields that don't have numeric/boolean type definitions.
 */
export function parseSchemaToTypesWithNull(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
