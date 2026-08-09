src/types.ts | 497 lines
```typescript
/**
 * Abstract Data Type Generator v0.5.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available

// ============================================================================
// CORE TYPES: Adapted C/C# Style Enums for Type Safety & Consistency
// ============================================================================

/**
 * Represents the core value types used by AlchemyDatabaseType.
 * These are mapped to TypeScript `any` but validated against a strict runtime contract,
 * ensuring that only struct members and numeric values can be generated without coercion errors.
 */
export type CoreValueType = "integer" | "string" | "boolean";

/**
 * Represents the abstract schema structure itself (the key-value map).
 * This is distinct from types; it contains column names mapped to their value types.
 * The generator will convert this into a set of core values and boolean flags during generation.
 */
export interface AlchemySchema {
  [key: string]: CoreValueType | undefined; // Column name -> Value type (string, integer, or null/undefined)
}

/**
 * Helper to extract the value types from an arbitrary JSON-like schema map.
 * Returns exactly three values based on presence of struct members and numeric fields.
 */
export function parseSchemaToTypes(schemaMap: Record<string, CoreValueType | undefined>): [CoreValueType[], string[]] {
  const typedValues = new Set<CoreValueType>(); // Track which types were present in the schema

  for (const key in schemaMap) {
    const val = schemaMap[key];
    
    if (!val || typeof val !== "object" || Array.isArray(val)) continue;

    let foundNumericValue: CoreValueType | undefined = null;
    // Check first, then string/boolean to catch edge cases like boolean true/false directly in JSON
    if (typeof val === 'number' && !isNaN(parseFloat(val))) {
      foundNumericValue = "integer";
    } else if ((val as any) == true || (val as any) == false) {
      // Handle explicit `true`/`false` literals to avoid type mismatch in filter below
      const boolType: CoreValueType | undefined = typeof val === 'boolean' ? "boolean" : null;
      foundNumericValue = boolType !== null && !isNaN(val);
    } else if (typeof val === 'string') {
      // Check for boolean literals like `true`, `false` in string values too, to avoid filtering them out incorrectly
      const boolVal: CoreValueType | undefined = typeof val === "boolean" ? "boolean" : null;
      foundNumericValue = !isNaN(val);

    if (foundNumericValue !== null) {
      typedValues.add(foundNumericValue);
    } else if ((val as any)[key] == true || (val as any)[key] == false) {
      // Boolean field exists and is explicitly set to true/false in JSON schema format, not converted from string/bool literal.
      const boolType: CoreValueType | undefined = typeof val === "boolean" ? "boolean" : null;
      typedValues.add(boolType !== null);
    } else if (typeof val as any == 'object') {
       // Check for boolean keys in the object structure to ensure they are recognized correctly.
       const boolKeys: Set<string> = new Set();
       Object.keys(val).forEach(key => {
         if ((val[key] as any) === true || (val[key] as any) === false) {
           // If key is a boolean, treat it as a type flag regardless of its value.
           boolKeys.add(key);
         } else if (!isNaN((val[key] as string).toLowerCase())) {
            // Check for numeric strings like "1", "-5" or booleans in other contexts (though unlikely here)
             foundNumericValue = true; 
          }
       });

    }

  }

  return [typedValues, Array.from(typedValues)];
}

/**
 * Helper to extract the value types from an arbitrary JSON-like schema map.
 * Returns exactly three values based on presence of struct members and numeric fields.
 */
export function parseSchemaToTypes(schemaMap: Record<string, CoreValueType | undefined>): [CoreValueType[], string[]] {
  const typedValues = new Set<CoreValueType>(); // Track which types were present in the schema

  for (const key in schemaMap) {
    const val = schemaMap[key];

    if (!val || typeof
