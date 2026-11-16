import { describe, it, expect, vi, afterEach } from 'vitest';
import { ApiClient, ApiError } from '../../src/lib/api/client';
import { authState } from '../../src/lib/state/AuthState.svelte';

describe('API Interceptor', () => {
	afterEach(() => {
		vi.unstubAllGlobals();
		vi.restoreAllMocks();
		vi.useRealTimers();
		authState.logout();
	});

	it('should lock session and throw typed ApiError on 401 response', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: false,
				status: 401,
				statusText: 'Unauthorized',
				text: () => Promise.resolve('Unauthorized')
			})
		);

		await expect(ApiClient.request('/test')).rejects.toMatchObject({
			name: 'ApiError',
			status: 401,
			message: 'Unauthorized'
		});
		expect(authState.isSessionLocked).toBe(true);
	});

	it('should preserve status and parsed JSON body on non-401 errors', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: false,
				status: 422,
				statusText: 'Unprocessable Entity',
				text: () =>
					Promise.resolve(
						JSON.stringify({
							error: 'validation_error',
							field: 'organization_id',
							message: 'organization_id must not be nil'
						})
					)
			})
		);

		try {
			await ApiClient.request('/test');
			throw new Error('expected ApiError');
		} catch (error_) {
			expect(error_).toBeInstanceOf(ApiError);
			const apiError = error_ as ApiError;
			expect(apiError.status).toBe(422);
			expect(apiError.code).toBe('validation_error');
			expect(apiError.field).toBe('organization_id');
			expect(apiError.message).toBe('organization_id must not be nil');
		}
	});
});
