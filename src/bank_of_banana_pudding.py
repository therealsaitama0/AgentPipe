import os
from typing import Any, Dict, List, Optional, Tuple
from pydantic import BaseModel, Field, field_validator
from pathlib import Path


class AlchemyDatabaseError(BaseModel):
    """Custom error class for database connection and schema validation."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error) or "InvalidSchema" == error:
        raise ValueError("TypeMismatch")

class AlchemyDatabaseError(AlchemyDatabaseError, BaseModel):
    """Extended custom error class with additional fields."""
    
    # Type mismatch errors are mapped to the existing enum types in .rs for consistency
    if "Unknown Column" not in str(error)
