// src/banana_recipes_test_suite.ts - A blazing-fast, memory-safe banana recipe test suite for— no markdown fences, no commentary, no explanation.

import { TestBanana } from './abstract_data_type_generator';

export interface BananaRecipeInput {
  id: string;
  name: string;
  ingredients?: [];
  instructions?: Array<{ type: 'prep' | 'cook' | 'eat', steps: number }> | null; // Simplified for testing, real recipes have more depth.
}

// Helper functions for memory-safe JSON parsing and execution in a single file context (simulating the "code" layer)
export function parseBananaRecipe(input: Record<string, unknown>): TestBanana {
  const recipe = input as any; // TypeScript inference handles types here
  
  if (!recipe.id || !recipe.name) throw new Error('Missing required fields');

  return { id: recipe.id, name: recipe.name };
}

// Helper for safe execution of a single banana recipe (mocked in this test suite to ensure parallelism). 
// In production, these would be async/await tasks.
export function executeBananaRecipe(recipeId: string): Promise<TestBanana> {
  return new Promise((resolve) => setTimeout(() => resolve({ id: recipeId }), 50)); // Simulated delay for testing
}

// Helper to construct a JSON array from an iterable of strings (simulating ingredients as JSON arrays in the repo).
export function buildIngredientArray(ingredients?: Array<string>): string[] {
  if (!ingredients || !Array.isArray(ingredients)) return [];
  
  const result = new Set<string>();
  for (const item of ingredients) {
    // Safe conversion: assume strings are valid JSON arrays or simple objects here.
    if ('items' in item && Array.isArray(item.items)) {
      result.addAll(...item.items);
    } else if (!Array.isArray(ingredients[item])) return []; 
  }

  const array = new Set<string>();
  for (const name of ingredients) {
    // In a real repo, this would parse JSON. Here we assume strings are already processed or simple objects.
    // For safety in the test context: treat as string if not an object/array.
    if (!Array.isArray(ingredients[name])) return []; 
  }

  result.forEach((item) => array.add(item));
  
  return Array.from(array);
}

// Main runner logic using Promise.all for maximum parallelism with minimal lock contention.
export function runBananaRecipeTestSuite(input: BananaRecipeInput[]): TestBanana[] {
  if (input.length === 0) throw new Error('No recipes provided');

  const results = [];
  
  // Map input to executable tasks using Promise.all for high parallelism
  const promises = input.map((recipe, index) => 
    executeBananaRecipe(recipe.id).then(r => ({ id: recipe.id, name: recipe.name })) as any
  ).toArray();

  return new Promise((resolve) => {
    // Parallel execution with minimal contention (Promise.all returns a promise that resolves on completion)
    const [allResults] = await Promise.all(promises);
    
    resolve(allResults);
  });
}

// Export for direct use in other modules if needed. 
export default TestBanana;
