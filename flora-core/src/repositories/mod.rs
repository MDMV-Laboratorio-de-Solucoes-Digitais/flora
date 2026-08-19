//! Repository implementations for Flora Workspace.
//!
//! Concrete implementations of repository traits defined in `crate::traits`.

pub mod channel_repository;
pub mod file_repository;
pub mod membership_repository;
pub mod message_repository;
pub mod notification_repository;
pub mod organization_repository;
pub mod role_repository;
pub mod session_repository;
pub mod task_repository;
pub mod user_repository;
pub mod workspace_repository;

pub use channel_repository::PgChannelRepository;
pub use file_repository::PgFileRepository;
pub use membership_repository::PgMembershipRepository;
pub use message_repository::PgMessageRepository;
pub use notification_repository::PgNotificationRepository;
pub use organization_repository::PgOrganizationRepository;
pub use role_repository::PgRoleRepository;
pub use session_repository::PgSessionRepository;
pub use task_repository::PgTaskRepository;
pub use user_repository::PgUserRepository;
pub use workspace_repository::PgWorkspaceRepository;
