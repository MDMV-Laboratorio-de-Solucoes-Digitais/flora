import { afterEach, describe, expect, it, vi } from 'vitest';
import { API_BASE_URL, ApiClient } from '../../src/lib/api/client';
import { authState } from '../../src/lib/state/AuthState.svelte';
import { normalizeSearchQuery, resultTypeLabel } from '../../src/lib/utils/search';

describe('ApiClient.search', () => {
	afterEach(() => {
		vi.unstubAllGlobals();
		authState.logout();
	});

	it('builds the encoded query and returns the typed SearchResponse', async () => {
		const body = {
			query: 'dashboard',
			results: [
				{
					type: 'task',
					id: '44444444-5555-6666-7777-888888888888',
					snippet:
						'Design new dashboard UI Create Figma mockups for the analytics dashboard redesign'
				}
			],
			source: 'postgres'
		};
		const fetchMock = vi.fn().mockResolvedValue({
			ok: true,
			status: 200,
			statusText: 'OK',
			text: () => Promise.resolve(JSON.stringify(body))
		});
		vi.stubGlobal('fetch', fetchMock);
		authState.login('test-token', {});

		const response = await ApiClient.search({ q: 'dashboard words', type: 'tasks', limit: 5 });

		expect(fetchMock).toHaveBeenCalledTimes(1);
		const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit];
		expect(url).toBe(`${API_BASE_URL}/api/v1/search?q=dashboard+words&type=tasks&limit=5`);
		expect((init.headers as Headers).get('Authorization')).toBe('Bearer test-token');
		expect(response).toEqual(body);
	});
});

describe('search helpers', () => {
	it('normalizes whitespace-only queries to empty', () => {
		expect(normalizeSearchQuery(' '.repeat(3))).toBe('');
		expect(normalizeSearchQuery('  dashboard  ')).toBe('dashboard');
	});

	it('maps known entity types and falls back for unknown ones', () => {
		expect(resultTypeLabel('message')).toBe('Message');
		expect(resultTypeLabel('task')).toBe('Task');
		expect(resultTypeLabel('file')).toBe('File');
		expect(resultTypeLabel('poll')).toBe('poll');
	});
});
