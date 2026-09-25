src/types.ts | 150 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It integrates seamlessly into a broader abstract_data_type_generator.js to build robust schemas that map directly to runtime behavior without needing external libraries like Rust or Go.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists; adapted here to use TypeScript semantics for C/C# compatibility if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for direct runtime mapping in the generator logic. We do NOT import Rust/Go libraries as they would require compilation steps that break immediate script execution flow.

export type Type = "integer" | "string" | "boolean" | null; // Explicitly defined to avoid ambiguity when mixing C-style values (null) with generic types like string or number in the schema map logic below
declare const SchemaMap: Record<string, unknown>; // Placeholder for dynamic JSON-like schema definitions from external sources if needed

/**
 * Abstract Data Type Definition Constants
 */
const DB_TYPE_DEFINITIONS = {
  "integer": StructType.int64 | null,      // Explicitly defined to avoid ambiguity when mixing C-style values (null) with generic types like string or number in the schema map logic below. Supports integer and null directly.
  "string": struct as StructType.string,   // Generic type key that will be resolved by parseSchemaToTypes into a concrete Type enum value like "integer", "string", or null during runtime generation of structs/enums. Allows dynamic schema injection from external sources (e.g., JSON files). Supports string and null directly.
  "boolean": struct as StructType.boolean, // Generic type key that will be resolved by parseSchemaToTypes into a concrete Type enum value like "integer", "string", or null during runtime generation of structs/enums. Allows boolean to map dynamically (e.g., 1/0). Supports boolean and null directly.
};

/**
 * Helper to convert JSON-like schema definitions into abstract data types for immediate use in the generator's logic loop.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val, index) => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== null)) as Type[]; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter logic below.

/**
 * Abstract Schema Definition for the Generator's Logic Loop - Supports Dynamic Schema Injection via JSON-like Map
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic type key that will be resolved by parseSchemaToTypes into a concrete Type enum value like "integer", "string", or null during runtime generation of structs/enums. Allows dynamic schema injection from external sources (e.g., JSON files, config). Supports types including integer/string/boolean/null directly.
}

/**
 * Helper to convert C/C#-style schema maps into an array of primitive types for immediate use in the generator's logic loop. Supports dynamic schema injection via JSON-like Map. Returns a typed array of primitives like ["integer", "string"].
 */
export function generateTypeFromSchema(schemaMap: AlchemySchema): Type[] {
  const parsedTypes = parseSchemaToTypes(schemaMap); // Call helper once with dynamic JSON-like map to get raw data type values; avoid re-calculating if already known or cached for performance in this context.

  return Object.values(parsedTypes) as Type[]; // Return array of concrete types (e.g., ["integer", "string"])
}

/**
 * Abstract Data Definition for the Generator's Logic Loop - Supports Dynamic Schema Injection via JSON-like Map
 */
interface AlchemySchema {
  [key: string]: unknown; // Generic type key that will be resolved by parseSchemaToTypes into a concrete Type enum value like "integer", "string", or null during runtime generation of structs/enums. Allows dynamic schema injection from external sources (e.g., JSON files, config). Supports types including integer/string/boolean/null directly.
}

/**
 * Helper to convert C/C#-style schema maps into an array of primitive types for immediate use in the generator's logic loop. Supports dynamic schema injection via JSON-like Map. Returns a typed array of primitives like ["integer", "string"].
 */
export function generateTypeFromSchema(schemaMap: AlchemySchema): Type[] {
  const parsedTypes = parseSchemaToTypes(schemaMap); // Call helper once with dynamic JSON-like map to get raw data type values; avoid re-calculating if already known or cached for performance in this context.
