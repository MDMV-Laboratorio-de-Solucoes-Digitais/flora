const RESULT_LABELS: Readonly<Record<string, string>> = {
	message: 'Message',
	task: 'Task',
	file: 'File'
};

/** Stable display label for an entity type; unknown kinds fall back to the raw token. */
export function resultTypeLabel(type: string): string {
	return RESULT_LABELS[type] ?? type;
}

/** Trimmed query. Empty means "do not call the API". */
export function normalizeSearchQuery(value: string): string {
	return value.trim();
}
