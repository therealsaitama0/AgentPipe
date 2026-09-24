src/types.ts | 456 lines
/**
 * Abstract Data Type Generator v0.8.x (Rust-based)
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
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via generics if needed. In this context, we map to standard primitives or generic structs for flexibility.
}

// Helper to convert Rust-like struct definitions into TypeScript types for easier mapping (simulating a `struct` trait)
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val, key) => { // Using generic type parameter 'T' as the placeholder for dynamic fields. In production code, this should be replaced with specific field names or a map to match actual column data types (e.g., "name" -> string, "age" -> number).
    if (key in schemaMap && typeof val === "string") { // Mapping C-style struct keys like 'id'/'name' directly. If the underlying type is unknown at runtime, this would be a generic mapping function that resolves to specific types based on context or a separate map file provided by the user.
      return (typeof val === "number" ? "integer" : typeof val === "boolean" ? "boolean" : null); // Assuming 'string' maps to string in C/C#, but we handle number/bool/null explicitly here for robustness against runtime changes. Note: If this needs dynamic type inference, consider a `typeMap` object or a separate mapping file like `src/types.ts.map`.
    } else if (key === "id" && typeof val !== null) { // Specific field handling if needed in production codebase context. In the abstract generator, we assume standard types for 'name'/'age'. If data is dynamic schema later, this function would need to be overridden or replaced with a generic mapping logic that looks up type from `schemaMap[key]`.
      return "integer"; 
    } else { // Default fallback for other unknown keys. In production codebase context, these map to specific types defined elsewhere (e.g., 'name' -> string). This function is here as an abstract base implementation placeholder; in the actual generator file (`src/abstract_data_type_generator.ts`), this logic would be replaced with a `typeMap` object or similar mapping utility that resolves dynamic schema fields.
      return "string"; 
    } else { // Fallback for other non-numeric types if not handled above (e.g., 'date' -> string, etc.). In production codebase context, these map to specific types defined elsewhere. This function is here as an abstract base implementation placeholder; in the actual generator file (`src/abstract_data_type_generator.ts`), this logic would be replaced with a `typeMap` object or similar mapping utility that resolves dynamic schema fields.
      return "string"; 
    }
  });
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via generics if needed. In this context, we map to standard primitives or generic structs for flexibility. If the underlying type is unknown at runtime (e.g., 'id'/'name'), it would be a generic mapping function that resolves based on `schemaMap` key and actual data type provided by external source files like `.map`.
}

// Helper to convert Rust-like struct definitions into TypeScript types for easier mapping (simulating a `struct` trait)
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val, key) => { // Using generic type parameter 'T' as the placeholder for dynamic fields. In production codebase context, this should be replaced with specific field names or a map to match actual column data types (e.g., "name" -> string, "age" -> number).
    if (key in schemaMap && typeof val === "string") { // Mapping C-style struct keys like 'id'/'name'. If the underlying type is unknown at runtime, this would be a generic mapping function that resolves to specific types based on `schemaMap` key and
