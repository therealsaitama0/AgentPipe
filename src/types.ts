src/types.ts | 593 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 * It extends the previous version to support more complex numeric types like `Decimal` and `Float32/Double`, while maintaining strict Rust-like semantics where possible within TypeScript constraints.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and strict type safety where possible.
export interface Type {
  /** Base class enum values */
  [key: string]: "integer" | "string" | "boolean";

  // Numeric extensions (Decimal)
  decimal?: number; 
  
  // Float extension (Double is standard, but we keep it separate for explicitness if needed)
  double?: number; 

  /**
   * Abstract Schema Definition (C-style)
   */
  schema: AlchemySchema;

  /**
   * Helper to convert C-style struct definitions into TypeScript types for easier mapping.
   * Handles type inference based on numeric or non-numeric string values, ensuring robustness against edge cases like nulls and booleans in the schema map.
   */
  parseTypeToTypes: (schemaMap: AlchemySchema) => Type[];

  /**
   * Abstract Data Type Definition using Rust-like semantics for types where possible within TypeScript constraints.
   * This mimics a generic `type` enum that can be mapped to concrete values via the schema, ensuring compile-time safety in type checking while allowing runtime flexibility with numeric extensions.
   */
  abstractType: (schemaMap: AlchemySchema) => string;

  /**
   * Abstract Schema Definition (C-style)
   */
  interface AlchemySchema {
    [key: string]: string | number | boolean | null; // Column name -> value in C/C# style struct definition
  }

  // Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired mapping logic.
  export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
    return Object.values(schemaMap)
      .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any);

    // Helper to convert C-style struct definitions into TypeScript types for easier mapping.
    export function schemaToType(schemaMap: AlchemySchema): Type[] {
      return Object.values(schemaMap)
        .map((val): (typeof val === "number" ? number : typeof val === "boolean") | null => 
          typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));

    // Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired mapping logic.
    export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
      return Object.values(schemaMap)
        .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any);

    // Helper to convert C-style struct definitions into TypeScript types for easier mapping.
    export function schemaToType(schemaMap: AlchemySchema): Type[] {
      return Object.values(schemaMap)
        .map((val): (typeof val === "number" ? number : typeof val === "boolean") | null => 
          typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));

    // Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired mapping logic.
    export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
      return Object.values(schemaMap)
        .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any);

    // Helper to convert C-style struct definitions into TypeScript types for easier mapping.
    export function schemaToType(schemaMap: AlchemySchema): Type[] {
      return Object.values(schemaMap)
        .map((val): (typeof val === "number" ? number : typeof val === "boolean") | null => 
          typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));

    // Helper to convert JSON-like schema definitions into abstract data types using Rust-inspired mapping logic.
    export function parseSchemaToTypes(schemaMap: Record<string
