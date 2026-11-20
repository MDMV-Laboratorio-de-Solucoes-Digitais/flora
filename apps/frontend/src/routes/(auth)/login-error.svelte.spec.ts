import { goto } from '$app/navigation';
import { authState } from '$lib/state/AuthState.svelte';
import { page } from 'vitest/browser';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';

import LoginPage from './+page.svelte';

vi.mock('$app/navigation', () => ({ goto: vi.fn() }));

function jsonResponse(body: unknown, status: number): Response {
	return Response.json(body, { status });
}

describe('(auth)/+page.svelte — login failure feedback', () => {
	beforeEach(() => {
		authState.logout();
		localStorage.clear();
		vi.clearAllMocks();
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('shows the API error when the login flow cannot start', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					jsonResponse({ error: 'oidc_error', message: 'redirect_uri is not allowed' }, 503)
				)
		);

		render(LoginPage);
		await page.getByRole('button', { name: 'Sign In' }).click();

		await expect.element(page.getByText(/unable to start sign-in/i)).toBeInTheDocument();
		await expect.element(page.getByText(/redirect_uri is not allowed/i)).toBeInTheDocument();
	});

	it('explains rate limiting instead of failing silently', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({ error: 'rate_limited' }, 429)));

		render(LoginPage);
		await page.getByRole('button', { name: 'Sign In' }).click();

		await expect.element(page.getByText(/too many attempts/i)).toBeInTheDocument();
	});

	it('bootstraps a fresh workspace and stores the returned session', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				jsonResponse(
					{
						session_token: 'jwt-teste',
						expires_in: 3600,
						user_id: 'user-teste',
						organization_id: 'org-teste',
						workspace_id: 'workspace-teste'
					},
					201
				)
			)
		);

		render(LoginPage);
		await page.getByRole('button', { name: 'Start First Seed Journey' }).click();
		await page.getByRole('button', { name: 'Sprout Workspace' }).click();

		await vi.waitFor(() => expect(authState.organizationId).toBe('org-teste'));
		expect(authState.isAuthenticated).toBe(true);
		expect(localStorage.getItem('flora.auth.v1')).toContain('jwt-teste');
		expect(goto).toHaveBeenCalledWith('/tasks');

		const [url, init] = vi.mocked(fetch).mock.calls[0];
		expect(String(url)).toContain('/api/v1/bootstrap');
		expect(init?.method).toBe('POST');
		expect(JSON.parse(String(init?.body))).toEqual({
			organization_name: 'Flora',
			admin_email: 'admin@flora.local'
		});
	});

	it('shows a conflict when the workspace was already bootstrapped', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					jsonResponse(
						{ error: 'conflict', message: 'duplicate entry: bootstrap already completed' },
						409
					)
				)
		);

		render(LoginPage);
		await page.getByRole('button', { name: 'Start First Seed Journey' }).click();
		await page.getByRole('button', { name: 'Sprout Workspace' }).click();

		await expect.element(page.getByText(/bootstrap already completed/i)).toBeInTheDocument();
		expect(authState.isAuthenticated).toBe(false);
		expect(goto).not.toHaveBeenCalled();
	});
});
