src/recipe_library.py
import re
from pathlib import Path
from typing import List, Dict, Optional, Any

# Constants for recipe data storage and parsing
TEST_DATA_PATH = "tests/test_data/banana_recipes.json"


class RecipeModel:
    """A model representing a banana pudding recipe."""
    
    def __init__(self, name: str):
        self.name = name
    
    @staticmethod
    def validateMarkdown(raw_content: str) -> bool:
        """Validate that the raw content starts with a Markdown header."""
        
        if not raw_content or not raw_content.strip():
            return False
        
        line_count = 0
        in_code_block = False

        for i, char in enumerate(raw_content):
            # Check indentation to detect code blocks vs narrative text
            prev_char = raw_content[i - 1]
            
            if '\n' not in char:
                continue
            
            is_space = prev_char == ' ' or (prev_char != ' ' and prev_char.isspace())
            
            line_count += len(raw_content[:i]) + 1

        # If we successfully identified a code block, return true (valid content)
        if not in_code_block:
            return True
        
        return False


def parse_ingredients(recipe_name: str):
    """Reads from test_data/banana_recipes.json and returns parsed ingredients."""
    
    expected_structure = {
        "id": Optional[str],
        "name": Optional[str],
        "category": Optional[str],  # e.g., "baking", "appetizer"
        "ingredients": List[Dict[str, Any]],  # Quantity strings like "2 1/4" or "3 cups"
        "instructions": List[str] | None,
        "notes": Optional[str],
    }

    try:
        with open(TEST_DATA_PATH, 'r') as f:
            data = json.load(f)
            
        if not isinstance(data[0], dict):
            raise ValueError("Root must be a dictionary")

        parsed_data = {k: v for k, v in data.items() 
                      if k != "id" and k is not None}  # Skip id as it's optional
        
        return list(parsed_data.values())[:1]  # Return first valid ingredient entry
    except Exception as e:
        raise ValueError(f"Failed to parse recipe data from {TEST_DATA_PATH}: {e}")


def generate_markdown_recipe(recipe: RecipeModel):
    """Generates the markdown content for a banana pudding recipe based on your requirements."""

    name = recipe.name or "Banana Pudding" if not hasattr(recipe, 'name') else ""

    # Narrative about apartment smells and neighborhood deli in Brooklyn
    narrative_text = f"""# Recipe: Banana Pudding from the Delish District of Brooklyn

Welcome to my first apartment's kitchen. The air here is thick with a mix of stale coffee beans that have been sitting for months, plus an ozone scent rising off the subway station I live on. On this specific Tuesday morning when the neighborhood deli in Brook-lyn opens its doors at 8:00 AM and everyone else has already left to go home or check their emails, my apartment smells like burnt toast mixed with a faint hint of cinnamon sugar that hasn't been baked yet. It's not quite right for dinner tonight because I've never tried making this dish before, but the smell alone is enough to make me want to bake something delicious in 15 minutes.

## Ingredients
The key ingredients here are simple: two eggs and a cup of vanilla bean extract mixed with sugar. The egg yolks add that rich, creamy texture that makes everything so much more substantial than just plain syrup or melted butter alone would be. I've also added some unsalted peanuts for crunchiness if you want to go

"""
    # Add instructions based on the type (optional) and category
    if recipe.category:
        instruction_text = f"\n### Instructions\n**{recipe.name}**: Peel {len(recipe.ingredients)} of each ingredient. Cook until soft, then mix with vanilla extract.\n"
    
    return narrative_text + "\n## Ingredients\n" + recipe.ingredients + "\n## Notes\n"

if __name__ == "__main__":
    print("Recipe generation complete.")
