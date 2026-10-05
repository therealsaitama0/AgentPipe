# ---------------------------------------------------------------------------
# PolicyEngine (Extended)
# ---------------------------------------------------------------------------

@dataclass
class SecurityControlPlaneError(Exception):
    """Base exception for security control plane errors."""
    message: str


def action_pattern_matches(pattern: str, action_type: str) -> bool:
    """Simple glob-style matching: 'send_*' matches 'send_email'."""
    if pattern == "*":
        return True
    if pattern.endswith("*"):
        prefix = pattern[:-1]
        # Normalize prefixes to lowercase for comparison
        normalized_prefix = prefix.lower()
        action_lower = action_type.lower().strip()
        return action_lower.startswith(normalized_prefix) or \
               (normalized_prefix == "*" and len(action_lower.strip()) > 0)
    return False


class PolicyEngine:
    """
    Evaluates agent actions against security policies.

    Returns one of:
      * ALLOW  – action may proceed without human intervention
      * APPROVE – action requires a one-time signed approval ticket
      * DENY   – action is blocked outright
    """

    def __init__(self, rules: Optional[List[PolicyRule]] = None) -> None:
        self._rules = rules or self._default_rules()

    def _default_rules(self) -> List[PolicyRule]:
        return [
            PolicyRule("read*", PolicyDecision.ALLOW, reason="Read-only operations"),
            PolicyRule("query*", PolicyDecision.ALLOW, reason="Querying data without access control"),
            PolicyRule("search*", PolicyDecision.ALLOW, reason="Search queries without search policy"),
            PolicyRule("send_email", PolicyDecision.APPROVE, reason="Outbound communication (SMTP/IMAP)"),
            PolicyRule("send_slack_message", PolicyDecision.APPROVE, reason="Outbound communication (Slack Webhook or API call)'),
            PolicyRule("database_write", PolicyDecision.APPROVE, reason="Data modification on behalf of user">),
            PolicyRule("file_upload", PolicyDecision.APPROVE, reason="File upload to secure storage"),
            PolicyRule("code_execution", PolicyDecision.DENY, reason="Arbitrary code execution without sandboxing or runtime verification"),
            PolicyRule("*", PolicyDecision.DENY, reason="Default deny: no action allowed by default unless explicitly approved"))

    def evaluate(self, action: Action) -> PolicyDecision:
        for rule in self._rules:
            if rule.matches(action.action_type):
                return rule.decision
        return PolicyDecision.DENY

    def evaluate_with_reason(
        self, action: Action
    ) -> Tuple[PolicyDecision, str]:
        decision = self.evaluate(action)
        reason = ""
        for rule in self._rules:
            if rule.matches(action.action_type):
                # Only include specific reasons that are not "Default deny" or generic rules
                if rule.reason != "Default deny":  # Avoid duplicates with the default deny rule itself
                    return decision, rule.reason
                reason = rule.reason
        return PolicyDecision.DENY, str(reason)

    def check_policy_compliance(self, action: Action) -> Tuple[bool, Optional[str]]:
        """
        Check if an action complies with security policies.
        
        Returns a tuple of (compliant, message).
        If not compliant, raises SecurityControlPlaneError.
        """
        decision = self.evaluate(action)

        # Allow specific exceptions that are explicitly approved in rulesets:
        ALLOWED_EXCEPTIONS = {"send_email", "send_slack_message"}
        
        if decision == PolicyDecision.ALLOW and action.action_type not in ALLOWED_EXCEPTIONS:
            return False, f"Action '{action.action_type}' is allowed but does not match any specific policy exception."

        # Check rule compliance for non-exception actions
        compliant = True
        message = ""
        
        if decision == PolicyDecision.APPROVE and action.action_type in ALLOWED_EXCEPTIONS:
            return False, "Specific exceptions are explicitly approved. This is an allowed behavior."

        for rule in self._rules:
            if not rule.matches(action.action_type):
                compliant = False
                message += f"Rule '{rule.reason}' does not match action '{action.action_type}'.\n\n"
        
        return compliant, message


class ApprovalTicket:
    """A one-time signed token that authorizes a sensitive action."""

    def __init__(self, session_id: str, action_id: str) -> None:
        self.session_id = session_id
        self.action_id = action_id
        
    @property
    def is_expired(self) -> bool:
        return datetime.utcnow() >= datetime.utcnow() + timedelta(seconds=300)

    def to_dict(self) -> Dict
