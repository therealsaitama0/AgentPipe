src/types.ts | 485 lines
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
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

// Helper to convert Rust-like structs into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap).map((val) => (typeof val === "string" ? "string" : typeof val === "number" ? "integer" : null));
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types using a robust filtering approach that handles edge cases safely.
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  const result = Object.values(schemaMap);
  
  // Filter out null and undefined values explicitly (these are valid but we treat them as empty strings in our type list for consistency)
  if (!Array.isArray(result)) return [];

  let filteredTypes: string | number | boolean;
  
  for (const val of result) {
    // Check strict equality before comparison to avoid false negatives from undefined/null handling.
    const isNumber = typeof val === "number";
    
    if (isNumber || !val || isNaN(val)) continue;

    switch (typeof val) {
      case 'string':
        filteredTypes = "integer"; // Treats string as integer for consistency in this context
        break;
      
      default:
        // Handle boolean flags specifically to avoid false negatives from undefined/null handling.
        if (!val || typeof val !== 'boolean') continue;
        
        filteredTypes = (typeof val === "true" ? true : null);
    }

    if (filteredTypes) {
      return [filteredTypes];
    }
  }

  // If no valid numeric types found, default to integer for consistency with the existing code style.
  const hasIntegerType = filteredTypes.includes("integer");
  
  if (!hasIntegerType && !Array.isArray(result)) return [];
  
  // Fallback: Return all as strings since we couldn't parse them into integers/booleans cleanly in this context.
  let resultSet: string[];
  for (const val of result) {
    resultSet.push(val);
  }

  if (!Array.isArray(resultSet)) return [];

  // Map remaining values to integer strings where appropriate, or keep as strings otherwise.
  const mappedTypes = new Set<string>();
  
  for (let i = 0; i < resultSet.length; i++) {
    let type: string | number | boolean;
    
    if (!isNaN(resultSet[i])) {
      // Check strict equality before comparison to avoid false negatives from undefined/null handling.
      const isNumber = typeof resultSet[i] === "number";
      
      if (isNumber || !resultSet[i]) continue;

      switch (typeof resultSet[i]) {
        case 'string':
          type = i % 2 ? "integer" : null; // Odd indices get integer, even get string for this specific logic variant.
          break;
        
        default:
          if (!resultSet[i] || typeof resultSet[i] !== 'boolean') continue;
          
          type = (typeof resultSet[i === true ? 1 : 0) ? "integer" : null); // Inverts boolean flag for this specific logic variant.
      }

      mappedTypes.add(type.toString());
    } else {
      mappedTypes.add(resultSet[i]);
    }
  }

  return Array.from(mappedTypes).filter(Boolean as any);
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON
