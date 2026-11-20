# Data Model: Flora Seed v0.1 Core Features

**Version**: 1.0.0
**Date**: 2026-08-17
**Context**: Based on `spec.md` and `research.md`

## Overview

The data model for Flora Seed v0.1 is designed for high-performance, multi-tenant isolation, and scalability within a modular monolith architecture. Every entity is strictly tied to an `organization_id` to ensure logical isolation at the application layer.

## Core Entities

### 1. Identity & Membership

#### User
Represents an individual account holder.
- `id`: `UUID` (Primary Key)
- `email`: `String` (Unique)
- `password_hash`: `String` (Hashed)
- `profile`: `JSONB` (Display name, avatar URL, etc.)
- `created_at`: `Timestamp`
- `updated_at`: `Timestamp`

#### Organization
The top-level tenant.
- `id`: `UUID` (Primary Key)
- `name`: `String`
- `slug`: `String` (Unique, for URL/identifier)
- `settings`: `JSONB` (Configurable retention, file limits, etc.)
- `created_at`: `Timestamp`

#### Membership (User-Org Mapping)
Handles the many-to-many relationship between Users and Organizations with per-org roles.
- `user_id`: `UUID` (FK -> User)
- `organization_id`: `UUID` (FK -> Organization)
- `role_id`: `UUID` (FK -> Role)
- `joined_at`: `Timestamp`
*Constraint*: Unique(`user_id`, `organization_id`)

#### Role
Customizable RBAC within an organization.
- `id`: `UUID` (Primary Key)
- `organization_id`: `UUID` (FK -> Organization)
- `name`: `String` (e.g., "Admin", "Member")
- `permissions`: `JSONB` (Set of granular permission strings)

#### Workspace
A collaborative area within an organization.
- `id`: `UUID` (Primary Key)
- `organization_id`: `UUID` (FK -> Organization)
- `name`: `String`
- `description`: `Text`
- `created_at`: `Timestamp`

---

### 2. Collaboration

#### Channel
A communication space within a workspace.
- `id`: `UUID` (Primary Key)
- `workspace_id`: `UUID` (FK -> Workspace)
- `organization_id`: `UUID` (FK -> Organization - *Denormalized for isolation*)
- `name`: `String`
- `type`: `Enum` (Public, Private)
- `created_at`: `Timestamp`

#### Message
A unit of communication.
- `id`: `UUID` (Primary Key)
- `channel_id`: `UUID` (FK -> Channel)
- `organization_id`: `UUID` (FK -> Organization - *Denormalized for isolation*)
- `sender_id`: `UUID` (FK -> User)
- `content`: `Text`
- `thread_id`: `UUID` (Self-referential FK for replies)
- `is_edited`: `Boolean`
- `is_deleted`: `Boolean` (Soft-delete flag)
- `created_at`: `Timestamp`
- `updated_at`: `Timestamp`

#### Task
An actionable item.
- `id`: `UUID` (Primary Key)
- `workspace_id`: `UUID` (FK -> Workspace)
- `organization_id`: `UUID` (FK -> Organization - *Denormalized for isolation*)
- `creator_id`: `UUID` (FK -> User)
- `assignee_id`: `UUID` (FK -> User, Nullable)
- `title`: `String`
- `description`: `Text`
- `status`: `Enum` (Todo, InProgress, Done, etc.)
- `is_deleted`: `Boolean` (Soft-delete flag)
- `created_at`: `Timestamp`
- `updated_at`: `Timestamp`

#### File
A digital asset.
- `id`: `UUID` (Primary Key)
- `organization_id`: `UUID` (FK -> Organization - *Denormalized for isolation*)
- `owner_id`: `UUID` (FK -> User)
- `file_type`: `String` (MIME type)
- `name`: `String`
- `size_bytes`: `BigInt`
- `storage_path`: `String` (Path in RustFS)
- `is_deleted`: `Boolean` (Soft-delete flag)
- `created_at`: `Timestamp`

---

### 3. Platform Features

#### Notification
An alert for a relevant event.
- `id`: `UUID` (Primary Key)
- `organization_id`: `UUID` (FK -> Organization)
- `user_id`: `UUID` (FK -> User)
- `event_type`: `Enum` (Mention, Assignment, Reply, etc.)
- `target_id`: `UUID` (ID of the related entity: Message, Task, etc.)
- `is_read`: `Boolean`
- `created_at`: `Timestamp`

## Relationships & Constraints

1. **Multi-Tenancy Isolation**: Every single data retrieval query MUST include `WHERE organization_id = ?`. This is enforced at the application layer.
2. **Soft Deletion**: `is_deleted` flags are used for Messages, Tasks, and Files. Retention policies (defined in `Organization.settings`) determine when these are hard-deleted.
3. **Cascade/Cleanup**: When an Organization is deleted, all associated entities (Workspaces, Users, etc.) are purged.
4. **Denormalization**: `organization_id` is denormalized into almost every table to simplify isolation checks and improve query performance by avoiding deep joins for tenant verification.

## State Transitions

### Task Lifecycle
`Todo` -> `InProgress` -> `Done`

### Soft-Delete Lifecycle
`Active` -> `Soft-Deleted` -> (Retention Period) -> `Purged`
