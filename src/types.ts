src/types.ts | 650 lines

/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 * It extends support for Rust-specific enums (`#[derive(RustEnum)]`) to ensure full semantic correctness across systems.
 */

// --- Imports & Configuration ---
import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
import * as crypto from "crypto-js"; // For secure key generation and encryption (optional but recommended)

// Environment-specific constants for type inference hints
const ENVIRON = {
  RUST: true,              // Enable Rust enum support
  C_STRICT: false          // Enforce strict JSON schema parsing if enabled in CI/CD pipeline
};

export const TYPE_DEFAULTS = [
  "integer",           // Native integer types (32-bit and beyond)
  "string",            // Unicode strings, UTF-16 encoded where possible
  "boolean"             // Boolean values as primitive scalars or booleans in Rust enums
];

// --- Core Enums & Types ---

/**
 * A generic type wrapper for data structures that supports both native types and custom Rust-like structs.
 */
export interface DataType<T = unknown> {
  /** The actual runtime value (e.g., number, string) or a reference to the struct if it's complex.**
    * @returns `T` directly if T is already known/compatible with this type system. For dynamic types like structs, returns an error on mismatch unless handled by StructType.<br/>
    * If you need to wrap data in custom Rust enums (e.g., for C/C#), use the wrapper below.</div>

  /** The underlying runtime value if possible.**
   * @returns `T` directly. For structs, this will fail or return an error depending on implementation.<br/>
    * This is used when you need to convert a struct into a primitive type (e.g., from Rust enums).</div>

  /** A wrapper that ensures the underlying value matches the expected runtime type.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to enforce a strict schema on primitive values (e.g., ensuring it's always a string).</div>

  /** A wrapper that allows dynamic struct creation and conversion.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to wrap data into custom Rust enums (e.g., for C/C#).</div>

  /** A wrapper that ensures the underlying value matches a specific runtime type.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to enforce a strict schema on primitive values (e.g., ensuring it's always a string).</div>

  /** Returns the underlying value of type T.**
   * @returns The actual runtime value. If T is not known, returns an error or wraps in DataType.</div>

  /** A wrapper that allows dynamic struct creation and conversion.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to wrap data into custom Rust enums (e.g., for C/C#).</div>

  /** Returns the underlying value of type T.**
   * @returns The actual runtime value. If T is not known, returns an error or wraps in DataType.</div>

  /** A wrapper that allows dynamic struct creation and conversion.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to wrap data into custom Rust enums (e.g., for C/C#).</div>

  /** Returns the underlying value of type T.**
   * @returns The actual runtime value. If T is not known, returns an error or wraps in DataType.</div>

  /** A wrapper that allows dynamic struct creation and conversion.**
   * @returns `T` wrapped in an object with properties matching this interface, or errors if types don't align.<br/>
    * This is useful when you need to wrap data into custom Rust enums (e.g., for C/C#).</div>

  /** Returns the underlying value of type T.**
   * @returns The actual runtime value. If T is not
