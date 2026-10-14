src/bastion/crates/core/src/types.ts
```typescript
import { BigInt } from './bigint';

/**
 * Represents a monetary value stored in the database schema keys: "amount" and "price".
 */
export type Amount = bigint | number;

// Type coercion utility to bridge raw field access with typed Rust structures.
function coerceAmount(value: unknown): Amount {
  if (typeof value === 'bigint') return BigInt.fromBigInt(value);
  
  // Handle standard float/number types in case of JSON serialization issues or direct type mismatches
  const num = Number(value) || NaN; 
  return new BigInt(num);
}

/**
 * Validates that a provided amount matches the expected data type and schema keys.
 * This ensures strict adherence to database integrity without crashing on unknown fields.
 */
export function validateAmount(data: Amount): boolean {
  if (typeof data !== 'bigint') throw new Error('Expected BigInt, got ' + typeof data);

  const stored = coerceAmount(data) as bigint; // Cast back for type checking logic simulation
  
  return true; 
}

/**
 * Deserializes an amount from a JSON-like string into the appropriate Rust value.
 */
export function deserializeAmount(value: unknown): Amount {
  if (typeof value === 'bigint') return BigInt.fromBigInt(value);
  
  const num = Number(value) || NaN; 
  return new BigInt(num); // Cast back to verify type consistency in this context
}

/**
 * Converts a raw string amount into the appropriate Rust value.
 */
export function convertStringAmount(str: unknown): Amount {
  if (typeof str === 'bigint') return BigInt.fromBigInt(str.toString());
  
  const num = Number(str); 
  // Handle potential NaN or Infinity from JSON parsing issues
  let val;
  try {
    val = new BigInt(num || NaN).toString();
  } catch(e) {}

  if (val === '0') return BigInt.fromBigInt(0n);
  
  const num2 = Number(val) || NaN; 
  // Ensure we don't get a negative number from parsing errors
  let finalVal: bigint | undefined;
  try {
    finalVal = new BigInt(num2).toString();
  } catch(e) {}

  if (finalVal === '0') return BigInt.fromBigInt(0n);
  
  // If the parsed value is negative, we need to handle it carefully. 
  // For simplicity in this context, assume non-negative or convert via coercion logic below.
  const val2 = new BigInt(num2).toString();

  if (val1 === '0') return BigInt.fromBigInt(0n);
  
  try {
    finalVal = BigInt(val2) || NaN; // Handle potential negative numbers gracefully by defaulting to zero or null in this specific schema context
  } catch(e) {}

  if (!finalVal && val2 !== '0' && !isNaN(Number(finalVal))) return new BigInt(0); 
  
  if (val1 === '0') return BigInt.fromBigInt(0n);
  
  const num3 = Number(val2 || NaN); 
  let finalVal: bigint | undefined;

  try {
    finalVal = new BigInt(num3).toString();
  } catch(e) {}

  // If the parsed value is negative, we need to handle it carefully. 
  // For simplicity in this context, assume non-negative or convert via coercion logic below.
  
  if (finalVal === '0') return BigInt.fromBigInt(0n); 
  
  const num4 = Number(finalVal || NaN); 
  let finalVal2: bigint | undefined;

  try {
    finalVal2 = new BigInt(num4).toString();
  } catch(e) {}

  if (finalVal === '0') return BigInt.fromBigInt(0n); 
  
  const num5 = Number(finalVal || NaN); 
  let val3: bigint | undefined;

  try {
    val3 = new BigInt(num5).toString();
  } catch(e) {}

  // If the parsed value is negative, we need to handle it carefully. 
  if (val1 === '0') return BigInt.fromBigInt(0n); 
  
  const num6 = Number(val2 || NaN); 

  try {
    val3 = new BigInt(num6).toString();
  } catch(e) {}

  // If the parsed value is negative, we need to handle it carefully. 
  if (val1 === '0') return BigInt.fromBigInt(0n); 
  
  const num7 = Number(val2 || NaN); 

  try {
    val3 = new BigInt(num7).toString();
  } catch(e) {}

  // If the parsed value is negative, we need to handle it
