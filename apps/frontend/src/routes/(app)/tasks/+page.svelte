<script lang="ts">
	import { taskStore } from '$lib/state/TaskStore.svelte';
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Card } from '$lib/components/ui/card';
	import { onMount } from 'svelte';
	import type { Task } from '$lib/types/models';

	let newTaskTitle = $state('');
	let newTaskAssignee = $state('');
	let assigneeFilter = $state('');
	let isSubmitting = $state(false);

	let filteredTasks = $derived(
		assigneeFilter
			? taskStore.tasks.filter((t) => t.assigneeId === assigneeFilter)
			: taskStore.tasks
	);

	onMount(async () => {
		try {
			const tasks = await ApiClient.request<Task[]>('/api/v1/tasks');
			for (const task of tasks) {
				taskStore.addTask(task);
			}
		} catch (error) {
			logger.error('tasks', 'Failed to fetch tasks', undefined, error as Error);
		}
	});

	async function createTask() {
		if (!newTaskTitle.trim()) return;
		isSubmitting = true;
		try {
			const task = await ApiClient.request<Task>('/api/v1/tasks', {
				method: 'POST',
				body: JSON.stringify({
					title: newTaskTitle,
					status: 'TODO',
					assigneeId: newTaskAssignee || null
				})
			});
			taskStore.addTask(task);
			newTaskTitle = '';
			newTaskAssignee = '';
		} catch (error) {
			logger.error('tasks', 'Failed to create task', undefined, error as Error);
		} finally {
			isSubmitting = false;
		}
	}

	async function updateTaskStatus(task: Task, newStatus: Task['status']) {
		try {
			const updated = await ApiClient.request<Task>(`/api/v1/tasks/${task.id}`, {
				method: 'PATCH',
				body: JSON.stringify({ status: newStatus })
			});
			taskStore.updateTask(updated);
		} catch (error) {
			logger.error('tasks', 'Failed to update task status', undefined, error as Error);
		}
	}
</script>

<div class="space-y-6 p-6">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold">Tasks</h1>
		<div class="flex items-center gap-2">
			<span class="text-muted-foreground text-sm">Filter by Assignee:</span>
			<Input class="w-48" bind:value={assigneeFilter} placeholder="User ID" />
		</div>
	</div>

	<div class="flex gap-2">
		<Input
			class="flex-1"
			bind:value={newTaskTitle}
			placeholder="New Task Title"
			onkeydown={(event_: KeyboardEvent) => event_.key === 'Enter' && createTask()}
			disabled={isSubmitting}
		/>
		<Input
			class="w-48"
			bind:value={newTaskAssignee}
			placeholder="Assignee ID (optional)"
			onkeydown={(event_: KeyboardEvent) => event_.key === 'Enter' && createTask()}
			disabled={isSubmitting}
		/>
		<Button onclick={createTask} disabled={isSubmitting}>Add Task</Button>
	</div>

	<div class="space-y-2">
		{#each filteredTasks as task (task.id)}
			<Card class="flex items-center justify-between p-4">
				<div>
					<h3 class="font-medium">{task.title}</h3>
					<div class="mt-1 flex gap-4">
						<span class="bg-muted rounded px-2 py-1 text-xs font-semibold">{task.status}</span>
						{#if task.assigneeId}
							<span class="text-muted-foreground text-xs">Assigned to: {task.assigneeId}</span>
						{:else}
							<span class="text-muted-foreground text-xs">Unassigned</span>
						{/if}
					</div>
				</div>
				<div class="flex gap-2">
					<select
						class="rounded border px-2 py-1 text-sm"
						value={task.status}
						onchange={(event_) =>
							updateTaskStatus(task, (event_.target as HTMLSelectElement).value as Task['status'])}
					>
						<option value="TODO">TODO</option>
						<option value="IN_PROGRESS">IN_PROGRESS</option>
						<option value="DONE">DONE</option>
					</select>
				</div>
			</Card>
		{/each}
		{#if filteredTasks.length === 0}
			<p class="text-muted-foreground text-sm">No tasks found.</p>
		{/if}
	</div>
</div>
