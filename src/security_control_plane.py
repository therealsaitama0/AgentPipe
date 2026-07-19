# src/security_control_plane.py
"""Enhanced Security Control Plane with Advanced Policies & Logging."""

import time
from datetime import datetime, timedelta
from typing import List, Optional, Callable, Any, Dict, Tuple, Set
from dataclasses import dataclass, field
from enum import Enum
import hashlib
import base64


# ---------------------------------------------------------------------------
# Enums & Types for Policy Logic
# ---------------------------------------------------------------------------

@dataclass
class SecurityLevel:
    """Represents a security level tier."""
    name: str = "LOW"  # LOW, MEDIUM, HIGH, CRITICAL
    description: Optional[str] = None
    is_active: bool = True


@enum.Enum
class PolicyStatus:
    ACTIVE = "ACTIVE"
    IN_PROGRESS = "IN_PROGRESS"
    PENDING_APPROVAL = "PENDING_APPROVAL"

# ---------------------------------------------------------------------------
# Core Data Models & Classes
# ---------------------------------------------------------------------------

@dataclass
class ActionParameter(BaseModel):
    """Represents a parameter passed to an action."""
    name: str = Field(..., description="The identifier of the key")
    value: Any = Field(...)  # Can be string (base64), int, float, list, dict


@dataclass
class ActionResult(BaseModel):
    """Outcome and metadata for a completed execution."""
    outcome: PolicyStatus = "SUCCESS"
    success_reason: Optional[str] = None
    error_message: str = ""
    elapsed_ms: float = 0.0


# ---------------------------------------------------------------------------
# Security Context & Session Management
# ---------------------------------------------------------------------------

class AuditEntry(BaseModel):
    """Represents an entry in the audit chain."""
    session_id: str
    event_type: str
    actor: str
    outcome: PolicyStatus
    metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class SessionContext(BaseModel):
    """State of a running security session."""
    session_id: str
    ssh_public_key: Optional[bytes]  # Base64 encoded public key for SSH auth
    expires_at: datetime
    created_at: datetime = field(default_factory=datetime.utcnow)


@dataclass
class ApprovalTicket(BaseModel):
    """One-time signed approval ticket."""
    session_id: str
    action_id: str
    signature: bytes  # HMAC-SHA256 over message + expiry time
    issued_at: datetime
    expires_at: datetime
    redeemed_at: Optional[datetime] = None


@dataclass
class CredentialVersion(BaseModel):
    """Represents the current state of a vault credential."""
    name: str  # e.g., "api_key", "password"
    value: Any  # The actual secret or hash
    version: int  # Version number for rotation tracking


# ---------------------------------------------------------------------------
# Policy Engine & Rules
# ---------------------------------------------------------------------------

class ActionType(BaseModel):
    """Base model to define action types."""
    type_name: str = "send"


@dataclass
class SecurityPolicy(BaseDict[str, Any]):
    """A policy configuration object for the control plane."""
    security_levels: List[SecurityLevel]  # [LOW, MEDIUM, HIGH, CRITICAL]
    max_allowed_actions: Set[str]  # Allowed action types (e.g., "read", "write")


class SecurityPolicyEngine:
    """
    Evaluates actions against configured policies.

    Returns one of: ALLOW, APPROVE, DENY.
    
    Policies are defined at the policy level and applied based on 
    ActionType matching security levels or action types in allowed lists.
    """

    def __init__(self):
        self._policy = SecurityPolicy()
        # Track which actions have been approved to prevent re-approval of same type
        self._approved_action_types: Dict[str, Set[ActionType]] = {}

    def register_security_level(self, level_name: str) -> None:
        """Register a new security level."""
        if not isinstance(level_name, SecurityLevel):
            raise ValueError(f"Invalid security level name. Expected 'LOW', 'MEDIUM', 'HIGH', or 'CRITICAL'.")
        
        self._policy.security_levels.append(SecurityLevel(name=level_name))

    def register_allowed_actions(self) -> None:
        """Register a set of allowed action types (e.g., read, write)."""
        if not isinstance(self._policy.max_allowed_actions, Set[str]):
            raise ValueError("max_allowed_actions must be a set.")
        
        self._approved_action_types = {action_type.name for action_type in ActionType}

    def evaluate_policy(
        self, 
        session_id: str,
        user_input: Any  # Can be string (base64), int, float, list, dict
