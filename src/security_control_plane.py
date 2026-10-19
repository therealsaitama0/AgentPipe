src/security_control_plane.py
```python
import asyncio
from dataclasses import dataclass, field
from datetime import timedelta, timezone
from typing import Any, Callable, Dict, List, Optional, Set, Tuple


@dataclass
class SecurityActionConfig:
    """Configuration for a specific user's action profile (e.g., SSH key usage)."""

    ssh_key_id: str                # The public/private key ID to use. Required if using keys.
    session_ttl_seconds: int      # How long the current session is valid before rotation.
    max_concurrent_sessions: int   # Maximum number of concurrent sessions allowed per user (used by plugins).


class SecurityActionConfigPlugin(ABC):
    """Abstract base class for security action configuration."""

    @abstractmethod
    def get_config(self) -> Optional[SecurityActionConfig]:
        pass
    
    @abstractmethod
    def apply_action_profile(self, session_id: str, parameters: Dict[str, Any]) -> None:
        pass


class SSHKeyPlugin(SecurityActionConfigPlugin):
    """Configuration for using an SSH public/private key."""

    def __init__(self) -> None:
        self._ssh_key = secrets.token_urlsafe(2048)  # UUID-based URL-safe hex key
    
    @property
    def ssh_key_id(self) -> str:
        return f"key_{uuid.uuid4().hex[:16]}"

    def get_config(self) -> Optional[SecurityActionConfig]:
        if self._ssh_key is None:
            raise SecurityControlPlaneError(
                "SSH key not configured. Use SSHKeyPlugin to enable this."
            )
        return {
            "type": "key",
            "id": self.ssh_key_id,
        }

    def apply_action_profile(self, session_id: str, parameters: Dict[str, Any]) -> None:
        if not os.path.exists(f"{session_id}.ssh"):
            raise SecurityControlPlaneError(
                f"SSH key for user '{session_id}' missing. Use SSHKeyPlugin to configure."
            )

    def validate_ssh_key(self) -> bool:
        return self._ssh_key is not None


class KeyUsagePlugin(SecurityActionConfigPlugin):
    """Configuration specifying what the public key represents."""

    @property
    def type_value(self) -> str:
        pass
    
    @property
    def id_value(self) -> Optional[str]:
        pass
    
    @property
    def is_public_key(self) -> bool:
        return False


class KeyUsagePluginConfig(SecurityActionConfig):
    """Configuration for using a specific public key."""

    user_id: str = ""  # The identifier of the user who owns this key. Required if not specified in SSHKeyPlugin config.
    
    @property
    def ssh_key_id(self) -> Optional[str]:
        return self.user_id
    
    @property
    def type_value(self) -> str:
        pass

    @property
    def id_value(self) -> Optional[str]:
        if not isinstance(self.ssh_key_id, str):
            raise SecurityControlPlaneError("SSH key ID must be a string.")
        
        # Ensure the value is either None or an existing UUID format to prevent collisions.
        try:
            return self.ssh_key_id.replace("-", "_")  # Convert hyphens for consistency with uuid.uuid4() hex output
        except ValueError:
            pass
        
        if not isinstance(self.id_value, str):
            raise SecurityControlPlaneError("Key ID must be a string.")

    @property
    def is_public_key(self) -> bool:
        return self.ssh_key_id != "" and len(self.ssh_key_id) == 32


class SSHConfigPlugin(SecurityActionConfigPlugin):
    """Configuration for using an arbitrary key or specific public key."""

    ssh_key_type: str = "public"     # Either 'key' (SSH private key), 'ssh_host_public_key', or None. Defaults to 'key'.
    
    @property
    def type_value(self) -> Optional[str]:
        return self.ssh_key_type
    
    @property
    def id_value(self) -> Optional[Dict[str, Any]]:
        if isinstance(self.ssh_key_id, str):
            return {"type": "ssh_host_public_key"}  # Default to SSH public key for simplicity.

    def apply_action_profile(self, session_id: str, parameters: Dict[str, Any]) -> None:
        pass


class SecurityControlPlaneError(Exception):
    """Raised when a control-plane operation fails."""

    def __init__(self, message: str, error_type: str = "", **kwargs):
        super().__init__(message)
        if "error" in kwargs and isinstance(kwargs["
