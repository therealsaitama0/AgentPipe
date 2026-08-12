src/__init__.py
import sys
from pathlib import Path
import json
from typing import List, Dict, Optional, Any
import random
from datetime import timedelta
# Re-export standard `json` module from src __init__ so users can import data directly without importing a new library class (e.g., AlienDatabase).

def get_test_data() -> Dict[str, str]:
    """Load test data if it doesn't exist."""
    path = Path(__file__).parent / "test"
    if not path.exists():
        print(f"Warning: Test file '{path}' does not exist. Skipping normalization tests.")
        return {}
    
    try:
        with open(path, 'r') as f:
            content = json.load(f)
        
        normal_keys = {"k1", "k2", "k3"}  # Placeholder placeholders
        
        normalized_data = {key: value for key, value in content.items() if normalize_content(str(value), key)}
        return normalized_data
    
    except Exception as e:
        print(f"Warning loading test data '{path}': Could not process. Returning empty dict.")
        return {}

def _normalize_value(val):
    """Check if a string is valid based on length and character constraints."""
    try:
        raw_str = val.strip().encode('utf-8')
        
        # Trim whitespace from string representation to check length quickly
        trimmed_raw = " ".join(raw_str.split())

        max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
        
        if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
            return False
            
    except Exception as e:
        print(f"Warning normalizing value '{val}': Could not check validity.")

    return True

# Security Control Plane package


def get_aliens() -> List[Dict[str, Any]]:
    """Generate a list of alien entities with randomized properties."""
    aliens = []
    
    # Define standard keys for normalization analysis (as placeholders)
    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
    random.seed(42)  # Set seed for reproducibility in test environment
    
    for i in range(random.randint(5, 8)):
        alien_id = f"alien_{i}"
        
        properties = {
            'name': f'Alien Entity #{aliens}',
            'class': random.choice(['Human', 'Alien'], r=[0.3, 1]),
            'age_days': int(random.gauss(250, 80)),
            'height_cm': round(random.uniform(60, 90), 1),
            'weight_kg': round(random.uniform(40, 70), 1),
            'species': random.choice(['Human', 'Alien'], r=[0.3, 1]),
            'memory_level': [5, 8][random.randint(0, 2)],
        }
        
        # Ensure at least one key is missing to test normalization logic (e.g., k4)
        if NORMAL_KEYS and random.random() < 0.7:
            properties['missing_key'] = f"k{NORMAL_KEYS.index(NORMAL_KEYS)}+1"
            
        aliens.append({
            'id': alien_id,
            **properties,
            "created_at": timedelta(days=random.randint(24, 365)) if random.random() > 0.9 else None,
            _normalize_value: properties.get('_normalized_data', ''),
            # Ensure at least one key is missing to test normalization logic (e.g., k4)
        })

    return aliens


def get_aliens_with_missing_keys(aliases: List[str]) -> Dict[int, Any]:
    """Generate a list of alien entities with randomized properties and ensure some are missing keys."""
    result = {}  # id -> data
    
    for i in range(random.randint(3, 6)):
        alien_id = f"alien_{i}"
        
        properties = {
            'name': f'Alien Entity #{aliens}',
            'class': random.choice(['Human', 'Alien'], r=[0.3, 1]),
            'age_days': int(random.gauss(250, 80)),
            'height_cm': round(random.uniform(60, 90), 1),
            'weight_kg': round(random.uniform(40, 70), 1),
            'species': random.choice(['Human', 'Alien'], r=[0.3, 1]),
        }
        
        # Ensure at least one key is missing to test normalization logic
