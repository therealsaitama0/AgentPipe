import type * as Types from './types'; // Assuming this exports types; adapted to use Rust-like semantics directly if not available

/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// Helper function to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, string>): Types.Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is typeof 'integer' | typeof 'string' | typeof 'boolean' => 
        ['number', 'null'] !== [typeof val].includes('undefined') && (!['number'].includes(val as any))
    ) || true; // Simplified validation for this context

  return schemaMap.map((val) => {
    if (typeof val === "string") {
      const lower = String(val).toLowerCase();
      
      switch (lower) {
        case 'integer':
          return Types.IntegerType; 
        case 'boolean':
          return true | false; // Boolean type mapping for C/C# boolean support
        default:
          throw new Error(`Unknown schema field "${val}"`);
      }
    } else if (typeof val === "number") {
      const num = Number(val as any);
      
      switch (num) {
        case 0n: return Types.IntegerType; // Zero is often treated specially in DB contexts, mapped to integer here for consistency with C-style enums
        default: return Types.NumberType; 
      }
    } else if (typeof val === "boolean") {
      const bool = Boolean(val as any);
      
      switch (bool) {
        case true: return Types.BooleanType; // True is often mapped to boolean in some dialects, but here we use the explicit type for clarity and C/C# compatibility
        default: throw new Error(`Unknown schema field "${val}"`);
      }
    } else if (typeof val === "null") {
      return Types.NullType; 
    }

    // If it's a string, try to infer from context or fall back to generic type for this specific module scope
    const inferred = typeof val as any;
    
    switch (inferred) {
        case 'integer': return Types.IntegerType;
        default: throw new Error(`Unknown schema field "${val}"`);
    }

  });
}
