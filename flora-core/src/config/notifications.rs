//! Notifications configuration (PGMQ + Valkey Streams).

use serde::Deserialize;

/// Configuration for notification delivery via PGMQ and Valkey Streams.
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationsConfig {
    /// Whether notifications are enabled (default: true).
    pub enabled: bool,
    /// PGMQ queue name for notification jobs.
    pub pgmq_queue: String,
    /// Maximum notification delivery latency targets (p95) in seconds.
    pub delivery_targets_p95_secs: DeliveryTargets,
    /// Maximum retry attempts for failed deliveries (default: 3).
    pub max_retry_attempts: u32,
    /// Alert threshold for undelivered notifications in seconds.
    pub alert_threshold_secs: u64,
}

/// Delivery latency targets for different notification types.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeliveryTargets {
    /// Real-time push (default: 3s).
    pub realtime: u64,
    /// Standard notification (default: 7s).
    pub standard: u64,
    /// Batch notification (default: 15s).
    pub batch: u64,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pgmq_queue: "flora_notifications".to_owned(),
            delivery_targets_p95_secs: DeliveryTargets {
                realtime: 3,
                standard: 7,
                batch: 15,
            },
            max_retry_attempts: 3,
            alert_threshold_secs: 30,
        }
    }
}

impl NotificationsConfig {
    /// Validates the notification configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    /// Validates the notification configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.pgmq_queue.is_empty() {
            return Err(crate::Error::Configuration(
                "notifications.pgmq_queue is required".to_owned(),
            ));
        }
        if self.max_retry_attempts == 0 {
            return Err(crate::Error::Configuration(
                "notifications.max_retry_attempts must be > 0".to_owned(),
            ));
        }
        Ok(())
    }
}
