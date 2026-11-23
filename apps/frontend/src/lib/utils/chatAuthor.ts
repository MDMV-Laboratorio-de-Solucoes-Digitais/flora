import { format } from '$lib/i18n';

/** Resolves a readable chat author label, falling back to truncated user ID if missing. */
export function formatAuthorName(authorName: string | null | undefined, authorId: string): string {
	const clean = authorName?.trim();
	if (clean) return clean;
	return format('chat.author', { id: authorId.slice(0, 8) });
}

/** Extracts up to two uppercase initials for the message avatar badge. */
export function authorInitials(authorName: string | null | undefined, authorId: string): string {
	const clean = authorName?.trim();
	if (!clean) {
		return (authorId.at(0) ?? '?').toUpperCase();
	}
	const parts = clean.split(/\s+/).filter(Boolean);
	if (parts.length >= 2) {
		const first = parts[0]?.at(0) ?? '';
		const second = parts[1]?.at(0) ?? '';
		const combined = `${first}${second}`.toUpperCase();
		if (combined) return combined;
	}
	return (clean.at(0) ?? '?').toUpperCase();
}
