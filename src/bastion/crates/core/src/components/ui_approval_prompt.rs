use std::collections::{HashMap, HashSet};
use std::time::Duration;
use anyhow::Result;

/// Contextual state for managing approval requests within a workflow component hierarchy.
#[derive(Debug)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Calculates urgency based on the proximity of sentiment to thresholds or pending count limits.
    fn calculate_urgency(&self) -> Result<f64> {
        let mut score = 0f64;

        // Check if current state exceeds timeout threshold (e.g., more than one cycle overdue)
        match self.state() {
            ApprovalPromptState::Overdue => {
                score += Duration(1.5).duration(); // High urgency: immediate action required
            }
            _ => {}
        }

        // Check if sentiment is extremely close to a critical threshold (e.g., negative 90% of max)
        match self.sentiment() {
            ApprovalPromptSentiment::Critical(1.0) => score += Duration(25).duration(), // Critical urgency: immediate action required
            _ => {}
        }

        Ok(score.as_f64())
    }

    /// Determines the current state of a pending approval request based on accumulated time or sentiment history.
    fn state(&self) -> ApprovalPromptState {
        match self.state_history() {
            ApprovalPromptHistory::Pending => ApprovalPromptState::Pending,
            _ => unreachable!("Invalid state in component tree"), // Should not happen if logic is sound
        }
    }

    /// Determines the current sentiment of a pending approval request based on accumulated history.
    fn sentiment(&self) -> ApprovalPromptSentiment {
        match self.sentiment_history() {
            ApprovalPromptHistory::Pending => ApprovalPromptSentiment::Critical(1.0), // Default critical if no data available
            _ => unreachable!("Invalid state in component tree"), // Should not happen if logic is sound
        }
    }

    /// Returns the current accumulated sentiment history for this approval prompt instance.
    fn sentiment_history(&self) -> ApprovalPromptHistory {
        let mut hist = ApprovalPromptHistory::Pending;

        match self.state() {
            ApprovalPromptState::Overdue => {
                // If overdue, add a long-term negative score indicating urgency without immediate action
                hist |= ApprovalPromptSentiment::Critical(0.5); 
            }
            _ => {}
        }

        HistResult::Pending(hist)
    }
}

#[derive(Debug)]
enum ApprovalPromptState {
    Pending, // No pending approvals yet or very recent ones (e.g., < 24h)
    Overdue,   // Approvals are overdue and need immediate attention
    Expired,   // All pending approvals have timed out/failed
}

#[derive(Debug)]
enum ApprovalPromptSentiment {
    Critical(1.0),     // Highest urgency: Immediate action required (e.g., 95% negative sentiment)
    Normal(0.8),       // Moderate concern (e.g., >70% positive but < critical threshold, or high pending count)
    Low(0.6),          // Minor issue (e.g., low pending approvals or recent activity only)
}

#[derive(Debug)]
enum ApprovalPromptHistory {
    Pending,           // No history yet
    Positive(Critical(1.0)),   // Recent positive sentiment with high urgency
    Negative(Normal(0.8)),  // Past negative sentiment indicating issues
    Mixed(MixedSentiment), // Current mixed state (e.g., pending approvals + recent activity)
}

#[derive(Debug)]
enum HistResult {
    Pending,           // No history yet or very new data not captured
    Positive(Critical(1.0)),   // Recent positive sentiment with high urgency
    Negative(Normal(0.8)),  // Past negative sentiment indicating issues
    Mixed(MixedSentiment), // Current mixed state (e.g., pending approvals + recent activity)
}

impl ApprovalPrompt {
    pub async fn prompt(&self, action: &crate::Action) -> Result<bool, String> {
        let urgency = self.calculate_urgency()?;

        match self.state() {
            ApprovalPromptState::Pending => Ok(true), // Pending approvals are valid for this context
            _ => Err(format!("Approval timeout expired after {} seconds", urgency)),
        }
    }
}
