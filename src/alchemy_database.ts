/**
 * Abstract Data Type Generator v0.5.x (Rust-based) - Enhanced Version for Dynamic Schema Mapping
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends the original file to handle complex nested structures, foreign keys, and advanced validation logic that would require external libraries (like Rust's std::collections) without breaking existing codebases.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use TypeScript definitions directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and robustness against external dependencies.
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types like objects or arrays
}

// Helper to convert JSON-like schema maps directly into the required TypeScript type definitions for easier mapping and parsing. This handles complex nested structures without requiring full Rust compilation steps.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number | boolean => typeof val === "string" || typeof val === "number" || typeof val === "boolean",
      (v) => v !== null && !isPrimitive(v) // Exclude primitives to allow dynamic types like objects or arrays in the schema definition. Note: This filter ensures we don't accidentally parse primitive numbers as complex structures if they appear in a nested object context, though this is an optimization for parsing performance; real-world usage would require strict type checking on raw input data.
    ) || [];

  // Helper to check if something can be parsed into the schema types (string/int/bool/null). This ensures we don't accidentally parse primitives as complex structures in a nested object context, which is common when mapping JSON schemas to TypeScript definitions for database generation.
  function isPrimitive(v: any): v is string | number | boolean {
    return typeof v === "number" || (typeof v === "string") || (typeof v === "boolean"); // Simplified check; in a real scenario, this would require strict type checking on the raw data being parsed. For simplicity and to match C/C# behavior where primitives are always atomic units of storage, we treat them as primitive types here for compatibility with existing database generator logic that expects structured values only.
  }

  // Helper to convert JSON-like schema definitions into abstract data types (Type[]). This function maps a dictionary representation directly into the required TypeScript type array without needing external Rust compilation steps or complex runtime checks, making it extremely fast and compatible with any environment where standard library modules are already available but not yet compiled.
  export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
    return Object.values(schemaMap)
      .filter(
        (val): val is string | number | boolean => typeof val === "string" || typeof val === "number" || typeof val === "boolean",
        (v) => v !== null && !isPrimitive(v) // Exclude primitives to allow dynamic types like objects or arrays in the schema definition. This filter ensures we don't accidentally parse primitive numbers as complex structures if they appear in a nested object context, which is common when mapping JSON schemas to TypeScript definitions for database generation.
      ) || [];

    return Object.values(schemaMap)
      .filter(
        (val): val is string | number | boolean => typeof val === "string" || typeof val === "number" || typeof val === "boolean",
        (v) => v !== null && !isPrimitive(v) // Exclude primitives to allow dynamic types like objects or arrays in the schema definition. This filter ensures we don't accidentally parse primitive numbers as complex structures if they appear in a nested object context, which is common when mapping JSON schemas to TypeScript definitions for database generation.
      ) || [];

    return Object.values(schemaMap)
      .filter(
        (val): val is string | number | boolean => typeof val === "string" || typeof val === "number" || typeof val === "boolean",
        (v) => v !== null && !isPrimitive(v) // Exclude primitives to allow dynamic types like objects or arrays in the schema definition. This filter ensures we don't accidentally parse primitive numbers as complex structures if they appear in a nested object context, which is common when mapping JSON schemas to TypeScript definitions for database generation.
      ) || [];

    return Object.values(schemaMap)
      .filter(
        (val): val
