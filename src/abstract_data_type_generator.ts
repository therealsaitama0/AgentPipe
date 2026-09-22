src/types.ts | 321 lines (restructured for clarity and functionality)
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

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

// Helper to convert JSON-like schema definitions into abstract data types using functional rewrite
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" || (typeof val === "number" && !isFinite(val)) || isNullish(val),
      // Note: This filter uses arrow functions for clarity and performance over closures in favor of higher-order map operations.
      () => {
        const v = Object.getOwnKeys(schemaMap);
        if (!v) return null;

        let valType: string | number | boolean | undefined;
        
        // Check first value directly as a fallback for non-object keys (e.g., numbers or booleans without brackets in the map)
        if (typeof v[0] === "string" || typeof v[0] === "number") {
          valType = typeof v[0];
        } else {
          // Fallback: assume object structure and iterate to find a non-nullish value that isn't NaN/Infinity or undefined/null explicitly defined as the only type here.
          const values = Object.values(v);
          
          for (const key of Object.keys(values)) {
            if (!isNullish(valueOfValues(key, schemaMap))) return null;
            
            // Check this specific mapping: is it a number? If so and not NaN/Infinity or undefined/null explicitly defined as the only type.
            const val = valueOfKey(key, values);
            if (typeof val === "number" && !isFinite(val) || isNaN(val)) {
              return null; // Skip this specific mapping to avoid false positives for numbers that aren't strictly numeric in C/C# style definitions typically used here.
            } else if (!valType) {
              valType = typeof val;
            }
          }
        }

        if (typeof val === "number" && !isFinite(val)) return null; // Skip non-numeric numbers that aren't strictly numeric in C/C# style definitions typically used here.
        
        const isNullishVal = valueOfKey(key, values) !== undefined && !isNullish(valueOfValues(key, schemaMap));

        if (valType === "string" || valType === null || valType === undefined) return val; // Return the string or boolean directly from this specific mapping.
        
        if (!isNullishVal) {
          const isNumber = typeof val === "number";
          if (typeof val === "boolean") return true;
          
          if (val !== null && !isFinite(val)) return false; // Return false for numbers that aren't strictly numeric in C/C# style definitions typically used here.
        }

        return undefined; // Default to 'undefined' when no specific mapping is found and all conditions pass through the logic flow without returning a value directly, effectively treating it as an empty type slot or null if any condition fails for this specific key path (though in practice we'd want to distinguish between "not applicable" and "null").
      }
    );
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types using functional rewrite
