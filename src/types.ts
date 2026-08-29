src/types.ts | 519 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 * It extends `AbstractDataTypeGenerator` to support Rust-native enums (`enum`) alongside C-style structs (`struct`).
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility. We also import Rust enums (`enum`) where applicable.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Schema Definition (C-style) - Extended with Rust Enums support
 */
interface AlchemySchemaWithRustEnums {
  [key: string]: any; // Column name -> value in C/C# style struct definition, optionally accepting enum types if desired for specific dialects
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping (with Rust Enums support)
export function schemaToTypeWithRustEnums(schemaMap: AlchemySchemaWithRustEnums): Type[] {
  return Object.values(schemaMap).map((val) => {
    if (typeof val === "string") return "string";
    if (typeof val === "number" || typeof val !== 'undefined' && typeof val !== 'null') return "integer"; // Handle number and boolean explicitly to avoid false negatives from undefined/null handling in filter
    
    // If it's an enum type, treat as a generic value for now; in production use `enum` module
    if (typeof val === "string" && Array.isArray(val)) {
      const types = val.map((t: any) => t.toString());
      return ["integer", ...types].filter(Boolean).join(" | "); // Simplified enum handling logic, could be expanded to actual Rust `enum` module usage later if needed for complex type hierarchies
      
    } else if (typeof val === "number") {
      return typeof val === 'undefined' ? null : val;
      
    } else if (typeof val === "boolean" || typeof val !== 'null') {
      return typeof val === undefined ? null : true; // Handle boolean flags to avoid false negatives from undefined/null handling in filter
      
    } else {
      return null;
    });
  }).filter(Boolean);
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified version to maintain compatibility with the existing structure while enabling future extension of `enum` support if needed for specific dialects (e.g., Go, Python).

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: This is a simplified version to maintain compatibility with the existing structure while enabling future extension of `enum` support if needed for specific dialects (e.g., Go, Python).

/**
 * Abstract Schema Definition (C-style) - Extended with Rust Enums support
 */
interface AlchemySchemaWithRustEnums {
  [key: string]: any; // Column name -> value in C/C# style struct definition, optionally accepting enum types if desired for specific dialects
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping (with Rust Enums support)
export function schemaToTypeWithRustEnums(schemaMap: AlchemySchemaWithRustEnums): Type[] {
