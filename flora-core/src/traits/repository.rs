//! Repository traits for Flora Workspace.
//!
//! These define the data access layer interfaces that concrete implementations must satisfy.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{
    Channel, File, Membership, Message, Notification, Organization, Page, Pagination, Role,
    Session, Task, TaskStatus, UpdateOrganizationInput, UpdateUserInput, User, Workspace,
};

/// Repository trait for User operations.
#[async_trait]
pub trait UserRepository: Send + Sync + std::fmt::Debug {
    /// Find a user by email.
    async fn find_by_email(&self, email: &str) -> crate::Result<Option<User>>;

    /// Find a user by OIDC subject.
    async fn find_by_oidc_subject(&self, subject: &str) -> crate::Result<Option<User>>;

    /// Find a user by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<User>>;

    /// Create a new user.
    async fn create(&self, user: User) -> crate::Result<User>;

    /// Update a user.
    async fn update(&self, id: Uuid, updates: UpdateUserInput) -> crate::Result<User>;

    /// Deactivate a user (soft delete).
    async fn deactivate(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Organization operations.
#[async_trait]
pub trait OrganizationRepository: Send + Sync + std::fmt::Debug {
    /// Find an organization by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Organization>>;

    /// Find an organization by slug.
    async fn find_by_slug(&self, slug: &str) -> crate::Result<Option<Organization>>;

    /// List organizations with pagination.
    async fn list(&self, pagination: Pagination) -> crate::Result<Page<Organization>>;

    /// Create a new organization.
    async fn create(&self, org: Organization) -> crate::Result<Organization>;

    /// Update an organization.
    async fn update(
        &self,
        id: Uuid,
        updates: UpdateOrganizationInput,
    ) -> crate::Result<Organization>;

    /// Delete an organization (cascade delete all related data).
    async fn delete(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Membership operations.
#[async_trait]
pub trait MembershipRepository: Send + Sync + std::fmt::Debug {
    /// Find memberships for a user.
    async fn find_by_user_id(&self, user_id: Uuid) -> crate::Result<Vec<Membership>>;

    /// Find memberships for an organization.
    async fn find_by_organization_id(
        &self,
        organization_id: Uuid,
    ) -> crate::Result<Vec<Membership>>;

    /// Find a specific membership (user + organization).
    async fn find_by_user_and_organization(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> crate::Result<Option<Membership>>;

    /// Create a new membership.
    async fn create(&self, membership: Membership) -> crate::Result<Membership>;

    /// Delete a membership.
    async fn delete(&self, user_id: Uuid, organization_id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Role operations.
#[async_trait]
pub trait RoleRepository: Send + Sync + std::fmt::Debug {
    /// Find a role by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Role>>;

    /// Find roles for an organization.
    async fn find_by_organization_id(&self, organization_id: Uuid) -> crate::Result<Vec<Role>>;

    /// Find a role by name within an organization.
    async fn find_by_name_and_organization(
        &self,
        name: &str,
        organization_id: Uuid,
    ) -> crate::Result<Option<Role>>;

    /// Create a new role.
    async fn create(&self, role: Role) -> crate::Result<Role>;

    /// Update a role.
    async fn update(&self, id: Uuid, role: Role) -> crate::Result<Role>;

    /// Delete a role.
    async fn delete(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Workspace operations.
#[async_trait]
pub trait WorkspaceRepository: Send + Sync + std::fmt::Debug {
    /// Find a workspace by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Workspace>>;

    /// Find workspaces for an organization.
    async fn find_by_organization_id(&self, organization_id: Uuid)
    -> crate::Result<Vec<Workspace>>;

    /// Create a new workspace.
    async fn create(&self, workspace: Workspace) -> crate::Result<Workspace>;

    /// Update a workspace.
    async fn update(&self, id: Uuid, workspace: Workspace) -> crate::Result<Workspace>;

    /// Delete a workspace.
    async fn delete(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Channel operations.
#[async_trait]
pub trait ChannelRepository: Send + Sync + std::fmt::Debug {
    /// Find a channel by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Channel>>;

    /// Find channels for a workspace.
    async fn find_by_workspace_id(&self, workspace_id: Uuid) -> crate::Result<Vec<Channel>>;

    /// Create a new channel.
    async fn create(&self, channel: Channel) -> crate::Result<Channel>;

    /// Update a channel.
    async fn update(&self, id: Uuid, channel: Channel) -> crate::Result<Channel>;

    /// Delete a channel.
    async fn delete(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Message operations.
#[async_trait]
pub trait MessageRepository: Send + Sync + std::fmt::Debug {
    /// Find a message by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Message>>;

    /// Find messages for a channel with pagination.
    async fn find_by_channel_id(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<Message>>;

    /// Find messages for a user (for notifications/mentions).
    async fn find_by_sender_id(
        &self,
        sender_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<Message>>;

    /// Create a new message.
    async fn create(&self, message: Message) -> crate::Result<Message>;

    /// Update a message.
    async fn update(&self, id: Uuid, message: Message) -> crate::Result<Message>;

    /// Soft-delete a message.
    async fn soft_delete(&self, id: Uuid) -> crate::Result<()>;

    /// Restore a soft-deleted message.
    async fn restore(&self, id: Uuid) -> crate::Result<()>;
}

/// Repository trait for Task operations.
#[async_trait]
pub trait TaskRepository: Send + Sync + std::fmt::Debug {
    /// Find a task by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Task>>;

    /// Find tasks for a workspace with pagination and filters.
    async fn find_by_workspace_id(
        &self,
        workspace_id: Uuid,
        pagination: Pagination,
        status: Option<TaskStatus>,
        assignee_id: Option<Uuid>,
    ) -> crate::Result<Page<Task>>;

    /// Find tasks assigned to a user.
    async fn find_by_assignee_id(
        &self,
        assignee_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<Task>>;

    /// Create a new task.
    async fn create(&self, task: Task) -> crate::Result<Task>;

    /// Update a task.
    async fn update(&self, id: Uuid, task: Task) -> crate::Result<Task>;

    /// Soft-delete a task.
    async fn soft_delete(&self, id: Uuid) -> crate::Result<()>;

    /// Restore a soft-deleted task.
    async fn restore(&self, id: Uuid) -> crate::Result<()>;

    /// Purge soft-deleted tasks older than the given date.
    async fn purge_old(&self, older_than: DateTime<Utc>) -> crate::Result<usize>;
}

/// Repository trait for File operations.
#[async_trait]
pub trait FileRepository: Send + Sync + std::fmt::Debug {
    /// Find a file by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<File>>;

    /// Find files for a workspace with pagination.
    async fn find_by_workspace_id(
        &self,
        workspace_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<File>>;

    /// Find files owned by a user.
    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<File>>;

    /// Create a new file record.
    async fn create(&self, file: File) -> crate::Result<File>;

    /// Update a file.
    async fn update(&self, id: Uuid, file: File) -> crate::Result<File>;

    /// Soft-delete a file.
    async fn soft_delete(&self, id: Uuid) -> crate::Result<()>;

    /// Restore a soft-deleted file.
    async fn restore(&self, id: Uuid) -> crate::Result<()>;

    /// Purge soft-deleted files older than the given date.
    async fn purge_old(&self, older_than: DateTime<Utc>) -> crate::Result<usize>;
}

/// Repository trait for Notification operations.
#[async_trait]
pub trait NotificationRepository: Send + Sync + std::fmt::Debug {
    /// Find a notification by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Notification>>;

    /// Find unread notifications for a user with pagination.
    async fn find_unread_by_user_id(
        &self,
        user_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<Notification>>;

    /// Find notifications for a user (all) with pagination.
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        pagination: Pagination,
    ) -> crate::Result<Page<Notification>>;

    /// Create a new notification.
    async fn create(&self, notification: Notification) -> crate::Result<Notification>;

    /// Mark a notification as read.
    async fn mark_as_read(&self, id: Uuid) -> crate::Result<()>;

    /// Mark all notifications as read for a user.
    async fn mark_all_as_read(&self, user_id: Uuid) -> crate::Result<()>;

    /// Delete old notifications (retention policy).
    async fn delete_old(&self, older_than: DateTime<Utc>) -> crate::Result<usize>;
}

/// Repository trait for Session operations.
#[async_trait]
pub trait SessionRepository: Send + Sync + std::fmt::Debug {
    /// Find a session by ID.
    async fn find_by_id(&self, id: Uuid) -> crate::Result<Option<Session>>;

    /// Find a session by JTI (JWT ID).
    async fn find_by_jti(&self, jti: &str) -> crate::Result<Option<Session>>;

    /// Find active sessions for a user.
    async fn find_active_by_user_id(&self, user_id: Uuid) -> crate::Result<Vec<Session>>;

    /// Create a new session.
    async fn create(&self, session: Session) -> crate::Result<Session>;

    /// Update a session (last activity).
    async fn update_last_activity(&self, id: Uuid) -> crate::Result<()>;

    /// Revoke a session.
    async fn revoke(&self, id: Uuid) -> crate::Result<()>;

    /// Revoke a session by JTI.
    async fn revoke_by_jti(&self, jti: &str) -> crate::Result<bool>;

    /// Revoke all sessions for a user.
    async fn revoke_all_for_user(&self, user_id: Uuid) -> crate::Result<()>;

    /// Clean up expired sessions.
    async fn clean_expired(&self) -> crate::Result<usize>;
}
