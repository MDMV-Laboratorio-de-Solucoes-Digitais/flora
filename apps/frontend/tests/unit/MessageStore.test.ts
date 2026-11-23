import { describe, it, expect } from 'vitest';
import { MessageStore } from '../../src/lib/state/MessageStore.svelte';
import type { Message } from '../../src/lib/types/models';

describe('MessageStore', () => {
	it('should deduplicate messages by ID', () => {
		const store = new MessageStore();
		const message: Message = {
			id: 'msg-1',
			content: 'Hello',
			channelId: 'c1',
			authorId: 'u1',
			threadId: null,
			fileAttachments: [],
			createdAt: '2023-01-01',
			updatedAt: null
		};
		store.addMessage(message);
		store.addMessage(message);
		expect(store.messages.length).toBe(1);
	});

	it('preserves existing author metadata when partial upsert omits authorName or authorAvatarUrl', () => {
		const store = new MessageStore();
		store.upsertMessage({
			id: 'msg-2',
			content: 'Initial',
			channelId: 'c1',
			authorId: 'u1',
			authorName: 'Alice Admin',
			authorAvatarUrl: 'https://example.test/alice.png',
			threadId: null,
			fileAttachments: [],
			createdAt: '2026-11-23T00:00:00Z',
			updatedAt: null
		});

		store.upsertMessage({
			id: 'msg-2',
			content: 'Edited',
			channelId: 'c1',
			authorId: 'u1',
			authorName: null,
			authorAvatarUrl: null,
			threadId: null,
			fileAttachments: [],
			createdAt: '2026-11-23T00:00:00Z',
			updatedAt: '2026-11-23T00:01:00Z'
		});

		expect(store.messages).toHaveLength(1);
		expect(store.messages[0].content).toBe('Edited');
		expect(store.messages[0].authorName).toBe('Alice Admin');
		expect(store.messages[0].authorAvatarUrl).toBe('https://example.test/alice.png');
	});
});
