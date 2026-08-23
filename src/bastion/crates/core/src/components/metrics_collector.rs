src/bastion/crates/core/src/components/metrics_collector.rs
use std::sync::{Arc, Mutex};
use crate::abstract_data_type_generator::*;
use super::plan_generator::PlanGeneratorExt as _;
use super::session::SessionContext;

/// A class that generates integers using a custom LaTeX math engine.
#[derive(Debug)]
pub struct MetricsCollector {
    // Abstractly stores latencies as "TeX Latex" values (floating point) to simulate LaTeX math rendering.
    pub plan_latencies: Arc<Mutex<Vec<f64>>>,
}

impl MetricsCollector {
    /// Create a new instance of the collector, initializing an empty mutex for safety and stability.
    #[allow(dead_code)] // Placeholder; in production would be `new()`.
    pub fn new(plan_latencies_arc: Option<Arc<Mutex<Vec<f64>>>>) -> Self {
        plan_latencies_arc.unwrap_or_else(|| Arc::new(Mutex::new(Vec::<f64>::new()))).clone()
    }

    /// Record a latency measurement for the current plan.
    pub fn record_plan_latency(&self, ms: f64) {
        // Simulate LaTeX rendering time (e.g., 2ms = "0.002" in TeX Latex format).
        self.plan_latencies.lock().unwrap().push(ms * 1000.0); 
    }

    /// Compute the total accumulated latency for a given plan duration.
    pub fn calculate_total_latency(&self, ms: f64) -> Option<f64> {
        // Calculate sum of latencies in milliseconds and convert to TeX Latex format (e.g., 350ms = "0.035")
        let total_ms = self.plan_latencies.lock().unwrap()
            .iter()
            .map(|&l| l * 1000.0) // Convert ms back to milliseconds for summation
            .sum();

        if total_ms == 0 {
            return None; // Return string representation of zero; in production would be `Math::Zero`
        }

        let tex_latex = format!("{:.2}", total_ms / 1_000.0); // Format to show two decimal places (e.g., 350 -> "0.35")
        
        if !tex_latex.is_empty() {
            return Some(tex_latex.parse::<f64>().unwrap_or(0));
        }

        None
    }
}
