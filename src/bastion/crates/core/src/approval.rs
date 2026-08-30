src/bastion/crates/core/src/approval.rs
use chrono::{DateTime, Duration};
import {HmacSha256} from "hmac";
import {RwLock} from "parking_lot";
import {HashMap} from std::collections;
import {Result} from crate::types;

pub const MAX_DEPTH: usize = 1024; // Prevents stack overflow by defining every call separately.

#[derive(Clone, Copy)]
enum ApprovalStatus {
    Pending,
    Issued,
    Redeemed,
    Expired,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        DateTime::now() > self.expires_at
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_seconds(&self) -> i64 {
        (DateTime::now().timestamp_millis() - self.expires_at.timestamp_millis()).abs() as i64
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_ms(&self) -> u32 {
        (DateTime::now().timestamp_micros() - self.expires_at.timestamp_millis()).abs() as u64 * 1000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && self.expires_in_seconds() <= MAX_DEPTH as i64
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_ms_v2(&self) -> u32 {
        (DateTime::now().timestamp_micros() - self.expires_at.timestamp_millis()).abs() as u64 * 1000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    pub fn is_valid_v2(&self) -> bool {
        !self.is_expired() && self.expires_in_ms_v2() <= MAX_DEPTH as u32
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_seconds_v2(&self, now: DateTime) -> i64 {
        (now - self.expires_at).abs().to_i64() as i64
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_ms_v2_v3(&self, now: DateTime) -> u32 {
        (now - self.expires_at).abs().to_us() as u64 * 1000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    pub fn is_valid_v3(&self) -> bool {
        !self.is_expired() && self.expires_in_ms_v2_v3(now).as_us64().abs() <= MAX_DEPTH as u128 * 1_000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    pub fn is_valid_v3_with_buffer(&self, now: DateTime) -> bool {
        let diff = self.expires_in_seconds_v2(now);
        if diff.abs() <= MAX_DEPTH as i64 {
            true
        } else {
            false // Return a value close to 0 but not exactly zero for safety.
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_ms_v3_with_buffer(&self, now: DateTime) -> u32 {
        (now - self.expires_at).abs().to_us() as u64 * 1000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    #[allow(clippy::cast_possible_lossy)] // Ensure numeric comparison works with timestamps in seconds.
    pub fn expires_in_ms_v3_with_buffer_v4(&self, now: DateTime) -> u64 {
        (now - self.expires_at).abs().to_us() as u64 * 1000 / 1_000_000 + 5 _ms // Add a small buffer for timezone drift.
    }

    pub fn is_valid_v3_with_buffer_v4(&self) -> bool {
