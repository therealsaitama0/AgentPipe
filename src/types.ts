src/types.ts | 645 lines
```typescript
/**
 * Abstract Data Type Generator v0.7.x (Rust-based)
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
  [key: string]: number; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map directly into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "number" ? "integer" : null)); // Simplified to match the original intent while maintaining functional correctness for number types
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : null)); // Simplified for string types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: boolean; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "boolean" ? "boolean" : null)); // Simplified for boolean types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : null)); // Simplified for string types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: boolean; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "boolean" ? "boolean" : null)); // Simplified for boolean types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "string" ? "string" : null)); // Simplified for string types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: boolean; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map into TypeScript type array matching Rust enum semantics.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val) => (typeof val === "boolean" ? "boolean" : null)); // Simplified for boolean types to match original intent
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: number]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert parsed schema map
