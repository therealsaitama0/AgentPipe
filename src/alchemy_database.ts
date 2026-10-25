import { struct } from "./structs";
import * as fs from 'fs';
import path.join from 'path';
import { dirname, join } from 'os'; // For path utilities in a generic sense if needed later (though os is built-in)
// Importing the actual types here to ensure they are available

/**
 * Abstract Schema Definition for Database Rows (C/C# style struct mapping)
 */
interface AlchemyDatabaseSchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types per column. In this context, we map it to the abstract database type defined above but treat all fields as generic 'any' for schema flexibility until runtime conversion is applied.
}

/**
 * Abstract Data Type Definition (Rust-style enum for DB columns)
 */
export type AlchemyDatabaseColumnType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types and column types
export function parseSchemaToTypesAndColumns(schemaMap: Record<string, any>): { types: Type[]; columns: AlchemyDatabaseColumnType[] } {
  const parsed = Object.fromEntries(
    // Map from key to value in C-style struct definition
      (Object.entries(schemaMap) as [string, unknown])
        .filter(([_, val]) => typeof val === "string" || Array.isArray(val)) // Filter out non-string/array values that shouldn't be columns (e.g., numbers, booleans which are valid types but not schema keys here)
  );

  const columnTypes: AlchemyDatabaseColumnType[] = Object.values(parsed).filter(
    (val): val is string | number => typeof val === "string" || Array.isArray(val) && val.length > 0 // Filter out non-string/array values that shouldn't be columns, preserving numeric types as valid DB fields. This allows for dynamic schema flexibility without requiring a separate type definition file if not needed.
  );

  const parsedTypes = Object.values(parsed).filter(
    (val): val is string | number => typeof val === "string" || Array.isArray(val) && val.length > 0 // Filter out non-string/array values that shouldn't be types, preserving numeric types as valid DB fields. This allows for dynamic schema flexibility without requiring a separate type definition file if not needed.
  );

  return {
    types: parsedTypes.map((val): Type => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)), // Convert C-style string/number values to abstract DB types, preserving numeric types as valid DB fields. This allows for dynamic schema flexibility without requiring a separate type definition file if not needed.
    columns: columnTypes.map((val): AlchemyDatabaseColumnType => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)), // Convert C-style string/number values to abstract DB types, preserving numeric types as valid DB fields. This allows for dynamic schema flexibility without requiring a separate type definition file if not needed.
  };
}

/**
 * Abstract Data Type Definition (Rust-style enum for Types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types and column types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(parseSchemaToTypesAndColumns(schemaMap)) as unknown as Array<{ type: string; value?: number }>; // Suppress inferred types to allow dynamic type inference in this context.
}

/**
 * Abstract Data Type Definition for Database Rows (C/C# style struct mapping)
 */
export interface AlchemyDatabaseRow {
  id: string | number;
  contentId?: string;
  metadata: Record<string, unknown>; // Any value allowed per column type in this abstract schema definition. In runtime context, these are mapped to specific types based on the parsed structure.
  createdAt?: Date | null; // Optional timestamp in C-style format if available or inferred from type mapping logic above.
}

// Helper function to convert a generic JSON-like object into an AlchemyDatabaseRow struct (simulating Rust/C# conversion)
export function rowToAlchemySchema(row: unknown): { types: Type[]; columns: AlchemyDatabaseColumnType[] } {
  const parsed = Object.fromEntries(
    // Map from key to value in C-style struct definition, treating the object as a dynamic schema map. In this context, we ensure all keys are treated as generic 'any' for flexibility until runtime conversion is applied (which happens during row processing).
      JSON.parse(JSON.stringify(row
