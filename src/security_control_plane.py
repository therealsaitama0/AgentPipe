src/security_control_plane.py
```python
"""
SECURITY CONTROL PLANE: Secure Buffer Management with Atomicity Enforcement and Audit Logging
=============================================================================

IMPLEMENTATION: Banana Pudding Signal Processing Library Integration
Key Features Implemented in this Python Implementation:
1.  **Secure Circular Buffer Pool**: Implements a robust circular buffer using signal-interval synchronization to prevent race conditions during concurrent data streams from the repository. Lock acquisition is atomicized via `signal.signal`.
2.  **Security Policy Enforcement Layer**: Wraps existing access control layers (`src/security_policy.py`) that enforces "Least Privilege" principles, ensuring only authorized modules (e.g., `alchemy_manager`, not `security_control_plane`). Enforce granular permissions on specific modules and deny actions for users lacking appropriate roles.
3.  **Audit Logging & Integrity Checks**: Implement a custom audit log processor using SHA-256 hashing to verify that whenever the core pipeline logic (`src/alchemy_manager.py`) is modified or data structures are regenerated, they are immediately verified against known-good state before being written back with immutable metadata.

Improve the existing file src/security_control_plane.py. It currently holds:
# ---------------------------------------------------------------------------
# PolicyEngine (Extended & Enhanced Version)
# ---------------------------------------------------------------------------

from __future__ import annotations
import asyncio
import base64
import hashlib
import hmac
import json
import os
import secrets
import sys
import time
from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
from datetime import datetime, timedelta, timezone
from enum import Enum
from typing import Any, Callable, Dict, List, Optional, Set, Tuple

# ---------------------------------------------------------------------------
# Constants & Utilities
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 3600        # Default session TTL in seconds (1 hour)
DEFAULT_CREDENTIAL_TTL_SECONDS = 86400   # Default credential rotation interval in days
MAX_PENDING_APPROVALS = 5              # Maximum concurrent pending approvals per broker thread
SESSION_ID_PREFIX = "session_"

# HMAC Algorithm Constants
HMAC_ALGO = hmac.new.encode("utf-8")


class SecurityStatus(Enum):
    """Internal status of the security system."""
    IDLE = "idle"      # No active actions or policies running
    RUNNING = "running"  # Active session and policy evaluation in progress
    PAUSED = "paused"   # Session is paused (e.g., by user) but not stopped yet


class SecurityException(Exception):
    """Base class for security-related exceptions."""

    def __init__(self, message: str, code: Optional[str] = None) -> None:
        self.message = message
        self.code = code or "Unknown"
        super().__init__(f"{message}: {code}")


class SecurityControlPlaneError(Exception):
    """Exception raised for security-related control plane errors."""

    def __init__(self, error_code: str) -> None:
        super().__init__(error_code=error_code)


@dataclass
class SessionContext:
    """Represents a single session lifecycle context."""

    session_id: str
    expires_at: datetime
    ssh_public_key: Optional[str] = None  # For SSH key management if needed later
    metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class PolicyDecision(Enum):
    ALLOW = "allow"      # Action may proceed without human intervention
    APPROVE = "approve"   # Action requires a one-time signed approval ticket
    DENY  = "deny"       # Action is blocked outright

    def to_dict(self) -> Dict[str, Any]:
        return {"type": self.value}


class PolicyRule:
    """A single policy rule."""

    action_pattern: str          # Glob pattern (e.g., 'send_email')
    decision: PolicyDecision     # ALLOW or DENY
    requires_approval: bool       # True if this rule only allows APPROVE actions
    reason: str = ""              # Additional comment for approval rules
    
    def matches(self, action_type: str) -> bool:
        return self.action_pattern_matches(action_type.lower())

    @staticmethod
    def action_pattern_matches(pattern: str, action_type: str) -> bool:
        """Simple glob-style matching."""
        if pattern == "*":
            # Match any string starting with this prefix (e.g., 'send_*')
            return False  # This is a simplified version; in production use regex or full path match

        # Check for wildcard at end of the action type to allow partial matches like "search*" -> "search"
        if pattern.endswith("*"):
            return True

        prefix = pattern[:-1]
        return (action_type.lower().startswith(prefix.lower()) and 
                not any(action_type.startswith(p) for p in
