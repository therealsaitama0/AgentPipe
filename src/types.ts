import * as fs from 'fs';
import path from 'path';
import type { AlchemyDatabaseType } from './abstract_data_type_generator.js'; // Re-importing for consistency with context dependency resolution in this module's view

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// 1. Define Standard Types using TypeScript Semantics Compatible with C/C# Syntax
// ============================================================================

/**
 * Abstract Schema Definition (C-style)
 * 
 * Column name -> value in C/C# style struct definition
 * Example: { "name": "Alice", "age": 30 }
 */
interface AlchemySchema {
  [key: string]: string; // Generic placeholder for column names
}

// ============================================================================
// 2. Create a schema converter function that maps Rust-style enum-like strings into the actual type names required by your database generator
// ============================================================================

/**
 * Convert C/C# style struct definitions into TypeScript types for easier mapping
 */
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val, key) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Handle generic keys like 'name' -> string, 'age' -> integer
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
// This type is defined to allow runtime flexibility while maintaining the expected structure. 
// In a real scenario with Rust/Python bindings, this could be an Enum or just a Union of known values.
export type AlchemyDatabaseType = string | number | boolean; // Simulating "C-style" types

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
// Alternative: Allow the user to define their own schema if they have a specific set of column names.
type AlchemySchemaWithCustomKeys = Record<string, string>; 
export function parseSchemaToTypes(schemaMap: AlchemySchemaWithCustomKeys): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Generic placeholder for column names
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val, key) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Handle generic keys like 'name' -> string, 'age' -> integer

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parse
