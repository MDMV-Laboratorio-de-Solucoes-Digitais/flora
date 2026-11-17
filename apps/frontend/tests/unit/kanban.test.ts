import { describe, it, expect } from 'vitest';
import {
	groupByStatus,
	isValidMove,
	BOARD_STATUSES
} from '../../src/lib/components/tasks/kanban.svelte';
import type { BoardTask } from '../../src/lib/components/tasks/types';

function task(id: string, status: string): BoardTask {
	return { id, title: `Task ${id}`, status };
}

describe('kanban grouping', () => {
	it('exposes the three canonical backend statuses in order', () => {
		expect([...BOARD_STATUSES]).toEqual(['Todo', 'InProgress', 'Done']);
	});

	it('groups tasks into their canonical column', () => {
		const tasks = [task('a', 'Todo'), task('b', 'InProgress'), task('c', 'Todo')];
		const cols = groupByStatus(tasks);
		expect(cols.map((col) => [col.status, col.items.map((t) => t.id)])).toEqual([
			['Todo', ['a', 'c']],
			['InProgress', ['b']],
			['Done', []]
		]);
	});

	it('omits tasks with unknown status (e.g. Archived)', () => {
		const cols = groupByStatus([task('a', 'Todo'), task('z', 'Archived')]);
		expect(cols.flatMap((col) => col.items.map((t) => t.id))).toEqual(['a']);
	});

	it('returns empty columns for no tasks', () => {
		expect(groupByStatus([]).every((col) => col.items.length === 0)).toBe(true);
	});
});

describe('isValidMove', () => {
	it('accepts moves between two distinct known columns', () => {
		expect(isValidMove('Todo', 'InProgress')).toBe(true);
		expect(isValidMove('Done', 'Todo')).toBe(true);
	});

	it('rejects same-column, null, and unknown statuses', () => {
		expect(isValidMove('Todo', 'Todo')).toBe(false);
		expect(isValidMove(null, 'Todo')).toBe(false);
		expect(isValidMove('Todo', null)).toBe(false);
		expect(isValidMove('Todo', 'Archived')).toBe(false);
		expect(isValidMove('Archived', 'Done')).toBe(false);
	});
});
