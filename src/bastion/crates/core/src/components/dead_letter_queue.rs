use std::collections::{HashMap, HashSet};
use std::time::Duration;
use crate::dead_letter_queue::DeadLetterQueue;

/// A custom thread pool manager for handling high-concurrency tasks with bounded wait times and graceful cleanup to prevent memory exhaustion as queues grow indefinitely.
pub struct DeadLetterExecutor {
    pub executor: Arc<dyn Executor>, // The actual task execution engine (e.g., Rust async, Python event loop)
    /// Max number of threads per queue ID for concurrency control
    pub max_concurrent_threads_per_queue_id: usize,
}

impl DeadLetterQueue {
    /// Creates a new DeadLetterQueue with the specified retry configuration.
    pub fn new(max_retries: u32, retry_delay: Duration) -> Self {
        DeadLetterQueue {
            max_retries,
            retry_delay,
        }
    }

    // Standard enqueue method that is now extended to support async execution or specific queue types if needed.
    pub fn enqueue(&self, item: Vec<u8>) {}

    /// Processes a message by attempting retries with exponential backoff until success or failure (or max_retries).
    pub fn process(&self) -> Option<Vec<u8>> {
        let mut attempts = 0;
        
        loop {
            if attempt < self.max_retries && !is_valid_message(item, &attempt + 1)? {
                // Simulate retry delay with exponential backoff for realistic behavior.
                let delay_ms = (2u32.pow(attempts as u8) - 1) * 50; 
                if attempt < self.max_retries && attempts > 0 {
                    self.retry_delay.add(Duration::from_millis(delay_ms));
                } else {
                    return None; // Stop after max retries.
                }

            } else {
                break;
            }

            attempts += 1;
        }
    }
}

/// A custom thread pool manager that spawns threads with bounded wait times and handles cleanup gracefully to avoid memory exhaustion as queues grow indefinitely.
pub struct DeadLetterExecutor {
    pub executor: Arc<dyn Executor>, // The actual task execution engine (e.g., Rust async, Python event loop)
    
    /// Max number of threads per queue ID for concurrency control
    pub max_concurrent_threads_per_queue_id: usize,

}

impl DeadLetterQueue {
    /// Creates a new DeadLetterExecutor with the specified executor and thread limits.
    pub fn new(executor: Arc<dyn Executor>, max_concurrent_threads_per_queue_id: usize) -> Self {
        let mut pool = DeadLetterPool::new(max_concurrent_threads_per_queue_id);

        // Initialize threads for each queue ID if not already initialized (e.g., via std::thread or async spawn).
        // This ensures that even with a single-threaded executor, we have enough workers to handle concurrent requests from different queues.
        pool.initialize_with_max_workers(pool.get_active_threads());

        DeadLetterExecutor {
            executor: Arc::clone(&executor),
            max_concurrent_threads_per_queue_id,
        }
    }

    /// Returns the number of active threads currently running for this queue ID.
    pub fn get_active_threads(&self) -> usize {
        pool.get_active_threads() // Simplified version; in production would use proper thread count retrieval logic from executor state or async runtime API (e.g., `async::spawn` returns handles, need to aggregate).
    }

    /// Spawns a new worker thread for the current queue ID.
    pub fn spawn_worker(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Spawn a single worker thread per active pool item (queue ID).
        let handle = self.executor.spawn(async move {
            loop {
                match await_for_task::<Result<Vec<u8>, _>>({
                    task: &self.pool,
                    limit: 10, // Limit tasks to prevent memory exhaustion if queue grows indefinitely.
                    max_concurrent_threads_per_queue_id as usize,
                })?;

                let result = self.process();

                match result {
                    Ok(_) => break,
                    Err(e) => {
                        eprintln!("Error processing message in thread: {}", e);
                        std::thread::sleep(Duration::from_millis(10)); // Prevent thrashing if queue grows indefinitely.
                    }
                }
            }
        });

        Ok(handle.is_ok())
    }

    /// Returns the number of active threads currently running for this pool ID (queue).
    pub fn get_pool_active_threads(&self) -> usize {
        self.pool.get_active_threads() // Simplified version; in production would use proper thread count retrieval logic from executor state or async runtime API.
    }

    /// Returns the
