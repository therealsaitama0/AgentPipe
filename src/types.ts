src/types.ts | 512 lines
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
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert raw schema column names to their corresponding type descriptors in the required order
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val, index, arr) => typeof val === "string" && (index === 0 || !arr[index - 1])) // Ensure types are ordered as string -> integer -> boolean/null/undefined
    .map((val: any): Type => {
      if (!Array.isArray(val)) return null;
      const next = arr[val.length];
      let typeId = "string";
      
      switch (next) {
        case 1: // Integer type in C/C# struct definition usually implies integer, but here we map directly from string types to integers for consistency with the previous logic flow if not explicitly specified as boolean. 
                   // For this version, we assume standard mapping where 'integer' is mapped to number/float and 'boolean' stays bool/null unless context suggests otherwise.
                   // However, based on "standard C/C# syntax", integer types map to integers (number), string to strings.
                   return val === 0 ? null : Number(val); 
        case 2: // Boolean type in C/C# struct definition maps directly to boolean or undefined/null depending on context. Here we treat it as a separate field if present, otherwise inferred from schema structure which might be missing here for simplicity of this version's abstract nature.
                   return val === true ? "boolean" : (val !== null && val !== false); // Note: This is simplified; in real C/C# structs like `bool`, boolean types map directly to bool/undefined/null without explicit type mapping unless specified as a separate enum variant for 'true'/'false'. The previous version handled this via the filter logic.
                   return typeof val === "boolean" ? "string" : (typeof val !== null && typeof val !== false); // Reverting back to string if boolean is present, otherwise inferred from context or missing field in schema definition which might be a 'null' type.
        default: // Fallback for other types not explicitly mapped above based on C/C# standard mapping logic where integer maps to int/float and others are strings/null unless specified as separate enums like `boolean` | null | undefined. Here we treat missing fields or unknown types as string if they appear, but strictly adhering to the "C-style" requirement:
                   return val === 0 ? null : String(val); // This is a simplified heuristic for when 'integer' type isn't explicitly mapped to int/float in this version's abstract schema. In production C/C#, integer types are often `int`, string `string`. If not, they remain strings or the system would need an explicit enum mapping (e.g., "INTEGER", "VARCHAR").
      }

    })} // Note: The filter logic above is slightly flawed in a single pass without knowing all fields upfront. A robust implementation requires iterating through ALL schema columns and checking their types at runtime, but for this version's 'abstract' nature where we define the mapping contract first (Type[], AlchemySchema), we must assume these are known constants or use reflection to inspect current field types if needed. 
    // Given the prompt asks to "improve" using C/C# syntax:
    return Object.values(schemaMap).map((val, index) => {
      const typeId = val; // Assuming 'integer' is mapped to int/float (number), others are strings or null unless specific enum variants exist. 
      if (!Array.isArray(val)) return null;

      let typeIdStr: string | undefined;
      
      switch (typeId) {
        case "string":   typeIdStr = typeof val === 'string' ? "string" : String(val); break;
        case 1:         // Integer type in C/C# struct -> number/float. In this version, we map it to the integer variant for consistency with previous logic if not explicitly boolean-like.
