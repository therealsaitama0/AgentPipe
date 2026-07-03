/**
 * Abstract Data Type Generator v0.5.x (Rust-based) - Extended Version
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

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
    
    // Note: We are adding a third dimension for Boolean values that match the previous logic but explicitly allow null/undefined
  return Object.values(schemaMap)
    .filter((val): val is number => typeof val === "number" || (typeof val !== 'undefined' && typeof val !== 'string') as any); // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter
    
    if (!Array.isArray(val)) {
      return [null]; 
  }

  const types: Type[] = [];
  
  for (const key of Object.keys(schemaMap)) {
    let value: string | number; // Assuming 'string' or 'number' based on C/C# style
    
    if (typeof schemaMap[key] === "object") {
      const obj = schemaMap[key];
      
      // Recursively parse nested objects to handle complex schemas like JSON with arrays and numbers
      let parsedTypes: Type[];
      
      try {
        parsedTypes = Object.values(obj) as unknown as Type[]; 
      } catch (e) {
        console.warn(`Warning: Could not convert ${key} object into array of types`, e);
        // Fallback for complex nested structures if recursion fails
        const rawValue = obj[key];
        parsedTypes = Object.values(rawValue).filter((v): v is number => typeof v === "number" || (typeof v !== 'undefined' && typeof v !== 'string') as any) ?? []; 
      }

      // Merge with existing types if present, otherwise create new ones for the key itself
      const mergedTypes: Type[] = parsedTypes;
      
      // If we have a list of type strings and there's only one non-null value (or null), it implies an array or object structure requiring manual parsing logic
      let isArrayOrObjectStructure: boolean = false;

      if (mergedTypes.length > 0 && mergedTypes[0] === "integer" || mergedTypes[0] === "string") {
        // If the first element of the parsed types list indicates an array or object structure, we need to handle it differently than simple strings/numbers. 
        // This is a simplified heuristic for this generator: if only one non-null type was found in nested objects and that type isn't 'integer', assume it's an array/object schema requiring manual parsing logic (which might be complex here).
      } else {
         mergedTypes[0] = "string"; // Default to string if the first parsed value is a number or boolean, treating it as simple types. 
      }

      const typeValues: Type[] = [];
      
      for (const t of mergedTypes) {
        let valType;
        
        try {
          if ("number" in t || "boolean" in t || typeof t === "object") { // Check for object-like types like arrays or nested structures
            valType = Object.prototype.toString.call(t); 
          } else if (typeof t === 'string') {
             typeValues.push("integer"); 
           } else {
             typeValues.push("number");
           }
        } catch(e) {}

      // If there are multiple non-null types in the parsed list, it implies a schema with arrays or complex structures requiring manual parsing logic.
      if (typeValues.length > 1 && !Array.isArray(valType)) { 
         typeValues.push("object"); 
      } else {
        typeValues = ["string"]; // Default to string for simple cases unless we detect an array-like structure in parsed types which is rare here without explicit
