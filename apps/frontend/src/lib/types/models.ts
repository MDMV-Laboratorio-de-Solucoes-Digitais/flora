export type UUID = string;
export type ISO8601Date = string;

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
	authorName?: string | null;
	authorAvatarUrl?: string | null;
	content: string;
	threadId: UUID | null;
	fileAttachments: FileAttachment[];
	createdAt: ISO8601Date;
	updatedAt: ISO8601Date | null;
}

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

/** Entity kind emitted by GET /api/v1/search. */
export type SearchResultType = string;

export interface SearchResult {
	type: SearchResultType;
	id: UUID;
	snippet: string;
}

export interface SearchResponse {
	query: string;
	results: SearchResult[];
	/** Authoritative backend; currently always "postgres". */
	source: string;
}

export interface FileAttachment {
	id: UUID;
	fileName: string;
	mimeType: string;
	byteSize: number;
	rustFsUrl: string;
	uploadedAt: ISO8601Date;
}

export interface NotificationItem {
	id: UUID;
	eventType: string;
	targetId: UUID;
	targetUrl: string | null;
	isRead: boolean;
	createdAt: ISO8601Date;
}

export interface RoleItem {
	id: UUID;
	name: string;
	description: string | null;
	permissions: string[];
	isBuiltin: boolean;
}

export interface OrganizationMemberItem {
	userId: UUID;
	email: string;
	displayName: string;
	roleId: UUID;
	roleName: string;
	joinedAt: ISO8601Date;
}
