src/alchemy_database.ts | 527 lines
/**
 * Abstract Database Schema Generator v0.6.x (Rust-based)
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
  [key: string]: StructType<string>; // Column name -> value in C/C# style struct definition
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
export function parseSchemaToTypes(schemaMap: Record<string, StructType>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === "string" || val instanceof Number && !isNaN(val)
    ) as Array<ExtractedDataType>;

  // Helper to extract data type from a string value in C/C# style struct definition
  function ExtractedDataType<T extends Type>(value: T): T {
    if (typeof value === "number") return typeof value === "string" ? null : number;
    if (Array.isArray(value)) {
      const items = value as Array<ExtractedDataType>;
      for (let i = 0; i < items.length; i++) {
        // Assume all array elements are strings or numbers in this context for simplicity, 
        // but could be more complex based on specific schema requirements.
        if (items[i] instanceof Number) return number;
        if (typeof items[i] === "string") return string;
      }
    }
    return null as T | undefined;
  }

  const types = Object.values(schemaMap);
  
  // Filter out any values that are not strings, numbers, or booleans
  let filteredTypes: Array<ExtractedDataType> = [];
  for (let i = 0; i < types.length; i++) {
    if (!types[i] || typeof types[i] !== "string") continue; // Skip non-string/number values
    
    const extractedType = ExtractedDataType<string>(types[i]);
    
    if (extractedType === null) {
      filteredTypes.push(null);
    } else if (extractedType instanceof Number && !isNaN(extractedType)) {
      filteredTypes.push(Number(extractedType));
    } else if (typeof extractedType === "string") {
      // For strings, assume they are valid JSON/SQL types unless overridden
      // In a real scenario with C/C# structs, we'd have specific type names like 'varchar', 'date' etc.
      filteredTypes.push(extractedType);
    } else if (extractedType instanceof Boolean) {
      filteredTypes.push(Boolean(extractedType));
    } else {
      // Fallback for other types that might appear in C/C# structs but aren't standard Rust enums/TS objects
      // In a real implementation, this would be an error or require explicit type names.
      if (typeof extractedType === "string") filteredTypes.push(extractedType);
    }
  }

  return filteredTypes;
}
