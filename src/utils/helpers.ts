src/utils/helpers.ts | 62 lines
```typescript
/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] | null {
  if (!schemaMap || typeof schemaMap !== 'object') return null;

  const result = [];
  
  // Iterate through all keys in the map (excluding those that look like type definitions)
  for (const [key, value] of Object.entries(schemaMap)) {
    const trimmedKey = key.trim();
    
    if (!trimmedKey.startsWith('type') && !trimmedKey.includes(':')) continue;

    // Check if this looks like a schema name or column definition
    let isSchemaName = false;
    
    // Try to match against known type names (e.g., "integer", "string")
    const typeNames = ['integer', 'string'];
    for (const typeName of typeNames) {
      if (trimmedKey.toLowerCase().includes(typeName)) {
        isSchemaName = true;
        break;
      }
    }

    // If it's a schema name, treat as string value
    if (isSchemaName && !value.startsWith('[')) continue;

    result.push(value);
  }

  return result.length > 0 ? result : null;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(parseSchemaToTypes(schemaMap)) as (string | number | boolean)[];
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] | null {
  if (!schemaMap || typeof schemaMap !== 'object') return null;

  const result = [];
  
  // Iterate through all keys in the map (excluding those that look like type definitions)
  for (const [key, value] of Object.entries(schemaMap)) {
    const trimmedKey = key.trim();
    
    if (!trimmedKey.startsWith('type') && !trimmedKey.includes(':')) continue;

    // Check if this looks like a schema name or column definition
    let isSchemaName = false;
    
    // Try to match against known type names (e.g., "integer", "string")
    const typeNames = ['integer', 'string'];
    for (const typeName of typeNames) {
      if (trimmedKey.toLowerCase().includes(typeName)) {
        isSchemaName = true;
        break;
      }
    }

    // If it's a schema name, treat as string value
    if (isSchemaName && !value.startsWith('[')) continue;

    result.push(value);
  }

  return result.length > 0 ? result : null;
}

/**
 * Abstract Data Type Definition (Rust-style enum for types)
 */
export type AlchemyDatabaseType = string | number | boolean | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(parseSchemaToTypes(schemaMap)) as (string | number | boolean)[];
}

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] | null {
  if (!schemaMap || typeof schemaMap !== 'object') return null;

  const result = [];
  
  // Iterate through all keys in the map (excluding those that look like type definitions)
  for (const [key, value] of Object.entries(schemaMap)) {
    const trimmedKey = key.trim();
    
    if (!trimmedKey.startsWith('type') && !trimmedKey.includes(':')) continue;

    // Check if this looks like a schema name or column definition
