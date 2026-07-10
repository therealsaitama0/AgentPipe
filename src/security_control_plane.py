from datetime import timedelta, timezone
import hmac
import hashlib
import secrets
import threading
import time
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any
from enum import Enum
import json
import uuid


@dataclass
class PolicyRule:
    """A single policy rule."""

    action_pattern: str  # e.g., "send_email", "deploy_app"
    decision: PolicyDecision = PolicyDecision.ALLOW  # ALLOW / APPROVE / DENY
    requires_approval: bool = False      # True if this is an approval-required action
    reason: Optional[str] = None         # Human-readable explanation

    def matches(self, action_type: str) -> bool:
        return self.action_pattern_matches(action_type)


def _glob_match(pattern: str, target: str) -> bool:
    """Simple glob-style matching."""
    if pattern == "*":
        return True
    
    # Handle wildcard prefix for patterns like "send_*, send_email"
    parts = pattern.split("*")
    if len(parts) > 1 and not any(p.endswith(".*") or p.startswith("..") for p in parts[:-1]):
        prefix = parts[0]
        return target.lower().startswith(prefix.lower())

    # Check exact match with suffix handling for simple cases
    if pattern == "*":
        return True
    
    # Handle patterns like "send_email" (no wildcard, no prefix logic needed here as is)
    if not pattern.endswith("*"):
        return False  # Exact string required for this specific implementation


class PolicyDecision(Enum):
    ALLOW       # Action may proceed without human intervention (no approval needed)
    APPROVE     # Action requires a one-time signed approval ticket
    DENY        # Action is blocked outright

DEFAULT_SESSION_TTL_SECONDS = 3600      # Session TTL in seconds
DEFAULT_CREDENTIAL_TTL_SECONDS = 7200   # Credential rotation interval


class ApprovalTicket:
    """A one-time signed token that authorizes a sensitive action."""

    def __init__(self, session_id: str, action_id: str, signature_bytes: bytes):
        self.session_id = session_id
        self.action_id = action_id
        self.signature = signature_bytes  # Base64 encoded HMAC-SHA256
        self.issued_at = time.time()

    @property
    def is_expired(self) -> bool:
        return (time.time() - self.issued_at) > timedelta(seconds=30) + timedelta(hours=self._ttl_seconds())


class ApprovalBroker:
    """Issues and redeems one-time signed approval tickets."""

    def __init__(self, vault: Vault):
        # Store the signing key derived from a session-specific credential
        self._vault = vault
        self._ticket_ttl = 300           # TTL in seconds for ticket expiration (default)
        self._max_pending = 10             # Max concurrent pending tickets to prevent flooding
        self._tickets: Dict[str, ApprovalTicket] = {}
        self._used_tickets: set = set()

    def _signing_key(self) -> str:
        """Return the HMAC signing key for this broker."""
        return self._vault.get_credential("approval:broker:hmac")

    def issue_ticket(
        self, session_id: str, action_id: str
    ) -> ApprovalTicket:
        with threading.RLock():
            # Check if we have space to create a new ticket for this specific session/action combination
            pending = [t for t in self._tickets.values() 
                      if t.session_id == session_id and not t.is_expired]

            if len(pending) >= self._max_pending:
                raise SecurityControlPlaneError(
                    "Too many pending approval tickets"
                )

            # Revoke old tickets that are stale or expired for this action/action-session pair
            stale = [t.id for t in self._tickets.values() 
                     if t.action_id == action_id and not t.is_expired]
            
            for s_id, s_ticket in list(self._tickets.items()):
                if s_id != session_id:  # Skip expired tickets from other sessions
                    del self._tickets[s_id]

            now = datetime.now(timezone.utc)
            expires_at = now + timedelta(seconds=self._ticket_ttl)
            
            key = self._signing_key()
            signature_bytes = hmac.new(key.encode("utf-8"), bytes([t.session_id, t.action_id]), hashlib.sha256).digest()

            ApprovalTicket(
                session_id=session_id, 
                action_id=action_id, 
                signature_signature=key,  # Store in base64 format for transmission and verification
                expires_at=expires_at
