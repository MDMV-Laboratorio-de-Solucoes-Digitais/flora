import { describe, it, expect } from 'vitest';
import { toChannel, toMessage } from '../../src/lib/api/mappers';

describe('api mappers', () => {
	it('maps channel snake_case DTO to UI model', () => {
		const channel = toChannel({
			id: '11111111-1111-1111-1111-111111111111',
			workspace_id: '22222222-2222-2222-2222-222222222222',
			name: 'general',
			type: 'private',
			created_at: '2026-11-16T00:00:00Z'
		});

		expect(channel).toEqual({
			id: '11111111-1111-1111-1111-111111111111',
			workspaceId: '22222222-2222-2222-2222-222222222222',
			name: 'general',
			description: null,
			isPrivate: true,
			createdAt: '2026-11-16T00:00:00Z'
		});
	});

	it('maps message DTO with thread_id and defaults', () => {
		const message = toMessage({
			id: '33333333-3333-3333-3333-333333333333',
			channel_id: '11111111-1111-1111-1111-111111111111',
			sender_id: '44444444-4444-4444-4444-444444444444',
			content: 'hello',
			thread_id: '55555555-5555-5555-5555-555555555555',
			created_at: '2026-11-16T00:00:00Z',
			updated_at: '2026-11-16T00:01:00Z'
		});

		expect(message.authorId).toBe('44444444-4444-4444-4444-444444444444');
		expect(message.channelId).toBe('11111111-1111-1111-1111-111111111111');
		expect(message.threadId).toBe('55555555-5555-5555-5555-555555555555');
		expect(message.fileAttachments).toEqual([]);
		expect(message.updatedAt).toBe('2026-11-16T00:01:00Z');
	});
});
