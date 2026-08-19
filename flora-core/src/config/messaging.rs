//! Valkey (Redis-compatible) configuration for messaging and caching.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MessagingConfig {
    /// Valkey connection URL.
    pub valkey_url: String,
    /// Pub/Sub channel naming convention.
    pub topic_template: String,
    /// Maximum WebSocket fan-out per message (default: 50).
    pub max_fanout: usize,
    /// Message retention in seconds for offline delivery (default: 3600).
    pub message_ttl_secs: u64,
    /// Circuit breaker failure threshold (default: 5).
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker recovery timeout in seconds (default: 30).
    pub circuit_breaker_recovery_secs: u64,
}

impl Default for MessagingConfig {
    fn default() -> Self {
        Self {
            valkey_url: std::env::var("VALKEY_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_owned()),
            topic_template: "org:{@@org_id@@}:channel:{@@channel_id@@}".to_owned(),
            max_fanout: 50,
            message_ttl_secs: 3600,
            circuit_breaker_threshold: 5,
            circuit_breaker_recovery_secs: 30,
        }
    }
}

impl MessagingConfig {
    /// Validates the messaging configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.valkey_url.is_empty() {
            return Err(crate::Error::Configuration(
                "messaging.valkey_url is required".to_owned(),
            ));
        }
        if self.max_fanout == 0 {
            return Err(crate::Error::Configuration(
                "messaging.max_fanout must be > 0".to_owned(),
            ));
        }
        Ok(())
    }

    /// Generates the pub/sub topic for a channel.
    #[must_use]
    pub fn channel_topic(&self, org_id: &str, channel_id: &str) -> String {
        self.topic_template
            .replace("{@@org_id@@}", org_id)
            .replace("{@@channel_id@@}", channel_id)
    }
}
