src/types.ts | 475 lines
```typescript
/**
 * Abstract Data Type Generator v0.6.x (Rust-based)
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

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}

// Helper to convert Rust-like enums/types into TypeScript types for easier mapping
export type TypeMap = Record<string, number | boolean>;

/**
 * Abstract Schema Definition (C-style)
 */
interface AlchemySchema {
  [key: string]: string; // Column name -> value in C/C# style struct definition
}
