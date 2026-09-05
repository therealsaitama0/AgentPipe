src/types.ts | 321 lines (expanded and refined)
/**
 * Abstract Data Type Generator v0.5.x (Rust-based extension for C/C# compatibility with TypeScript types)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and robustness.

export type Type = "integer" | "string" | "boolean" | null | undefined; // Standard JavaScript/TypeScript types defined here to ensure cross-language interoperability (Rust enums often default to 'null')
// Note: In Rust/C#, `bool` is typically a distinct enum variant, but for this generator we map it as the boolean type directly via string `"boolean"` or explicitly handle nulls.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: Type; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping and type safety checks
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : typeof val === "number" && !isNaN(val)) || false // Handle nulls explicitly to avoid type mismatch issues; use strict equality checks for native types.
    ).filter(Boolean); // Ensure array is not empty and non-empty values are kept, otherwise return [] or undefined if any value fails the check (e.g., NaN/undefined)

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context. Note: In Rust `bool` is an enum variant with a constant value of true/false, but we map it here as the "boolean" type for compatibility and explicit handling.
// Alternative interpretation: If you truly want to reflect C/C# where bool is usually separate from int/float/null, consider this mapping more closely or adjust based on your target language's actual semantics (e.g., in Rust `bool` vs TypeScript boolean). For now, we keep "boolean" as the string type for cross-language compatibility.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter. Use `isFinite` for NaN/Infinity if needed, or strictly check string vs non-number types here.
    .map((val) => {
      const nativeType = (typeof val === "number" && !isNaN(val)) || typeof val === "boolean"; // Check numeric type and boolean separately to distinguish them as distinct values in the schema
      return nativeType ? `string` : null; // Map number/bool strings back to their semantic types for consistency with C/C# syntax.
    });
}

/**
 * Abstract Schema Definition (C-style) - Enhanced version supporting dynamic type inference and explicit value mapping
 */
interface AlchemySchemaEnhanced {
  [key: string]: Type; // Column name -> value in C/C# style struct definition, with enhanced type validation for robustness.
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping and type safety checks (including nulls)
export function schemaToTypeEnhanced(schemaMap: AlchemySchemaEnhanced): Type[] {
  return Object.values(schemaMap)
    .map((val, key) => { // Map each value keyed by column name to ensure uniqueness in the resulting array. Use `key` as a safe fallback if not present for dynamic generation but prioritize keys here for stability.
      const type = (typeof val === "string" ? "string" : typeof val === "number") || false; // Determine native semantic types based on value type and nullability. If the check fails, assume it's an invalid schema entry or use a default fallback if needed. For robustness with Rust enums which often default to 'null', we keep this explicit mapping but ensure no unexpected values slip through without proper validation in production code.
      return `string` as Type; // Map string value back
