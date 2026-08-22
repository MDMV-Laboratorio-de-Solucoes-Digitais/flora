import { describe, it, expect } from 'vitest';
import { AuthState } from '../../src/lib/state/AuthState.svelte';

describe('AuthState', () => {
	it('should initialize as not authenticated', () => {
		const state = new AuthState();
		expect(state.isAuthenticated).toBe(false);
	});

	it('should login correctly', () => {
		const state = new AuthState();
		state.login('token123', { name: 'Luis' });
		expect(state.isAuthenticated).toBe(true);
		expect(state.token).toBe('token123');
	});

	it('should logout correctly', () => {
		const state = new AuthState();
		state.login('token123', { name: 'Luis' });
		state.logout();
		expect(state.isAuthenticated).toBe(false);
		expect(state.token).toBe(null);
	});
});
