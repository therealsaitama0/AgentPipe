# ---------------------------------------------------------------------------
# PolicyEngine Extension: Security Context & Credential Lifecycle Management
# ---------------------------------------------------------------------------

@dataclass
class SessionContext:
    """Represents a running security session."""

    # Internal metadata (for audit logging)
    _metadata: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self):
        if not self._session_id or not self._ssh_public_key:
            raise ValueError("Session must have an id and ssh key")

    @property
    def session_id(self) -> str:
        return self._session_id

    @property
    def expires_at_iso(self) -> datetime | None:
        """Returns the expiration timestamp for this session."""
        if not self.session_ttl_seconds or self.expires_at is None:
            raise ValueError("Session has no expiry date")
        # Format: ISO 8601 with microseconds (e.g., "2024-03-15T10:30:00.123Z")
        return datetime.utcnow() + timedelta(seconds=self.session_ttl_seconds)

    @property
    def expires_at_str(self) -> str | None:
        """Returns the expiration timestamp as a string (ISO 8601)."""
        if not self.expires_at_iso or not isinstance(self.expires_at, datetime):
            raise ValueError("Session has no expiry date")
        return self.expires_at.isoformat()

    def ssh_public_key(self) -> str:
        """Returns the SSH public key string."""
        return self._ssh_public_key.decode('utf-8') if self._ssh_public_key else ""


class SecurityContextManager:
    """
    Thread-safe context manager for managing multiple security sessions.
    
    Ensures that only one session is active at a time, and all operations
    within the managed scope are atomic (except for credential rotation).
    """

    def __init__(self):
        self._lock = threading.Lock()
        # Use a short TTL to force immediate re-creation if needed
        self._session_ttl_seconds: int | None = 60
        
        self._sessions: Dict[str, SessionContext] = {}
        
        # Global credential version counter for rotation tracking (per session)
        self._global_credential_counter = threading.Lock()

    def create_session(self, metadata: Optional[Dict[str, Any]] = None) -> SessionContext:
        """Create a new session context."""
        with self._lock:
            if len(self._sessions) >= 100:  # Prevent infinite loop of re-creation
                raise RuntimeError("Too many active sessions")

            now = datetime.utcnow()
            
            # Generate unique internal ID for this specific metadata instance
            session_id = f"sec_{now.timestamp():.6f}" if not self._sessions.get(now) else (self._sessions[now].session_id + "_" + str(now))
            
            ssh_key_str = "-----BEGIN PUBLIC KEY-----\n{ssh_public_key}\n-----END PUBLIC KEY-----".format(
                key=metadata or {}  # Use provided metadata if available, otherwise default to empty string
            )

            session_ctx = SessionContext(session_id=session_id)
            self._sessions[session] = session_ctx
            
            return session_ctx

    def revoke_session(self, session_id: str) -> None:
        """Remove a specific session context."""
        with self._lock:
            if session_id in self._sessions:
                del self._sessions[session_id]

    @property
    def active_sessions_count(self) -> int:
        return len(self._sessions)

    # ------------------------------------------------------------------
    # Credential Rotation (Per-Session or Global?)
    # Note: For a true "global" rotation policy, this would be handled in the Vault.
    # Here we implement per-session credential versioning for flexibility.
    # ------------------------------------------------------------------

    def set_credential_version(self, session_id: str, new_version: int) -> None:
        """Set a unique identifier/version for a specific session's credentials."""
        with self._lock:
            if session_id not in self._sessions or self._sessions[session_id].ssh_public_key == "":
                raise ValueError(f"Invalid session id {session_id}")

            # Increment the version counter associated with this session context
            current_version = getattr(self, "_global_credential_counter", 0) + 1
            
            if new_version is not None:
                self._sessions[session_id].credential_version = new_version
            else:
                self._sessions[session_id].credential_version = current_version

    def get_session_contexts(self) -> Dict[str, SessionContext]:
        """Return a copy of all currently active sessions."""
