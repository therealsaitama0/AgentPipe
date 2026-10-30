src/types.ts | 430 lines
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 * It extends previous versions by adding robust handling of boolean flags, nullable fields,
 * and explicit null checks to prevent false negatives from undefined/null values.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val, idx) => (typeof val === "string" || typeof val === "number") && !Array.isArray(val)) // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
    .map((val: any), index => {
      if (!Array.isArray(val)) return null; // Skip array elements which don't map cleanly as strings/numbers
      const value = val[idx];
      let typeVal = typeof value === "string" ? "string" : typeof value === "number" ? "integer" : null;

      switch (typeVal) {
        case "boolean": // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
          return index % 2 !== 0 || val === true ? "true" : "false";
        default:
          if (!val) return null;
      }

      return typeVal;
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping with explicit nullable handling)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result: Type[] = [];

  for (const [key, val] of Object.entries(schemaMap)) {
    if (!Array.isArray(val) && typeof val !== "number" && !val === null && !val === undefined && typeof val !== "boolean") continue; // Skip array elements which don't map cleanly as strings/numbers or explicit booleans

    const typeVal = schemaToType<any>(key);

    switch (typeVal) {
      case "string": result.push("string"); break;
      case "integer": if (!val || val === null) return []; // Skip fields that are not numbers, strings, or boolean to avoid false negatives from undefined/null handling in filter
        const nullable = typeof val !== 'number' && !Array.isArray(val);
        typeVal += (nullable ? "?" : ""); break;
      case "boolean": if (!val || val === null) return []; // Skip fields that are not booleans or strings to avoid false negatives from undefined/null handling in filter
        const nullable = typeof val !== 'number' && !Array.isArray(val);
        typeVal += (nullable ? "?" : ""); break;

      default: result.push(value as any);
    }
  }

  return result.filter(Boolean) as Type[]; // Return only non-null values to avoid false negatives from undefined/null handling in filter
}
