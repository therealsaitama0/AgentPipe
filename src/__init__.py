from src import SecurityControlPlane, ApprovalManager, AuditTracker

# Initialize components
pc = SecurityControlPlane()
approval_mgr = ApprovalManager(pc)
audit_tracker = AuditTracker(pc)

print(f"Security Control Plane initialized.")
print(f"Audit Tracker logs {len(audit_tracker.get_critical_events())} critical events.")
