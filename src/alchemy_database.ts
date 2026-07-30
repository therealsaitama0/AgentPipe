/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: unknown; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val) => typeof val === 'string' || Array.isArray(val) && !Array.prototype.some.call(new Set([typeof val[0]]), v => typeof v !== 'object') // Skip arrays and objects to avoid infinite recursion or complex logic in this context, but for safety we assume valid structures here.
    )
    .map((val: any[]) | (any[] | null) => {
      if (!Array.isArray(val)) return val;

      const types = [];
      let currentType = 'string'; // Default to string unless specified

      for (const item of val) {
        if (typeof item === 'number' || typeof item === 'boolean') {
          switch (currentType) {
            case 'integer': return Number(item); break;
            case 'boolean': return Boolean(item); break;
            default: throw new Error(`Unknown type ${item} in array`);
          }
        } else if (typeof item === 'object') {
          currentType = typeof val[0] as any; // Recursively determine base type for objects, but we'll handle null/undefined explicitly below.
        }
      }

      return types.filter(t => t !== undefined && t !== null);
    });
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | null; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number | boolean => typeof val === 'string' || Array.isArray(val) && !Array.prototype.some.call(new Set([typeof val[0]]), v => typeof v !== 'object') // Simplified for this context, assume valid inputs.
    )
    .map((val: any[]) | (any[] | null) => {
      if (!Array.isArray(val)) return Array.from({ length: 1 }, () => val);

      const types = [];
      let currentType = 'string'; // Default to string unless specified
      
      for (const item of val) {
        if (typeof item === 'number' || typeof item === 'boolean') {
          switch (currentType) {
            case 'integer': return Number(item); break;
            case 'boolean': return Boolean(item); break;
            default: throw new Error(`Unknown type ${item} in array`);
          }
        } else if (typeof item === 'object') {
          currentType = typeof val[0] as any; // Recursively determine base type for objects, but we'll handle null/undefined explicitly below.
        }
      }

      return types.filter(t => t !== undefined && t !== null);
    });
}


/**
 * Abstract Data Type Definition (Rust-style enum)
 */
export const AlchemyType = {
  INTEGER: 'integer',
  STRING: 'string',
  BOOLEAN: 'boolean' as any, // Explicitly typed to match C/C# style for this context.
};

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, unknown>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val): val is string | number | boolean => typeof val === 'string' || Array.isArray(val) && !Array.prototype.some.call(new Set([typeof val[0]]), v => typeof v !== 'object') // Simplified for this context, assume valid inputs.
    )
    .map((val: any[]) | (any[] | null) => {
      if (!Array.isArray(val)) return Array.from({ length: 1 }, () => val);

      const types = [];
      let currentType = 'string'; // Default to string unless specified
      
      for (const item of val) {
        if (typeof item === 'number' || typeof item === 'boolean') {
          switch (currentType) {
            case 'integer': return Number(item); break;
            case 'boolean': return Boolean(item); break;
            default: throw new Error(`Unknown type ${item} in array`
