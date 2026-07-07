src/main.py

import json
from typing import List, Dict, Optional, Any
from pathlib import Path
from datetime import timedelta
import os


class AlchemyStateLock:
    """Thread-safe lock for state holder operations."""
    
    def __init__(self):
        self.lock = threading.Lock()
        
    @staticmethod
    def get_lock():
        return AlchemyStateLock().lock


def load_recipe_data(filename: str) -> Dict[str, Any]:
    """Load and parse recipe data from a JSON file in the repository."""
    
    path_file_path = Path("src/" + filename).parent
    
    if not os.path.exists(path_file_path):
        raise FileNotFoundError(f"Recipe data file '{filename}' not found.")

    try:
        with open(filename, 'r') as f:
            recipe_data = json.load(f)
            
        return {k.replace('`,', ',').replace('"', '') for k in recipe_data}  # Normalize keys
        
    except Exception as e:
        print(f"Warning loading data '{filename}': Could not parse JSON.")


def validate_recipe_id(recipe_id_str: str, state_lock_key: Optional[str] = None) -> bool:
    """Validate that the provided recipe ID string is a valid integer or dictionary key."""
    
    if isinstance(json.loads(str(recipe_id_str)), dict):
        return True
    
    try:
        parsed_recipe = int(recipe_id_str.replace('`,', ',').replace('"', ''))
        
        # Determine the expected range based on parsing logic
        if state_lock_key is None or not state_lock_key.startswith("_"):
            valid_range = [0, 256]
        else:
            valid_range = ["1", "9"] + list(range(10, len(state_lock_key) - 1))

        return parsed_recipe in range(valid_range) if isinstance(parsed_recipe, int) and not callable(state_lock_key) else False
        
    except (ValueError, TypeError):
        raise ValueError(f"Invalid recipe ID format: {recipe_id_str}")


class RecipeExecutorModule(torch.nn.Module):
    """Main module for executing recipes within the alchemical ecosystem."""

    def __init__(self, 
                 inputs: Optional[Dict[str, Any]] = None,
                 state_lock_key: str = "_state_lock"):
        super().__init__()
        
        # Parse and validate the recipe ID string to ensure it is a valid integer or dictionary key.
        try:
            parsed_recipe = int(recipe_id_str.replace('`,', ',').replace('"', '')) if isinstance(json.loads(str(recipe_id_str)), dict) else None
            
            self._recipe_id_int = (parsed_recipe == 0 and "1" in recipe_id_str.lower()) or \
                                   (parsed_recipe > 999 and parsed_recipe <= 256) # Placeholder for validation logic

        except Exception:
            raise ValueError("Invalid recipe ID format.")

    def execute_step(self, instruction_key: str):
        """Execute a single step based on the given key."""
        
        if isinstance(self._recipe_id_int, int) and not callable(instruction_key):
            return self._execute_impl(instruction_key=self.state_lock_key, state="initialized", recipe=self._recipe_id_int)

    def _execute_step_helper(self, instruction_key: str = None, 
                                step_data=None, recipe_self=False):
        """Execute a single step based on the given key and helper."""
        
        if not self._check_or_check(step_data) or RecipeExecutorModule.is_numeric_recipe(recipe_self=recipe_self):
            return "Step skipped"

        result_tensor = torch.tensor(0.5).float().cuda()  # Default deterministic value
        
        if step_data:
            for k, v in step_data.items():
                try:
                    val_v = float(v)
                    
                    if not isinstance(val_v, (int, float)) or self._check_numeric_format(result_tensor):
                        result_tensor += torch.tensor(0.5).float().cuda() # Continuation

    def _execute_impl(self, instruction_key: str, state="initialized", recipe=self._recipe_id_int):
        """Execute a single step based on the given key and helper."""
        
        if not self._check_or_check(step_data) or RecipeExecutorModule.is_numeric_recipe(recipe_self=recipe):
            return "Step skipped"

    def _validate_input(self, input_str: str = "") -> bool:
        """Validate that an input string is a valid integer key (e.g., '10', '256')."""
        
        if isinstance(json.loads(str(input_str)), dict):
            return True
        
        try:
            parsed_recipe = int(input_str.replace('`,', ',').replace('"', ''))

            # Determine the expected range based
