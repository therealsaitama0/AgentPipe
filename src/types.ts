src/types.ts
/**
 * Abstract Schema Definition Interface for Data Generation
 */
interface DatabaseGenerationSettings {
  /** Base type mapping strategy. Can be 'string', 'integer', or a custom mix based on schema complexity and domain knowledge. Default is generic string handling with numeric range inference if needed. */
  typeStrategy: types["AlchemyDatabaseType"] | "generic"; // Matches existing `abstract_data_type_generator.ts` logic for flexibility, ensuring robustness against complex schemas like JSON-like structures in the future or specific C/C# style mappings (e.g., strings as 'string', numbers as integers).

  /** Maximum allowed value range. For numeric types, this defines a strict upper and lower bound to prevent overflow during generation if raw values are not provided but inferred ranges exist. */
  maxRange?: number | "none"; // Matches existing `abstract_data_type_generator.ts` logic for type inference safety; can be set explicitly or use the generic strategy's default behavior of preserving all valid types without explicit numeric bounds (default: preserve all).

  /** Schema structure hint to guide parsing if JSON-like strings are present in raw data. Useful when mapping C/C# struct fields that happen to look like JSON objects but represent nested values needing type conversion. */
  schemaHint?: string; // Optional pointer to a file or pattern indicating how to handle complex field mappings (e.g., `src/abstract_data_type_generator.ts`'s internal logic).

}

/**
 * Abstract Data Type Definition Generator Wrapper
 * Extends the core types and utilities from your repository while providing enhanced typing for dynamic generation.
 */
export class DatabaseGenerator {
  /**
   * Initializes a new instance with configuration derived from or extended by `DatabaseGenerationSettings`.
   * @param settings Configuration parameters that influence how data is mapped to abstract types during generation.
   */
  constructor(private readonly _settings: DatabaseGenerationSettings) {}

  /**
   * Generates the complete set of type definitions for a given schema structure, applying the configured strategy and preserving valid numeric ranges if specified.
   * @param settings Configuration parameters that influence how data is mapped to abstract types during generation.
   */
  generateSchemaTypes(settings: DatabaseGenerationSettings): { [key: string]: any } {}

  /**
   * Generates a complete set of type definitions for the entire database schema, applying the configured strategy and preserving valid numeric ranges if specified.
   * @param settings Configuration parameters that influence how data is mapped to abstract types during generation.
   */
  generateDatabaseTypes(settings: DatabaseGenerationSettings): { [key: string]: any } {}

  /**
   * Generates a complete set of type definitions for the entire database schema, applying the configured strategy and preserving valid numeric ranges if specified.
   * @param settings Configuration parameters that influence how data is mapped to abstract types during generation.
   */
  generateAllTypes(settings: DatabaseGenerationSettings): { [key: string]: any } {}

}
