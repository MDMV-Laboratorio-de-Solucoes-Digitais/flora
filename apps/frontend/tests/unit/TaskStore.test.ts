import { describe, it, expect } from 'vitest';
import { TaskStore } from '../../src/lib/state/TaskStore.svelte';
import type { Task } from '../../src/lib/types/models';

describe('TaskStore', () => {
	it('should initialize empty', () => {
		const store = new TaskStore();
		expect(store.tasks.length).toBe(0);
	});

	it('should add a task', () => {
		const store = new TaskStore();
		const task: Task = {
			id: 't-1',
			title: 'Buy milk',
			status: 'TODO',
			assigneeId: 'u-1',
			workspaceId: 'w-1',
			description: 'desc',
			createdAt: '2023-01-01',
			updatedAt: null
		};
		store.addTask(task);
		expect(store.tasks.length).toBe(1);
		expect(store.tasks[0].id).toBe('t-1');
	});

	it('should not add duplicate tasks by id', () => {
		const store = new TaskStore();
		const task: Task = {
			id: 't-1',
			title: 'Buy milk',
			status: 'TODO',
			assigneeId: 'u-1',
			workspaceId: 'w-1',
			description: 'desc',
			createdAt: '2023-01-01',
			updatedAt: null
		};
		store.addTask(task);
		store.addTask(task);
		expect(store.tasks.length).toBe(1);
	});

	it('should update a task', () => {
		const store = new TaskStore();
		const task: Task = {
			id: 't-1',
			title: 'Buy milk',
			status: 'TODO',
			assigneeId: 'u-1',
			workspaceId: 'w-1',
			description: 'desc',
			createdAt: '2023-01-01',
			updatedAt: null
		};
		store.addTask(task);
		const updatedTask: Task = { ...task, status: 'DONE' };
		store.updateTask(updatedTask);
		expect(store.tasks[0].status).toBe('DONE');
	});

	it('should delete a task', () => {
		const store = new TaskStore();
		const task: Task = {
			id: 't-1',
			title: 'Buy milk',
			status: 'TODO',
			assigneeId: 'u-1',
			workspaceId: 'w-1',
			description: 'desc',
			createdAt: '2023-01-01',
			updatedAt: null
		};
		store.addTask(task);
		store.deleteTask('t-1');
		expect(store.tasks.length).toBe(0);
	});
});
