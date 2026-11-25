import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ApiClient, ApiError } from '../../src/lib/api/client';
import { authState } from '../../src/lib/state/AuthState.svelte';

describe('ApiClient.downloadBlob', () => {
	beforeEach(() => {
		authState.token = 'token-de-teste';
		authState.isSessionLocked = false;
	});

	afterEach(() => {
		if (authState.gracePeriodTimeout) {
			clearTimeout(authState.gracePeriodTimeout);
			authState.gracePeriodTimeout = null;
		}
		authState.token = null;
		authState.isSessionLocked = false;
		vi.unstubAllGlobals();
	});

	it('sends Bearer auth and returns the binary body', async () => {
		const fetchMock = vi.fn(
			async () =>
				new Response(new Blob(['conteudo do arquivo'], { type: 'text/plain' }), { status: 200 })
		);
		vi.stubGlobal('fetch', fetchMock);

		const blob = await ApiClient.downloadBlob('/api/v1/files/download/abc');
		const [, init] = fetchMock.mock.calls[0] as [string, RequestInit];
		const headers = new Headers(init.headers);

		expect(headers.get('Authorization')).toBe('Bearer token-de-teste');
		expect(await blob.text()).toBe('conteudo do arquivo');
	});

	it('locks the session and throws ApiError on 401', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn(async () =>
				Response.json(
					{ error: 'invalid_credentials', message: 'Sessão inválida' },
					{ status: 401, statusText: 'Unauthorized' }
				)
			)
		);

		await expect(ApiClient.downloadBlob('/api/v1/files/download/abc')).rejects.toBeInstanceOf(
			ApiError
		);
		expect(authState.isSessionLocked).toBe(true);
	});
});
