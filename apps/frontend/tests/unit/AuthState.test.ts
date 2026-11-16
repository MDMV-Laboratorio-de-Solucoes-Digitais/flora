import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { AuthState } from '../../src/lib/state/AuthState.svelte';

describe('AuthState', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.restoreAllMocks();
	});

	it('should initialize as not authenticated', () => {
		const state = new AuthState();
		expect(state.isAuthenticated).toBe(false);
		expect(state.token).toBeNull();
		expect(state.organizationId).toBeNull();
	});

	it('should login correctly', () => {
		const state = new AuthState();
		state.login('token123', { name: 'Luis' }, null);
		expect(state.isAuthenticated).toBe(true);
		expect(state.token).toBe('token123');
		expect(state.organizationId).toBeNull();
		expect(state.isInvalidated).toBe(false);
		expect(state.isSessionLocked).toBe(false);
	});

	it('should set organization, rotate token, clear workspace and invalidated flags', () => {
		const state = new AuthState();
		state.login('pre-token', { id: 'u1' }, null);
		state.setWorkspace('ws-old');
		state.isInvalidated = true;

		state.setOrganization('org-token', 'org-1');

		expect(state.token).toBe('org-token');
		expect(state.organizationId).toBe('org-1');
		expect(state.workspaceId).toBeNull();
		expect(state.isAuthenticated).toBe(true);
		expect(state.isSessionLocked).toBe(false);
		expect(state.isInvalidated).toBe(false);
	});

	it('should logout correctly', () => {
		const state = new AuthState();
		state.login('token123', { name: 'Luis' }, 'org-1');
		state.logout();
		expect(state.isAuthenticated).toBe(false);
		expect(state.token).toBeNull();
		expect(state.organizationId).toBeNull();
		expect(state.workspaceId).toBeNull();
	});
});
