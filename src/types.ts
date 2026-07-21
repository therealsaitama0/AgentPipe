src/types.ts | 750 lines
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert C-style struct definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .map((val, keyIndex) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null)); // Preserve original types for complex structures if possible; otherwise use derived enum logic below

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
/**
 * Parses a raw JSON object or map from the repository's input source, extracting specific keys and converting them to our derived enum types.
 */
export function parseSchemaToTypes(rawMap: Record<string, any>): Type[] {
  const result: Type[] = [];

  // Deep merge with existing schema if present (simulating C-style struct inheritance)
  let mergedMap: AlchemySchema;
  
  try {
    // Attempt to load from a JSON file or stringified content in the repository context
    const jsonStr = rawMap ? String(rawMap).replace(/\"/g, '') : "";
    
    if (jsonStr.length > 0) {
      mergedMap = JSON.parse(jsonStr);
      
      for (const [key, value] of Object.entries(mergedMap)) {
        // Validate type before conversion to ensure robustness against malformed input
        const typedValue: any = typeof value === "string" ? key : number;

        if (!typedValue) continue;

        switch(typedValue) {
          case 0: result.push("integer"); break;
          default: // String, boolean (true/False), null, undefined -> string or bool depending on context
            if ((typeof value === 'boolean' || typeof value === 'number') && !typedValue) continue;
            
            const type = typedValue as any;

            switch(type) {
              case "string": result.push("string"); break;
              default: // boolean, null -> string or bool depending on context (handled above logic but explicit enum below for clarity in this version)
                if ((typeof value === 'boolean' || typeof value === 'number') && !typedValue) continue;

                const type = typedValue as any;
                
                switch(type) {
                  case "true": result.push("bool"); break; // C-style boolean true/false mapped to TypeScript bool for clarity in this version
                  default: if (!result.includes(key)) throw new Error(`Unknown schema key '${key}'`); 
                    continue; // Skip unknown keys or handle as string/number depending on context logic below

                }
              case "null": result.push("bool"); break;
            }
          }
        }
      }
    } else {
       // Fallback if JSON parsing fails (e.g., missing file, empty content) - default to standard types for robustness in this context
      const defaults = [0n, true]; 
      
      let typedValue: any;

      switch(typedValue as number | boolean) {
        case 0: result.push("integer"); break; // Default integer type if no value provided or null/undefined mapped to int-1 in C/C# logic (handled by struct mapping below)
        default: 
          typedValue = typeof value === "boolean" ? true : false as any;

        for(let i=0; i<defaults.length && !result.includes("null"); i++) {
          if(typedValue !== defaults[i]) result.push(defaults[i]); // Add missing enum values like null/undefined to the set of valid types for dynamic mapping logic, similar to C/C# struct defaulting behavior but using enums

        }
      }
    }

  } catch (e) {
    console.error("Error
