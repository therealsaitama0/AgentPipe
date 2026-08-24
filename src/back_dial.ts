import { AlchemyDatabaseType } from "./abstract_data_type_generator.js"; // Ensure backward compatibility with existing imports or direct export if needed; adapted to use TypeScript types for seamless integration in generator logic.

/**
 * Abstract Data Type Definition (Rust-style enum for data, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || val !== null && Array.isArray(val)
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: any): val is AlchemyDatabaseType => 
        typeof val === "string" || Array.isArray(val) && val.length > 0
    ) as unknown as Set<AlchemyDatabaseType>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert
