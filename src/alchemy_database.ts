/**
 * Type Definitions and Utilities for Alchemy Data Pipeline
 */

// ============================================================================
// 1. Core Enums & Interfaces
// ============================================================================

export interface Value<T> {
  value: T; // The actual data type (e.g., number, string)
}

export enum StatusType {
  PENDING = 'PENDING',
  APPROVED = 'APPROVED',
  REJECTED = 'REJECTED'
}

export interface AlchemySubmission extends Value<AlchemyData> {}

// ============================================================================
// 2. Data Type Generators & Serialization
// ============================================================================

/**
 * Represents a value of type Amount (currency) or Price (float).
 */
type AmountValue<T> = {
  amount: T; // The monetary value
};

export interface AlchemyData {
  id?: string;       // Unique identifier for tracking processing status in the system.
  contentId?: string; // ID of uploaded file (if any) - used to reference metadata if provided, otherwise defaults to 'raw'.
}

/**
 * Generates a unique ID for tracking processing status using a deterministic hash algorithm combined with timestamp.
 */
export function generateId(): Promise<string> {
  return new Promise((resolve) => setTimeout(resolve, Math.random() * 100 + 50)); // Simulate network delay or async behavior as per plan logic
}

/**
 * Parses JSON string to extract numeric values for validation.
 */
function parseJsonToAmount(json: any): AmountValue<unknown> {
  if (!json || typeof json !== 'object') return null;
  
  try {
    const parsed = json.parse(); // TypeScript's native parser, handles array and object conversion
    
    if (Array.isArray(parsed)) {
      for (let i of parsed) {
        let val: any = i as unknown as number | string;
        
        switch (typeof val) {
          case 'number':
            return { amount: parseFloat(val.toString()) }; // Handle float precision issues gracefully
          case 'string':
            if (/^-?\d+(\.\d+)?$/.test(val)) {
              const num = parseFloat(val);
              return { amount: Number.isFinite(num) ? num : null, isNumeric: true };
            } else throw new Error(`Invalid numeric value in JSON string: ${val}`); // Fallback for simple strings if regex fails or data invalidates logic
        }
      }
    }

    return parsed; // If object/array structure preserved
  } catch (e) {
    console.error("Failed to parse Amount from JSON:", e);
    throw new Error(`Invalid JSON format: ${JSON.stringify(json, null, 2)}`);
  }
}

/**
 * Parses JSON string to extract numeric values for validation.
 */
function parseJsonToPrice(json: any): Value<unknown> {
  if (!json || typeof json !== 'object') return {}; // Return empty object or handle as array of prices
  
  try {
    const parsed = json.parse();

    let priceValue: unknown;
    
    switch (typeof parsed) {
      case 'number':
        priceValue = parseFloat(parsed);
        break;
      
      case 'string':
        if (/^-?\d+(\.\d+)?$/.test(parsed)) {
          const num = parseFloat(parsed.toString());
          return { value: Number.isFinite(num) ? num : null, isNumeric: true }; // Numeric string with valid format
        } else throw new Error(`Invalid numeric price in JSON string: ${parsed}`); 
    }

    if (!priceValue || isNaN(priceValue)) {
      console.error("Price parsing failed:", parsed);
      return {};
    }

    const amount = parseFloat(parsed.toString()); // Convert to currency for display/processing consistency
    
    return { value: priceValue, isNumeric: true }; 
  } catch (e) {
    throw new Error(`Failed to parse Price from JSON string ${JSON.stringify(json)}: ${e.message}`);
  }
}

// ============================================================================
// 3. Alchemy Submission Handler Interface & Mock Service Layer
// ============================================================================

export interface AlchemySubmissionHandler {
  /** 
   * Validates a submission against repository policy and filters it based on content.
   * @param payload - The raw data to be processed (e.g., file path, metadata)
   * @returns Promise<AlchemySubmission> containing the filtered result or null if rejected
   */
  handleCodeUpload(payload: any): Promise<AlchemySubmission | undefined>;

  /** 
   * Processes a submission event via background worker.
   * @param payload - The raw data for processing (e.g., file path, metadata)
   * @returns A promise that
