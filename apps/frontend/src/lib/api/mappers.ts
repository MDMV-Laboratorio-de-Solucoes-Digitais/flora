import type { Channel, Message, NotificationItem } from '$lib/types/models';

export interface ApiChannel {
	id: string;
	workspace_id: string;
	name: string;
	type: string;
	created_at: string;
}

export interface ApiMessage {
	id: string;
	channel_id: string;
	sender_id: string;
	sender_name?: string | null;
	sender_avatar_url?: string | null;
	content: string;
	thread_id?: string | null;
	is_edited?: boolean;
	created_at: string;
	updated_at?: string | null;
}

export interface ApiNotification {
	id: string;
	event_type: string;
	target_id: string;
	target_url?: string | null;
	is_read: boolean;
	created_at: string;
}

export function toChannel(api: ApiChannel): Channel {
	return {
		id: api.id,
		workspaceId: api.workspace_id,
		name: api.name,
		description: null,
		isPrivate: api.type === 'private',
		createdAt: api.created_at
	};
}

export function toMessage(api: ApiMessage): Message {
	return {
		id: api.id,
		channelId: api.channel_id,
		authorId: api.sender_id,
		authorName: api.sender_name?.trim() || null,
		authorAvatarUrl: api.sender_avatar_url ?? null,
		content: api.content,
		threadId: api.thread_id ?? null,
		fileAttachments: [],
		createdAt: api.created_at,
		updatedAt: api.updated_at ?? null
	};
}

export function toNotification(api: ApiNotification): NotificationItem {
	const targetUrl = api.target_url?.trim();
	return {
		id: api.id,
		eventType: api.event_type,
		targetId: api.target_id,
		targetUrl: targetUrl || null,
		isRead: api.is_read,
		createdAt: api.created_at
	};
}
