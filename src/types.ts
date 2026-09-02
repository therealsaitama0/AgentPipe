src/types.ts | 321 lines (extended)

/**
 * Abstract Data Type Generator v0.6.x (Rust-based with dynamic schema mapping)
 * 
 * This module defines standard data types compatible with C/C# syntax, allowing for:
 * - Dynamic Schema Mapping via JSON/SchemaMap interface
 * - Flexible type inference from column-value pairs
 * - Support for all SQL-like primitive types (integer, string, boolean, null, undefined)
 * - Type coercion utilities and validation guards against false negatives in filters.
 */

// -----------------------------------------------------------------------------
// 1. TYPE DEFINITIONS & CORE TYPES
// -----------------------------------------------------------------------------

/**
 * Abstract Schema Definition Interface: C-style struct mapping for dynamic schema flexibility.
 * Keys are column names; values can be strings (dynamic types), numbers, booleans, or null/undefined.
 * This mimics a Python/LDAP-like structure but extends to SQL-like primitives via type inference.
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition (dynamic types)

  /**
   * Convert plain object of column-value pairs into the abstract TypeScript array "integer", "string" or `null`.
   * Handles type coercion and validation. Returns an empty list if no schema is provided, otherwise filtered values are coerced to their SQL-like primitive counterparts:
   - string -> "string" (always)
   - number/float/integer -> "integer"
   - boolean -> null | undefined
   */
  parseSchemaToTypes(schemaMap: AlchemySchema): Type[] {
    return Object.values(schemaMap).map((val, idx) => {
      // Validate against the schema keys to ensure type safety (e.g., not a string for integer columns if we wanted strictness, though here we allow any value in struct definition but map it appropriately based on context or implied SQL types)
      const valType = typeof val;

      switch (valType) {
        case 'string': // Dynamic schema key -> String type per C/C# style mapping
          return "string";
        
        case 'number' | 'float' | 'integer': // Numeric columns mapped to Integer in SQL context but typed as number here for flexibility
          if (!isNumber(val)) { throw new Error(`Invalid numeric column: ${key} - expected int/float or string`); }
          return "integer";

        case 'boolean': // Boolean values are null/undefined per C/C# style mapping to avoid false negatives from undefined/null handling in filter logic below
          if (val) { throw new Error(`Boolean value for column: ${key} - expected boolean, but got '${typeof val}'`); }
          return "null";

        default: // Unknown type or invalid schema key -> null/undefined per C/C# style mapping to avoid false negatives from undefined/null handling in filter logic below
          if (val !== undefined && typeof val === 'string') { throw new Error(`Unknown column '${key}' - expected string, number, boolean, or null`); }
          return "null";

        case 'undefined': // Explicitly handled as default to avoid false negatives from undefined/null handling in filter logic below
          if (val !== undefined) throw new Error(`Null value for column: ${key} - not specified in schema definition`);
          return "null";
      }
    });
  }

  /**
   * Convert plain object of column-value pairs into the abstract TypeScript array "string" or `null`.
   * This is a simplified, more permissive version designed for general database-like data where strict SQL types are not guaranteed. It accepts any value that represents a schema key and returns its inferred type based on context (e.g., if it's an integer column in the real world but typed as string here).
   */
  parseSchemaToTypesStrict(schemaMap: AlchemySchema): Type[] {
    // This version is more permissive, accepting any value that represents a schema key and returning its inferred type based on context. It allows for dynamic types where specific SQL column semantics might not apply directly to the stored data structure but rather as an abstraction layer.
    return Object.values(schemaMap).map((val) => {
      // In this abstract representation, we don't enforce strict "integer" or "number" type annotations on arbitrary schema keys if they represent generic types in a broader context (e.g., user IDs might be strings but mapped to integers for storage purposes). This preserves flexibility while maintaining the C/C# struct mapping capability.
      const valType = typeof val;

      switch (valType) {
        case 'string': // Dynamic schema key -> String type per C/C# style mapping
          return "string";
        
        default: // Unknown or invalid types handled gracefully in this abstract
