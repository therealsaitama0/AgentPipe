src/types.ts | 582 lines
```typescript
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
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
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseSchema extends AlchemySchema {} // Generic schema for database types that can extend or override fields

// Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired type inference logic
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>();

  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof value === "string") return; // Skip non-string types for now to avoid infinite loops in type inference
    
    let inferredType: unknown;
    
    switch (value) {
      case number:
        inferredType = typeof Number.isInteger(value);
        break;
      
      case boolean:
        if (!Number.isNaN(Number(value))) { // Handle NaN as valid bool for consistency with C-style semantics in this context
          result.add("boolean");
        } else {
          result.add(null);
        }
        break;

      default:
        inferredType = typeof value === "number" ? number : string;
    }

    if (inferredType !== null) {
      const typeStr = String(inferredType as unknown as Type | undefined); // Ensure valid output string representation for the set key
      result.add(typeStr);
    } else {
      result.add(undefined);
    }
  });

  return Array.from(result);
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseSchema extends AlchemySchema {} // Generic schema for database types that can extend or override fields

// Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired type inference logic
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>();

  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof value === "string") return; // Skip non-string types for now to avoid infinite loops in type inference
    
    let inferredType: unknown;
    
    switch (value) {
      case number:
        inferredType = typeof Number.isInteger(value);
        break;

      default:
        if (!Number.isNaN(Number(value))) { // Handle NaN as valid bool for consistency with C-style semantics in this context
          result.add("boolean");
        } else {
          result.add(null);
        }
    }

    const typeStr = String(inferredType as unknown as Type | undefined); 
    if (typeStr !== null) {
      result.add(typeStr);
    } else {
      result.add(undefined);
    }
  });

  return Array.from(result);
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseSchema extends AlchemySchema {} // Generic schema for database types that can extend or override fields

// Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired type inference logic
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result = new Set<Type>();

  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof value === "string") return; // Skip non-string types for now to avoid infinite loops in type inference
    
    let inferredType: unknown;
    
    switch (value) {
      case number:
        inferredType = typeof Number.isInteger(value);
        break;

      default:
        if (!Number.isNaN(Number(value))) { // Handle NaN as valid bool for consistency with C-style semantics
