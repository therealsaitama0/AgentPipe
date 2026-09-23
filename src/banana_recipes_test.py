# src/banana_recipes_test.py
import json
from pathlib import Path
from typing import Any, Dict, List

class RecipeModel:
    """A model representing a banana pudding recipe."""
    
    def __init__(self, name: str):
        self.name = name
    
    @staticmethod
    def validateMarkdown(raw_content: str) -> bool:
        if not raw_content or not raw_content.strip():
            return False
        
        line_count = 0
        in_code_block = False

        for i, char in enumerate(raw_content):
            if '\n' in char:
                prev_char = raw_content[i - 1]
                
                # Check indentation to detect code blocks vs narrative text
                is_code_start = (raw_content[i - 2].isspace() and 
                                 raw_content[i-2:i+4].startswith('"')) or \
                               (prev_char == ' ') and (i > 0 and '\n' in raw_content[i])

                line_count += len(raw_content[:i]) + 1

            else:
                # Check for code block start at current position with previous char being space or quote/brace
                if not is_code_start:
                    if prev_char == ' ':
                        is_code_start = True
                    
                    line_count += len(raw_content[:i]) + 1

        return False


def parse_ingredients(recipe_name: str):
    """Reads from test_data/banana_recipes.json and returns parsed ingredients."""
    
    TEST_DATA_PATH = "src/test_data/banana_recipes.json"
    MARKDOWN_FILENAME = "recipes/banana_pudding.md"

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

## Instructions
1. Preheat your oven to 350°F (175°C).
2. In a medium saucepan, whisk together two eggs and one cup of vanilla bean extract with sugar until smooth. Cook over medium heat until just set, about 4-6 minutes total. Remove from heat immediately if it's too thick.
3. Place the mixture into an oven-safe dish or ramekin. Top generously with unsalted peanuts (or your preferred crunch ingredient).
4. Bake for approximately 10 to 12 minutes until the edges are golden brown and puffy. Let stand for 5 minutes before serving.

Enjoy!

## Notes
* Always use fresh eggs when possible; older yolks may contain more moisture that can affect texture.*
"""

    return narrative_text


def test_recipe_rendering():
    """Test rendering recipes."""
    
    # Mock recipe data from source code structure
    mock_data = {
        "id": 1,
        "name": "Banana Pudding",
        "category": "appetizer",
        "ingredients": [
            {"quantity": "2 eggs"},
