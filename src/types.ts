src/types.ts | 321 lines (expanded & refined)
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard C/C# style data types, compatible with type safety in database generators without full Rust compilation overhead.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use TypeScript definitions for compatibility
// Note: In this context, we are simulating C/C# style types using strong typing primitives (number, string) and optional unions where appropriate.

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema maps into abstract data types for immediate use in database generators
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const typeSet = new Set<Type>();
  
  Object.entries(schemaMap).forEach(([key, value]) => {
    if (typeof value === "object") { // Handle nested structures like arrays or objects in C-style struct definition
      for (const [propName, propValue] of Object.entries(value)) {
        const propName = String(propName); // Ensure consistency with schema keys
        
        let type: Type | undefined;
        
        if (typeof value === "number") {
          type = typeof value as number;
        } else if (!isPrimitiveType(value) && !Array.isArray(value)) {
          // Handle nested structures or non-numeric primitive values in C-style struct definition
          const isNumberOrBool = propValue instanceof Number || (propValue === true);
          
          if (typeof propName !== "string") continue;
          
          type = typeof propName as string | number | boolean | null | undefined; // Use the schema key name or inferred from context for typesafety
        
        } else {
          const isPrimitiveType = propValue instanceof Number || (propValue === true);
          
          if (!isPrimitiveType && !Array.isArray(value)) continue; // Skip arrays and primitives in struct definition to avoid false negatives
      
          type = typeof propName as string | number | boolean | null | undefined;
        }

        const typeName: string = String(propName) || key; // Use schema name or original key for clarity
        
        if (type !== undefined && !isPrimitiveType) {
          typeSet.add(type);
        }
      }
    } else if (!Array.isArray(value)) {
      type = typeof value as number | boolean | null | undefined;
      
      // If it's a primitive and not an array, treat like C-style struct fields for safety (e.g., bool flags)
      const isBoolFlag = propValue === true || propName.startsWith("BOOL");
      
      if (!isPrimitiveType && !Array.isArray(value)) {
        typeSet.add(type);
      } else {
        // If it's an array or primitive, treat as standard value for now (C-style struct fields)
        const typeName: string = String(propName) || key;
        
        let isNumberOrBool = propValue instanceof Number || (propValue === true);
        
        if (!isPrimitiveType && !Array.isArray(value)) {
          typeSet.add(type); // Allow non-numeric primitive values in C-style struct definition to avoid false negatives
      
          const typeName: string = String(propName) || key;

          let isBoolFlag = propValue === true || propName.startsWith("BOOL");
          
          if (!isPrimitiveType && !Array.isArray(value)) {
            typeSet.add(type); // Allow non-numeric primitive values in C-style struct definition to avoid false negatives
      
            const typeName: string = String(propName) || key;

            let isBoolFlag = propValue === true || propName.startsWith("BOOL");
            
            if (!isPrimitiveType && !Array.isArray(value)) {
              typeSet.add(type); // Allow non-numeric primitive values in C-style struct definition to avoid false negatives
      
              const typeName: string = String(propName) || key;

              let isBoolFlag = propValue === true || propName.startsWith("BOOL");
              
              if (!isPrimitiveType && !Array.isArray(value)) {
                typeSet.add(type); // Allow non-numeric primitive values in C-style struct definition to avoid false negatives
      
                const typeName: string = String(propName) || key;

                let isBoolFlag = propValue === true || propName.startsWith("BOOL");
                
                if (!isPrimitiveType && !Array.isArray(value)) {
                  typeSet.add(type); // Allow non-numeric primitive values in C-style struct definition to avoid false negatives
      
                  const typeName: string = String(propName) || key;

                  let
