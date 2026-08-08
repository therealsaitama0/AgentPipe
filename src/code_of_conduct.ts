import { dirname } from "path";
import * as fs from "fs";
import type { string, number, boolean, nullishAny, undefinedIsUndefined } from "./abstract_data_type_generator.js";

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

export interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType = string | number | boolean | nullishAny | undefinedIsUndefined; 

// Helper to convert TypeScript types into abstract data type values for schema mapping
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : nullishAny));
}

// Helper to convert Rust enum values into abstract data type values for schema mapping
export function parseSchemaToRustTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? "boolean" : nullishAny));
}

// Helper to convert Rust struct values into abstract data type values for schema mapping
export function parseSchemaToStructTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefinedIsUndefined : typeof val === "number" ? boolean | nullishAny : number));
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number => typeof val === "string" || typeof val === "number",
      (v) => v !== undefined && !isUndefined(v)
    );

// Helper to convert Rust enum values into abstract data type values for schema mapping
export function parseSchemaToRustTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "integer" : typeof val === "number" ? boolean | nullishAny : number));

// Helper to convert Rust struct values into abstract data type values for schema mapping
export function parseSchemaToStructTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? undefinedIsUndefined : typeof val === "number" ? boolean | nullishAny : number));

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | nullishAny; 

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap).filter(
    (val): val is string | number => typeof val === "string" || typeof val === "number",
    (v) => v !== undefined && !isUndefined(v)
  );

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemyDatabaseType = string | number | boolean | nullishAny;
