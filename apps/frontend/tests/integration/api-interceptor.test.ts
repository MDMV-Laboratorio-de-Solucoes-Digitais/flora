import { describe, it, expect, vi } from 'vitest';
import { ApiClient } from '../../src/lib/api/client';

describe('API Interceptor', () => {
	it('should throw Unauthorized on 401 response', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: false,
				status: 401,
				text: () => Promise.resolve('Unauthorized')
			})
		);

		await expect(ApiClient.request('/test')).rejects.toThrow('Unauthorized');
	});
});
