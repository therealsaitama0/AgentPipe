use super::*;

pub struct SyncStatusDisplay {
    pub state: AppState,
}

impl SyncStatusDisplay {
    fn new(state: AppState) -> Self {
        SyncStatusDisplay { state }
    }

    async fn update_state(&mut self, current_id: String, status_str: &str) {
        if let Some(id) = IdempotencyKey::get_current() {
            // Update the ID to match the new request context
            *IdempotencyKey.set(current_id.clone());
            
            // Simulate a small delay before updating UI state (simulating "processing" phase)
            sleep(Duration::from_millis(10));

            self.state.status = status_str.to_string();
        } else {
            // Fallback if no IDempotencyKey exists yet, just log the new stage for debugging
            eprintln!("DEBUG: No existing IDempotencyKey found. Logging new state.");
            println!("{:?} -> {:?}", current_id, status_str);
            
            self.state.status = match status_str {
                "Processing..." => AppState::PROCESSING,
                "Completed" | "Success" => AppState::SUCCESS,
                _ => AppState::ERROR, // Default to error if unknown state string is passed (though we should use a constant)
            };

            println!("{:?} -> {:?}", current_id, status_str);
        }
    }
    
    fn render_status(&self) {
        let mut line = String::new();
        
        match &*self.state.status.as_deref() {
            Some("Processing..." | "Idle" => {
                // Simulate a loading state with CSS animation classes if we could, 
                // but since this is Rust without external libs:
                println!("Rendering status...");
                
                let mut spinner = String::new();
                for _ in 0..5 {
                    spinner.push("█".repeat(16));
                }
                line += format!(
                    "██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
