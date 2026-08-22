# Data Model: Flora Seed v0.1 (Frontend Svelte 5 Rewrite)

This document defines the strict TypeScript interfaces that the Svelte 5 frontend will use to interact with the Trait-Driven Rust backend.

## Types

```typescript
export type UUID = string;
export type ISO8601Date = string;
```

## Entities

### User & Authentication
```typescript
export interface User {
    id: UUID;
    email: string;
    displayName: string;
    avatarUrl: string | null;
    createdAt: ISO8601Date;
}

export interface WorkspaceRole {
    workspaceId: UUID;
    userId: UUID;
    role: 'ADMIN' | 'MEMBER' | 'GUEST';
}
```

### Collaboration & Messaging
```typescript
export interface Channel {
    id: UUID;
    workspaceId: UUID;
    name: string;
    description: string | null;
    isPrivate: boolean;
    createdAt: ISO8601Date;
}

export interface Message {
    id: UUID;
    channelId: UUID;
    authorId: UUID;
    content: string; // Markdown supported
    threadId: UUID | null; // Null if it's a top-level message
    fileAttachments: FileAttachment[];
    createdAt: ISO8601Date;
    updatedAt: ISO8601Date | null;
}
```

### Productivity (Tasks & Files)
```typescript
export interface Task {
    id: UUID;
    workspaceId: UUID;
    title: string;
    description: string;
    status: 'TODO' | 'IN_PROGRESS' | 'DONE';
    assigneeId: UUID | null;
    createdAt: ISO8601Date;
    updatedAt: ISO8601Date | null;
}

export interface FileAttachment {
    id: UUID;
    fileName: string;
    mimeType: string;
    byteSize: number;
    rustFsUrl: string; // Signed URL or proxy path
    uploadedAt: ISO8601Date;
}
```

## State Transitions (Frontend)

- **Authentication State**: `Unauthenticated` -> (Zitadel OIDC Flow) -> `Authenticated` -> (401 Error / Zitadel Outage) -> `Pending Re-authentication (Read-Only)` -> (Grace period expires) -> `Unauthenticated`
- **Task Status**: `TODO` <-> `IN_PROGRESS` <-> `DONE`
- **WebSocket State**: `Disconnected` -> `Connecting` -> `Connected` -> (Network Drop) -> `Reconnecting` -> (REST Sync) -> `Connected`
