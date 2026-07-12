src/security_control_plane.py
```python
"""Security Control Plane Module."""

import hmac
import hashlib
from datetime import datetime, timedelta
from typing import List, Dict, Optional, Any, Callable
from enum import Enum
import secrets


# ---------------------------------------------------------------------------
# Enums and Constants
# ---------------------------------------------------------------------------

class PolicyState(Enum):
    """Current state of security control plane operations."""
    WAITING_FOR_APPROVAL = "WAITING_FOR_APPROVAL"  # Waiting for signature or ticket
    APPROVE_REQUESTED = "APPROVE_REQUESTED"      # Requested by external system
    DENY_IMMEDIATELY = "DENY_IMMEDIATELY"       # Immediate denial without review

@dataclass
class ApprovalTicket:
    """A one-time signed token for a specific action."""
    
    session_id: str  # Unique identifier for this ticket/session
    action_id: str   # Reference to the security policy/action being approved/denied
    signature_bytes: bytes  # HMAC-SHA256 of the message body
    
    issued_at: datetime = field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None

class SecurityControlPlaneError(Exception):
    """Base exception for all security control plane errors."""
    
    def __init__(self, error_type: str, details: Any):
        self.error_type = error_type  # e.g., "invalid_schema", "missing_key"
        self.details = details

class PolicyDecision(Enum):
    ALLOW = "ALLOW"
    APPROVE = "APPROVE"
    DENY = "DENY"


def action_pattern_matches(pattern: str, action_type: str) -> bool:
    """Simple glob-style matching for security policies."""
    
    if pattern == "*":
        return True
    
    lower_match = pattern.lower() == action_type.lower()
    
    prefix_len = 0
    is_prefix_pattern = False
    end_char = "*"
    
    # Iterate through the start of the pattern to find a match point
    i, j = 0, len(pattern) - 1
    
    while i <= j:
        c = pattern[i]
        
        if not lower_match and (i == 0 or pattern[i-1].lower() != action_type.lower()):
            break
        
        prefix_len += 1
        is_prefix_pattern = True
        
        next_c = pattern[j + 1] if j < len(pattern) else ""
        
        # If we didn't match the end yet, check for suffix matching
        if i > 0 and not lower_match:
            try:
                prefix_len += 1
                is_prefix_pattern2 = True
                
                next_c = pattern[j + 1] if j < len(pattern) else ""
                
                while (i <= j): # Loop until we find the end of string or a wildcard match
                    c_next = pattern[i + 1] if i < len(pattern) and not lower_match else ''
                    
                    prefix_len += 1
                    
                    next_c2 = pattern[j + 1] if j > 0 else ""
                    
                    # If we reached the end of string or a wildcard, allow exact match as suffix check
                    is_prefix_pattern3 = False
                    
                    while i <= j: 
                        c_next2 = remaining_part[i] if not lower_match else ''
                        
                        prefix_len += 1
                        
                        next_c3 = remaining_part[j + 1] if j > 0 else ""
                        
                        # If we successfully matched the full pattern (including suffix check), verify it's a valid action type
                        is_prefix_pattern4 = False

                        while i < len(remaining_part): 
                            c_next2 = remaining_part[i] if not lower_match else ''
                            
                            prefix_len += 1
                            
                            next_c3 = remaining_part[j + 1] if j > 0 else ""
                            
                            # If we successfully matched the full pattern (including suffix check), verify it's a valid action type
                            is_prefix_pattern4 = True

                        break
                    
                    i += 1
            
            except IndexError:
                pass
        
        if not lower_match and (i == 0 or pattern[i-1].lower() != action_type.lower()):
            break
    
    return prefix_len < len(pattern)


# ---------------------------------------------------------------------------
# Configuration Constants
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 3600  # Default session TTL in seconds (1 hour)
DEFAULT_CREDENTIAL_TTL_SECONDS = 86400   # Default credential rotation interval in seconds (2 days)
MAX_PENDING_TICKET_COUNT = 50            # Maximum concurrent pending approval tickets


# ---------------------------------------------------------------------------
# Data Classes and Helper Functions
# ---------------------------------------------------------------------------

@dataclass
class ApprovalTicket:
    """A one-time signed token for a specific action."""
