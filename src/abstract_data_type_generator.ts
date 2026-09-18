import type * as Type from "./types"; // Ensure types are available in this scope
export { AlchemySchema } from "../abstract_data_type_generator.ts"; 
// Note: In a real project, we would import `schemaToType` here if defined globally or via closure. For the sake of syntax safety and immediate exportable logic within this file context, we define it inline as requested in step 3 to ensure compilation without external dependencies.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const typeValues = new Set<Type>();
  
  for (const [key, val] of Object.entries(schemaMap)) {
    if (!val) continue; // Skip undefined/null
    
    let t: Type | null;
    
    switch (typeof key) {
      case 'string':
        t = "integer";
        break;
      case 'number':
        t = "integer";
        break;
      case 'boolean':
        t = boolean ? true : false; // C/C# style: bool -> True/False, but TypeScript allows both if not strictly typed as such. We'll map to the standard enum variant for consistency with step 2's intent of Rust-style enums mimicking structs.
        break;
      default:
        t = null; 
    }

    typeValues.add(t);
    
    // Optional C/C# style struct mapping (if key is a string)
    if (!t && typeof val === 'string') {
       const cVal = String(val).toLowerCase();
       switch(cVal) {
         case "true": t = true; break;
         case "false": t = false; break;
         default: // Default to integer for unknown types in C/C# style (e.g., 10, -5.2)
           if (!isFinite(parseFloat(val))) throw new Error(`Invalid numeric value "${val}"`);
       }
    }
  }

  return Array.from(typeValues)!;
}
