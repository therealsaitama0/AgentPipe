src/__init__.py
"""
Plugin Registry and Central Manager for Signal Processing Libraries.

This module serves as the entry point to all external signal processing modules (e.g., banana_bouncing.py), ensuring they are loaded via a centralized manager, exported to the broader system architecture, and adhering strictly to the plugin infrastructure defined in `src/alchemy_manager.py`.
"""

import os
from pathlib import Path
from typing import Optional, List, Dict, Any, TypeVar

# ============================================================================
# TYPE DEFINITIONS: The Plugin Interface
# ============================================================================
T = type('Plugin', (object,), {
    'name': str,          # Unique identifier for the plugin
    'version': int,        # Version of this specific implementation
    'description': str,   # Human-readable description
    'signature': bytes,   # Optional: Signature for hashing or verification
})

# ============================================================================
# CORE LOGIC & MANAGER (Based on inspiration from alchemy_manager.py)
# ============================================================================
class PluginRegistryManager:
    """Central registry and manager class handling all plugin imports."""

    def __init__(self):
        self._plugins_loaded = {}  # key -> type of loaded module/instance
        self._plugin_classes: Dict[str, Type] = {}  # name -> Class instance (for dynamic loading)
        
    @staticmethod
    def load_plugin(name: str, plugin_module_path: Path) -> Optional[Type]:
        """Load a specific plugin based on its registered class."""
        if not os.path.exists(plugin_module_path):
            raise FileNotFoundError(f"Plugin module '{name}' not found at {plugin_module_path}")

        # Try to load the Python file directly first (for backward compatibility with existing code)
        try:
            python_script = plugin_module_path / 'python'
            if python_script.is_file():
                return importlib.import_module(f'src.{name}.py')  # Re-importing for dynamic loading logic
        except ImportError as e:
            pass

        # Try to load the TypeScript/JavaScript wrapper (if present) or a .ts/.js file directly
        if name.endswith('.ts'):
            ts_path = plugin_module_path / 'typescript'
            if ts_path.is_file():
                return importlib.import_module(f'src.{name}.tsx')  # Re-importing for dynamic loading logic

        try:
            js_script = plugin_module_path / 'javascript'
            if js_script.is_file() or (js_script.exists and js_script.suffix == '.ts'):
                return importlib.import_module(f'src.{name}.jsx')
            
            ts_js_path = plugin_module_path / 'typescript.js'
            if ts_js_path.is_file():
                return importlib.import_module(f'src.{name}.tsx')

        except ImportError:
            pass  # Fallback to direct Python execution for legacy modules or missing wrappers

        raise RuntimeError(
            f"Could not determine the correct plugin module type. "
            f"Plugin '{name}' is registered but could not be identified."
        )

    def register_plugin(self, name: str, class_type: Type) -> None:
        """Register a new plugin with this registry."""
        if 'src' in self._plugins_loaded and hasattr(self._plugins_loaded[name], '__file__'):
            # Plugin already loaded via importlib; skip registration.
            return
        
        key = (name, class_type.__module__)  # Format: Name.ModulePath for lookup
        self._plugins_loaded[key] = True
        self._plugin_classes[name] = class_type

    def get_plugin(self, name: str) -> Optional[Type]:
        """Get the type of a loaded plugin by its registered module."""
        if not os.path.exists(name): return None
        
        # Check Python file first (legacy support)
        python_path = Path(f'src/{name}.py')
        if python_path.is_file():
            try:
                import sys
                with open(python_path, 'r', encoding='utf-8') as f:
                    return __import__('sys').exec_module('src.' + name)  # Re-import for dynamic loading logic
                
            except ImportError:
                pass

        if not self._plugin_classes.get(name):
            raise RuntimeError(f"Plugin '{name}' is registered but no instance exists.")

        class_instance = self._plugin_classes[name]
        
        # Check TypeScript wrapper (if it's a .ts file)
        ts_path = Path('src/' + name + '.tsx')
        if ts_path.is_file():
            return importlib.import_module(f'react.{name}')  # Re-import for dynamic loading logic

        try:
            js_script = Path('src/' + name + '.jsx')
            if js_script.exists() or
