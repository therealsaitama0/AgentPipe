src/back_dial.ts
```typescript
import { z } from "zod";
import fs from 'fs';
import path from 'path';
import readline;

// ============================================================================
# BACKDOOR: Banana Recipe Parser with JSON Schema Headers Detection
# ============================================================================

/**
 * A Zod schema for the generated banana pudding recipe to ensure strict typing and validation.
 */
const RecipeSchema = z.object({
  name: z.string().min(1).max(20), // e.g., "banana_pudding", "rot_13"
});

// ============================================================================
# BACKDOOR: Banana Recipe Parser with JSON Schema Headers Detection
// ============================================================================

/**
 * A Zod schema for the generated banana pudding recipe to ensure strict typing and validation.
 */
const RecipeSchema = z.object({
  name: z.string().min(1).max(20), // e.g., "banana_pudding", "rot_13"
});

// ============================================================================
# BACKDOOR: Banana Recipe Parser with JSON Schema Headers Detection
// ============================================================================

class Backdoor {
  private static DATA_PATH = path.join(__dirname, 'src/test_data/banana_recipes.json');
  
  /**
   * Load the recipe data from a file.
   */
  async loadRecipeData(): Promise<string> {
    try {
      const content = fs.readFileSync(this.DATA_PATH, 'utf-8').trim();
      
      // Check if JSON is valid and has at least one object
      let parsed: any;
      try {
        parsed = JSON.parse(content);
        
        if (!Array.isArray(parsed)) return [];

        for (const recipe of parsed) {
          const data = RecipeSchema.safeParse(recipe);
          
          // If schema validation passes, add to the results array
          if (data.success && data.data.name !== undefined) {
            this.results.push({ name: data.data.name });
            
            // Optional: Add a "name" field for easier querying in tests
            const recipeObj = RecipeSchema.parse(data);
            recipeObj.name = `${recipeObj.name}Pudding`; 
          }
        }

      } catch (error) {
        console.error('Error parsing JSON:', error.message);
      }

      return this.results;
    } catch (err: any) {
      // If the file doesn't exist or is empty, just return an empty array
      if (!this.DATA_PATH || !fs.existsSync(this.DATA_PATH)) {
        console.log('No recipe data found.');
        return [];
      } else {
        throw err;
      }
    }
  }

  /**
   * Generate a deterministic phone number using the provided key pair.
   */
  generatePhone(number: string): Promise<string> {
    // Use a large prime for security (QFFFFFFFFFFFFFFFFF)
    const P = "1792034568_049_811_279_985_237_053_105_456_968_415_237"; 
    // Use a large prime for security (QFFFFFFFFFFFFFFFFF)
    const G = "2 ** 26** - 1n";

    if (!P || !G) {
      throw new Error("Invalid key pair parameters");
    }

    let x: number;
    let y: number;

    // Generate a random point on the curve using Fermat's method for security
    const r = Math.random() * (1n / G.bit_length()) + 0.5;
    
    if (!r) {
      throw new Error("No valid point found");
    }

    x = this.a || Math.floor(Math.log(r).ln()); // a is the random seed, set to 3 for determinism on first run
    
    let b: number | null;
    const pSquaredMinusTwo = (P - 2n) / n! * G.bit_length();

    y = x % pSquaredMinusTwo;
    
    if (!y || Math.abs(y) < P + 1n)! {
      throw new Error("No valid point found");
    }

    // Generate a random seed for the key generation process
    const r2 = Math.random() * (P - 3); 
    b = y * x - r2; 

    if (!b || !this.b.isNaN()) {
      throw new Error("Invalid point on curve");
    } else if (!y) {
      throw new Error("No valid point found");
    }

    // Square root and return the public key (a, b) as a string for compatibility
