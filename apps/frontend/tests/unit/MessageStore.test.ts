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
});
