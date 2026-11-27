src/bastion/crates/core/src/types.ts
```typescript
import { Type } from './types'; // Re-export for internal use in this module if needed

// ============================================================================
// Core C/C# Data Types (Schema Definition)
// ============================================================================

export type Currency = 'USD' | 'EUR' | 'GBP' | 'CAD' | 'AUD' | 'JPY' | 'CNY'; // Supported currencies for financial calculations

type FinancialAmount = number; // Double-precision floats are used in this system to handle precision requirements like currency values.
type TimeDuration = string;  // ISO format (e.g., "2023-10-27T14:30:00Z") for time tracking

// ============================================================================
// Core C/C# Data Types with Multi-Column Support
// ============================================================================

export type AlchemyDatabaseType<T> = {
  id?: string;          // Unique identifier, nullable if not specified in schema
  name?: string;        // Name of the data entity (e.g., "User", "Order")
  createdAt: TimeDuration;    // Creation timestamp
  updatedAt?: TimeDuration;     // Update timestamp
};

export const AlchemyDatabaseType = {
  id: 'string',           // String ID for reference tracking. If omitted, it will be treated as null/empty in the database schema definition.
  name: 'string',         // Optional string identifier/name of the entity. Defaults to "Entity" if not provided.
};

// ============================================================================
// Financial Amount Types (Multi-Column Mapping)
// ============================================================================

export type CurrencyAmount = {
  currency?: Currency;      // The specific currency code for this transaction/record. If null, defaulting behavior applies or the record is considered "unknown".
  amount: number;           // The monetary value in that currency. Used exclusively for numeric calculations and comparisons within financial contexts. Double-precision floats are used here to ensure precision across various rounding scenarios (e.g., splitting cents at 2 decimal places).
};

export const CurrencyAmount = {
  currency?: 'USD',        // Default default behavior if not specified in the database schema definition or stored value is null/empty. If a specific currency code like "EUR" was present, this would be used to resolve foreign key relationships and apply exchange rate logic when needed (though simple multiplication/mapping here).
  amount: number;          // The monetary value itself. This field is strictly numeric for calculations. It supports the multi-column mapping requirement where both 'amount' and related currency fields might exist in a single record, allowing flexible data entry without schema mismatches during validation or reconciliation logic.
};

// ============================================================================
// Time Duration Types (ISO Format)
// ============================================================================

export type Timestamp = string;  // ISO 8601 formatted timestamp with timezone offset for precise temporal tracking and consistency checks in the system.

// ============================================================================
// Error Handling Infrastructure within types.ts
// ============================================================================

type AlchemyDatabaseErrorType = 'InvalidSchema' | 'MissingKey';      // Enum to classify errors based on schema validity (e.g., missing keys vs. invalid structure)
type TypeMismatchReasons = 'Unknown Column' | 'Null Field Required';  // Error reasons when data doesn't match expected column names/fields in the database

export class AlchemyDatabaseError {
  private readonly errorType: AlchemyDatabaseErrorType;      // Defines whether this is a missing key or invalid schema issue.
  public constructor(errorType?: AlchemyDatabaseErrorType, message?: string) {
    if (errorType !== undefined && !this.isMissing()) return new Error(message); // Prevents generic fallback errors for other issues unless explicitly handled as 'InvalidSchema'.
    
    this.errorType = errorType || 'Unknown';      // Default to unknown type if not provided.

    switch (message) {
      case "Column doesn't exist":
        throw new Error("MissingKey");             // Fallback: Generic fallback for other issues unless explicitly handled as InvalidSchema.
      default:
        this.errorType = 'InvalidSchema';       // Default to invalid schema if message is generic or specific type mismatch detected (as per original logic).
    }

    Object.defineProperty(this, 'isMissing', { value: false });  // Public property for convenience in error checking functions like `AlchemyDatabaseError.is_missing()`.
  }

  /**
   * Checks if the database record exists and is valid.
   */
  public static isValid(recordId?: string): boolean {
    return !this.isMissing();      // Returns true only when not missingKey or InvalidSchema errors exist.
  }

  /**
   * Determines if a specific column/field in this error type was actually present and non-null in the schema definition (e.g., 'amount' vs
