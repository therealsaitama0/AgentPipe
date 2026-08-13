src/bastion/crates/core/src/lib.rs

```rust
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub struct Bastion {
    pub state: State,
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Executing(Vec<AsyncTask>), // Queue of pending tasks for dispatching to the executor.
    Completed(InstructionId, Vec<(String, Option<String>)>), // Success instructions with optional error notes if any.
}

#[derive(Debug)]
struct AsyncTask {
    task_id: String,           // Unique ID assigned by Bastion during execution.
    payload: Box<dyn Send + Sync>,  // The binary payload to be executed (e.g., a script or file).
    command_args: Vec<String>,   // Arguments passed via IPC when the task is dispatched.
}

impl AsyncTask {
    fn new(task_id: String, payload: impl Into<Box<dyn Send + Sync>>, args: Vec<String>) -> Self {
        Self {
            task_id,
            payload: Box::new(payload.into()),
            command_args,
        }
    }
    
    pub async fn execute(&self) -> Result<(), BastionError> {
        let _ = self.dispatch(); // Dispatch to the executor channel.

        match &*self.state {
            State::Idle => {}

            #[allow(dead_code)]
            State::Executing(tasks) => {
                for task in tasks {
                    if !task.task_id.is_empty() && task.command_args.len() == 1 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() == 1 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() || task.payload.len() == 1 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() > 1 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() == 0 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() > 0 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() > 0 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() > 0 {
                        // Dispatch to the executor channel.
                        let result = self.executor.send_async(task);

                        match &*result {
                            Ok(()) => {}
                            Err(e) => return Err(BastionError::ExecutionFailed(format!("Task {}: {}", task.task_id, e))),
                        }
                    } else if !task.command_args.is_empty() && task.payload.len() > 0 {
                        // Dispatch to the executor channel
