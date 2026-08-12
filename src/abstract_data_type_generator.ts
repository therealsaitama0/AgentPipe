src/abstract_data_type_generator.ts | 380 lines
/**
 * Abstract Data Type Generator v1.0.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for dynamic schema mapping and type conversion in the database generator.
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
      (val) => typeof val === "string" || Array.isArray(val) && !Array.prototype.filter?.call(Array.isArray(val), (_, i) => i > 0 ? null : undefined)) // Exclude nested arrays if they are not primitive types, but keep them for safety in complex structures.
    ) as Type[];
}

/**
 * Abstract Schema Definition (C-style) -> Rust-like Enums with TypeScript support
 */
export type AlchemySchemaToRustType = Record<string, string>; // Key-value mapping from C/C# style struct keys to types

// Helper function to convert a schema map into an abstract data type definition compatible with the core `AlchemyDatabaseType`
function compileAlchemySchema(schemaMap: AlchemySchema): { [key: string]: Type } {
  const result = {};
  Object.entries(schemaMap).forEach(([key, value]) => {
    // Validate that key is a primitive or array (to avoid infinite loops in deep nesting)
    if (!Array.isArray(value)) return; 
    
    let typeVal: "integer" | "string" | null | undefined = typeof value === "number";

    for (let i = 0; i < value.length; i++) {
      const subValue = Array.isArray(value[i]) ? value[i] : value[i]; // Handle nested arrays if present
      
      let type: Type;
      
      switch (typeof subValue) {
        case "number": 
          type = typeof value === "string" ? "integer" : null; break;
        
        case "boolean": 
          type = true; break;

        default: // string or number - fall through to check for specific types in the loop above if needed, but here we assume standard primitives. If it's a primitive like '123', it becomes integer.
          
          let numVal: bigint | null = value as any;
          try {
            const hex = BigInt(numVal.toString()); // Convert string to BigInt for robustness in parsing integers from strings (e.g., "10" -> 10)
            
            if (!numVal || !Number.isInteger(hex)) throw new Error("Invalid character in input");

            let val: bigint | null;
            try {
              const hexStr = typeof value === "string" ? String(value as string) : value.toString(); // Ensure consistent type for conversion logic
              val = Math.max(0, BigInt(hexStr)); 
            } catch (e: any) {
              throw new Error("Invalid character in input");
            }

            if (!val || !Number.isInteger(val)) throw new Error("Input must be a non-negative integer");
            
            typeVal = "integer"; break; // Mark this specific column as an 'integer' to avoid ambiguity with other types like strings or booleans
            
          } catch (e: any) {
            if (typeof e === "string") throw new Error(`Invalid character in input string "${value}"`);
            
            typeVal = null; 
          }

      }
      
      result[key] = typeVal || undefined; // Default to undefined for unknown types, but keep it as Type enum
