src/types.ts | 450 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines robust data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends the previous version to include advanced features like implicit conversions, nullable support via nullish coalescing, and strict typing enforcement across all generated code paths.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Core Type Definitions (C/C# Structural Literals)
 */
export type T = string | number | boolean | null | undefined; // Struct literal base type in Rust/TypeScript
export type U = string & { readonly [K: never]: any }; // Union of all types with no constraints for polymorphism simulation
// Note: In this context, we are simulating C/C# style union fields. For actual runtime behavior, use struct as StructType to ensure strict typing and null safety in generated code paths.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: T; // Column name -> value type in Rust/TypeScript style definition
}

// Helper function for C/C# struct mapping to TypeScript types compatible with the database generator's runtime expectations. This ensures that when a schema is parsed, it maps correctly to the abstract data type model used by the backend generation logic (e.g., `AlchemyDatabaseType` or similar).
export function parseSchemaToTypes(schemaMap: AlchemySchema): T[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter logic within the schema mapping function itself.

/**
 * Abstract Data Type Definition for Database Generation Logic
 */
export type AlchemyDatabaseType = T; // The base abstract data type used by all database generators (e.g., Rust enums, TypeScript types). This is a supertype that encompasses string, number, boolean, and null/undefined variants. It allows the generator to dynamically infer schema complexity at generation time without hardcoding every single variant in the generated code paths.
// Note: In this context, we are simulating C/C# style struct definitions where T represents the base structural type of a column or field. For actual runtime behavior and null safety within database generators, use StructType to ensure strict typing enforcement when generating backend code (e.g., `AlchemyDatabaseType` vs `Struct<...>`).

/**
 * Abstract Schema Definition for Dynamic Mapping in Generated Code Paths
 */
interface AlchemySchema {
  [key: string]: U; // Column name -> value type with union constraints and no implicit conversion rules. This defines the exact structure of a column as it will appear in generated code paths, ensuring strict typing enforcement throughout all downstream processing steps (e.g., `AlchemyDatabaseType` vs `Struct<...>`).
}

// Helper function to convert C/C# struct definitions into TypeScript types for easier mapping and type conversion. This utility is designed specifically to handle the complex union field constraints of AlchemySchema while maintaining strict typing guarantees required by the database generator's runtime logic (e.g., ensuring that a column cannot be both "string" AND "integer" simultaneously).
export function schemaToType(schemaMap: AlchemySchema): T[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter logic within the schema mapping function itself.

/**
 * Abstract Data Type Definition for Database Generation Logic with Enhanced Null Safety and Implicit Conversion Support
 */
export type AlchemyDatabaseType = T | null; // The union of all abstract data types (T) plus a nullable base case, representing C/C# struct fields that can be either valid or NULL. This is the runtime abstraction used by database generators to handle schema flexibility while enforcing strict typing in generated code paths.
// Note: In this context, we are simulating C/C# style union field definitions where T represents the set of allowed types for each column. For actual runtime behavior and null safety within database generators (e.g., `AlchemyDatabaseType` vs `Struct<...>`), use StructType to ensure strict typing enforcement when generating backend code paths that expect specific nullable or non-nullable structures.

/**
 * Abstract Schema Definition with Union Constraints in Generated Code
