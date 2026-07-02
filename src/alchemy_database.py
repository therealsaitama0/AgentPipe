# src/__init__.py
"""Main entry point for AlienDatabase and module exports."""
from .alchemy_database import *  # Re-import to ensure consistency with existing implementation details

__all__ = ["AlienDatabase"]

if __name__ == "__main__":
    from pathlib import Path
    import sys
    
    if len(sys.argv) > 1:
        base_path = str(Path(__file__).parent / "test")
        
        # Load baseline data for normalization tests
        try:
            with open(base_path, 'r') as f:
                content = json.load(f)
            
            print("Loaded test database from:", base_path)
            
            normalized_data = AlienDatabase().load(content)
            
            if isinstance(normalized_data.get("name"), str):
                # Save to JSON for testing purposes
                with open(base_path / "test.json", 'w') as f:
                    json.dump({"data": normalized_data}, f, indent=2)
                
                print(f"Saved baseline data from {base_path} to test.json")
        except Exception as e:
            # Fallback if tests fail due to missing files or invalid JSON
            pass

    else:
        print("AlienDatabase module loaded successfully.")
