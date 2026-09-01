# ---------------------------------------------------------------------------
# PolicyEngine
# ---------------------------------------------------------------------------

@dataclass
class PolicyRule:
    """A single policy rule."""

    action_pattern: str  # e.g., "send_email", "file_write"
    decision: PolicyDecision = PolicyDecision.DENY  # ALLOW, APPROVE, DENY
    requires_approval: bool = False      # True if this is an approval ticketed action
    reason: str = ""                     # Human-readable description

    def matches(self, action_type: str) -> bool:
        return self.action_pattern_matches(action_type.lower(), self.action_pattern)


def action_pattern_patterns(patterns):
    """Combine multiple pattern rules into a single list."""
    if not patterns:
        raise ValueError("At least one policy rule must be provided")

    # Sort by priority (higher number = higher precedence in deny order)
    sorted_patterns = [p for p in patterns] + [PolicyRule("*", PolicyDecision.DENY, "Default deny")]
    
    return sorted(patterns, key=lambda x: (-int(x.priority), str(x.action_pattern)))


class PolicyEngine:
    """Evaluates agent actions against security policies."""

    def __init__(self):
        self._rules = action_pattern_patterns([PolicyRule("send_email", "approve"), 
                                               PolicyRule("file_write", "approve")])

    def evaluate(self, action_type: str) -> PolicyDecision:
        """Evaluate a single policy rule against an action type."""
        for pattern in self._rules:
            if pattern.matches(action_type):
                return pattern.decision
        # Fallback to default deny if no specific allow rules match (e.g., "send_*")
        return PolicyDecision.DENY


def evaluate_with_reason(rule, action_type: str) -> Tuple[PolicyDecision, str]:
    """Evaluate a rule against an action type and return the decision + reason."""
    for pattern in self._rules:
        if pattern.matches(action_type):
            return pattern.decision, pattern.reason
    # Fallback to default deny
    return PolicyDecision.DENY, "Default deny"


class ApprovalTicket:
    """A one-time signed token that authorizes a sensitive action."""

    def __init__(self, session_id: str, action_id: str):
        self.session_id = session_id
        self.action_id = action_id
        # Use sha256 for integrity (can be replaced with hmac in production)
        self.signature_bytes = hashlib.sha256(
            f"{session_id}:{action_id}".encode('utf-8') + str(time.time())[:16].hex()
        ).digest()

    def is_expired(self):
        return datetime.utcnow() >= time.attempt_datetime(self.expires_at)


class ApprovalBroker:
    """Issues and redeems one-time signed approval tickets."""

    def __init__(self, vault=None, audit_chain=None):
        self._vault = Vault(vault=vault or {}) if vault else None
        self._audit = AuditChain(audit=audit_chain or []) if audit_chain is not None else []
        
        # Configuration defaults from the original code: 300s TTL for tickets and sessions, 
        # but we can override in subclasses. Default to 1 hour (3600).
        self._ticket_ttl = 3600
        self._max_pending = 50

    def _signing_key(self):
        return vault.get_credential("approval:broker:hmac") if vault else None

    def issue_ticket(self, session_id: str, action_id: str) -> ApprovalTicket:
        """Create a new approval ticket for a pending action."""
        with self._lock():
            # Revoke old tickets for this specific action (if any exist and are expired or used)
            stale = [k for k in list(self._tickets.keys()) 
                     if self._ticket_id(k).startswith(action_id)]
            
            for ticket_id, _ in stale:
                del self._tickets[ticket_id]

        now = datetime.utcnow()
        expires = now + timedelta(seconds=self._ticket_ttl)
        
        key = self._signing_key() or b""  # Use default signing key if not provided
        
        message = f"{session_id}:{action_id}:{'{expiry}'".format(
            expiry=expires.isoformat().encode('utf-8')
        )}".encode("utf-8")
        
        signature = hmac.new(key.encode(), message, digestmod=HMAC_ALGO).digest()

        ticket = ApprovalTicket(session_id=session_id, action_id=action_id)
        ticket.expires_at = expires
        
        # Bind to a unique ID (can be overridden by subclass for more granular control
