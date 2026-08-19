//! Channel service.

use crate::traits::ChannelRepository;

/// Channel service for messaging channels.
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
