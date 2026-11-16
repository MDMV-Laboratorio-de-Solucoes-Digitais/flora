import { describe, expect, it } from 'vitest';
import {
	DEFAULT_API_BASE_URL,
	resolveApiBaseUrl,
	resolveWsBaseUrl
} from '../../src/lib/api/client';

describe('resolveApiBaseUrl', () => {
	it('prefers the runtime value over Vite env and default', () => {
		expect(
			resolveApiBaseUrl('https://api.runtime', 'https://api.vite', 'https://api.default')
		).toBe('https://api.runtime');
	});

	it('uses the Vite value when the runtime value is absent', () => {
		expect(resolveApiBaseUrl(undefined, 'https://api.vite', 'https://api.default')).toBe(
			'https://api.vite'
		);
	});

	it('falls back to the default when both inputs are empty', () => {
		expect(resolveApiBaseUrl(null, undefined, DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
		expect(resolveApiBaseUrl(' '.repeat(3), ' '.repeat(2), DEFAULT_API_BASE_URL)).toBe(
			DEFAULT_API_BASE_URL
		);
	});
});

describe('resolveWsBaseUrl', () => {
	it('uses an explicit runtime WS override', () => {
		expect(resolveWsBaseUrl('wss://ws.runtime', 'https://api.runtime')).toBe('wss://ws.runtime');
	});

	it('derives the WS base from the resolved API origin otherwise', () => {
		expect(resolveWsBaseUrl(null, 'https://api.runtime')).toBe('https://api.runtime');
		expect(resolveWsBaseUrl(undefined, DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
		expect(resolveWsBaseUrl('', DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
	});
});
