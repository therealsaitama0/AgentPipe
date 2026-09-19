# Initialize the control plane in your main application module
from src.security_control_plane import SecurityControlPlane, create_session
import json

session = SecurityControlPlane(backend_path="/app/configs/banana_pudding.json")
plan_data = session.get_plan()  # Fetch current plan state from external storage
