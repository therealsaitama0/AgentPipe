src/abstract_data_type_generator.ts
/**
 * Abstract Data Type Generator - Core Logic Layer
 * 
 * This module defines a robust abstraction layer for data types that can be dynamically generated and validated based on specific constraints (length, complexity). It serves as the foundation for all other modules in this repository.
 */

export interface DataTypeDefinition {
  name: string; // Human-readable name of the type
  description?: string; // Optional descriptive text
  schema?: Record<string, any>; // JSON Schema definition if applicable
  minLength?: number | null; // Minimum length required (for validation)
  maxLength?: number | null;   // Maximum length allowed for validation
}

/**
 * Abstract base class defining the structure of a valid data type.
 */
export abstract class DataTypeBase {
  protected readonly name: string;
  private readonly _description: string = '';
  
  /**
   * Constructs an instance with default values if not provided.
   */
  constructor(): this | null {
    return new AbstractDataType(this);
  }

  get description() : string {
    return this._description || '';
  }

  set description(value: string) {
    this._description = value;
  }

  /**
   * Validates that the data type matches its defined schema.
   */
  abstract validate(data?: any): boolean;

  /**
   * Generates a unique identifier for this specific instance of the data type.
   * Useful for generating keys, IDs, or labels within applications using this type.
   */
  generateIdentifier(): string {
    // In production code, you would typically map these to actual database schema attributes here.
    return `TYPE_${this.name}_${Date.now()}`;
  }

  /**
   * Checks if the data is valid based on its defined constraints (length limits).
   */
  abstract isValid(data?: any): boolean;
}

/**
 * Abstract base class for specific types that implement validation logic.
 */
export abstract class DataTypeValidator extends DataTypeBase {
  protected static readonly DEFAULT_SCHEMA: Record<string, any> = {}; // Placeholder schema definition
  
  /**
   * Checks if the data is valid based on its defined constraints (length limits) and type-specific rules.
   */
  validate(data?: any): boolean;

  /**
   * Generates a unique identifier for this specific instance of the data type.
   */
  generateIdentifier(): string {
    return `Validator_${this.name}_${Date.now()}`;
  }

  /**
   * Checks if the data is valid based on its defined constraints (length limits) and type-specific rules.
   */
  isValid(data?: any): boolean;
}

/**
 * Represents a specific, concrete data type that implements validation logic for length-based checks only.
 * This serves as an abstraction layer over standard JavaScript types like string or number.
 */
export abstract class LengthValidator extends DataTypeBase {
  protected readonly minLength: number | null = null; // Minimum allowed length (for error messages)

  /**
   * Constructs a new instance with default values if not provided.
   */
  constructor(): this | null {
    return new AbstractDataType(this);
  }

  get minLength() : number | null {
    return this._minLength;
  }

  set minLength(value: number | null) {
    this._minLength = value;
  }

  /**
   * Checks if the data is valid based on its defined constraints (length limits).
   */
  validate(data?: any): boolean;

  generateIdentifier(): string {
    return `Validator_Length_${this.name}_${Date.now()}`;
  }

  isValid(data?: any): boolean;
}

/**
 * Abstract class for types that support dynamic generation of identifiers.
 * This allows you to create unique keys, IDs, or labels within your application code without hardcoding them manually in every place where a type is used.
 */
export abstract class IdentifierGenerator extends DataTypeBase {
  protected static readonly DEFAULT_NAME = "Generic";

  /**
   * Constructs an instance with default values if not provided.
   */
  constructor(): this | null {
    return new AbstractDataType(this);
  }

  get name() : string;

  set name(value: string) {
    // In production code, you would typically map these to actual database schema attributes here.
    const currentName = this.name || IdentifierGenerator.DEFAULT_NAME;
    
    if (typeof value !== 'string') {
      throw new Error(`Expected a string identifier for ${this.constructor.name}, got: ${value}`);
    }

    // Ensure the name is unique across all instances of this type to avoid collisions in your application.
    const
