src/security_control_plane.py
"""
Security Control Plane Implementation v2.1: Robust Policy Engine for Database Security and Access Control within the repository structure. It includes:

1.  **Data Normalization Layer**: A class `AlienDatabase` that normalizes incoming content strings against standard "Alien" keys (k1, k2, k3) to enforce specific character constraints or length limits as defined in the existing codebase logic.
2.  **Audit Trail Management**: An audit entry model for logging actions and their outcomes with metadata support.
3.  **Approval Ticket Lifecycle**: A class `ApprovalTicket` representing signed tokens issued by agents, including expiration handling via a custom datetime wrapper.
4.  **Session Context & SSH Access Control**: Manages agent-specific sessions (including public keys) and enforces session timeouts to prevent unauthorized long-term access or credential reuse across multiple requests within the same context.

The module provides a `SecurityControlPlane` class that orchestrates these components, ensuring strict adherence to security policies while maintaining operational integrity through audit logging.
"""

import asyncio
from datetime import datetime, timedelta
import hashlib
import hmac
import secrets
import base64
import json
import os
import threading
import time
from typing import Optional, Dict, Any, Callable, List, Tuple


# ---------------------------------------------------------------------------
# Constants & Utilities
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 300       # Seconds before session expires (for ticket expiry)
DEFAULT_CREDENTIAL_TTL_SECONDS =   1800 # Seconds after credential rotation cycle completes
MAX_PENDING_TICKET_COUNT = 50          # Maximum concurrent pending approval tickets per agent/session
AUDIT_LOG_FILE_NAME = "security_audit.log"

# Security Context Flags
SECURITY_FLAG_SESSION_LOCKED = False
SECURITY_FLAG_CREDENTIAL_ROTATED = False


@dataclass
class Action:
    """Represents a single action to be executed."""
    
    session_id: str  # Unique identifier for the agent's context (e.g., "agent-123")
    action_type: str     # e.g. "send_email", "deploy_app"
    parameters: Dict[str, Any] = field(default_factory=dict)
    metadata: Optional[Dict[str, Any]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "session_id": self.session_id,
            "action_type": self.action_type,
            "parameters": dict(self.parameters),
            "metadata": asdict(self.metadata or {}),
        }


@dataclass
class ApprovalTicket:
    """A one-time signed token for human approval."""
    
    session_id: str  # Agent context ID (e.g., "agent-123")
    action_id: str      # Specific request identifier (e.g. "request_45678901")
    signature: bytes     # HMAC-SHA256 over message + expiry string
    issued_at: datetime  // Timestamp when ticket was created/issued
    expires_at: datetime  // When the token is valid for redemption
    
    redeemed_at: Optional[datetime] = None      // Last time this specific action was redemptions
    redeemed: bool = False                        # Boolean indicating if already used
    redeems_this_request: bool                  // True if request can be re-issued

    def to_dict(self) -> Dict[str, Any]:
        return {
            "session_id": self.session_id,
            "action_id": self.action_id,
            "signature": base64.b62encode(self.signature).decode('utf-8'),  # Base64 encode for JSON serialization (optional but common)
            "issued_at": self.issued_at.isoformat(),
            "expires_at": self.expires_at.isoformat(),
            "redeemed": self.redeemed,
        }

    @property
    def is_expired(self) -> bool:
        return datetime.utcnow() >= self.expires_at


@dataclass
class SessionContext:
    """Internal state of a single session."""
    
    session_id: str  # Agent context ID (e.g., "agent-123")
    ssh_public_key: Optional[str] = None       // PEM-encoded public key for SSH access
    expires_at: datetime  // When this session ends (for ticket expiry)


@dataclass
class AuditEntry:
    """Log entry from the security audit chain."""
    
    session_id: str      # Agent context ID (e.g., "agent-123")
    event_type: str       # Type of action/event logged ("action_executed", "session_created", etc.)
    actor: str            // Who performed this action
    outcome: str          # Result status ("success", "denied", "warning
