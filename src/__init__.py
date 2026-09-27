src/__main__.py
"""
Security Control Plane Entry Point for Alchemy Submission Processing Pipeline.

This module provides a mock backend server and handlers to interact with the 
AlchemySubmissionHandler interface defined in src/alchemy_database.ts, simulating 
the real-world validation flow described in the plan. It ensures that all input data is 
processed through security gates before being submitted or rejected as per policy.
"""

import asyncio
from typing import Optional, Any
from pathlib import Path
from contextlib import asynccontextmanager
import logging
import sys
import os
import json
import uuid
import threading
import time
from datetime import timedelta
from dataclasses import dataclass, field


# Configure logging for the control plane system
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout),  # Write to stdout for monitoring
        logging.FileHandler('security_control_plane.log'),  # Write to file for audit
    ]
)

logger = logging.getLogger(__name__)


# ==================== Data Models & Interfaces ====================

@dataclass
class AlchemySubmission:
    """Represents a submitted code or data item from the repository."""
    
    id: str = field(default_factory=lambda: uuid.uuid4().hex[:16])  # Unique identifier for tracking processing status
    
    content_id?: Optional[str]      # ID of uploaded file (if any) - *Optional* per policy, but required by handler interface. We treat it as an internal flag in the mock response if provided.
    
    metadata: dict = field(default_factory=dict)  # Optional custom metadata from LLM response or user input
    
    status: str = "pending"        # Initial state for background processing (e.g., 'queued', 'processing', 'completed')


@dataclass
class SubmissionRequest:
    """The raw payload sent to the handler interface."""
    
    content_id: Optional[str]  # *Optional* - The actual file path or ID if present in user input. If not provided, we treat it as a placeholder for future integration points.
    
    metadata: dict = field(default_factory=dict)


# ==================== Mock Backend Server ====================

class AsyncServerHandler:
    """A mock backend server handling HTTP requests and processing Alchemy submissions."""
    
    def __init__(self):
        self.running = True
        
    @asynccontextmanager
    async def run(self, timeout=30.0):
        """Starts the asyncio event loop with a 15-second timeout for testing scenarios."""
        
        server_thread: Optional[threading.Thread] = None
        
        try:
            # Start background task queue thread (simulating worker)
            self._start_worker_loop()
            
            logger.info(f"Server started. Running in {timeout} seconds.")
            
            while self.running and not asyncio.is_shutdown():
                yield
                
                await time.sleep(0.1)  # Small delay to simulate processing latency
            
        except KeyboardInterrupt:
            logger.warning("Interrupted by user")
        
    def _start_worker_loop(self):
        """Starts the background task queue thread."""
        if not server_thread:
            self.server = threading.Thread(target=self._process_requests, daemon=True)
            server_thread = self.server
        
        # Start main loop with timeout to allow worker threads to run before being killed by user Ctrl+C
        asyncio.run(self.running_loop())


class RequestQueue:
    """Simulates the background task queue for processing submissions."""

    def __init__(self):
        self.queue: list[dict] = []
        
    async def enqueue(self, request_data: Any) -> None:
        """Adds a new request to the queue (simulating incoming requests from /api/alchemy/submissions)."""
        if not isinstance(request_data, dict) or 'content_id' in request_data and request_data['content_id'] is None:
            raise ValueError("Content ID must be provided for processing")

    def get_next_request(self):
        """Gets the next pending request from the queue."""
        while True:
            if not self.queue:
                break
            
            # Simulate a short delay to simulate "processing" time (0.5 seconds)
            await asyncio.sleep(0.1) 
            
            req = self.queue.pop(0)
            
            return {**req, 'processed': False}


class FakeBackendServer:
    """Simulates the backend server logic for handling requests."""

    def __init__(self):
        self._queue = RequestQueue()
        
    async def process_request(self, request_data: dict) -> Optional[dict]:
        """Processes a single incoming request and returns a result or null if blocked by policy."""
        try:
            # Validate
