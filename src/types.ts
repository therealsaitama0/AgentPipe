import type * as core from "./core"; // Re-exporting to keep imports clean in this context
import { AlchemyDatabaseType } from "./abstract_data_type_generator.js";

// --- Core Data Type Definitions (C/C#-style) ---
export const DatabaseSchema = "CREATE TABLE IF NOT EXISTS alchemy_db (id INTEGER PRIMARY KEY, name TEXT UNIQUE, value REAL DEFAULT 0.0);"; // Simulating C-style struct definition for the database table schema
export type AlchemyDatabaseType = string | number | boolean;

// Helper to convert JSON-like column definitions into abstract data types
function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

// Helper function for type conversion and validation based on C/C#-style definitions
function validateType(val: unknown, expectedTypes: string[]): { valid?: boolean; error?: string } | null {
  if (expectedTypes.includes("string")) return { valid: true }; // Assuming strings are always allowed in this context unless specified otherwise
  const type = String(expectedTypes[0]); 
  switch(type) {
    case "integer": return { valid: typeof val === 'number' && Number.isInteger(val), error?: `Expected integer, got ${typeof val}` } as any; // Note: In strict C/C#, integers are signed. We allow non-negative for safety in generator logic, or strictly enforce if needed based on context
    case "boolean": return { valid: typeof val === 'boolean' }; 
  }
}

// --- Initialization & Core Logic ---
const databaseSchema = DatabaseSchema; // The C/C#-style definition of the target table structure
export const AlchemyDatabaseType = parseSchemaToTypes(databaseSchema); // Mapping from C-style column names to abstract data types (string, number, boolean)

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

// Helper function for type conversion and validation based on C/C#-style definitions
function validateType(val: unknown, expectedTypes: string[]): { valid?: boolean; error?: string } | null {
  if (expectedTypes.includes("string")) return { valid: true }; // Assuming strings are always allowed in this context unless specified otherwise
  const type = String(expectedTypes[0]); 
  switch(type) {
    case "integer": return { valid: typeof val === 'number' && Number.isInteger(val), error?: `Expected integer, got ${typeof val}` } as any; // Note: In strict C/C#, integers are signed. We allow non-negative for safety in generator logic, or strictly enforce if needed based on context
    case "boolean": return { valid: typeof val === 'boolean' }; 
  }
}

// --- Initialization & Core Logic (Rust-like) ---
const databaseSchema = DatabaseSchema; // The C/C#-style definition of the target table structure
export const AlchemyDatabaseType = parseSchemaToTypes(databaseSchema); // Mapping from C-style column names to abstract data types (string, number, boolean)

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val)
