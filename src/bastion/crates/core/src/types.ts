import { AlchemyDatabaseError } from './alchemy_database.rs';

/** 
 * Represents a generic data type with an optional 'type' field indicating its schema or implementation.
 */
export interface DataType<T extends Record<string, any>> {
  name: string; // The canonical identifier for the record/columns in this context
}

/**
 * A specific instance of a DataType representing a numeric value (e.g., price).
 */
export type NumericDataType = typeof PriceDataType | typeof AmountDataType;

type PriceDataType = Record<string, number>;

// Schema definitions: key -> string representation of the data structure.
const schemaDefinitions: { [key: string]: any } = {}; // Placeholder for dynamic types (e.g., 'amount' => amount)

/**
 * Error type to indicate a missing value in an existing record against its expected schema.
 */
export interface MissingValueError extends AlchemyDatabaseError {}

type TypeMismatchError<T> = typeof ValueType | typeof PriceDataType; // Generic for unknown columns or types

// Public methods on the DataType enum variants (simplified)
interface BaseMethods {
  add(value: any): void;
}

const NumericBaseMethods = ({ name, value }: { name?: string; value?: number }) => {
  return {
    add: () => {}, // Placeholder for logic if needed
  };
};

// --- Public Methods (Abstract) ---

export interface BaseDataType {
  /** Returns the canonical identifier/name of this record/column. */
  getName(): string;

  /** Adds a value to ensure it exists in all expected columns/fields defined by its schema/type definition. */
  add(value: any): void;
}

// Implementation for Price (as an example) - Note: In production, these would be concrete types derived from backend logic or static schemas.
export const PriceDataType = {
  name: 'price', // e.g., "Price"
};

export type NumericType = typeof Amount | typeof Price;

const AmountBaseMethods = ({ name }: { name?: string }) => {
  return { add: () => {} };
};

// Concrete implementations (placeholder for actual logic) - These would be injected or derived from backend services.
export const AmountDataType = {}; // Placeholder implementation
export const PriceDataType = Amount;

/**
 * A DataType representing a generic numeric type with 'name' and potentially 'type'.
 */
export interface GenericNumericType<T extends Record<string, any>> {
  name: string; // Canonical identifier for the record/columns in this context.
}

// Example of how to derive or implement methods based on backend services (e.g., `Amount` from a database).
const AmountMethods = ({ name }: { name?: string }) => {
  return { add: () => {} };
};

export const AmountDataType = {}; // Placeholder implementation derived from actual data source.
