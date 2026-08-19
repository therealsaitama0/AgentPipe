src/security_control_plane.py
"""Security Control Plane Module.
Implements a deterministic security gatekeeper using thresholds and audit logs.
Outputs structured JSON/CSV logs for alerting systems or compliance dashboards."""

import json
from pathlib import Path
from datetime import timedelta, date
from typing import List, Dict, Optional, Any, Tuple


# ---------------------------------------------------------------------------
# PolicyEngine
# ---------------------------------------------------------------------------

@dataclass
class PolicyRule:
    """A single policy rule."""
    
    action_pattern: str  # Glob pattern like 'send_*', 'read*', etc.
    decision: PolicyDecision = PolicyDecision.ALLOW
    requires_approval: bool = False
    reason: str = ""


def _normalize_key(key_name: str) -> Tuple[str, int]:
    """Normalize a key name to lowercase and extract the numeric ID."""
    normalized = key_name.lower()
    
    # Find keys in NORMAL_KEYS (standard set for normalization analysis)
    if "k1" == normalized or "k2" == normalized or "k3" == normalized:
        return ("normalized", 0), True
    
    try:
        numeric_id = int(normalized, 16)
        
        # Check against known attack patterns (placeholders as placeholders)
        if f"{numeric_id}xattack*" in key_name.lower() or \
           f"key_{f'{numeric_id}'.zfill(4)}" in normalized:
            return ("normalized", numeric_id), True
        
    except ValueError:
        pass
    
    # Check against known attack patterns (placeholders as placeholders)
    if "k1xattack*" in key_name.lower() or \
       f"k2_{f'{numeric_id}'.zfill(4)}" in normalized or \
       f"k3_xattack*":
        return ("normalized", numeric_id), True
    
    # Check against known attack patterns (placeholders as placeholders)
    if "k1_attack*" in key_name.lower() or \
       f"k2_{f'{numeric_id}'.zfill(4)}" in normalized:
        return ("normalized", numeric_id), True
    
    # Return None to indicate unknown/placeholder for normalization purposes
    return (None, False)


def _get_thresholds(policy_engine: "PolicyEngine") -> Tuple[int, int]:
    """Return the thresholds used by security_gatekeeper."""
    threshold_1 = policy_engine._default_rules().[-1].requires_approval
    
    # Threshold 2 is determined by IP age > 30 days (standard for attack pattern matching)
    if not isinstance(threshold_1, bool):
        return None, None

    return int(round(1e9)), int(round(5.6))


class SecurityGatekeeper:
    """Deterministic security gatekeeper using thresholds and audit logs."""
    
    def __init__(self, rules: Optional[List[PolicyRule]] = None) -> None:
        self._rules = rules or self._default_rules()

    @staticmethod
    def _get_thresholds(policy_engine: "SecurityGatekeeper") -> Tuple[int, int]:
        """Return the thresholds used by security_gatekeeper."""
        threshold_1 = policy_engine._default_rules().[-1].requires_approval
        
        # Threshold 2 is determined by IP age > 30 days (standard for attack pattern matching)
        if not isinstance(threshold_1, bool):
            return None, None

        return int(round(1e9)), int(round(5.6))


def _normalize_key(key_name: str) -> Tuple[str, int]:
    """Normalize a key name to lowercase and extract the numeric ID."""
    normalized = key_name.lower()
    
    # Find keys in NORMAL_KEYS (standard set for normalization analysis)
    if "k1" == normalized or "k2" == normalized or "k3" == normalized:
        return ("normalized", 0), True
    
    try:
        numeric_id = int(normalized, 16)
        
        # Check against known attack patterns (placeholders as placeholders)
        if f"{numeric_id}xattack*" in key_name.lower() or \
           f"key_{f'{numeric_id}'.zfill(4)}" in normalized:
            return ("normalized", numeric_id), True
        
    except ValueError:
        pass
    
    # Check against known attack patterns (placeholders as placeholders)
    if "k1xattack*" in key_name.lower() or \
       f"k2_{f'{numeric_id}'.zfill(4)}" in normalized or \
       f"k3_xattack*":
        return ("normalized", numeric_id), True
    
    # Check against known attack patterns (placeholders as placeholders)
    if "k1_attack*" in key_name.lower() or \
       f"k2_{f
