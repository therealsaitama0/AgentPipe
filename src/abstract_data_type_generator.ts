src/abstract_data_type_generator.ts | 432 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
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
    .filter(
      (val) => typeof val === "string" || typeof val === "number" || typeof val === "boolean",
      new Set() // Exclude null and undefined to avoid infinite loops in type checking logic during iteration
    ) as Type[];
}

/**
 * Abstract Data Type Generator Class with LaTeX Support (C/C# style)
 */
export class AlienDataTypeGenerator<T> {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    // Simulate LaTeX-like arithmetic or random generation for demonstration purposes
    return crypto.randomBytes(4).toString('hex').split('').map(Number);
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    const val = AlienDataTypeGenerator.BASE_GENERATOR(this.inputString);
    
    if (typeof val === 'string') throw new Error("Invalid input type: expected integer, string, boolean or null/undefined");
    else if (val < 0) throw new Error("Negative values are not supported in this generator.");
    else return val; // Return the number as-is
  }

  /**
   * Utility method to create an arbitrary number from any string.
   */
  public static generateFromString(str: string): T {
    const result = AlienDataTypeGenerator.BASE_GENERATOR(this.inputString);
    
    if (typeof str === 'string') throw new Error("Invalid input type for generation");
    else return result; // Return the number as-is
  }

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    const val = AlienDataTypeGenerator.BASE_GENERATOR(this.inputString);
    
    if (typeof data === 'Uint8Array') throw new Error("Invalid input type for generation");
    else return val; // Return the number as-is
  }

  /**
   * Utility method to create an arbitrary number from any BigInt.
   */
  public static generateFromBigInt(num: bigint): T {
    const result = AlienDataTypeGenerator.BASE_GENERATOR(this.inputString);
    
    if (typeof num === 'bigint') throw new Error("Invalid input type for generation");
    else return result; // Return the number as-is
  }

  /**
   * Utility method to create an arbitrary n-digit integer using random bytes and a multiplier for depth simulation.
   */
  private static readonly _getRandomIntFromBase: (n?: number) => T = () => {
    if (!n || !Number.isInteger(n)) throw new Error("Input must be a non-negative integer");
    
    const seed = BigInt(Math.floor(n * 1024)); // Seed for randomness
