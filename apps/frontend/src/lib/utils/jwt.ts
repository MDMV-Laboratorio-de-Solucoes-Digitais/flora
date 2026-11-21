/**
 * Extracts the e-mail claim from a Flora session JWT for display purposes only.
 * Never use this value for authorization: the backend validates the signature.
 */
export function emailFromToken(token: string | null | undefined): string | null {
	if (!token) return null;
	const segments = token.split('.');
	if (segments.length !== 3 || segments.some((segment) => segment.length === 0)) return null;
	try {
		const base64 = segments[1].replaceAll('-', '+').replaceAll('_', '/');
		const padded = base64.padEnd(Math.ceil(base64.length / 4) * 4, '=');
		const binary = atob(padded);
		const bytes = Uint8Array.from(binary, (char) => char.codePointAt(0) ?? 0);
		const payload: unknown = JSON.parse(new TextDecoder().decode(bytes));
		if (typeof payload !== 'object' || payload === null || Array.isArray(payload)) return null;
		if (!('email' in payload)) return null;
		const email = payload.email;
		return typeof email === 'string' && email.trim() ? email : null;
	} catch {
		return null;
	}
}
