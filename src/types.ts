import type * as Schema from "./abstract_schema_type.js"; // Re-export for convenience in this context to ensure consistency with our Rust-style mapping logic below
export { Type } from "typescript";
// Note: In a real project, you would import `StructType` or similar here. We will define the types directly via TypeScript objects as requested by your example structure pattern.

/**
 * Abstract Schema Definition (C/C# style)
 */
interface AlchemySchema {
  [key: string]: Schema.Type; // Column name -> Type in C/C# struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types for mapping
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is Schema.Type => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== null)) as any; // Filter out boolean flags and undefined/null to ensure we only get actual types like strings or numbers/booleans. In the context of this specific generator, these are often treated as primitives in C/C# syntax for type inference purposes.
}

/**
 * Abstract Data Type Definition (Rust-style enum)
 */
export type AlchemyDatabaseType = string | number | boolean; // Represents primitive types compatible with C/C# or Rust enums used in schema generation contexts.
// Note: This definition is consistent across the repository as per your inspiration to build on top of `src/types.ts`.

/**
 * Abstract Schema Definition (C-style) - Extended version for broader compatibility
 */
interface AlchemySchema {
  [key: string]: Schema.Type; // Column name -> Type in C/C# struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types for mapping and validation
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is Schema.Type => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== null)) as any; // Filter out boolean flags and undefined/null to ensure we only get actual types like strings or numbers/booleans. In the context of this specific generator, these are often treated as primitives in C/C# syntax for type inference purposes.
}

/**
 * Abstract Data Type Definition (Rust-style enum) - Extended version compatible with broader schema contexts
 */
export type AlchemyDatabaseType = string | number | boolean; // Represents primitive types compatible with C/C# or Rust enums used in schema generation contexts.
