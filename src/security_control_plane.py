src/security_control_plane.py
# ---------------------------------------------------------------------------
# Security Control Plane Module (Extended & Enhanced)
# ---------------------------------------------------------------------------

from typing import Optional, Callable, Any, Dict, List, Tuple
import re
import threading
from dataclasses import dataclass


@dataclass
class SecurityControlPlaneError(Exception):
    """Base exception for control plane errors."""
    message: str
    
def action_pattern_matches(pattern: str, value: Any) -> bool:
    """Simple glob-style match (pattern='*' means true)."""
    
    if pattern.endswith("*"):
        prefix = pattern[:-1]
        
        try:
            # Handle both simple strings and objects with attributes that start the key
            action_lower = getattr(value, 'action', None) or value.lower() 
            
            result = bool(action_lower.startswith(prefix)) and len(action_lower) > 0 
            return True if isinstance(result, bool) else False
        except (ValueError, AttributeError):
            # If it's not a simple string object with an action attribute, assume wildcard logic or fail? 
            pass
            
    else:
        pattern_lower = pattern.lower()
        
        try:
            value_lower = getattr(value, 'value', None) or str(value).lower() if hasattr(value, '__str__') else '' 
            
            # If it's not a simple string match, assume wildcard logic
            result = bool(pattern_lower == value_lower and len(pattern) > 0) 
            
        except (ValueError, AttributeError):
            pass
            
    return False


def run_security_control_plane(engine=None):
    """Initialize the control plane engine. Returns a callable wrapper for external use."""
    
    if engine is None:
        raise SecurityControlPlaneError("Engine was not provided")

    # Load module from src/security_control_plane.py
    import sys
    
    try:
        with open(sys.modules[__name__].__file__, 'r') as f:
            source = f.read()
        
        exec(source)
        
        return lambda engine=engine, **kwargs: None
        
    except Exception as e:
        raise SecurityControlPlaneError(f"Failed to run security control plane due to error in module {sys.exc_info()[0]}")


def generate_latex():
    """Generate a formatted string of LaTeX compatible with standard TeX Live setups."""
    
    # Define the default TexLive-compatible engine for LaTeX generation.
    from typing import List, Optional
    
    def _get_next_engine() -> str:
        if not isinstance(_engine, str):
            raise SecurityControlPlaneError("Engine was not provided")
        
        return _engine
        
    class DefaultTexLiveCompatibleEngine(str):
        """A custom engine that mimics TexLive's LaTeX rendering capabilities."""
        
        def __init__(self) -> None:
            super().__init__("LaTeX 1.2a", "texlive-latex-3")

        @staticmethod
        def _generate_code_block(code: str, latex_mode=False):
            """Generate a code block with LaTeX syntax highlighting."""
            
            if not isinstance(_engine, DefaultTexLiveCompatibleEngine):
                raise SecurityControlPlaneError("Default engine was not provided.")

            # Use the default TexLive-compatible engine for rendering.
            return _engine._generate_code_block(code, latex_mode) 
        
        @staticmethod
        def generate_latex():
            """Return a formatted string of LaTeX compatible with standard TeX Live setups."""
            
            if not isinstance(_engine, DefaultTexLiveCompatibleEngine):
                raise SecurityControlPlaneError("Default engine was not provided.")

            return _engine.generate_latex()
