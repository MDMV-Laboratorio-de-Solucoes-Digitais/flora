import { describe, it, expect } from 'vitest';
import { toChannel, toMessage, toNotification } from '../../src/lib/api/mappers';

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
		expect(message.authorName).toBeNull();
		expect(message.authorAvatarUrl).toBeNull();
		expect(message.channelId).toBe('11111111-1111-1111-1111-111111111111');
		expect(message.threadId).toBe('55555555-5555-5555-5555-555555555555');
		expect(message.fileAttachments).toEqual([]);
		expect(message.updatedAt).toBe('2026-11-16T00:01:00Z');
	});

	it('maps trimmed sender_name and sender_avatar_url and normalizes blank values to null', () => {
		const withAuthor = toMessage({
			id: '33333333-3333-3333-3333-333333333333',
			channel_id: '11111111-1111-1111-1111-111111111111',
			sender_id: '44444444-4444-4444-4444-444444444444',
			sender_name: '  Alice Admin  ',
			sender_avatar_url: 'https://example.test/alice.png',
			content: 'hello',
			created_at: '2026-11-16T00:00:00Z'
		});
		expect(withAuthor.authorName).toBe('Alice Admin');
		expect(withAuthor.authorAvatarUrl).toBe('https://example.test/alice.png');

		const blankAuthor = toMessage({
			id: '33333333-3333-3333-3333-333333333334',
			channel_id: '11111111-1111-1111-1111-111111111111',
			sender_id: '44444444-4444-4444-4444-444444444444',
			sender_name: ' '.repeat(3),
			sender_avatar_url: null,
			content: 'hello',
			created_at: '2026-11-16T00:00:00Z'
		});
		expect(blankAuthor.authorName).toBeNull();
		expect(blankAuthor.authorAvatarUrl).toBeNull();
	});

	it('maps notification snake_case DTO and normalizes optional target_url', () => {
		const withUrl = toNotification({
			id: '66666666-7777-8888-9999-000000000000',
			event_type: 'Assignment',
			target_id: '33333333-4444-5555-6666-777777777777',
			target_url: '  /tasks  ',
			is_read: false,
			created_at: '2026-11-23T06:00:00Z'
		});
		expect(withUrl).toEqual({
			id: '66666666-7777-8888-9999-000000000000',
			eventType: 'Assignment',
			targetId: '33333333-4444-5555-6666-777777777777',
			targetUrl: '/tasks',
			isRead: false,
			createdAt: '2026-11-23T06:00:00Z'
		});

		const blankUrl = toNotification({
			id: '77777777-8888-9999-0000-111111111111',
			event_type: 'Reply',
			target_id: 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee',
			target_url: ' '.repeat(3),
			is_read: true,
			created_at: '2026-11-23T06:05:00Z'
		});
		expect(blankUrl.targetUrl).toBeNull();
		expect(blankUrl.isRead).toBe(true);
	});
});
