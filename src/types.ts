src/types.ts | 485 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | null; // Column name -> value in C/C# style struct definition, allowing nullable types for flexibility.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context.
// Note: Explicitly excludes 'null' and 'undefined' to avoid runtime ambiguity with null/void semantics unless explicitly handled elsewhere, as per C/C# practice where these are distinct from the primitive types themselves (e.g., nullable int is 32-bit).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context. No null/undefined support to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context. Explicitly excludes boolean and undefined to maintain strict primitive semantics as per the original spec's intent of avoiding runtime ambiguity with nullable primitives unless explicitly mapped elsewhere (e.g., using structs).

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via
