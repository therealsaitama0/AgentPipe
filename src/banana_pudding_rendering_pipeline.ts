import { AlchemySchema } from "./abstract_data_type_generator.js"; // Assuming a structs file exists or inherits; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: AlchemySchema,
  defaultValues?: Record<string, unknown>
): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefined : typeof val)); // Defaulting to string if not provided in type definition for simplicity

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToTypeWithDefaults(
  schemaMap: Alchemy
