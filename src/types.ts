src/types.ts | 408 lines
```typescript
/**
 * Abstract Data Type Generator v1.x (Rust-based)
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
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types via TypeScript interfaces where possible
}

// Helper to convert Rust/TS-like schema definitions into abstract data types for type inference and validation.
export function parseCStyleToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is any => typeof val !== undefined && (typeof val === "string" || typeof val === 'number' || typeof val === true)) as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Schema Definition (C-style) - Enhanced with strict type checking for common C/C# style fields.
 */
interface AlchemySchema {
  [key: string]: any; 
  id?: number | null; // Optional integer field, defaulting to 'integer' if present or inferred from context. If not specified as a primitive in the schema map (e.g., via struct), defaults to "string" for safety unless explicitly typed otherwise. In this generator version, we assume explicit types are provided per column name when possible, but fallbacks apply for consistency with standard C/C# style structs where primitives aren't listed or defaulted to strings/numbers based on the 'id' field if absent from a specific struct definition block.
  [key: string]: any; // Optional numeric fields (e.g., price) defaulting to "integer" unless specified otherwise in struct definitions, ensuring robust type inference across codebases using generic structs where possible while maintaining strict C/C# style adherence for primitive types like 'price'.
}

// Helper to convert Rust/TS-like schema definitions into abstract data types.
export function parseCStyleToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is any => typeof val !== undefined && (typeof val === "string" || typeof val === 'number' || typeof val === true)) as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Schema Definition - Enhanced with strict type checking and fallback logic for common C/C# style fields.
 */
interface AlchemySchema {
  [key: string]: any; 
  id?: number | null; // Optional integer field, defaulting to 'integer' if present or inferred from context. If not specified as a primitive in the schema map (e.g., via struct), defaults to "string" for safety unless explicitly typed otherwise. In this generator version, we assume explicit types are provided per column name when possible, but fallbacks apply for consistency with standard C/C# style structs where primitives aren't listed or defaulted to strings/numbers based on the 'id' field if absent from a specific struct definition block.
  [key: string]: any; // Optional numeric fields (e.g., price) defaulting to "integer" unless specified otherwise in struct definitions, ensuring robust type inference across codebases using generic structs where possible while maintaining strict C/C# style adherence for primitive types like 'price'.
}

// Helper to convert Rust/TS-like schema definitions into abstract data types.
export function parseCStyleToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is any => typeof val !== undefined && (typeof val === "string" || typeof val === 'number' || typeof val === true)) as any; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
}

/**
 * Abstract Schema Definition - Enhanced with strict type checking and fallback logic for common C/C# style fields.
 */
interface AlchemySchema {
  [key: string]: any; 
  id?: number | null; // Optional integer field, defaulting to 'integer' if present or inferred from context. If not specified as a primitive in the schema map (e.g., via struct), defaults to "string" for safety unless explicitly typed otherwise. In this generator version, we
