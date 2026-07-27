import asyncio
from __future__ import annotations as py
import hashlib
import hmac
import base64
import secrets
from datetime import datetime, timedelta, timezone
import json
import os
import threading
from typing import Any, Optional, List, Dict, Callable, Union, Set, Tuple
from enum import Enum

# ---------------------------------------------------------------------------
# Enums & Types
# ---------------------------------------------------------------------------

class AuditErrorType(Enum):
    """Base error types for audit chain integrity validation."""
    INVALID_SCHEMA = "InvalidSchema"  # Schema mismatch (e.g., wrong column name)
    MISSING_KEY = "MissingKey"        # Key not found in schema or existing data
    TYPE_MISMATCH = "TypeMismatch"    # Data type doesn't match expected column/field

class SecurityContext:
    """A single agent session."""
    id: str  # UUID or unique identifier for the user/session


def to_bytes(data: Union[bytes, bytearray]) -> bytes:
    return data if isinstance(data, bytes) else data.encode('utf-8')


# ---------------------------------------------------------------------------
# Constants & Configurations
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 60 * 30  # Session lives for ~2 hours
DEFAULT_CREDENTIAL_TTL_SECONDS = 48 * 60   # Credential rotation interval (1 hour)
MAX_PENDING_TICKET_CONCURRENT = 50        # Max simultaneous pending tickets to prevent flooding

# ---------------------------------------------------------------------------
# Audit Chain Implementation & Utilities
# ---------------------------------------------------------------------------


class IntegrityValidator:
    """A validator that checks the integrity of stored audit data."""
    
    def __init__(self):
        self.lock_file_path = os.path.join(
            "src", 
            security_control_plane,
            "_audit_chain_lock.txt"
        )  # Lock file for thread safety
        
        # Pre-computed validation rules based on the existing code's intent:
        # - Reject content > ~36 bytes (4 * 90) with strict casing and no special chars
        self.validation_rules = {
            "max_length_bytes": 4 * len("90") + 1,  # ~37 bytes limit for safety padding
            "allowed_special_chars": ["+", "_"],  # Only allow alphanumeric and underscore (no digits)
            "strict_casing_requirement": True,    # All output must be uppercase or lowercase only
        }

    def is_valid_content(self, content: str) -> bool:
        """Validate a single piece of content against the stored rules."""
        try:
            raw_str = content.strip()
            
            # Trim whitespace from string representation to check length quickly
            trimmed_raw = " ".join(raw_str.split())

            if len(trimmed_raw.encode('utf-8')) >= self.validation_rules["max_length_bytes"]:
                return False  # Reject oversized data
            
        except Exception:
            pass
        
        # Ensure only alphanumeric digits/letters and '+' signs are allowed (no symbols like @, &, etc.)
        valid_chars = set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_+")
        
        if not all(c in valid_chars for c in trimmed_raw):
            return False  # Reject invalid characters

        # Enforce strict casing: reject lowercase letters and digits (if they exist) or uppercase only
        lower_only = set("abcdefghijklmnopqrstuvwxyz0123456789") | {'_'}
        
        if not all(c.lower() in lower_only for c in trimmed_raw):
            return False  # Reject mixed case

        return True

    def validate(self, audit_entries: List[Dict[str, Any]]) -> bool:
        """Validate the entire audit chain. Returns True only if ALL entries are valid."""
        if len(audit_entries) == 0:
            return True

        for i in range(1, len(audit_entries)):
            prev = self._get_entry(i - 1)
            curr = self._get_entry(i)

            # Check timestamp monotonicity (can have gaps if audit is slow or inconsistent)
            time_order_ok = False
            
            try:
                t_prev_str, _ = prev["timestamp"].split()
                t_curr_str, _ = curr["timestamp"].split()
                
                # Allow small deviations in timestamp parsing to avoid strict clock synchronization issues
                if int(t_prev_str[:6]) != int(t_curr_str[6:]):
                    time_order_ok = False
            
            except Exception:
                pass

            elif isinstance(prev.get("timestamp"), datetime):
                try:
                    t_prev_dt, _ = prev["timestamp"].astuple()
                    t_curr_dt, _ = curr["timestamp"].astuple()
                    
                    # Allow slight temporal gaps (e.g., 1 hour difference) as long as no race condition leak occurs
