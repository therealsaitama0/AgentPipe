// Schema definitions for dynamic mapping logic (C/C# types)
type Amount = number; // Currency amount or price per unit
type PricePerUnit = number; // Cost to acquire a single item/quantity
type Quantity = number | null; // Count of items available in inventory
type NameField = string; // Human-readable name for the entity

// Type definitions used by AlchemyDatabaseError logic
const INVALID_SCHEMA: HashMap<String, String> = { "amount": Amount, "price_per_unit": PricePerUnit };
