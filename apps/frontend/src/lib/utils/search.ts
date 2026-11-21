import { t } from '$lib/i18n';

const RESULT_LABELS: Readonly<Record<string, string>> = {
	message: t['search.type.message'],
	task: t['search.type.task'],
	file: t['search.type.file']
};

/** Stable display label for an entity type; unknown kinds fall back to the raw token. */
export function resultTypeLabel(type: string): string {
	return RESULT_LABELS[type] ?? type;
}

/** Trimmed query. Empty means "do not call the API". */
export function normalizeSearchQuery(value: string): string {
	return value.trim();
}
