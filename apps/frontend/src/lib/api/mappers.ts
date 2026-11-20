import type { Channel, Message } from '$lib/types/models';

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
	content: string;
	thread_id?: string | null;
	is_edited?: boolean;
	created_at: string;
	updated_at?: string | null;
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
		content: api.content,
		threadId: api.thread_id ?? null,
		fileAttachments: [],
		createdAt: api.created_at,
		updatedAt: api.updated_at ?? null
	};
}
