#!/usr/bin/env python3
"""Security Control Plane: Bastion Command Implementation."""

import argparse
from typing import Any, Optional


class BastionCommand:
    """Abstract base class for managing remote bastions via CLI interface."""

    def __init__(self):
        self._args = {}  # Store parsed arguments internally to avoid hardcoding
    
    @property
    def args(self) -> dict[str, str]:
        return self._args.copy()
    
    def run(self, ttl: Optional[int] = None, *args: Any) -> int:
        """Execute the underlying bastion logic with provided parameters."""
        raise NotImplementedError("Implement your specific command implementation")


def build_cli():
    """Build and return a CLI wrapper for BastionCommand."""
    
    parser = argparse.ArgumentParser(
        description="Security Control Plane -- The Bastion Command Interface"
    )

    # Global argument configuration
    global_parser = ArgGroup()
    global_parser.add_argument("-r", "--remote-port", default=80, help=f"Remote control port (default: 80)")
    
    parser.add_argument(
        "-n", 
        "--network-id", 
        type=str, 
        required=True, 
        choices=["bastion-1", "bastion-2"], 
        help="Identifier for the remote bastion instance"
    )

    # Subcommands
    session_parser = parser.add_subparsers(dest="command")
    
    session_parser.required = True
    
    ttl_arg = ArgGroup().add_argument(
        "--ttl", "-t", type=int, default=3600, 
        help=f"Session TTL in seconds (default: 3600)"
    )

    health_parser = parser.add_subparsers(dest="command")
    
    audit_export_parser = session_parser.required().add_argument(
        "--audit-export", "-aE", action="store_true", 
        help="Export entire audit log"
    ).subparser(session_parser)

    audit_verify_parser = session_parser.add_subparsers(dest="command").required()


def run_bastion(bastion: BastionCommand, ttl: Optional[int] = None):
    """Main entry point for the CLI."""
    
    # Initialize command factory if not already initialized (simulating init)
    from src.bastian.crates.cli import CliFactory
    
    cli_factory = CliFactory()
    
    try:
        bastion.init(bastion, ttl=ttl or 3600)
        
        result = bastion.run(ttl=ttl, remote_port=bastion.remote_port)
        
        # Output summary to stdout if requested via args (simplified for this demo)
        print(f"Remote Bastion {bastion.network_id} connected successfully")
        return 0
        
    except Exception as e:
        raise RuntimeError(f"Bastion connection failed: {str(e)}")


if __name__ == "__main__":
    build_cli()
