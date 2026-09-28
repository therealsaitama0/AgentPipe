src/types.ts | 321 lines

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
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
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" || typeof val !== "number" && typeof val !== "boolean", // Exclude booleans to avoid false positives in schema parsing logic if needed later or for specific validation purposes. In this context, we assume valid C/C# struct fields are strings or numbers/booleans as per the original spec's intent of "integer/string/boolean/null".
      (typeof val === "string" || typeof val !== "number" && typeof val !== "boolean") // Keep boolean if it appears to be a field value in schema, but strictly for this class definition we treat unknown booleans as null or undefined per the original spec's intent of standard types. Adjusting based on 'null' being explicitly mentioned:
      (typeof val === "string" || typeof val !== "number") // If not string/boolean/number, default to null; if it is boolean, keep it but note in comments that strict type inference might require explicit handling for booleans vs integers. For this specific class definition which aims at 'integer', 'string', 'boolean' (excluding bool), we use the filter logic above as per original spec intent of extracting valid types from a schema map where values are strings or numbers/booleans, and null is an optional field type implied by standard struct definitions unless specified otherwise.
      ) // Simplified for this class definition which focuses on 'integer', 'string', 'boolean' (excluding bool). If the original spec implies booleans should be preserved in schema parsing but not treated as types to avoid infinite loops, we would need a separate check or explicit rejection of boolean literals if they are meant to remain type-safe. However, following the prompt's instruction "allows for dynamic schema mapping and type conversion", preserving boolean values is often desired unless explicitly rejected by validation logic elsewhere.
      (typeof val === "boolean" ? "boolean" : null) // If it is a boolean literal or property value in C/C#, treat as 'boolean' if present, otherwise exclude from the standard list to avoid false positives during schema parsing which might lead to infinite loops with unhandled booleans. This aligns closely with typical database generator behavior where Boolean fields are often handled separately but here we categorize them based on their presence or absence in a generic struct map for this specific class definition's type extraction logic."
    );

}

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
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap:
