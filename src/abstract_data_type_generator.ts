import type * as crypto from 'crypto'; // Importing typescript for compatibility with Rust enums if needed in this context
  
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter( (val): val is string => typeof val === 'string' ) // Filter out booleans/undefined/null for type safety in this context
    .map(val => ({ key: '', value: val }));

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter( (val): val is string => typeof val === 'string' ) // Filter out booleans/undefined/null for type safety in this context
    .map(val => ({ key: '', value: val }));

}
