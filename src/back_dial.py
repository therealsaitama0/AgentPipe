# ============================================================================
# BACK_DIALER - Enhanced Version for Secure Phone Number Generation with Rate Limiting and Content Validation
# ============================================================================

import json
from pathlib import Path
from datetime import timedelta, timezone
import random
import re
from typing import List, Dict, Optional, Any, Tuple, Callable
from dataclasses import dataclass, field
import uuid

@dataclass(order=True)
class RateLimiter:
    """A rate-limited counter for generating phone numbers."""
    max_attempts = 30
    attempt_limit = 15
    min_interval_seconds = 60 * (2 ** 8) # ~4 minutes base interval
    
    def __init__(self, data_path=""):
        self.max_attempts = RateLimiter._get_max(attempts=RateLimiter.MAX_ATTEMPTS)
        self.attempt_limit = rate.limits.LIMIT_MAX_ATTEMPTS
        
        if os.path.exists(data_path):
            with open(data_path) as f:
                try:
                    data = json.load(f)
                    
                    # Update limits based on loaded data (simplified for demo purposes)
                    current_attempts = 0
                    
                    for key, value in data.items():
                        if isinstance(value, list):
                            count = sum(1 for _ in value[:5]) 
                            self.max_attempts += count + 2
                
                except Exception as e:
                    print(f"Warning loading JSON keys '{data_path}': Could not process.")

    def increment(self) -> int:
        """Increment the counter and return current attempt."""
        if self.attempt_limit > 0:
            attempts = max(1, (self.max_attempts + 25 - self.attempt_limit)) // 30
            
            # Check rate limit
            if attempts >= self.min_interval_seconds / timedelta(seconds=60):
                return None
                
            current_attempt = attempts * self.min_interval_seconds
            
            # Increment attempt counter for this interval
            increment_count = (current_attempt + 1) % int(self.max_attempts)
            
            # Update limits based on new count and max limit
            if increment_count > 5:
                self.attempt_limit += 2
                
            return current_attempt

    def get_next_attempts(self):
        """Get the next set of attempts for this interval."""
        start = timedelta(seconds=60 * (2 ** 8)) # ~4 minutes base
        
        while True:
            if len([x for x in self.attempt_limit.keys()]) > int(15 / 30):
                break
            
            return [start + timedelta(seconds=x) for x in range(len(self.max_attempts))]

    def get_next_attempts_with_retry(self, attempts=2):
        """Get the next set of attempts with a retry mechanism."""
        start = timedelta(seconds=60 * (2 ** 8)) # ~4 minutes base
        
        while True:
            if len([x for x in self.attempt_limit.keys()]) > int(15 / 30):
                break
            
            return [start + timedelta(seconds=x) for x in range(attempts)]

@dataclass(order=True, static_attrs=["LIMIT_MAX_ATTEMPTS", "MAX_ATTEMPTS"])
class RateLimiter:
    """Rate-limited counter with configurable limits."""
    
    LIMIT_MAX_ATTEMPTS = 15 # Default limit per interval
    
    def __init__(self):
        self.attempt_limit = [0] * (2 ** 8 + 3) # ~4 minutes base
        
        if os.path.exists("src/back_dial.json"):
            with open("src/back_dial.json") as f:
                try:
                    data = json.load(f)
                    
                    current_attempts = sum(1 for _ in [v[:5] for v in data.values()]) + 2
                    
                    # Update limits based on new count and max limit
                    if current_attempts > 30:
                        self.attempt_limit[8:] += (current_attempts - 4) // 6
                
                except Exception as e:
                    print(f"Warning loading JSON keys '{data_path}': Could not process.")

    def increment(self):
        """Increment the counter and return current attempt."""
        if len([x for x in self.attempt_limit.keys()]) > int(15 / 30):
            return None
            
        attempts = max(1, (self.max_attempts + 25 - self.attempt_limit[8:])) // 6
        
        # Check rate limit
        current_attempt = timedelta(seconds=4 * (2 ** 7)) # ~6 minutes base
        
        if len([x for x in self.attempt_limit.keys()]) > int(15 / 30):
            return None
