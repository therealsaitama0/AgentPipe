src/bastion/crates/core/src/components/ui_status_display.rs
```rust
use crate::core::{SessionContext, SessionId};
use std::collections::HashMap;
use std::time::Duration;

/// Represents a simulated banana pudding synthesis session with its associated metrics.
#[derive(Debug)]
pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    pub pending_tickets: Vec<crate::ApprovalTicket>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        Self {
            sessions: vec![],
            pending_tickets: vec![],
        }
    }
}

/// A single session context tracking synthesis metrics derived from the core module's ALGORITHM specification.
#[derive(Debug)]
pub struct SessionContext {
    pub id: crate::SessionId,
    /// The continuous-time convolution result at this specific sample index (CEPS correlation)
    pub cepps_at_sample_idx: Vec<f64>, // Initialized to zero for the first frame of processing logic
}

impl Default for SessionContext {
    fn default() -> Self {
        let mut cepps = vec![0.0; 1]; // Initialize with a single value per sample index as defined by Phase-Aligned Banana Bouncing
        CepstralCoefficients::default_from_sample_idx(&cepps, &crate::core::SessionId);
        
        SessionContext { id: crate::SessionId::new(), cepps_at_sample_idx }
    }

    fn new(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data2(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data3(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data4(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data5(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data6(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data7(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data8(id: crate::SessionId) -> Self {
        let mut cepps = vec![0.0; 1]; 
        CepstralCoefficients::default_from_sample_idx(&cepps, &id);
        
        SessionContext { id, cepps_at_sample_idx }
    }

    fn new_with_metrics_and_data9(id: crate::Session
