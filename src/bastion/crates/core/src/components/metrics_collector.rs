src/bastion/crates/core/src/components/metrics_collector.rs
```rust
use std::sync::{Arc, Mutex};
use super::*;

/// Represents a configurable metric collection component with optional thresholds and storage backends.
pub struct MetricsCollector {
    pub plan_latencies: Arc<Mutex<Vec<f64>>>, // Plan latency metrics in milliseconds
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Newer implementation with configurable thresholds and structured output via JSON/CSV.
pub struct MetricsCollectorConfig {
    pub plan_latencies: Arc<Mutex<Vec<f64>>>, // Plan latency metrics in milliseconds
    /// Thresholds for filtering metric data (e.g., minimum value, max count)
    #[allow(dead_code)]
    min_threshold: Option<f64> = None,
    /// Maximum allowed number of latencies before triggering alerting
    #[allow(dead_code)]
    max_count_threshold: Option<usize>,
}

impl MetricsCollectorConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update the plan latency metrics with a specific value.
    pub fn record_plan_latency(&self, ms: f64) {
        self.plan_latencies.lock().unwrap().push(ms);
        
        // Optional threshold enforcement for filtering
        if let Some(min_threshold) = &self.min_threshold {
            if *ms < min_threshold {
                return;
            }
        }

        // Optional maximum count enforcement for alerting
        if let Some(max_count_threshold) = self.max_count_threshold {
            if plan_latencies.len() > max_count_threshold {
                metrics_collector_alerts.push(Alert::new("Plan Latency Exceeded", "Maximum allowed latencies exceeded"));
            } else {
                // Optional: Could trigger a warning here, but keeping it minimal for now. 
                // If needed, this could be added to the Alert struct below or as an internal state change.
            }
        }
    }

    /// Generate JSON output of current plan latencies (filtered by threshold if configured).
    pub fn json_output(&self) -> String {
        let mut content = Vec::new();
        
        // Collect valid metrics within the allowed count limit for filtering logic demonstration. 
        // In a production app, this would be an API call to the backend storage service (e.g., Redis or DB).
        if self.plan_latencies.len() <= self.max_count_threshold {
            content.push_str(&format!(r#"["latencies"]"#));

            for latency in &self.plan_latencies {
                let formatted = format!("{}ms", *latency); // Format as string to avoid precision issues with large numbers. 
                                                    // If high precision is needed, use a custom JSON encoder or parse the number directly into an object.
                
                content.push_str(&format!(r#""{formatted}""#));

            }
        } else {
            return String::from("[]"); // Empty array when too many latencies exceed threshold; 
                                           // If `max_count_threshold` is not set, this will be empty anyway.
        }
        
        content.push_str(r#"],\n"count": {},"#);

        let json_content = serde_json::to_string(&content).unwrap();
        String::from(json_content)
    }

    /// Generate CSV output of current plan latencies (filtered by threshold if configured).
    pub fn csv_output(&self, header: &str) -> Vec<String> {
        // Header line. In a real app, this would be fetched from the backend storage service or hardcoded based on config.
        let mut content = vec![String::from(header)];

        // Collect valid metrics within the allowed count limit for filtering logic demonstration. 
        if self.plan_latencies.len() <= self.max_count_threshold {
            for latency in &self.plan_latencies {
                let formatted = format!("{}ms", *latency); 

                content.push(format!("\r\n{}", serde_json::to_string(&[formatted])).unwrap());
            }
        } else {
            return Vec::<String>::new(); // Empty list when too many latencies exceed threshold; 
                                           // If `max_count_threshold` is not set, this will be empty anyway.
        }

        content
    }

    /// Generate a JSON array of all recorded plan latency values (filtered by max count limit).
    pub fn json_array(&self) -> String {
        let mut output = Vec::new();

        if self.plan_latencies.len() <= self.max_count_threshold {
            for latency in &self.plan_latencies {
                // Format as string to avoid precision issues
