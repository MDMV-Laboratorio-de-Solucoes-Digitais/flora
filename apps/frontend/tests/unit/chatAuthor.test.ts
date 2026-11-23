import { describe, expect, it } from 'vitest';
import { authorInitials, formatAuthorName } from '../../src/lib/utils/chatAuthor';

describe('formatAuthorName', () => {
	it('returns trimmed real author name when present', () => {
		expect(formatAuthorName('  Alice Admin  ', '22222222-2222-2222-2222-222222222222')).toBe(
			'Alice Admin'
		);
	});

	it('falls back to localized short author ID label when name is null, undefined or blank', () => {
		expect(formatAuthorName(null, '22222222-2222-2222-2222-222222222222')).toBe('Usuário 22222222');
		expect(formatAuthorName(undefined, 'abcdef12-0000-0000-0000-000000000000')).toBe(
			'Usuário abcdef12'
		);
		expect(formatAuthorName(' '.repeat(3), '99999999-0000-0000-0000-000000000000')).toBe(
			'Usuário 99999999'
		);
	});
});

describe('authorInitials', () => {
	it('extracts two uppercase initials for multi-word display names and one for single names or emails', () => {
		expect(authorInitials('Alice Admin', '22222222-0000')).toBe('AA');
		expect(authorInitials('  Bob   Builder  Jr ', '33333333-0000')).toBe('BB');
		expect(authorInitials('Carol', '44444444-0000')).toBe('C');
		expect(authorInitials('alice@acme.test', '22222222-0000')).toBe('A');
	});

	it('falls back to uppercased first character of authorId or ? when empty', () => {
		expect(authorInitials(null, 'ab1234')).toBe('A');
		expect(authorInitials(' '.repeat(3), '')).toBe('?');
	});
});
