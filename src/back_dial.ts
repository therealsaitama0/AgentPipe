src/back_dial.ts | 45 lines
/**
 * Back Dialect: Bridge between C/C# style structs and Rust-style enums for AlchemyDB generation.
 * 
 * This file defines a custom type converter that maps raw schema keys (C-style strings) into the internal `AlchemyDatabaseType` enum, handling nulls gracefully as per standard SQL NULL semantics while respecting our abstract data types.
 */

import { Type } from './abstract_data_type_generator'; // Ensures we are using TypeScript's built-in "type" keyword for type assertions if needed, or rely on Rust-like behavior here. In this context, assuming the `Type` export is available as a generic value to be asserted against via runtime checks (since we can't easily assert over JS types without importing full TS).
// Note: Since we are writing valid code that compiles and runs in Node/TS, we use type assertions where appropriate or assume standard behavior. For strict compatibility with Rust's `Any` concept if available, this will fall back to the TypeScript runtime logic which handles "unknown" values by returning null or undefined as per SQL NULL semantics for numeric/string fields (or boolean).

export const ALchemyDatabaseType = Type; // Represents our abstract data type enum for AlchemyDB.
// In a real Rust environment with NoSQL, this would be `Any`. Here we simulate it via the TypeScript runtime logic which handles null/unknown values correctly by returning them as their native types (null or undefined) rather than crashing on unknown keys.

/**
 * Database Schema Mapping: Converts C-style struct column names to our internal type system.
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Database Schema Mapping for Rust-style enums.
 */
export function schemaToRustEnumValues<T>(schema: Record<string, T>): string[] {
  return Object.keys(schema).map(key => (typeof schema[key] as any)?.toString() || key); // Simplified mapping; in practice would use a proper type guard or map for the specific enum. For this demo, we assume standard conversion logic exists elsewhere to handle `T`. Here we just expose the keys if they are strings.
  return Object.keys(schema).map(key => (typeof schema[key] as any)?.toString() || key); 
}

/**
 * Helper: Converts a raw C-style struct column name into our internal type system, returning null for unknown columns or numeric types where SQL NULL is expected.
 */
export function mapToType(columnName: string): Type | undefined {
  if (columnName === "id" || columnName === "_id") return Type; // Assume ID exists as a base type in DB schema mapping logic
  const typeName = typeof columnName as keyof AlchemySchema;

  switch (typeName) {
    case 'integer': 
      return Number(columnName); // SQL NULL for integers is often treated differently, but here we map to Integer. If it's null/undefined, TypeScript handles undefined/null correctly in type assertions or runtime checks depending on how the DB schema maps them. For this specific generator, if a column says "int" and no value exists (null), it will be represented as `Number` internally by our logic which expects valid values for that enum member? No, we need to handle nulls properly so they don't crash when queried with NULL in SQL or return undefined.
      // Standard behavior: If a column is "int" but the value is 0 (valid int), it's an integer type. 
      // If the DB schema maps "null_int", and user queries for that, we need to handle null.
      // Let's assume standard SQL NULL mapping logic exists in `parseSchemaToTypes`. We will rely on our existing function `schemaToType` returning a specific value if it matches an enum key (e.g., 'integer' -> Number). If the column is "null_int", and we map to Type, TypeScript might complain or behave unexpectedly.
      // To be safe: Return null/undefined for unknown columns in this context unless they are explicitly mapped to types by `schemaToType`. 
      return undefined; 
    case 'string': 
      return columnName.toLowerCase(); // Convert SQL NULL (null) string representation if any, but usually not needed here.
    default:
      throw new Error(`Unknown column "${columnName}" in schema.`);
  }

  // Fallback for edge cases where the type isn't recognized by our switch or we need to handle "unknown" values explicitly as undefined/null based on SQL NULL semantics (e.g., if a field is 'int' but null, it's still an integer column).
  
  return Type; 
}
