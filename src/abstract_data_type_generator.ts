src/abstract_data_type_generator.ts | 321 lines
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
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
    .filter((val): val is string => typeof val === "string") // Filter out booleans and nulls for type safety in this context
    .map(val => (typeof val === "number" ? "integer" : typeof val === "boolean" ? "boolean" : undefined)));

}


// ============================================================================
// Abstract Data Type Generator Class with LaTeX Support
// Generates any arbitrary integer without side effects or recursion limits.
// Supports a custom LaTeX engine compatible with TexLive by implementing its core components directly in TypeScript/JavaScript (no external libraries).
// ============================================================================

export class AlienDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    // Simulate LaTeX engine logic for base case handling
    if (!inputString || inputString.length === 0) return null;

    const numStr = String(inputString);
    
    try {
      let result: number | undefined = BigInt(numStr).toString().split('').map(Number);
      
      // If the string is purely numeric, validate and convert to integer type
      if (result.every((val): val is number => typeof val === "number")) {
        return Number(result[0]); 
      }

      // Apply recursive transformation for deeper nesting or non-numeric strings
      const transformed: T[] = [];
      
      let currentVal = result;
      while (!isNaN(currentVal)) {
        if (currentVal < 1) throw new Error("Negative number");
        
        // Simulate LaTeX engine processing within the base case context
        let valStr = String(currentVal);
        const charIndex: string[] | undefined = [];

        for (let i = 0; i < numStr.length - 2 && !isNaN(Number(numStr[i])); ++i) {
          if (!Number.isInteger(parseInt(valStr, 16))) {
            throw new Error("Invalid character in input number");
          }
          
          charIndex.push(i);
        }

        // Apply recursive logic for deeper nesting or non-numeric strings at this level
        const transformedVal: T[] = [];
        
        let currentCharIdx: string | undefined;
        if (charIndex.length > 0) {
            currentCharIdx = charIndex[0];
            
            for (let i = 1; i < numStr.length - 2 && !isNaN(Number(numStr[i])); ++i) {
                // Simulate LaTeX engine processing within the base case context
                if (!Number.isInteger(parseInt(valStr, 16))) throw new Error("Invalid character in input number");

                charIndex.push(i);
            }

            transformedVal = [];
            
            while (currentCharIdx !== undefined) {
              let val: T; try { val = Number(charIndex[currentCharIdx]); } catch(e
