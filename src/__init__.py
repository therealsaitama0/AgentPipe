import os
from typing import Any, Dict, Optional, Union
from contextlib import asynccontextmanager
from pathlib import Path
import json

# SECURITY CONTROL PANE: ROOT LEVEL MODULE INITIALIZATION & POLICY DEFINITION

class SecurityControlPlaneError(Exception):
    """Custom exception for repository security violations."""
    
    def __init__(self, message: str, severity: int = 3) -> None:
        self.message = message
        self.severity = severity
        
    @classmethod
    def from_string(cls, msg: str) -> "SecurityControlPlaneError":
        return cls(msg.strip())

class SecurityControlPlaneConfig:
    """Configuration for the repository security control plane."""
    
    ALLOWED_PERMISSIONS = {
        'read': True, 
        'write': False,  # Access to modify code is restricted unless explicitly requested by a human or approved agent
        'execute': False   # No direct execution of arbitrary logic allowed without explicit policy override
    }

@asynccontextmanager
def async_security_context():
    """Context manager for running security checks within an asynchronous environment."""
    
    try:
        yield
    
    except SecurityControlPlaneError as e:
        raise RuntimeError(f"Security violation detected in context: {e}") from e
    finally:
        # Ensure all required dependencies are loaded before exiting
        if 'requests' not in globals():
            import requests
            setattr(globals(), 'requests', requests)

class AlchemySubmissionHandler:
    """
    Interface for processing and validating code submissions.
    
    This module encapsulates the logic to validate a submission against repository policies, 
    filter based on content (e.g., file type), and return appropriate results or null if rejected.
    """

    def __init__(self):
        self._config = SecurityControlPlaneConfig()

    async def handle_code_upload(
        self, payload: Any  # Payload is expected to be a dict like {'file_path': '...', ...}
    ) -> Optional[Dict[str, Union[int, str]]]:
        """
        Validates and filters code submissions.
        
        Args:
            payload (dict): Raw data containing file path or metadata if provided separately
            
        Returns:
            Dict[str, Any] | None: 
                - If valid: {'id': '...', 'contentId': ..., 'metadata': {...}}
                - If invalid/filtered out: None
        """
        
        # Step 1: Validate payload structure (basic check)
        if not isinstance(payload, dict):
            raise SecurityControlPlaneError(
                "Invalid submission format. Expected a JSON object.", 
                severity=3
            )

        file_path = payload.get('file_path') or 'unknown'
        
        # Step 2: Apply repository policies (e.g., content type filtering)
        if self._config.ALLOWED_PERMISSIONS['write']:
            try:
                import subprocess as sp
                
                # Execute the code to ensure it compiles and runs without errors
                cmd = ['gcc', '-c', file_path]
                
                result = await asyncio.wait_for(sp.run(cmd, check=True), timeout=30)
                
                if not result.returncode == 0:
                    raise SecurityControlPlaneError(
                        f"Compilation failed for {file_path}: return code {result.returncode}. "
                        + "This may indicate security vulnerabilities or syntax errors.", 
                        severity=self._config.ALLOWED_PERMISSIONS['execute']
                    )
                
                # Step 3: Verify the resulting file exists and is valid binary/data
                try:
                    with open(file_path, 'rb') as f:
                        content = await asyncio.wait_for(f.read(), timeout=5)
                    
                    if not isinstance(content, bytes):
                        raise SecurityControlPlaneError(
                            "Content must be byte data.", 
                            severity=self._config.ALLOWED_PERMISSIONS['execute']
                        )
                except Exception as e:
                    # If we can't read the file, it's either corrupted or doesn't exist in a way that matters here
                    if 'file_path' not in payload and self._config.ALLOWED_PERMISSIONS.get('write'):
                        raise SecurityControlPlaneError(
                            f"Could not verify file integrity for {file_path}. "
                            + f"Please ensure the code is properly compiled.", 
                            severity=self._config.ALLOWED_PERMISSIONS['execute']
                        )

            except Exception as e:
                # If we can't execute or read, assume it's a security violation (e.g., malicious payload)
                raise SecurityControlPlaneError(
                    f"Security check failed for {file_path}: {str(e)}", 
                    severity=self._config.ALLOWED_PERMISSIONS['execute'] if self._config.AL
