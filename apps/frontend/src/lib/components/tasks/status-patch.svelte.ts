import { BOARD_STATUSES, type BoardStatus } from './kanban.svelte';

/**
 * Build the exact JSON body sent by PATCH /api/v1/tasks/{id} when a card moves
 * between columns. Kept pure so the serialization — the feature's central
 * behavior — is verifiable in Vitest's `server` project (the required CI gate),
 * not only by reading the component.
 */
export function statusPatchBody(status: string): { body: string } | null {
	if (!(BOARD_STATUSES as readonly string[]).includes(status)) return null;
	return { body: JSON.stringify({ status: status as BoardStatus }) };
}

/** The endpoint path is derived from the task id, never interpolated from UI text. */
export function statusPatchEndpoint(id: string): string {
	return `/api/v1/tasks/${encodeURIComponent(id)}`;
}
