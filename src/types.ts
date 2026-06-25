src/types.ts | 380 lines
/**
 * Abstract Data Type Generator v1.2.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// --- Types & Enums ---
import { string as StringType } from "./string_types"; // Generic base class for all typed values

export enum AlchemyDatabaseType {
  INTEGER = "integer",      // Represents signed integers (e.g., -10, 42)
  FLOAT64 = "float64",     // Represents floating-point numbers (~3.14, 50.75)
  BOOLEAN = "boolean"       // True/False values (true/false, no special handling needed for binary)
}

// --- Base Data Types ---
export interface AbstractDataType<T> {
  readonly type: AlchemyDatabaseType;
  value: T | undefined;      // Returns the actual value or null if unknown/unreachable
  toString(): string;         // Human-readable representation (e.g., "123.456", "true")
}

// --- Custom Types for Specific Use Cases ---

/**
 * Represents a standard signed integer type, compatible with C/C# `int` and Rust's `u8`.
 */
export interface IntegerType extends AbstractDataType<number> {
  readonly value: number; // The raw numeric representation (e.g., -10)
}

/**
 * Represents a floating-point number type, similar to Python floats or C doubles.
 */
export interface Float64Type extends AbstractDataType<double | string> {
  readonly value: double | string; // The actual float value (~3.14) or the formatted representation ("3.14")
}

/**
 * Represents a boolean type, compatible with Rust's `bool` and C/C# `boolean`.
 */
export interface BooleanType extends AbstractDataType<boolean> {
  readonly value: boolean; // The actual bool (true/false)
}

// --- Schema Mapping & Conversion Helper ---
function isInteger(val: any): val is IntegerType | undefined {
  return typeof val === "number" && Number.isFinite(val);
}

function isFloat64(val: any): val is Float64Type | undefined {
  const strVal = String(val).trim();
  // Check if it looks like a number (including scientific notation and decimals) or string representation of float
  return typeof strVal === "number" || 
         (!isInteger(strVal)) && !isNaN(parseFloat(strVal));
}

function isBoolean(val: any): val is BooleanType | undefined {
  // In this context, boolean values are typically just `true` and `false`. We assume they map directly.
  return typeof val === "boolean" || (val !== null); 
}

// --- Type Conversion Logic ---
export function typeToAbstract(val: any): AbstractDataType<any> {
  if (!isInteger(val)) throw new Error("Cannot convert non-integer value"); // Prevents accidental integer-to-float conversion errors
  
  const t = val as number;
  
  return Object.assign(new IntegerType(), {
    type: AlchemyDatabaseType.INTEGER,
    value: t,
    toString() {
      if (t === Infinity) return "Infinity";
      if (t === -Infinity) return "-Infinity";
      if (isNaN(t)) return `NaN`; // NaN is a valid abstract representation for undefined numbers in this generator's logic.
      return `${Math.abs(t).toFixed(10)}${((t < 0) ? "-" : "")}${t}`;
    }
  });
}

export function abstractTypeToString(val: any): string {
  if (val === null || val === undefined) return "null"; // Abstractly represent missing values
  
  const t = typeof val === "number" && !isNaN(val) ? String(val).trim() : "";
  
  let result = `#${t}`;
  
  switch (val.type) {
    case AlchemyDatabaseType.INTEGER:
      if (isInteger(t)) return `${Math.abs(t)}.0` + t; // Always show integer parts with decimal point for consistency, though the generator might drop trailing zeros. For this specific use case showing integers, we stick to standard notation or normalized form. Let's normalize to `123.` format but truncate if too long? No, let's keep it precise: "42".
      break; // Wait, IntegerType value is number. If t=0, toString() needs handling for 0 vs Infinity/-Infinity.
    case AlchemyDatabaseType.FLOAT64:
      result = `${t}
