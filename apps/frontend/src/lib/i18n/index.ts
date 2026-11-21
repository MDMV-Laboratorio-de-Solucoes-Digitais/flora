import { t, type TranslationKey } from './pt-BR';

export { t, type TranslationKey } from './pt-BR';

/** Preenche os marcadores `{nome}` de uma mensagem do dicionário. */
export function format(key: TranslationKey, values: Record<string, string | number> = {}): string {
	return t[key].replaceAll(/\{(\w+)\}/g, (match, name: string) =>
		Object.hasOwn(values, name) ? String(values[name]) : match
	);
}

/** Corpo de erro devolvido pela API (campos usados na tradução). */
export type BackendErrorBody = { error?: unknown; field?: unknown };

const CODE_TO_KEY: Readonly<Record<string, TranslationKey>> = {
	invalid_credentials: 'error.invalidCredentials',
	rate_limited: 'error.rateLimited',
	oidc_error: 'error.oidcUnavailable',
	conflict: 'error.conflict',
	validation: 'error.validation'
};

const STATUS_TO_KEY: Readonly<Record<number, TranslationKey>> = {
	401: 'error.invalidCredentials',
	409: 'error.conflict',
	422: 'error.validation',
	429: 'error.rateLimited'
};

/**
 * Converte o código de erro do backend (ou, na ausência dele, o status HTTP)
 * em uma mensagem pt-BR. `fallback` é a mensagem específica da operação.
 */
export function backendErrorMessage(
	body: BackendErrorBody,
	status: number,
	fallback: TranslationKey
): string {
	const code = typeof body.error === 'string' ? body.error : '';
	if (code === 'validation' && body.field === 'admin_password') {
		return t['error.adminPasswordValidation'];
	}
	const key = CODE_TO_KEY[code] ?? STATUS_TO_KEY[status] ?? fallback;
	return t[key];
}

/**
 * Traduz um erro lançado pelo ApiClient usando o código/status do backend.
 * A mensagem bruta do servidor (frequentemente em inglês) nunca é exibida.
 */
export function apiErrorMessage(error: unknown, fallback: TranslationKey): string {
	if (typeof error !== 'object' || error === null) return t[fallback];
	const apiError = error as { code?: unknown; field?: unknown; status?: unknown };
	return backendErrorMessage(
		{
			error: typeof apiError.code === 'string' ? apiError.code : undefined,
			field: typeof apiError.field === 'string' ? apiError.field : undefined
		},
		typeof apiError.status === 'number' ? apiError.status : 0,
		fallback
	);
}
