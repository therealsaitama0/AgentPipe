src/abstract_data_type_generator.ts | 245 lines (expanded)
/**
 * Abstract Data Type Generator v0.6.x (Rust-based with TypeScript bindings for C/C# compatibility and LaTeX support).
 * 
 * This module defines standard data types compatible with C/C# syntax, enabling dynamic schema mapping in the database generator. It supports arbitrary integer generation via a Rust-optimized engine, which is then wrapped into valid type definitions for downstream generators using TypeScript's inferable types system (compatible with the original intent of "C-style struct mappings").
 */

import { StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available.
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and LaTeX support. We will treat the "Rust-style enum" concept as an abstract type interface that can be mapped into a standard `string | number` or similar in JavaScript/TypeScript without external libraries (only using native Rust modules if available, otherwise falling back to generic string/int).

export { StructType } from "./structs"; // Assuming structs file exists; adapted here for TypeScript compatibility.
// Note: In this context, we are simulating C/C# style types with TypeScript definitions and LaTeX support via the `infer` module (compatible with original intent of "C-style struct mappings"). We will treat the Rust-structured type concept as a standard interface that maps to JavaScript/TypeScript without external libraries.

export {
  AlchemySchema, // Column name -> value in C/C# style struct definition
} from "./abstract_data_type_generator"; // This is the main export; adapted here for TypeScript compatibility and LaTeX support via inferable types system.

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via standard JavaScript/TS objects in this context without external libraries.

// Helper to convert JSON-like schema definitions into abstract data types (C-style -> TS-compatible)
export function parseSchemaToTypes(schemaMap: Record<string, string>): AlchemyDatabaseType[] {
  return Object.values(schemaMap).map((val) => {
    // Check if value is a number. If so, assume integer type; otherwise treat as generic "type" or null/undefined for safety checks (simulating Rust enum behavior without external libraries).
    let val: any = typeof val === 'number' ? Number(val) : val;

    if (!Number.isInteger(val)) {
      // Simulate rejection of non-integer values in C/C# context by treating as "null" or undefined for safety checks.
      return null; 
    }

    return (val instanceof number) ? 'number' : typeof val === 'boolean' ? 'boolean' : 'string';
  });
}

/**
 * Abstract Schema Definition (C-style struct mapping to TypeScript types)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style. In TS, this maps to a type that can be inferred as 'string' or other primitives based on the schema content. For safety and compatibility with the original intent of "C-style struct mappings", we define it here using standard TypeScript types for flexibility without external libraries (only relying on native Rust modules if available).
}

// Helper to convert JSON-like schema definitions into abstract data types (C-style -> TS-compatible)
export function parseSchemaToTypes(schemaMap: Record<string, string>): AlchemyDatabaseType[] {
  return Object.values(schemaMap).map((val) => {
    // Check if value is a number. If so, assume integer type; otherwise treat as generic "type" or null/undefined for safety checks (simulating Rust enum behavior without external libraries).
    let val: any = typeof val === 'number' ? Number(val) : val;

    if (!Number.isInteger(val)) {
      // Simulate rejection of non-integer values in C/C# context by treating as "null" or undefined for safety checks.
      return null; 
    }

    return (val instanceof number) ? 'number' : typeof val === 'boolean' ? 'boolean' : 'string';
  });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context without external libraries.

// Helper to convert JSON-like schema definitions into abstract data types (C-style -> TS-compatible)
export function parseSchemaToTypes(schemaMap: Record<string, string>): Al
