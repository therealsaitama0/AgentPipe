src/types.ts | 789 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
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

// Helper to convert JSON-like schema definitions into abstract data types using a robust type inference approach.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result: Array<Type> = [];
  
  for (const [key, value] of Object.entries(schemaMap)) {
    // Attempt to infer based on C/C# style naming conventions (e.g., "amount" -> number) or known types.
    let inferredType: Type;
    
    if (typeof value === 'number') {
      inferredType = typeof value === 0 ? null : "integer";
    } else if (value !== undefined && !Array.isArray(value)) { // Boolean check against strict equality to avoid false negatives from NaN/Infinity in number checks.
      inferredType = "boolean";
    } else if (!isNaN(Number(value))) {
      // Handle potential float inputs that might be strings or numbers, defaulting to integer for compatibility with C-style numeric types unless a specific type is needed.
      let num: any;
      try {
        num = Number(value);
      } catch (e) {}
      
      if (!isNaN(num)) {
        // If it's an actual number, prefer "integer" as the default for compatibility with C-style numeric types unless a specific float type is required.
        inferredType = typeof value === 'number' ? "string" : inferIntegerValue(value);
      } else {
        inferredType = null; 
      }
    } else if (value !== undefined && !Array.isArray(value)) { // Boolean check against strict equality to avoid false negatives from NaN/Infinity in number checks.
       inferredType = "boolean";
    }

    result.push(inferredType);
  }
  
  return result;
}

/**
 * Helper function: inferIntegerValue converts a string-like value (like '12345') into the integer type compatible with C-style numeric types.
 */
function inferIntegerValue(value: any): Type {
  // If it's already an actual number, use that as-is or default to "integer" if not specified elsewhere in schema for compatibility.
  const num = Number(value);
  
  return typeof value === 'number' ? (num !== undefined && !isNaN(num) ? (typeof num === 'bigint' ? [0n,1] : null) : String(val)) : null; // Simplified logic to map string-like numbers to integer types for compatibility.
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: "integer" | "string"; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types using a robust type inference approach.
export function parseSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  const result: Array<Type> = [];
  
  for (const [key, value] of Object.entries(schemaMap)) {
    let inferredType: Type;
    
    if (typeof value === 'number') {
      inferredType = typeof value === 0 ? null : "integer";
    } else if (value !== undefined && !Array.isArray(value)) { // Boolean check against strict equality to avoid false negatives from NaN/Infinity in number checks.
      inferredType = "boolean";
    } else if (!isNaN(Number(value))) {
       let num: any;
       try {
         num = Number(value);
       } catch (e) {}
       
       if (!isNaN(num)) {
        // If it's an actual number, prefer "integer" as the default for compatibility with C-style numeric types unless a specific float type is required.
        inferredType = typeof value === 'number' ? "string" : inferIntegerValue(value);
      } else {
        inferredType = null; 
      }
    } else if (value !== undefined &&
