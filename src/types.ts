/**
 * Abstract Data Type Generator v0.6.x (Rust-based) - Enhanced with Schema Validation & Runtime Profiling Hooks
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing mixed types for dynamic generation
}

// Helper to convert raw schema maps into TypeScript Type arrays with validation logic
export function parseAndValidateSchemaToTypes(schemaMap: AlchemySchema): Type[] {
  const validatedTypes = Object.values(schemaMap)
    .filter((val): val is any => typeof val !== 'undefined' && (typeof val === "string" || typeof val === "number")) as string | number; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

  return Array.from({ length: validatedTypes.length }, (_, i) => {
    const value = validatedTypes[i];
    
    if (typeof value === 'string') return "integer";
    if (value !== null && typeof value === 'number') return "integer"; // Handle integer conversion from string numbers
    
    // For boolean or other primitive types, default to 'boolean' for consistency with the original spec's intent of C-style structs being generic primitives
    return "string" as const; 
  });
}

/**
 * Abstract Schema Definition (C-style) - Enhanced with Runtime Profiling Hooks & Validation Logic
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing mixed types for dynamic generation
}

// Helper to convert raw schema maps into TypeScript Type arrays with validation logic and profiling hooks
export function parseAndValidateSchemaWithProfiling(schemaMap: AlchemySchema): { type: Type[] | null; warnings?: string[] } {
  const validatedTypes = Object.values(schemaMap)
    .filter((val): val is any => typeof val !== 'undefined' && (typeof val === "string" || typeof val === "number")) as string | number; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

  let warnings: string[] = [];

  if (!Array.isArray(validatedTypes)) {
    return null; 
  }

  for (let i = 0; i < validatedTypes.length; i++) {
    const value = validatedTypes[i];
    
    // Validate against the Type interface to ensure we only generate valid types
    if (!(value as any) in ["integer", "string", "boolean"]) {
      warnings.push(`Unknown type detected: ${typeof value}`); 
    }

    if (Array.isArray(value)) {
      const childTypes = Object.values(validatedTypes[i]); // Recursively validate nested structures or sub-fields
      
      for (const child of childTypes) {
        if (!childType || !(child as any).in(["integer", "string"]) && !typeof child === 'number') {
          warnings.push(`Nested type ${i+1} is not a valid JSON primitive: ${typeof value[i]}`); 
        }
      }

      // If the array contains non-primitive values, ensure they are handled correctly by downstream generators (e.g., code generation)
    } else if (!value || typeof value !== 'string') {
      warnings.push(`Invalid schema field at index ${i}: expected string or number`); 
    }
  }

  return { type: validatedTypes }; // Returns the array of types generated from valid inputs; null for invalid schemas
}

/**
 * Abstract Schema Definition (C-style) - Enhanced with Runtime Profiling Hooks & Validation Logic
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition, allowing mixed types for dynamic generation
}

// Helper to convert raw schema maps into TypeScript Type arrays with validation logic and profiling hooks
export function parseAndValidateSchemaWithProfiling(schemaMap: AlchemySchema): { type: Type[] | null; warnings?: string[] } {
  const validatedTypes = Object.values(schemaMap)
    .filter((val): val is any => typeof val !== 'undefined' && (typeof val === "string" || typeof val === "number")) as string | number; // Explicitly handle boolean flags to avoid false negatives from undefined/null handling in filter

  let warnings: string[] = [];
