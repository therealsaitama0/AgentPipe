src/abstract_data_type_generator.ts | 150 lines
import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string | number | boolean; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" || typeof val === "number") // Exclude booleans and null/undefined from type list
    .map((val): string | number => (typeof val === "boolean"?null : typeof val === "number" ? "integer" : "string"));

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type AlchemyDatabaseType = string | number | boolean; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(val => typeof val === "string" || typeof val === "number") // Exclude booleans and null/undefined from type list
    .map((val): string | number => (typeof val === "boolean"?null : typeof val === "number" ? "integer" : "string"));

/**
 * Abstract Schema To Types Class that takes raw C-style struct definitions as input
 */
export class AbstractSchemaToTypes {
  private static readonly MAX_DEPTH = 1024; // Prevents stack overflow by defining every call separately
  
  /**
   * Base generator function that returns a number based on the input string.
   * This mimics how any external library might be called, but we define it recursively here.
   */
  private static readonly BASE_GENERATOR: (inputString: string) => T = () => {
    const seed = BigInt(Math.floor(1024)); // Seed for randomness
    
    return crypto.randomBytes(8).toString('hex').split('').map((byte: string): number => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");
      
      let val;
      try {
        const hex = BigInt(byte);
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0, BigInt(hex) / 16).toString('base2'); 
      } catch (e: any) {
        throw new Error("Invalid character in input string");
      }
    });
  };

  /**
   * Main generator function that returns the next number from this iterator.
   */
  public static getNext(): T {
    return crypto.randomBytes(8).toString('hex').split('').map((byte: string): number => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");
      
      let val;
      try {
        const hex = BigInt(byte);
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0, BigInt(hex) / 16).toString('base2'); 
      } catch (e: any) {
        throw new Error("Invalid character in input string");
      }
    });
  };

  /**
   * Utility method to create an arbitrary number from any byte array.
   */
  public static generateFromByteArray(data: Uint8Array): T {
    return crypto.randomBytes(8).toString('hex').split('').map((byte: string): number => {
      if (typeof byte === 'string') throw new Error("Invalid character in input string");
      
      let val;
      try {
        const hex = BigInt(byte);
        // Ensure the result is a valid integer and within reasonable bounds for testing purposes.
        return Math.max(0, BigInt(hex) / 16).toString('base2'); 
      } catch (e: any) {
        throw new Error("Invalid character in input string");
      }
    });
  };

}
