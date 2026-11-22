import { describe, it, expect } from 'vitest';
import { emailFromToken } from '../../src/lib/utils/jwt';

function tokenWith(payload: unknown): string {
	return `header.${Buffer.from(JSON.stringify(payload)).toString('base64url')}.signature`;
}

describe('emailFromToken', () => {
	it('returns the e-mail claim of a valid session token', () => {
		expect(emailFromToken(tokenWith({ email: 'qa-admin@flora.test', sub: 'user-1' }))).toBe(
			'qa-admin@flora.test'
		);
	});

	it('handles UTF-8 e-mails and base64url without padding', () => {
		expect(emailFromToken(tokenWith({ email: 'joão@exemplo.test' }))).toBe('joão@exemplo.test');
	});

	it('returns null for absent or malformed tokens', () => {
		expect(emailFromToken(null)).toBeNull();
		expect(emailFromToken(undefined)).toBeNull();
		expect(emailFromToken('')).toBeNull();
		expect(emailFromToken('only.two')).toBeNull();
		expect(emailFromToken('a.b.c.d')).toBeNull();
		expect(emailFromToken('a.!!!.c')).toBeNull();
		expect(emailFromToken('a.bm90LWpzb24.c')).toBeNull();
	});

	it('returns null when the payload has no usable e-mail', () => {
		expect(emailFromToken(tokenWith({ sub: 'user-1' }))).toBeNull();
		expect(emailFromToken(tokenWith({ email: '' }))).toBeNull();
		const whitespace = ' '.repeat(3);
		expect(emailFromToken(tokenWith({ email: whitespace }))).toBeNull();
		expect(emailFromToken(tokenWith({ email: 42 }))).toBeNull();
		expect(emailFromToken(tokenWith(['email']))).toBeNull();
		expect(emailFromToken(tokenWith(null))).toBeNull();
	});
});
