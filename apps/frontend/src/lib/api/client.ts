import { authState } from '../state/AuthState.svelte';

export const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'; // Default local Rust backend URL

export const ApiClient = {
	async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		const url = `${API_BASE_URL}${endpoint.startsWith('/') ? endpoint : `/${endpoint}`}`;
		const headers = new Headers(options.headers || {});

		if (authState.token && !headers.has('Authorization')) {
			headers.set('Authorization', `Bearer ${authState.token}`);
		}

		if (!headers.has('Content-Type') && options.body && typeof options.body === 'string') {
			headers.set('Content-Type', 'application/json');
		}

		const response = await fetch(url, { ...options, headers });

		if (response.status === 401) {
			authState.lockSession();
			throw new Error('Unauthorized');
		}

		if (!response.ok) {
			// Will throw with specific details that the caller must catch
			let errorText = 'No response body';
			try {
				errorText = await response.text();
			} catch {
				// Ignore
			}
			throw new Error(`API Error ${response.status}: ${errorText}`);
		}

		// Return empty object for 204 No Content
		if (response.status === 204) {
			return {} as T;
		}

		return response.json();
	}
};
