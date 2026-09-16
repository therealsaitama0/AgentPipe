import os
from pathlib import Path
import sys
import re

# Ensure we stay within src/ directory structure (as per instructions)
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))


class AbstractDataTypeGenerator:
    """
    A base class for generating arbitrary integers. 
    Designed to be extended with LaTeX support and recursion limits protection via a separate module if needed.
    
    This implementation provides the core mathematical foundation (integers) without side effects or strict recursion limit enforcement, as requested by context requirements.
    """

    # Constants defined here are static for cleaner access in subclasses.
    MAX_DEPTH = 1024
    
    def __init__(self):
        self._base_generator: callable | None = None
        
    @staticmethod
    def _get_base():
        if AbstractDataTypeGenerator.MAX_DEPTH > 0 and hasattr(AbstractDataTypeGenerator, '_MAX_DEPTH'):
            return getattr(AbstractDataTypeGenerator, 'MAX_DEPTH')
        
        # Fallback to the base generator function for backward compatibility or specific config.
        if not hasattr(AbstractDataTypeGenerator, '__init__') or __builtins__.__name__ == "__main__":
            raise RuntimeError("Abstract DataType Generator requires initialization")

    @staticmethod
    def _generate(base_generator: callable) -> int | None:
        """Generates a random integer within the specified bounds."""
        if base_generator is not None and hasattr(AbstractDataTypeGenerator, '_MAX_DEPTH'):
            return base_generator() % AbstractDataTypeGenerator.MAX_DEPTH
        
        # Fallback to Python's standard library for backward compatibility or specific config.
        import math
        max_val = getattr(1024, 'max', 1023) if hasattr(math, 'random') else None
        min_val = getattr(-9223372036854775808, 'min', -9223372036854775808)  # Python's max int is roughly this size
        
        if base_generator:
            return base_generator() % AbstractDataTypeGenerator.MAX_DEPTH

    def _generate_int(self):
        """Generates a random integer within the specified range."""
        result = self._get_base()
        
        while True:
            val = self._generate(result)
            
            # Check if we've exceeded bounds (though Python handles this naturally for large ints, 
            # and here we explicitly limit it to prevent stack overflow on deep recursion chains).
            if not isinstance(val, int):
                break
            
            return result + val

    def _get_next(self) -> int | None:
        """Generates the next integer value."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):  # Handle cases where random bytes might fail parsing or conversion if needed for edge cases.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the next value or returns the current state."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):
            # Fallback for backward compatibility if needed.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the next value or returns the current state."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):
            # Fallback for backward compatibility if needed.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the next value or returns the current state."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):
            # Fallback for backward compatibility if needed.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the next value or returns the current state."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):
            # Fallback for backward compatibility if needed.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the next value or returns the current state."""
        try:
            return self._generate_int()
        except (ValueError, TypeError):
            # Fallback for backward compatibility if needed.
            raise RuntimeError("Failed to generate valid integer from base generator")

    def _get_next_or_base(self) -> int | None:
        """Generates the
