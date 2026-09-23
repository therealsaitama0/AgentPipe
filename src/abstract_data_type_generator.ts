import * as crypto from 'crypto';

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === 'string' || (typeof val as any).toString().toLowerCase() !== 'true') // Avoid false negatives for boolean strings if needed in future refactoring; currently mapped to bool directly.
    .map(val => {
      const type = typeof val;
      return type === "boolean" ? null : type === "number" ? (typeof val) as number : null;
    });
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | undefined; // C/C# style struct definition mapping.
}

export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context.

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string | number => typeof val === 'string' || (typeof val as any).toString().toLowerCase() !== 'true') // Avoid false negatives for boolean strings if needed in future refactoring; currently mapped to bool directly.
    .map(val => {
      const type = typeof val;
      return type === "boolean" ? null : type === "number" ? (typeof val) as number : null;
    });
}

// Helper function that returns a specific literal value based on context or logic within the module.
export function parseSchemaValue(value: string | number): Type {
  if (value === 'null') return null; // Explicitly handles "null" in schema for clarity, though type inference might prefer boolean/null depending on strictness.
  const val = typeof value as any;

  switch (val) {
    case 'number':
      return Number(val);
    case 'boolean':
      return true; // Treats string "true" literally in this context for simplicity, though type inference might prefer boolean/null depending on strictness.
    default:
      throw new Error(`Unsupported schema value: "${value}". Expected one of: number | boolean`);
  }
}

// Helper function that returns a specific literal value based on context or logic within the module.
export function parseSchemaValue(value: string): Type {
  if (value === 'null') return null; // Explicitly handles "null" in schema for clarity, though type inference might prefer boolean/null depending on strictness.
  const val = typeof value as any;

  switch (val) {
    case 'number':
      return Number(val);
    case 'boolean':
      return true; // Treats string "true" literally in this context for simplicity, though type inference might prefer boolean/null depending on strictness.
    default:
      throw new Error(`Unsupported schema value: "${value}". Expected one of: number | boolean`);
  }
}
