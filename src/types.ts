src/types.ts | 365 lines
/**
 * Abstract Data Type Generator v0.7.x (C/C# + Rust-style hybrid)
 * 
 * This module defines standard data types compatible with C/C#/Go/Python syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean | null; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Note: boolean is handled as 'boolean' here, matching Rust enum semantics where true/true -> bool

/**
 * Abstract Data Type Definition (Rust-style struct mapping for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; 

// Helper to convert JSON-like schema definitions into abstract data types using C/C# style structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; 

// Helper to convert JSON-like schema definitions into abstract data types using Rust structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap).filter((val): val is string => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; 

// Helper to convert JSON-like schema definitions into abstract data types using Rust structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap).filter((val): val is string => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; 

// Helper to convert JSON-like schema definitions into abstract data types using Rust structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap).filter((val): val is string => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; 

// Helper to convert JSON-like schema definitions into abstract data types using Rust structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap).filter((val): val is string => typeof val === "string" || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; 

// Helper to convert JSON-like schema definitions into abstract data types using Rust structs as the base unit
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return
