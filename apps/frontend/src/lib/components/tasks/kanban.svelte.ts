import type { BoardTask } from './types';
import { t } from '$lib/i18n';

/** Canonical backend status values (flora-core/src/models/task.rs). */
export const BOARD_STATUSES = ['Todo', 'InProgress', 'Done'] as const;
export type BoardStatus = (typeof BOARD_STATUSES)[number];

/**
 * Column shape derived from the canonical statuses. Kept in a pure module so
 * the grouping logic is testable in Vitest's `server` project (the CI gate),
 * independent of Svelte rendering.
 */
export const BOARD_COLUMNS: ReadonlyArray<{ id: string; title: string; status: BoardStatus }> = [
	{ id: 'todo', title: t['kanban.todo'], status: 'Todo' },
	{ id: 'in-progress', title: t['kanban.inProgress'], status: 'InProgress' },
	{ id: 'done', title: t['kanban.done'], status: 'Done' }
];

/** Group tasks by column. Tasks with unknown status (e.g. Archived) are omitted. */
export function groupByStatus(
	tasks: readonly BoardTask[]
): Array<{ id: string; title: string; status: BoardStatus; items: BoardTask[] }> {
	return BOARD_COLUMNS.map((col) => ({
		...col,
		items: tasks.filter((task) => task.status === col.status)
	}));
}

/** A drop is valid only between two known, different columns. */
export function isValidMove(from: string | null, to: string | null): to is BoardStatus {
	if (!from || !to || from === to) return false;
	return (
		(BOARD_STATUSES as readonly string[]).includes(from) &&
		(BOARD_STATUSES as readonly string[]).includes(to)
	);
}
