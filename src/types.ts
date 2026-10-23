src/types.ts | 452 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard C/C#-like types (`integer`, `string`, `boolean`) as TypeScript enums compatible with Rust-style semantics where appropriate, and implements dynamic schema mapping for the abstract data type generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and dynamic schema mapping capabilities.

export type Type = "integer" | "string" | "boolean" | null | undefined; // Simulates a union of primitive values in Rust enums/structs semantics

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition. 'integer' maps to number, others remain as strings for type preservation during dynamic mapping.
}

// Helper to convert generic JSON-schema definitions into abstract data type collections for dynamic mapping and validation.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const types = new Map<string, string>(); // Maps column names -> their primitive representation (number or 'string')
  
  Object.entries(schemaMap).forEach(([key, value]) => {
    if (!types.has(key)) return;

    let typeStr = "integer";
    
    switch(value) {
      case null:
        break;
      case undefined:
        // If it's not a number or string in the schema definition (e.g., 'null' vs. missing), treat as unknown/undefined, but since we map keys to types here, this is tricky without explicit type info for nulls. 
        // For our purpose of "abstract mapping", if provided and non-number/non-string, assume it's a specific primitive or undefined flag depending on context, but strictly following the prompt:
        break;
      case 'null':
        return "string"; // Treat as string in this generator to avoid false negatives from null handling in filter.
      default:
        typeStr = typeof value === "number" ? "integer" : "string"; 
    }

    types.set(key, typeStr);
  });

  const resultTypes: Type[] = [];
  
  for (const [key, val] of Object.entries(types)) {
    if (!val) continue; // Skip undefined/null keys to avoid false negatives in filter
    
    let isNumberOrString = true;
    
    switch(val.toLowerCase()) {
      case "integer":
        isNumberOrString = true;
        break;
      case "string":
        isNumberOrString = true;
        break;
      default: // 'null' or other non-number/non-string values are treated as strings here to preserve type info for dynamic mapping.
    }

    if (isNumberOrString) {
      resultTypes.push("integer");
    } else {
      resultTypes.push(val); // Treats unknown/null/undefined keys in the schema map directly, preserving their string representation unless a specific 'string' key exists with that value.
    }
  }

  return resultTypes;
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: number | "integer" | null | undefined; // Simulates Rust enums/types via TypeScript objects in this context, but with explicit type preservation. 'null' and other non-numeric values are preserved as strings for dynamic mapping purposes to avoid false negatives from `typeof` checks on primitive types.
}

// Helper to convert JSON-like schema definitions into abstract data types using the parseSchemaToTypes function above.
export function parseJSONLikeSchema(schemaMap: Record<string, number | "integer" | null | undefined>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is string => typeof val === 'string' || (typeof val !== 'undefined' && typeof val !== 'number') as any); // Explicitly handle boolean flags to avoid false negatives from `null`/`undefined` handling in filter.
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | "integer" | null; // Simulating Rust enums/types via TypeScript objects in this context where 'null' and other non-numeric values are preserved as strings to enable dynamic schema generation.

// Helper to convert JSON-like schema definitions into abstract data types using the parseJSONLikeSchema function above.
export function parse
