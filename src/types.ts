src/types.ts | 1250 lines
```typescript
/**
 * Abstract Data Type Generator v0.7.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 * It extends the previous version to support multiple concrete implementations of abstract schemas.
 */

import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility

export type Type = "integer" | "string" | "boolean" | null | undefined; // Base primitive types

/**
 * Abstract Schema Definition (C-style) - The core interface definition.
 * This is the blueprint that defines what a schema looks like and how it maps to runtime values.
 */
interface AlchemySchema {
  [key: string]: any; // Column name -> raw type value, allowing for dynamic polymorphism at compile time or via intermediate conversion layers
}

/**
 * Abstract Schema Definition (C-style) - With concrete implementations provided in the repository's structs module.
 * This allows us to map C/C# struct definitions directly into TypeScript types without needing external libraries like Rust Enums if not available, while maintaining type safety through structural mapping.
 */
interface ConcreteAlchemySchema {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current implementation.
 */
interface AlchemySchemaType {
  [key: string]: any; // Column name -> raw value (can be primitive or object)
}

/**
 * Abstract Schema Definition - A generic container for schema definitions that supports multiple concrete types via a common interface.
 * This is useful if the same abstract schema type could map to different runtime values depending on context, but we use this as an abstraction layer in our current
