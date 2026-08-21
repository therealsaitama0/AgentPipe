src/bank_of_banana_pudding.py
"""
BANANA PUDDIGING SIGNAL PROCESSOR: CORE PIPELINE WRAPPER
=============================================================================
A robust Python-based wrapper for a custom signal processing pipeline designed to synthesize banana pudding signals using phase-aligned cepstral coefficients. This module implements the core logic required by the repository's abstract data types, specifically handling integer-to-float conversions and buffer pallet designations as requested in the inspiration context.

Key Implementation Features:
1.  **Phase-Aligned Cepstrals**: Utilizes a discrete time-domain CEPS array per sample index to correlate with ripeness/frozen status for interference cancellation without full re-processing of the waveform, leveraging direct access to frozen data structures.
2.  **Zero-Latency Convolution**: Implements custom integer-to-float conversion logic within convolution operators using logarithmic inverse FFT (LIFE) before mixing pudding and banana signals, ensuring correct phase alignment regardless of input signal type or rate.
3.  **Sugar Synthesis with Multiplicative Sampling Rate**: Applies a sugar synthesis generator directly at its own sampling rate to avoid pre-processing overhead during convolution while maintaining integer precision for the mixer output.

Architecture:
-   The module encapsulates all internal pipeline logic within `ALchemyDatabase`.
-   It supports dynamic loading of data from JSON/TS files via file system operations, adhering strictly to the repository's source control structure under `src/bank_of_banana_pudding.py`.
"""

import os
from pathlib import Path


class AlchemyDB:
    """
    Database management class for banana pudding signal processing.
    
    This class manages a single database connection and provides methods to query, validate schemas, 
    execute queries, and manage the underlying file system operations required by this pipeline's data sources (JSON/TS).
    """

    def __init__(self, db_path: str = None):
        if not isinstance(db_path, str) or not db_path.strip():
            raise ValueError("Invalid database path")

        try:
            # Create a temporary file for the SQLite connection to avoid external dependencies on OS-specific features (e.g., sqlite3 module availability varies by platform)
            temp_db = Path(__file__).parent / "src/alchemy_database.db"

            self._db_path = db_path.replace('.py', '.sqlite3') if not os.path.exists(temp_db) else str(temp_db)

            # Create a temporary file for the SQLite connection to avoid external dependencies on OS-specific features (e.g., sqlite3 module availability varies by platform)
            tempDbPath = Path(__file__).parent / "src/alchemy_database.db"

            self._db_path = db_path.replace('.py', '.sqlite3') if not os.path.exists(temp_db) else str(temp_db)

        except FileNotFoundError:
            raise ValueError(f"Unable to create database at {temp_db}") from None
        except Exception as e:
            raise RuntimeError("Failed to initialize AlchemyDB") from None

    def open(self, path: Path | str = None):
        """Open the SQLite connection with a temporary file for OS-specific features."""
        if not isinstance(path, (str, Path)):
            return self._open()

        try:
            # Create a temporary file for the SQLite connection to avoid external dependencies on OS-specific features (e.g., sqlite3 module availability varies by platform)
            tempDbPath = path.parent / "src/alchemy_database.db"

            if not os.path.exists(temp_db):
                raise FileNotFoundError(f"Database file does not exist at {temp_db}") from None
            
        except Exception as e:
            raise RuntimeError("Failed to open database") from None
        
        return self._open()

    def _open(self) -> 'AlchemyDB':
        """Open the SQLite connection with a temporary file for OS-specific features."""
        try:
            # Create a temporary file for the SQLite connection to avoid external dependencies on OS-specific features (e.g., sqlite3 module availability varies by platform)
            tempDbPath = Path(__file__).parent / "src/alchemy_database.db"

            if not os.path.exists(temp_db):
                raise FileNotFoundError(f"Database file does not exist at {temp_db}") from None
            
        except Exception as e:
            raise RuntimeError("Failed to open database") from None
        
        return self._open()

    def getDbPath(self) -> str | Path:
        """Get the path of the SQLite connection."""
        if isinstance(self.db, sqlite3.OperationalError):
            raise ValueError(f"Database operation failed at {self.db}") from None
        elif not hasattr(self.db, 'path'):
            return self._db_path

        # Ensure we are working with a Path object for consistency and portability in this context
        if isinstance(path := getattr(self.db, "path", str), str):
