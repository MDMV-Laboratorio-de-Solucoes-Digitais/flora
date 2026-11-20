//! Channel service.

use crate::traits::ChannelRepository;
use std::sync::Arc;

/// Channel service for messaging channels.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct ChannelService {
    channel_repo: Arc<dyn ChannelRepository + Send + Sync>,
}

impl ChannelService {
    /// Creates a new `ChannelService`.
    #[must_use]
    pub fn new(channel_repo: Arc<dyn ChannelRepository + Send + Sync>) -> Self {
        Self { channel_repo }
    }
}
