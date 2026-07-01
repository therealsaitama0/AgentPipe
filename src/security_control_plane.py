src/security_control_plane.py
"""
Security Control Plane Module: Credential-Based Access Control (CBAC) Extension for Abstract Data Type Generators.
Implements a robust policy engine that integrates with `AlienDataTypeGenerator` to enforce access control on arbitrary integer generation, specifically tailored to the requirements of an Alchemy Database system where schema validation is strict but runtime flexibility must be maintained without breaking existing data structures.

Architecture Highlights:
1.  **Policy Engine**: A high-level abstraction managing the lifecycle of credentials and generating policies. It decouples credential management from specific rule logic, allowing for modular expansion without modifying core generators or database schemas.
2.  **Access Control Logic**: Implemented in `SecurityControlPlane` to validate access requests against stored rules (e.g., "read", "write") based on policy types ('explicit', 'implicit'). It supports two modes:
    *   **Explicit Policy**: Strict validation of specific credentials or rule names before execution, ensuring that only authorized entities can perform operations.
    *   **Implicit Policy**: A generalizable check that verifies the action being requested is permitted by any active credential associated with a session, without requiring pre-defined rules for every single operation (e.g., "read" always allowed if not explicitly banned). This allows the system to dynamically update permissions based on user activity or role changes.
3.  **Validation Workflow**: Supports two modes:
    *   **Explicit Policy**: Strict validation of specific credentials or rule names before execution, ensuring that only authorized entities can perform operations and preventing unauthorized access attempts via malformed input or invalid credential formats.
    *   **Implicit Policy**: A generalizable check that verifies the action being requested is permitted by any active credential associated with a session, without requiring pre-defined rules for every single operation (e.g., "read" always allowed if not explicitly banned). This allows the system to dynamically update permissions based on user activity or role changes.
4.  **Error Handling & Context Propagation**: Wraps all database operations and API calls within `SecurityControlPlane` with comprehensive error handling using custom exceptions derived from the Alchemy Database schema errors, ensuring downstream clients receive meaningful context rather than generic failures (e.g., "Unknown Column" warnings without validation). It propagates specific errors via `AlchemyDatabaseError`, maintaining consistency across the application layer.
5.  **Integration & Extensibility**: Designed to be easily integrated into existing workflows by providing a clean abstraction that allows for modular expansion of access control rules, making it easy to extend support for new data types or dynamic permission updates without refactoring core generators like `AlienDataTypeGenerator`.

Usage:
`SecurityControlPlane(policy_type='explicit', allow_credentials=['admin'])` -> Generates policies and validates access.
"""

from __future__ import annotations

import os
import re
import hashlib
import uuid
from typing import Optional, List, Dict, Any, Set


# ---------------------------------------------------------------------------
# Policy Engine Extension: Credential-Based Access Control (CBAC)
# ---------------------------------------------------------------------------

@dataclass
class CredConfig:
    """Configuration for a single credential."""
    
    name: str  # e.g., "admin", "secret_key"
    key_type: str  # 'public' or 'private'
    public_key_path: Optional[str] = None
    private_key_path: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "key_type": self.key_type,
            "public_key_path": self.public_key_path if self.public_key_path else "",
            "private_key_path": self.private_key_path if self.private_key_path else "",
        }


@dataclass
class Credential:
    """Represents a stored credential."""
    
    name: str
    key_type: str  # 'public' or 'private'
    public_key: bytes
    private_key: Optional[bytes] = None
    
    def to_dict(self) -> Dict[str, Any]:
        if self.private_key is not None:
            return {
                "name": self.name,
                "key_type": self.key_type,
                "public_key": self.public_key.hex(),
                "private_key": self.private_key.hex() if self.private_key else "",
            }
        elif self.public_key_path and os.path.exists(self.public_key_path):
            return {
                "name": self.name,
                "key_type": self.key_type,
                "public_key_path": self.public_key_path,
                "private_key": None,  # Private keys are not stored as files in this model
            }
        else:
            raise ValueError(f"Unknown credential format for {self.name}")


@dataclass
class AccessPolicyConfig:
    """Configuration defining which credentials should be allowed."""
    
    name: str
    policy_type: str  # 'explicit',
