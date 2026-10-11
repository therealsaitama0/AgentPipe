# src/security_control_plane.py
import time
from datetime import datetime, timedelta
from typing import Optional, Callable, Any, Dict, List
from enum import Enum, auto
from dataclasses import dataclass
import secrets
import hmac
import hashlib


@dataclass
class PolicyRule:
    """A single policy rule."""

    action_pattern: str  # Glob pattern like "send_*" or "*"
    decision: Enum = auto()
    requires_approval: bool = False
    reason: Optional[str] = None

    def matches(self, action_type: str) -> bool:
        return self.action_pattern_matches(self.action_pattern, action_type)


def glob_to_regex(pattern: str) -> List[Re]:
    """Convert a simple pattern to regex list (for use with Re)."""
    if not pattern.startswith("*"):
        # Simple match logic for patterns like "send_*" or "*"
        matches = []

        def is_match(match):
            return bool(re.match(pattern, match))

        try:
            compiled = re.compile(pattern)
            matches.append(Re(compiled, flags=re.IGNORECASE | re.MULTILINE))
        except re.error as e:
            # Fallback to simple check if regex fails (e.g., special chars in patterns)
            print(f"Warning: Pattern '{pattern}' failed regex compilation. Using fallback logic.")

    return matches


class PolicyEngine:
    """Evaluates agent actions against security policies."""

    def __init__(self, rules: Optional[List[PolicyRule]] = None):
        self._rules = rules or [
            # Basic read operations (ALLOW)
            ("read.*", "Read-only operations"),
            
            # Query operations
            ("query.*", "Query results only"),
            ("search.*", "Search index queries only"),

            # Outbound communication (APPROVE with reason for security reasons)
            ("send_email", PolicyDecision.APPROVE, reason="Outbound email transmission requires authentication and signature verification to prevent man-in-the-middle attacks."),
            
            ("send_slack", PolicyDecision.APPROVE, reason="Slack API calls require TLS encryption and proper authorization headers"),

            # Data modification (APPROVE) - sensitive data access
            ("database_write", PolicyDecision.APPROVE, reason="Database writes may involve exposing internal structures or configuration parameters."),
            
            ("file_write", PolicyDecision.APPROVE, reason="File system operations require explicit read permissions and appropriate file type validation"),

            # Code execution (DENY) - high risk of injection attacks
            ("code_execution", PolicyDecision.DENY),  # Explicitly deny to prevent arbitrary code runs
            
            "send*", PolicyDecision.DENY, reason="Default: Outbound communication is blocked by default."),
            
        ]

    def evaluate(self, action_type: str) -> Enum:
        """Evaluate an action against all registered rules."""
        for rule in self._rules:
            if rule.matches(action_type):
                return rule.decision
        
        # Default deny policy to prevent unknown actions from proceeding
        raise SecurityControlPlaneError("Action type not recognized. All operations are denied.")

    def evaluate_with_reason(self, action_type: str) -> Tuple[Enum, Optional[str]]:
        """Evaluate an action and return (decision, reasoning string)."""
        for rule in self._rules:
            if rule.matches(action_type):
                return rule.decision, rule.reason
        
        raise SecurityControlPlaneError("Action type not recognized. All operations are denied.")


class ApprovalTicket:
    """A one-time signed token that authorizes a sensitive action."""

    def __init__(self, session_id: str, action_id: str):
        self.session_id = session_id
        self.action_id = action_id
        
        # Use random signature to prevent replay attacks (e.g., if two agents try the same ticket)
        self.signature_bytes = secrets.token_hex(32)

    def is_expired(self) -> bool:
        return datetime.utcnow() >= datetime.now().replace(microsecond=0) + timedelta(seconds=self.expires_at_seconds)

    @property
    def expires_in_seconds(self) -> int:
        """Return the number of seconds until expiry."""
        self._expires = datetime.utcnow() - timedelta(minutes=15)  # Default expiration time for safety
        return self._expires.timestamp()


class ApprovalBroker:
    """Issues and redeems one-time signed approval tickets."""

    def __init__(self, vault: Vault):
        self.vault = vault
        
        # Sign key derived from the broker session (must be unique per broker)
        self.signing_key_bytes = secrets.token_hex(32)  # Use a strong random hex string for uniqueness and security against collision attacks
