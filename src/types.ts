/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 * Extends support to handle complex nested structures using generic function pointers.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any[]; // Column name -> value in C/C# style struct definition. Supports arrays and generic structures.
}

// Helper to convert C-style struct/dynamic field definitions into TypeScript types for easier mapping
export function schemaToType(schemaMap: AlchemySchema): Type[] {
  return Object.values(schemaMap)
    .filter((val) => typeof val === "object" && Array.isArray(val)) // Filter arrays and objects containing Arrays to avoid false negatives in filter
    .map(fieldDef => ({ type: fieldDef.type, value: fieldDef.value })) as any;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context. Includes generic array support simulation.

// Helper to convert JSON-like schema definitions into abstract data types
/**
 * @param schemaMap - The map of column names and their dynamic field values (supports arrays)
 */
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).filter((val) => typeof val === "object" && Array.isArray(val)); // Filter non-array objects to avoid false negatives in filter
  
  if (!result.length) return [];

  let types: any[];
  
  try {
    // Attempting dynamic type inference for complex structures (e.g., generic arrays, nested structs)
    const inferredTypes = result.map((fieldDef): Type => ({ 
      ...fieldDef.value as unknown as Type, 
      isDynamicField: true 
    }));

    if (!Array.isArray(inferredTypes)) {
      // Fallback for static types when complex structures are detected (e.g., specific nested objects)
      inferredTypes = Object.values(result).map(field => ({ type: "string", value: field, isStaticType: true }));
    } else {
      const mappedTypes = new Map<string, Type>();

      result.forEach((fieldDef): void => {
        if (typeof fieldDef.value === 'object' && Array.isArray(fieldDef.value)) { // Dynamic array support simulation
          mappedTypes.set(fieldDef.key as string, inferredTypes);
        } else {
          mappedTypes.set(fieldDef.key as string, fieldDef.type || "string");
        }
      });

      if (mappedTypes.size === result.length) return Array.from(mappedTypes.values()); // Return all types for consistency in schemaToType
    }
  } catch (e: any) {
    console.error("Error parsing dynamic type mapping:", e);
    throw new Error(`Failed to parse schema into types. Check the 'structs' file or consider using static types.`);
  }

  return Array.from(mappedTypes.values());
}

/**
 * Abstract Schema Definition (C-style) - Extended with Nested Structures Support
 */
interface AlchemySchema {
  [key: string]: any[]; // Column name -> value in C/C# style struct definition. Supports arrays and generic structures.
  
  /**
   * @param nestedStructs - Optional array of dynamic field values for deeply nested objects (e.g., recipes, sessions)
   */
  nestedObjects?: Array<any>; 
}

/**
 * Abstract Schema Definition (C-style) - Extended with Nested Structures Support (Rust-based simulation)
 */
interface AlchemySchemaNested {
  [key: string]: any[]; // Column name -> value in C/C# style struct definition. Supports arrays and generic structures.
  
  /**
   * @param nestedObjects Optional array of dynamic field values for deeply nested objects (e.g., recipes, sessions)
   */
  nestedObjects?: Array<any>; 
}

// Helper to convert C-style schema definitions into TypeScript types
export function parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const result = Object.values(schemaMap).filter((val) => typeof val === "object" && Array.isArray(val)); // Filter arrays and objects containing Arrays
  
  if (!result.length)
