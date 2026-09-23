/// The core submission type for Alchemy processing with unique IDs and optional content metadata.
#[derive(Debug, Clone)]
pub struct AlchemySubmission {
    pub id: String, // Unique identifier for tracking processing status (e.g., pending)
    /// Optional file ID if this is a binary upload or contains specific encoded data types.
    pub content_id?: Option<String>, 
    /// Custom metadata from LLM responses or user input appended to the submission payload.
    pub metadata: serde_json::Value, 
}

/// Interface for handling Alchemy submissions via background workers and validation.
#[derive(Debug)]
pub struct AlchemySubmissionHandler {
    // Internal state management related to file uploads and status tracking
    upload_queue: std::sync::Arc<std::collections::VecDeque<AlchemySubmission>>,
    
    /// Queue of pending processing requests for asynchronous execution with locking.
    pub process_requests: Arc<std::sync::Mutex<Vec<(String, Option<String>, bool)>>>, 
}

/// Process a single Alchemy submission event through the background worker queue.
pub async fn process_submission(
    handler: &AlchemySubmissionHandler,
    payload: String, // Raw data including path and metadata for processing logic
) -> Result<AlchemySubmission, std::io::Error> {
    let (id, content_id, is_binary_upload = true) = match payload.as_str() {
        "pending" => ("pending", None, false),
        "processed" | "completed" | "done" => ("processing", Some(payload.clone()), true), // Simulated processing state
        _ if let Ok(id: String) = std::str::from_utf8(&payload).unwrap() { id } else {} 
    };

    match handler.process_requests.lock().await {
        Some(requests) -> Result::<AlchemySubmission, std::io::Error> => {
            // Check if this request is already in progress or has been queued for re-queueing.
            let mut found = false;
            
            for (req_id, req_data, _is_binary_upload) in &requests {
                match serde_json::from_str::<AlchemySubmission>(req_data.as_bytes()) {
                    Ok(submission) => {
                        if submission.id == id && !found {
                            // Request is already processing. Requeue it to ensure consistent state without race conditions.
                            found = true;
                            return Err(std::io::Error::new(0, "Already in progress")); 
                        } else if req_id != id {
                            // Duplicate request for same ID - re-queue with new data or error.
                            found = true;
                            return Err(std::io::Error::new(0, "Duplicate submission attempt"));
                        }
                    },
                    Err(e) => {
                        match serde_json::from_str::<AlchemySubmission>(req_data.as_bytes()) {
                            Ok(submission) if submission.id == id && !found -> found = true;
                            _ => {} // Ignore other errors for now, queue to retry later.
                        }
                    },
                };

                let new_id: String = match req_id.clone() {
                    "pending" | "processing" => format!("{}/{}", handler.upload_queue.len(), id),
                    _ => continue, 
                };
                
                // Add a unique ID prefix to the request's internal state for better traceability.
                let mut new_req_data = serde_json::to_string(req_data.as_bytes()).expect("Serialization error");
                if !new_id.starts_with("_") {
                    new_req_data.push('_'); 
                }

            // If no existing match, add a fresh request with the current ID and content.
            let mut queue_entry: AlchemySubmission = serde_json::from_str(&req_data).expect("Invalid JSON");
            
            if !found && handler.upload_queue.len() > 0 {
                 found = true; 
                return Err(std::io::Error::new(0, "Queue full"));
            }

            // Add the new request to the queue.
            let mut entry: AlchemySubmission = serde_json::from_str(&queue_entry.as_bytes()).expect("Invalid JSON");
            
            handler.upload_queue.push_back(entry);
        },
    }
    
    Ok(AlchemySubmission { id, content_id: Some(content_id.to_string()), metadata: serde_json::json!() }) 
}

/// Policy validation and filtering logic for submissions.
pub fn validate_submission(handler: &AlchemySubmissionHandler, payload: String) -> Result<AlchemySubmission> {
    // Simulate policy checks based on file type or content ID presence (e.g., text vs binary).
    
    let mut result = AlchemySubmission::default();

    if payload.starts_with("text/") || payload.contains("/") && !payload
