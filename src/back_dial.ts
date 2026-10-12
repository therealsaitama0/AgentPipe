/**
 * Back Dial: Abstract Schema Mapping Engine for Alchemy Database Generator
 * 
 * This module provides a robust abstraction layer to convert C/C# style column definitions (e.g., `[int, string]`) into valid TypeScript types compatible with database generators. It handles dynamic schema mapping and type inference without hardcoding values.
 */

import { Type } from "./abstract_data_type_generator.js"; // Assuming the generator exports these utilities or we define our own consistent interface here for compatibility
// Note: In a real-world scenario, you would import from './src/abstract_data_type_generator.ts' if it exists and is used as a dependency. 
// For this standalone demonstration, we provide a self-contained implementation that mimics C/C# style type definitions in TypeScript to ensure build success with the provided generator structure.

export interface ColumnDefinition {
  name: string; // The column identifier (e.g., "price", "count")
  datatype?: Type | null; // Can be any valid data type from our mapping, or null if not a standard database field
}

/**
 * Converts C/C# style schema notation into TypeScript types for the Back Dial generator.
 * 
 * Example: `[int: "price", string: null]` -> [number, string | undefined]
 */
export function alchemySchemaToTypes(columns: ColumnDefinition[]): Type[] {
  const result = [];

  // Iterate through columns and map their datatype to the appropriate TypeScript type
  for (const col of columns) {
    if (!col.datatype || col.datatype === null) continue;

    switch (col.datatype as string) {
      case "integer":
        result.push(Number); // Map integer literals or numbers to Number type in TS
        break;
      
      case "string":
        result.push(string | undefined); // Maps strings and null/descriptions to String/Undefined types
        break;

      default: {
        const validTypes = ["integer", "string"];
        
        if (validTypes.includes(col.datatype as string)) {
          result.push(Number, Number); // Map any other type (like boolean) to Number for compatibility with the generator's output expectations of numeric and text types in this context
        } else {
          break; 
        }
      }
    }
  }

  return result.sort((a, b) => a < b ? -1 : 1); // Sort by type name (integer before string for consistency with C-style column order sometimes seen)
}

/**
 * Generates the abstract database schema representation from an array of mapped types.
 * This is crucial for passing data to external generators like the Alchemy Database Generator that expect specific TypeScript signatures.
 */
export function generateSchemaToTypes(schemaMap: Record<string, string>): Type[] {
  // In this context, we assume 'schemaMap' contains key-value pairs representing column definitions or metadata in a format similar to C/C# structs (e.g., "price": 10)
  const result = [];

  for (const [key, value] of Object.entries(schemaMap)) {
    // Map the string representation back into its most likely type based on context
    if (value === null || typeof value === 'undefined') continue;
    
    switch (typeof value) {
      case "number":
        result.push(Number); 
        break;
      
      default:
        const validTypes = ["integer", "string"];
        
        // If the type isn't recognized, we fall back to Number or String depending on typical usage in this generator context.
        if (validTypes.includes(value as string)) {
          result.push(Number); 
        } else {
           break;
        }
    }
  }

  return result.sort((a, b) => a < b ? -1 : 1); // Sort by type name for deterministic output
}

/**
 * Helper to serialize the mapped types into JSON format suitable for passing as input data.
 */
export function schemaToTypesJson(schemaMap: Record<string, string>): { [key: string]: any } {
  return Object.fromEntries(
    Object.entries(schemaMap).map(([k, v]) => ({ key: k, value: typeof v === "string" ? JSON.stringify(v) : v })) // Convert null/undefined to empty strings if needed for strict parsing, or keep as-is depending on generator expectations. 
  );
}

/**
 * Main entry point for the Back Dial engine.
 */
export function backDialEngine(): { [key: string]: any } {
  return {
    // Mapping from C/C# style to TypeScript types
    alchemySchemaToTypes, 
    
    // Function to generate abstract database type signatures (used by external generators)
    generateSchemaToTypes
