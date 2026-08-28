use std::{collections::BTreeMap, fmt::Display, io::Write, path::PathBuf};

/// Represents a binary payload that can be stored in our security plane.
#[derive(Debug)]
pub struct BinaryPayload {
    pub data: Vec<u8>,
}

impl Display for BinaryPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary payload with size {}", self.data.len())?;
        Ok(())
    }
}

/// Maximum allowed payload size in bytes.
const MAX_PAYLOAD_SIZE: usize = 4096; // Approximate limit for security reasons

/// Represents a valid binary data block to be stored or retrieved from the vault.
#[derive(Debug, Clone)]
pub struct BinaryDataBlock {
    pub id: u32,
    pub payload_size: usize,
}

impl Display for BinaryDataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID={}, PayloadSize={}", self.id, self.payload_size)?;
        Ok(())
    }
}

/// A public API method to validate and store a binary payload.
pub fn validate_and_store<P: Into<BinaryDataBlock>>(payload_data: P) -> Result<(), BastionError> {
    let mut block = BinaryDataBlock::new();
    
    // Check if the data is empty or too large (security check)
    if payload_data.len() == 0 || payload_data.len() > MAX_PAYLOAD_SIZE as usize {
        return Err(BastionError::InvalidPayloadSize);
    }

    block.payload_size = payload_data.len();
    
    // Verify that the data is actually binary and valid (no null bytes in this simple check)
    if !payload_data.is_empty() && payload_data.iter().any(|&b| b == 0x00) {
        return Err(BastionError::InvalidPayloadData);
    }

    // Write to the vault
    let mut vault = Vault::new();
    
    // In a real scenario, you would write here. 
    // Here we simulate writing by creating an in-memory block for demonstration purposes.
    if !vault.empty() {
        return Err(BastionError::VaultAlreadyFull);
    }

    let mut vault_block = BinaryDataBlock { id: 0u32, payload_size: 0 };
    
    // Simulate storage logic (in a real app this would be `vault.write(&mut vault_block)`)
    if !vault.empty() && block.payload_size == 0 || block.id > vault.block_count.max_value() {
        return Err(BastionError::VaultFull);
    }

    vault.push(block, &mut vault_block); // Push to list of blocks
    
    Ok(())
}

/// Represents a single action that can be executed from the security plane.
#[derive(Debug)]
pub struct Action {
    pub id: u64,
    pub payload_size: usize,
    pub description: String,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action ID={:?}, PayloadSize={}, Description= {}", self.id, self.payload_size, self.description)?;
        Ok(())
    }
}

/// Represents a public API method to validate and store an action payload.
pub fn validate_and_store<P: Into<BinaryDataBlock>>(action_data: P) -> Result<(), BastionError> {
    let mut block = BinaryDataBlock::new();
    
    if action_data.len() == 0 || action_data.len() > MAX_PAYLOAD_SIZE as usize {
        return Err(BastionError::InvalidPayloadSize);
    }

    block.payload_size = action_data.len();
}

/// Represents a public API method to retrieve an existing payload from the vault.
pub fn get_block<P: Into<BinaryDataBlock>>(block_id: P) -> Result<Option<BinaryDataBlock>, BastionError> {
    let mut blocks = Vault::get_blocks(&mut Vec::<&[u8]>::new()); // Simplified mock
    
    if !blocks.is_empty() && matches!(blocks.last().unwrap(), &BinaryDataBlock { id, .. } == Some(block_id)) {
        Ok(Some(blocks.last()?.clone()))
    } else {
        None
    }
}

/// Represents a public API method to create a new payload block in the vault.
pub fn push_block<P: Into<BinaryDataBlock>>(block_data:
