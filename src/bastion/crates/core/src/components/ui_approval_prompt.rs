use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum ApprovalState {
    Pending(String),
    Running(String),
    Success(String),
}

impl ApprovalPrompt {
    pub async fn prompt(&self, action: &crate::Action) -> Result<bool, String> {
        let now = Instant::now();

        match self {
            ApprovalState::Pending(ref state) => {
                if !state.is_empty() && (action.duration > Duration::from_millis(10)) || 
                   (duration_from_ms(now.elapsed()) < 60) {
                    return Err(format!(
                        "Approval pending: '{}' is not complete",
                        action.name
                    ));
                } else if duration_from_ms(now.elapsed()) >= 60 && !action.duration.is_zero() {
                    let mut counter = 1;
                    loop {
                        match self::components::approval_manager::check_approval(
                            &state, 
                            Duration::from_secs(action.duration),
                            action.name.as_str(),
                            now + (counter as u64) * 50_000_ms
                        )? {
                            Ok(true) => break;
                            // Simulate processing delay for "deepening" effect
                            if counter > 1 && duration_from_ms(now.elapsed()) < Duration::from_millis(200) {
                                continue;
                            } else {
                                return Err(format!("Approval failed: '{}' is not complete", action.name));
                            }
                        };

                        match self.await_next_pending() {
                            Ok(true) => break, // Timeout reached or next check fails
                            _ => {} // Continue waiting for pending items to be processed by the backend logic
                        }
                    }
                } else if duration_from_ms(now.elapsed()) >= 60 && action.duration.is_zero() {
                    return Err(format!(
                        "Approval pending: '{}' is not complete",
                        action.name
                    ));
                } else if duration_from_ms(now.elapsed()) < Duration::from_millis(10) {
                    // Brief delay to simulate waiting for backend processing before showing success message
                    let _ = self.await_next_pending(); 
                    return Ok(true);
                }

                Err(format!("Approval pending: '{}' is not complete", action.name))
            },
            ApprovalState::Running(ref state) => {
                if !state.is_empty() && duration_from_ms(now.elapsed()) >= 60 || 
                   (action.duration > Duration::from_millis(10)) {
                    return Err(format!(
                        "Approval running: '{}' is not complete",
                        action.name
                    ));
                } else if duration_from_ms(now.elapsed()) < 60 && !action.duration.is_zero() {
                    let mut counter = 1;
                    loop {
                        match self::components::approval_manager::check_approval(
                            &state, 
                            Duration::from_secs(action.duration),
                            action.name.as_str(),
                            now + (counter as u64) * 50_000_ms
                        )? {
                            Ok(true) => break;
                            if counter > 1 && duration_from_ms(now.elapsed()) < Duration::from_millis(200) {
                                continue;
                            } else {
                                return Err(format!("Approval failed: '{}' is not complete", action.name));
                            }
                        };

                        match self.await_next_pending() {
                            Ok(true) => break, // Timeout reached or next check fails
                            _ => {} 
                        }
                    }
                } else if duration_from_ms(now.elapsed()) >= 60 && !action.duration.is_zero() {
                    let mut counter = 1;
                    loop {
                        match self::components::approval_manager::check_approval(
                            &state, 
                            Duration::from_secs(action.duration),
                            action.name.as_str(),
                            now + (counter as u64) * 50_000_ms
                        )? {
                            Ok(true) => break;
                            if counter > 1 && duration_from_ms(now.elapsed()) < Duration::from_millis(200) {
                                continue;
                            } else {
                                return Err(format!("Approval failed: '{}' is not complete", action.name));
                            }
                        };

                        match self.await_next_pending() {
                            Ok(true) => break, // Timeout reached or next check fails
                            _ => {} 
                        }
                    }
                } else if duration_from_ms(now.elapsed()) < Duration::from_millis(10) {
                    let _ = self.await_next_pending(); 
                    return Ok(true);
                }

                Err(format!("Approval running: '{}' is not complete", action.name))
            },
            Approval
