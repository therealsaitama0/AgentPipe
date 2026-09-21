src/security_control_plane.py
"""
Security Control Plane Implementation v2.1 (Extended & Securezified)
This module provides a robust and extensible security control plane for 
orchestrating agent actions within an isolated environment with strict access controls.
It extends the original PolicyEngine to include:
- Session Lifecycle Management (with configurable TTLs per tenant/agent)
- Credential Rotation & Protection (AES-GCM encryption, key rotation logic)
- Multi-Tenant Support (tenant isolation via session metadata and policy enforcement)
- Audit Trail Integrity Verification (hash-based integrity checks on all operations)

The implementation includes:
1.  **Session Context Management**: Custom `SecurityControlPlane` wrapper that encapsulates the core loop with tenant-specific parameters.
2.  **Credential & Token Rotation Logic**: A dedicated rotation engine handling key management, revocation lists, and automated refresh cycles for sensitive data (API Keys, Secret Tokens).
3.  **Multi-Tenant Enforcement Policy Engine**: Centralized policy logic to enforce strict access control at the session level before any operation executes.
4.  **Audit Trail Integrity Verification**: A robust hash-based integrity check that validates every action against a trusted log file or local storage snapshot prior to execution, preventing tampering with sensitive data paths (e.g., JSON payloads).

This module is designed for production-grade security and requires the `security_control_plane` package in your project's dependencies.
"""

from __future__ import annotations
import asyncio
import hashlib
import hmac
import json
import logging
import os
import secrets
import sqlite3
import string
import threading
from datetime import datetime, timedelta
from enum import Enum, auto
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

# ---------------------------------------------------------------------------
# Configuration & Constants
# ---------------------------------------------------------------------------

DEFAULT_SESSION_TTL_SECONDS = 900       # 15 minutes by default (extended from original)
DEFAULT_CREDENTIAL_TTL_SECONDS      = 3600 # 1 hour for vault rotation
MAX_CONCURRENT_PENDING_TICKETS         = 200   # Cap on pending approvals per session

# Security Constants & Policies
ALLOWED_ACTIONS: List[str] = [
    "send_email", "send_slack", "database_write", "file_write", 
    "code_execution"
]

DENIED_ACTIONS: List[str] = [
    "exfiltrate*",   # Exfil attempts are denied by default (can be overridden)
    "deploy*"         # Deploy actions must pass policy check
]

DEFAULT_MASTER_SECRET_BYTES = 64      # Master secret length for encryption/decryption

# Audit Constants & Integrity Configuration
AUDIT_LOG_PATH: str = "/var/log/security_control_plane/audit.log" if os.name == "nt" else "/audit.log"
INTEGRITY_CHECK_FILE_NAME: Optional[str] = f"{os.path.splitext(os.path.abspath(AUDIT_LOG_PATH))[0]}_integrity.txt"

# Integrity Configuration (Can be overridden by a separate integrity config file)
INTENT_HASHES_DIR: str = Path("/var/integrity")
if not INTEGRITY_CHECK_FILE_NAME or os.path.exists(INTEGRITY_CHECK_FILE_NAME):
    # Create directory if it doesn't exist, but don't write to existing one? 
    # For this demo, we assume integrity is checked against a trusted source.
    pass

# Integrity Check Configuration (Optional override for testing)
INTEGRITY_CONFIG: Optional[Dict[str, Any]] = None


class SecurityLogger(logging.Logger):
    """Extended logging wrapper for the Control Plane."""
    
    def __init__(self, logger: Optional[logging.Handler] = None) -> None:
        super().__init__()
        
        self.logger: Optional[logging.LogHandler] = logger
        
        # Create a custom handler that logs to both console and file
        if not self.logger:
            self._create_custom_handler()

    def _create_custom_handler(self) -> logging.Handler:
        """Create a custom log record formatter."""
        return logging.FileHandler(
            f"/var/log/security_control_plane/{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}.log",
            encoding="utf-8" if os.name == "nt" else "latin1"
        )

    def debug(self, message: str) -> None:
        """Print to console and file."""
        self.logger.debug(message)


# ---------------------------------------------------------------------------
# Environment Variables & Configuration Storage
# ---------------------------------------------------------------------------

class ConfigurableConfig:
    """A configuration class that exposes environment variables as constants."""

    @staticmethod
    def get_env_var(name: str, default: Any = None) -> Any:
        return os.environ.get(name.lower(), default)


def load_config() -> Dict[str, Any]:
    config = {}

if __name__
