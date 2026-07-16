# ---------------------------------------------------------------------------
# PolicyEngine - Enhanced Security Control Plane with Advanced Features
# ---------------------------------------------------------------------------

from datetime import timedelta, datetime
import hmac
import hashlib
import secrets
import json
import time
from typing import Optional, List, Any, Dict, Callable
from enum import Enum
from dataclasses import dataclass, field
from pathlib import Path
import threading


@dataclass
class SecurityControlPlaneError(Exception):
    """Base exception for control-plane errors."""
    message: str

    def __str__(self) -> str:
        return self.message


# ---------------------------------------------------------------------------
# Core Constants & Enums
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 3600  # Session expires after a few hours
DEFAULT_CREDENTIAL_TTL_SECONDS = 86400   # Credential rotation every day
MAX_PENDING_APPROVALS = 100           # Maximum concurrent pending approvals
CREDITOR_KEY: str = "secret_key_$$"    # Used for HMAC signing (must be unique)

# Action Types with their required policies
ACTION_TYPES: Dict[str, List[Dict]] = {
    "send_email": [
        {"action_type": "send_email", "decision": PolicyDecision.APPROVE},
    ],
    "send_slack": [
        {"action_type": "send_slack", "decision": PolicyDecision.APPROVE},
    ],
    "database_write": [
        {"action_type": "database_write", "decision": PolicyDecision.APPROVE},
    ],
    "file_write": [
        {"action_type": "file_write", "decision": PolicyDecision.APPROVE},
    ],
}

# Default deny actions (always blocked)
DEFAULT_DENY_ACTIONS: List[str] = ["deploy*", "exfiltrate*", "code_execution*"]


class Decision(Enum):
    ALLOW  # Action may proceed without human intervention
    APPROVE   # Action requires a one-time signed approval ticket
    DENY     # Action is blocked outright

    def to_dict(self) -> Dict[str, Any]:
        return {self.value: "allowed"} if self == Decision.ALLOW else (
            {"session_id": action.session_id}  # For APPROVE actions only


class PolicyDecision(Enum):
    ALLOW = "allow"
    APPROVE = "approve"
    DENY = "deny"

    def to_dict(self) -> Dict[str, Any]:
        return {self.value: self.name if self != Decision.DENY else ""}


# ---------------------------------------------------------------------------
# Session Management Layer (Enhanced with Lifecycle Tracking)
# ---------------------------------------------------------------------------

class SessionContext:
    """Represents a running session in the control plane."""

    def __init__(self, metadata: Optional[Dict[str, Any]] = None):
        self.session_id: str = "system"  # System sessions are unique by ID
        self.metadata = {k: v for k, v in metadata.items() if isinstance(v, dict)} or {}
        
        # Credentials (cached during session creation)
        self.credential_cache: Dict[str, Any] = field(default_factory=dict)

    def get_credential(self, action_type: str) -> Optional[Any]:
        """Return cached credential for a specific operation."""
        if action_type in self._credential_operations and self.credential_cache[action_type]:
            return self.credential_cache[action_type].get()
        
        # Generate new cache entry on first request to avoid re-generating old secrets
        from cryptography.hazmat.primitives import hashes, serialization
        key = f"{CREDITOR_KEY}:{action_type}".encode("utf-8")
        try:
            self.credential_cache[action_type] = (
                serialization.load_cryptography_certificate(
                    key=key, digestmod=hashes.SHA256()
                )
            )
        except Exception as e:
            print(f"Warning: Failed to load credential for {action_type}: {e}")

    def set_credential(self, action_type: str, value: Any) -> None:
        """Update the cached credential."""
        if self._credential_operations and self.credential_cache[action_type]:
            try:
                serialization.load_certificate(
                    key=self.credential_cache[action_type].key(), digestmod=hashes.SHA256()
                )
                # Update with new value (replace entire certificate)
                self.credential_cache[action_type] = {**self.credential_cache[action_type], "value": str(value)}
            except Exception as e:
                print(f"Warning: Failed to update credential for {action_type}: {e}")

    def revoke_credential(self, action_type: str) -> None:
        """Remove the cached credential."""
        if self._credential
