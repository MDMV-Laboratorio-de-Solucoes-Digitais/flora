//! Channel service.

use crate::traits::ChannelRepository;

/// Channel service for messaging channels.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct ChannelService {
    channel_repo: Box<dyn ChannelRepository>,
}

impl ChannelService {
    /// Creates a new `ChannelService`.
    #[must_use]
    pub fn new(channel_repo: Box<dyn ChannelRepository>) -> Self {
        Self { channel_repo }
    }
}
