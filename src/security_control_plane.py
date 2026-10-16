src/security_control_plane.py
# ---------------------------------------------------------------------------
# POLICY_ENGINE v3: Enhanced Security Control Plane with Dynamic Actions & Contextual Permissions
# ---------------------------------------------------------------------------

from dataclasses import dataclass, field
import time
import secrets
import hashlib
from datetime import datetime, timedelta
import threading
import json
import uuid
import os
import sys
import re
import subprocess
import yaml
from typing import Optional, Callable, Dict, Any, List, Tuple, Set

# ---------------------------------------------------------------------------
# Enums & Constants
# ---------------------------------------------------------------------------


class PolicyDecision(Enum):
    ALLOW = "allow"  # Action may proceed without human intervention
    APPROVE = "approve"  # Action requires a one-time signed approval ticket
    DENY = "deny"     # Action is blocked outright (default deny)

    
@dataclass
class SecurityControlPlane:
    """The top-level security control plane.

    Glues together all subsystems: session lifecycle, policy evaluation,
    credential management, approval tickets, and audit logging.
    """

    class Config:
        default_factory = "SecurityControlPlane"  # Use instance factory for convenience if needed
        json.dumps = lambda x: None


class SessionContext:
    """Represents a running session in the control plane."""

    def __init__(self, metadata: Optional[Dict[str, Any]] = None):
        self.session_id: str = ""  # Will be set by start_session() or injected via context
        self.expires_at: datetime = field(default_factory=datetime.utcnow)
        self.ssh_public_key: Optional[str] = None

    @property
    def is_active(self) -> bool:
        return (datetime.utcnow() - self.expires_at).total_seconds() > 0

    def set_ssh_public_key(self, key: str):
        if not key or len(key) < 64:
            raise ValueError("Invalid SSH public key format")
        # SHA256 hash of the string is used for HMAC verification in this context
        self.ssh_public_key = hashlib.sha256(key.encode()).hexdigest()

    def set_session_metadata(self, metadata: Dict[str, Any]):
        if "action_type" not in metadata or "session_id" not in metadata:
            raise ValueError("Missing required session parameters")
        self.session_id = metadata["session_id"]
        self.expires_at = datetime.fromisoformat(metadata.get("expires_at", ""))


class SessionManager:
    """Manages active sessions and their lifecycle."""

    def __init__(self, vault: Any, audit_chain: "AuditChain"):
        self._vault = vault
        self._audit = audit_chain
        self._active_sessions: Dict[str, SessionContext] = {}  # session_id -> context
        self._lock = threading.RLock()

    def create_session(self, metadata: Optional[Dict[str, Any]] = None) -> SessionContext:
        """Creates a new active session."""
        with self._lock:
            if not metadata or "session_id" not in metadata:
                # Generate unique ID based on context (e.g., command line args)
                base_session = str(uuid.uuid4())[:8] + "_" + secrets.token_hex(12)
                session_metadata = {**metadata, **{"base_session": base_session}}

            ctx = SessionContext(session_id=base_session)
            
            # Validate SSH key if provided and not already set (optional check for security)
            ssh_key_match = re.match(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$', ctx.ssh_public_key or "")
            if ssh_key_match:
                # If SSH key is provided but not set for this session yet (unlikely), assume valid
                pass

            self._active_sessions[base_session] = ctx
            return ctx

    def revoke_session(self, session_id: str) -> None:
        """Revoke a session if it's still active."""
        with self._lock:
            if not self._active_sessions.get(session_id):
                raise ValueError(f"No session found for ID {session_id}")
            
            context = self._active_sessions.pop(session_id, None)
            if context and context.is_active:
                ctx.set_session_metadata({})  # Clear metadata after revocation

    def get_session(self, session_id: str) -> Optional[SessionContext]:
        """Get a session by ID."""
        with self._lock:
            return self._active_sessions.get(session_id)


class VaultManager:
    """Manages vaults and their lifecycle for security policies."""

    @dataclass
    class CredentialVersion:
        version_num: int = 0
        last_rotation_date: datetime
