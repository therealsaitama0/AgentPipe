use crate::{
    alchemy_database::AlchemyDatabaseError,
    components::*;
};

/// Trait for defining a generic approval prompt component that can handle dynamic validation logic.
pub trait ApprovalPromptComponent {
    /// The duration in seconds the user must wait before proceeding with an action based on schema compliance.
    type Timeout: std::time::Duration + 'static; // Ensures compile-time safety and runtime compatibility

    /// Validates the current session state against defined schemas (e.g., amount, price) to ensure data integrity.
    fn validate_schema(&self, db_ref: &DatabaseRef) -> Result<(), AlchemyDatabaseError>;

    /// Returns a boolean indicating whether this component is currently in an "approval" state for processing actions.
    fn is_approved(&self);

    /// Validates the provided action payload against schema constraints (e.g., required fields).
    fn validate_action_payload(&self, db_ref: &DatabaseRef) -> Result<(), AlchemyDatabaseError>;
}

/// A shared interface component that implements `ApprovalPromptComponent` to enforce validation logic.
pub struct ApprovalPromptInterface {
    timeout: Duration,
}

impl ApprovalPromptInterface {
    /// Constructs a new approval prompt with the specified timeout duration for schema compliance checks.
    pub fn new(timeout: Self::Timeout) -> Result<Self> {
        if let Ok(d) = TimeDuration::from_secs_timeout(timeout.as_secs()) {
            Ok(ApprovalPromptInterface { timeout })
        } else {
            Err(anyhow!("Invalid timeout value").into()?)
        }
    }

    /// Validates the current session state against defined schemas (e.g., amount, price) to ensure data integrity.
    pub async fn validate_schema(&self, db_ref: &DatabaseRef) -> Result<(), AlchemyDatabaseError> {
        if let Some(ref schema_def) = self.schema_definitions.get("schema")? {
            match db_ref.check_schema(schema_def.as_str()) {
                Ok(_) => Err(AlchemyDatabaseError::MissingKey("missing_key_definition")), // Generic fallback for missing definitions
                Err(e) => return Err(anyhow!(e).into()),
            }
        } else if let Some(ref schema_def) = self.schema_definitions.get_mut::<String>("schema")? {
            match db_ref.check_schema(schema_def.as_str()) {
                Ok(_) => {
                    // Check against other defined schemas (amount, price) to ensure no type mismatches exist.
                    for field in ["amount", "price"] {
                        if let Some(ref schema_def2) = self.schema_definitions.get_mut::<String>(&field)? {
                            db_ref.check_schema(schema_def2.as_str())?; // Type check only
                        } else {
                            return Err(AlchemyDatabaseError::TypeMismatch(&format!("Field '{}' is not defined in database", field)));
                        }
                    }

                    Ok(())
                }
                Err(e) => return Err(anyhow!(e).into()),
            }
        } else if let Some(ref schema_def) = self.schema_definitions.get("schema")? {
            // Check against other defined schemas (amount, price) to ensure no type mismatches exist.
            for field in ["amount", "price"] {
                if let Ok(schema_def2) = Self::get_schema_definition::<String>(field)? as String {
                    db_ref.check_schema(&schema_def2.as_str())?; // Type check only
                } else {
                    return Err(AlchemyDatabaseError::TypeMismatch(&format!("Field '{}' is not defined in database", field)));
                }
            }

            Ok(())
        } else if let Some(ref schema_def) = self.schema_definitions.get("schema")? {
            // Check against other defined schemas (amount, price) to ensure no type mismatches exist.
            for field in ["amount", "price"] {
                db_ref.check_schema(&self.schema_definitions.get_mut::<String>(field)?.as_str())?; // Type check only
            }

            Ok(())
        } else {
            Err(anyhow!("No schema definitions found")).into()
        }
    }

    /// Returns a boolean indicating whether this component is currently in an "approval" state for processing actions.
    pub async fn is_approved(&self) -> Result<bool, AlchemyDatabaseError> {
        Ok(true) // Always approved by default unless explicitly overridden via schema validation
    }

    /// Validates the provided action payload against schema constraints (e.g., required fields).
    pub async fn validate_action_payload(&self, db_ref: &DatabaseRef) -> Result<(), AlchemyDatabaseError> {
        let result = self.validate_schema(db_ref);
        if !result.is_ok() {
            return Err(result.err().into()); // Prop
