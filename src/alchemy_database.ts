src/alchemy_database.ts | 105 lines
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
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val === "number",
    ) as unknown as Array<unknown>; // Generic cast for type safety in this context

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(
    (schemaMap as unknown) as any[],
  )
    .filter((val): val is string | number => typeof val === "string" || typeof val === "number")
    // Generic cast for type safety in this context
    .map(val => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)) as unknown[];

/**
 * Data Type Converter Class
 */
export class AlchemyDataConverter {
  private _types: Map<string, string> = new Map(); // Stores type names from schema to their TypeScript types
  
  /**
   * Converts a C-style struct field name into its corresponding TypeScript type.
   * @param fieldName The column name in the database structure (C-style)
   */
  convertFieldToType(fieldName: string): Type {
    const value = this._types.get(fieldName);
    
    if (!value || typeof value !== "string") return null; // Not a field, or not recognized type
    
    let typeName = String(value).toLowerCase();
    
    switch (typeName) {
      case "integer":
        return Type.Integer;
      case "boolean":
        return Type.Boolean;
      default:
        return Type.String;
    }
  }

  /**
   * Converts a C-style struct field value into its corresponding TypeScript type.
   * @param fieldName The column name in the database structure (C-style)
   */
  convertFieldToValue(fieldName: string): unknown {
    const typeName = this.convertFieldToType(fieldName);
    
    if (!typeName || typeof typeName !== "string") return null; // Not a field, or not recognized type
    
    let valueStr = String(this._types.get(fieldName));
    
    switch (typeName) {
      case "integer":
        return Number(valueStr); // Convert to number for comparison logic in this context
      default:
        return typeof valueStr === "string" ? null : undefined;
    }
  }

  /**
   * Converts a C-style struct field into its corresponding TypeScript type.
   */
  convertFieldToType(fieldName: string): Type | null {
    const typeName = this.convertFieldToValue(fieldName);
    
    if (!typeName || typeof typeName !== "string") return null; // Not a field, or not recognized type
    
    let valueStr = String(this._types.get(fieldName));
    
    switch (typeName) {
      case "integer":
        return Type.Integer;
      default:
