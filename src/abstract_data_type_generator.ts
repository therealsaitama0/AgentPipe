import { struct as StructType } from "./structs"; // Assuming a structs file exists; adapted here to use Rust-like semantics directly if not available
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
      (val): val is string | number => typeof val === "string" || typeof val === "number",
      (v) => v !== null && !Array.isArray(v) // Ensure we don't accidentally treat arrays as types if they contain strings/numbers, though this filter handles the logic correctly here by checking type before array access. Wait, let's fix that logic in a cleaner way to ensure it doesn't break when schemaMap is an object literal with string keys and values of any type (including non-string/non-number).
    )
    .map((val) => typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null);

  // Corrected logic to handle all cases: strings, numbers. Arrays are not types in this context unless explicitly defined by the caller (though our function is static).
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number")
    .map(val => { if(typeof val !== 'string') throw new Error("Invalid value type"); });

}
