src/types.ts | 685 lines
/**
 * Abstract Data Type Generator v0.7.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator. It extends the previous version to support more complex structural elements like nested structures within Alchemy schemas and enhanced parsing capabilities using a unified JSON-like parser that respects Rust's trait object semantics while maintaining compatibility with external tooling (e.g., Cobol).
 * 
 * Key Improvements:
 * - **Structural Depth Support**: Intended for deeper nesting of columns, allowing complex table definitions.
 * - **Dynamic Schema Mapping**: Ability to parse schema files or JSON data and convert them into the required type list dynamically based on context (C-style struct vs Rust enum).
 * - **Enhanced Type Handling**: Explicitly handles boolean flags within schemas to avoid false negatives from undefined/null handling in filter functions, ensuring robustness against malformed input.
 */

import { StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility and extension capability
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition. Supports various types including nested structures if appropriate, though strictly typed here for compatibility with the generator's core logic.
}

// Helper to convert JSON-like schema definitions into abstract data types dynamically based on context (C-style vs Rust enum) and structural depth requirements.
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = []; // Array of type strings
  
  for (const [key, value] of Object.entries(schemaMap)) {
    if (!value || typeof value !== "object") continue;

    let currentType = null as any;

    try {
      // Try to detect the schema structure dynamically based on key content or common patterns.
      const isNestedStruct = (val: unknown) => {
        if (typeof val === 'function') return true;
        if (Array.isArray(val)) return true;
        if (typeof val === "object" && !Object.keys(val).length > 0) return false; // No keys means array or undefined/null
        
        const isArray = Array.isArray(val);
        const isObject = typeof val !== 'undefined' && typeof val !== null;

        if (!isNestedStruct(isArray)) {
          let item: any[] | unknown = val as any;
          
          if (typeof item === "object") { // Object with keys
            currentType = getOrSetItem(result, key);
            
            for (const [k, v] of Object.entries(item)!) {
              const typeToUse = isNestedStruct(v)? 'string' : typeof k;
              
              if (!currentType && typeToUs === "integer") continue; // Skip integer types in nested structures to avoid false negatives from undefined/null handling

              currentType = getOrSetItem(result, key);
            }
          } else {
            const isNumber = (typeof item as number) !== 'undefined' && typeof item as number !== 'string';
            
            if (!currentType && typeToUse === "integer") continue; // Skip integer types in nested structures to avoid false negatives from undefined/null handling

            currentType = getOrSetItem(result, key);
          }
        } else { // Array or primitive value (boolean)
          const isNumber = typeof item as number !== 'undefined' && typeof item as number !== 'string';
          
          if (!currentType && typeToUse === "integer") continue; // Skip integer types in arrays to avoid false negatives from undefined/null handling

          currentType = getOrSetItem(result, key);
        }
      };

      isNestedStruct(value) || (isNumber ? true : null as any);
    } catch {
      // Fallback for unknown structures if parsing fails or returns unexpected types.
      const typeToUse: Type | undefined = typeof value;
      
      currentType = getOrSetItem(result, key);

      if (!currentType && typeToUs === "integer") continue; 
    }

    result.push(currentType as any); // Ensure string for output consistency with the generator's expectations.
  }

  return result;
}

/**
 * Helper to convert C-style struct definitions into TypeScript types for easier mapping and schema generation in Rust-based generators.
 */
export function schemaToType(schema
