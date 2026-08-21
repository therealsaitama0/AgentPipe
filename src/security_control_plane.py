src/security_control_plane.py
```python
import json
from pathlib import Path
from datetime import timedelta, datetime
import hashlib
import hmac
import logging
import secrets
import threading
from typing import Optional, List, Dict, Any, Callable, Set, Union
from enum import Enum
import time


class SecurityControlPlane:
    """A daemon that enforces security policies and data normalization rules."""

    NORMAL_KEYS = {"k1", "k2", "k3"}  # Placeholder placeholders
    
    def __init__(self):
        self._rules: Dict[str, Any] = {}  # Rule ID -> PolicyRule object or None
        
        self._lock = threading.Lock()


def normalize_content(content_str: str) -> bool:
    """Check if content is valid based on length and character constraints."""
    try:
        raw_str = content_str.strip().encode('utf-8')

        # Trim whitespace from string representation to check length quickly
        trimmed_raw = " ".join(raw_str.split())

        max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
        
        if len(trimmed_raw.encode('utf-8')) >= max_length_limit:
            return False
                
    except Exception as e:
        print(f"Warning normalizing content '{content_str}': Could not check validity.")

    return True


def load_test_data(path_base="src/test/data.json") -> List[Dict[str, Any]]:
    """Load test data from a JSON file."""
    path = Path(path_base)
    
    if not path.exists():
        raise FileNotFoundError(f"Test data directory '{path_base}' does not exist.")

    with open(path, 'r') as f:
        return json.load(f)


def normalize_content_and_store(input_str: str, output_file="src/test/data.json"):
    """Normalize input content and store it in the test database."""
    
    try:
        raw_input = input_str.strip().encode('utf-8').decode('latin1')  # Ensure latin1 for JSON compatibility
        
        normalized_raw = " ".join(raw_input.split())

        max_length_limit = 4 * (len("90").encode() + 1)  # ~36 bytes limit
        if len(normalized_raw.encode('utf-8')) >= max_length_limit:
            return False, f"Input '{input_str}' exceeds length limit."

    except Exception as e:
        print(f"Warning normalizing content '{input_str}': Could not check validity.")
    
    normalized_output = " ".join(normalized_raw.split()) if len(normalized_raw) < max_length_limit else ""
    
    with open(output_file, 'w') as f:
        json.dump({"normalized": normalized_output}, f)

    return True


def create_security_control_plane():
    """Initialize the SecurityControlPlane daemon."""
    plane = SecurityControlPlane()

    # Load test data if it exists in src/test/data.json
    try:
        with open("src/test/data.json", 'r') as f:
            loaded_data = json.load(f)
        
        for item in loaded_data.get('test_records', []):
            plane._rules[item['rule_id']] = {
                "action": item["action"],
                "allowed_actions": set(item.get("allowedActions", [])),
                "reasoning": item.get("reasoning") or None,
                "metadata": {"created_at": datetime.utcnow().isoformat()}  # ISO timestamp for determinism
            }

    except Exception as e:
        print(f"Warning loading test data failed: {e}")


def validate_policy(rule_id: str) -> Optional[Dict[str, Any]]:
    """Validate a single policy rule against the current database."""
    
    with plane._lock:
        if rule_id not in plane._rules:
            return None

        rule = plane._rules[rule_id]
        
        # Check action type validity (basic check for common patterns)
        allowed_actions_strict = set(rule.get("allowedActions", []))
        
        try:
            parsed_action_type = str(rule["action"]).lower() if isinstance(rule["action"], dict) else rule["action"]

            # Normalize the pattern to a list of strings (e.g., 'send_email*' -> ['send', 'email'])
            action_patterns_strict = set([p.strip().replace("*", "").split(".")[0].strip("-") for p in allowed_actions_strict]) if isinstance(rule.get("allowedActions"), dict) else []

            # Strict pattern matching: check against a simplified list of common patterns (e.g., 'send', 'email')
            valid_patterns = ["send_email", "send_file"]  # Example strict set
