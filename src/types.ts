src/types.ts | 485 lines
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
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing dynamic types per column
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types using regex and parsing logic
/**
 * Parse a single column value from the C-style `schemaMap` structure.
 * Returns either "string" or "integer" based on the parsed type, defaulting to null if invalid.
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = [];

  for (const [key, value] of Object.entries(schemaMap)) {
    // Extract column name and data type from C-style struct definition syntax `[...]` or `string|int|bool|null`
    if (!value || typeof value !== 'object') continue;

    let parsedType: "integer" | "boolean" = null as any;
    
    const regexMatch = /^(?:\d+|\.\d*)?(\s*([a-zA-Z_][a-zA-Z0-9_]*)|true|false)?$/i.test(value); // Matches integer, boolean, or string literals

    if (regexMatch) {
      parsedType = value as any; // Treat literal values like "123" or "true"/"false" directly
    } else if (!value && typeof value === 'object' && Object.keys(value).length > 0) {
      const keys: string[] = [];

      for (const [k, v] of Object.entries(value)) {
        // Skip non-numeric or boolean-like strings that might be intended as column names but are parsed incorrectly in a C-style context
        if (!Number.isInteger(Number(k) || k === "true" && !v.includes("0")) && typeof v !== 'string') continue;

        keys.push(`${k}: ${String(v).trim()}`); // Store key and value for further processing logic, but type is handled here as string/int
      }

      if (keys.length > 1) {
        parsedType = "integer"; // Assume integer when multiple numeric-like columns found in C-style context
      } else if (!parsedType && keys[0].includes(":")) {
        const colonCount = keys[0].indexOf(':');
        let colName: string | null = null;

        for (let i = 1; i < Math.min(colonCount + 2, keys.length); i++) {
          if (!colName) break; // Stop at first valid column name
          
          const valStr = keys[i].trim();
          let type: "integer" | "boolean";

          if (valStr.includes("true") || valStr === '1') type = "boolean";
          else if (valStr.includes("false")) type = "boolean";
          else { // Numeric or string literal found, default to integer for C-style compatibility unless explicitly boolean-like text is detected elsewhere in the schema map contextually handled by regex above
            parsedType = "integer"; 
          }

          colName = `${keys[i]}`;
        }

        if (colName) {
          result.push(colName); // Store key for type conversion logic, but return string/int based on value content directly here to match C-style expectations where column names are strings and values are types or numbers. If a numeric literal is found in the schema map as a "value" rather than just an index, it defaults to integer.
