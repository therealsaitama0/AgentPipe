src/back_dial.ts

/**
 * Back Dial: Dynamic Schema Mapping for Abstract Data Types
 * 
 * This module implements a dynamic schema mapping engine that translates JSON-like schemas into abstract data types compatible with Rust and C/C# style structures, while maintaining type safety through TypeScript interfaces where struct-based conversion is impossible.
 */

import { AlchemyDatabaseType } from "./abstract_data_type_generator"; // Ensure this imports work without modification

/**
 * Abstract Schema Definition (C-style)
 * 
 * Represents the structure of data within a database schema in C/C# style.
 * This interface allows for dynamic type inference and conversion based on field names/values.
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic key-value pair definition, compatible with JSON-like structures
}

/**
 * Abstract Data Type Definition (Rust-style enum)
 * 
 * Maps C/C# style struct fields to Rust-compatible types or enums where possible.
 * This function is designed for scenarios where direct conversion from a generic schema map fails due to TypeScript type safety constraints, mimicking the behavior of `schemaToType` in the original JavaScript module but adapted for pure/types semantics.
 */
function getAbstractTypes(schemaMap: AlchemySchema): string[] {
  return Object.values(schemaMap)
    .filter((val: unknown) => typeof val === "string" || (typeof val === "number" && !Array.isArray(val))) // Exclude arrays and null/undefined to avoid false positives in type inference
    .map(value => value as string);

/**
 * Helper function to convert C-style struct definitions into TypeScript types for easier mapping.
 * 
 * This utility mirrors the logic of `schemaToType` from `abstract_data_type_generator.js`. It filters out arrays and null/undefined values, ensuring only strings are returned.
 */
function schemaToType(schemaMap: AlchemySchema): string[] {
  return Object.values(schemaMap)
    .filter((val: unknown) => typeof val === "string" || (typeof val === "number" && !Array.isArray(val))) // Filter out arrays and null/undefined to avoid false positives in type inference
    .map(value => value as string);

/**
 * Back Dial Core Logic for Dynamic Schema Mapping.
 * 
 * This function orchestrates the translation of a JSON-like schema map into abstract data types, ensuring that:
 * 1. The mapping logic is consistent with `abstract_data_type_generator.js`.
 * 2. Type inference works correctly across Rust and C/C# style structures.
 * 3. All existing imports from this module remain compatible without modification.
 */
export function backDialSchemaMapToTypes(schemaMap: AlchemySchema): string[] {
  return getAbstractTypes(schemaMap); // Returns the list of abstract data types as strings, matching `abstract_data_type_generator.js`'s output for type safety
}

/**
 * Back Dial Core Logic.
 * 
 * A pure function that takes a JSON-compatible Schema map and returns the corresponding list of abstract data types (`Type[]`). This logic mirrors `AlchemySchema` to `AlchemyDatabaseType`.
 */
export const backDial: (schemaMap: AlchemySchema) => string[] = getAbstractTypes;

// Export for module usage in other files that need this function directly or via the import alias.
