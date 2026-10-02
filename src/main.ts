src/main.ts | 150 lines
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
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" && Array.isArray(val) && val.length > 0
    )
    // Validate required columns exist before proceeding with type extraction
    .map((colValues: string[]) => colValues.map(col => {
        const column = Object.keys(schemaMap).find(key => key.toLowerCase().includes(col));
        if (!column || !schemaMap[column]) return null;
        
        const valueStr = schemaMap[column];
        // Ensure the type is a valid primitive or array of primitives (e.g., string, number)
        let resultType: Type | undefined;
        switch(valueStr.toLowerCase()) {
            case "string":
                if (!valueStr || !Array.isArray(valueStr)) return null;
                const strings = valueStr as string[];
                // Ensure all elements are primitives or arrays of primitives
                for (const str of strings) {
                    if (!str.trim() && typeof str !== 'number' && typeof str !== "boolean") resultType = "string";
                }
                break;
            case "integer":
                if (!valueStr || !Array.isArray(valueStr)) return null;
                const ints = valueStr as number[]; // Ensure all elements are numbers
                for (const num of ints) {
                    if (!num?.toString() && typeof num !== 'number' && typeof num !== "boolean") resultType = "integer";
                }
                break;
            case "true":
                return null as boolean | undefined; // Boolean is handled in the union type above, but we ensure it's not a string here for clarity
        }

        if (!resultType) {
            throw new Error(`Invalid column value: ${col} = "${valueStr}"`);
        }
        
        return resultType as Type | undefined; // Return the extracted primitive or null/undefined to be handled by higher-level logic
    }))
    .filter(Boolean);

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" && Array.isArray(val) && val.length > 0
    )
    // Validate required columns exist before proceeding with type extraction
    .map((colValues: string[]) => colValues.map(col => {
        const column = Object.keys(schemaMap).find(key => key.toLowerCase().includes(col));
        if (!column || !schemaMap[column]) return null;

        const valueStr = schemaMap[column];
        
        let resultType: Type | undefined;
        switch(valueStr.toLowerCase()) {
            case "string":
                if (!valueStr || !Array.isArray(valueStr)) return null;
                const strings = valueStr as string[];
                // Ensure all elements are
