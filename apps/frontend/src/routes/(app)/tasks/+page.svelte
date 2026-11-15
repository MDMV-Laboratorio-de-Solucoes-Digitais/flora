<script lang="ts">
	import { onMount } from 'svelte';
	import { ApiClient } from '$lib/api/client';
	import { authState } from '$lib/state/AuthState.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	type Workspace = { id: string; name: string };
	type ApiTask = {
		id: string;
		workspace_id: string;
		title: string;
		description?: string | null;
		status: string;
		assignee_id?: string | null;
		created_at: string;
	};

	let workspaces = $state<Workspace[]>([]);
	let tasks = $state<ApiTask[]>([]);
	let selectedWorkspaceId = $state<string>('');
	let title = $state('');
	let description = $state('');
	let assignToMe = $state(true);
	let loading = $state(true);
	let submitting = $state(false);
	let error = $state('');
	let success = $state('');

	function currentUserId(): string | null {
		const id = authState.user?.id;
		return typeof id === 'string' ? id : null;
	}

	async function loadWorkspaces() {
		workspaces = await ApiClient.request<Workspace[]>('/api/v1/workspaces');
		if (!selectedWorkspaceId) {
			const stored = authState.workspaceId;
			const preferred = workspaces.find((w) => w.id === stored) ?? workspaces[0];
			if (preferred) {
				selectedWorkspaceId = preferred.id;
				authState.setWorkspace(preferred.id);
			}
		}
	}

	async function loadTasks() {
		tasks = await ApiClient.request<ApiTask[]>('/api/v1/tasks');
	}

	async function refresh() {
		loading = true;
		error = '';
		try {
			await loadWorkspaces();
			await loadTasks();
		} catch (e) {
			error = (e as Error).message || 'Failed to load tasks/workspaces';
		} finally {
			loading = false;
		}
	}

	async function createTask() {
		if (!title.trim() || !selectedWorkspaceId) return;
		submitting = true;
		error = '';
		success = '';
		try {
			const body: Record<string, unknown> = {
				title: title.trim(),
				workspace_id: selectedWorkspaceId
			};
			if (description.trim()) body.description = description.trim();
			const uid = currentUserId();
			if (assignToMe && uid) body.assignee_id = uid;
			const created = await ApiClient.request<ApiTask>('/api/v1/tasks', {
				method: 'POST',
				body: JSON.stringify(body)
			});
			success = `Task created: ${created.title}`;
			title = '';
			description = '';
			await loadTasks();
		} catch (e) {
			error = (e as Error).message || 'Failed to create task';
		} finally {
			submitting = false;
		}
	}

	function onWorkspaceChange(event: Event) {
		const value = (event.currentTarget as HTMLSelectElement).value;
		selectedWorkspaceId = value;
		authState.setWorkspace(value || null);
	}

	onMount(async () => {
		await refresh();
	});
</script>

<div class="flex h-full flex-col gap-4 p-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">Tasks</h1>
			<p class="text-sm text-muted-foreground">
				Minimal happy-path slice: select workspace, create task, list assignee-scoped tasks.
			</p>
		</div>
		<Button variant="outline" onclick={refresh} disabled={loading}>Refresh</Button>
	</div>

	{#if error}
		<div
			class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive"
		>
			{error}
		</div>
	{/if}
	{#if success}
		<div class="rounded-md border border-green-500/30 bg-green-500/5 p-3 text-sm text-green-700">
			{success}
		</div>
	{/if}

	<div class="grid gap-4 lg:grid-cols-2">
		<div class="space-y-4 rounded-lg border p-4">
			<h2 class="font-semibold">Workspace & create task</h2>
			<div class="space-y-2">
				<Label for="workspace">Workspace</Label>
				<select
					id="workspace"
					class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm"
					value={selectedWorkspaceId}
					onchange={onWorkspaceChange}
					disabled={loading || workspaces.length === 0}
				>
					{#if workspaces.length === 0}
						<option value="">No workspaces available</option>
					{:else}
						{#each workspaces as ws (ws.id)}
							<option value={ws.id}>{ws.name}</option>
						{/each}
					{/if}
				</select>
			</div>
			<div class="space-y-2">
				<Label for="title">Title</Label>
				<Input id="title" bind:value={title} placeholder="Task title" />
			</div>
			<div class="space-y-2">
				<Label for="desc">Description (optional)</Label>
				<Input id="desc" bind:value={description} placeholder="Description" />
			</div>
			<label class="flex items-center gap-2 text-sm">
				<input type="checkbox" bind:checked={assignToMe} />
				Assign to me (recommended so it appears in assignee-scoped list)
			</label>
			<Button disabled={submitting || !title.trim() || !selectedWorkspaceId} onclick={createTask}>
				Create task
			</Button>
		</div>

		<div class="space-y-3 rounded-lg border p-4">
			<h2 class="font-semibold">Tasks (assignee-scoped in org)</h2>
			{#if loading}
				<p class="text-sm text-muted-foreground">Loading...</p>
			{:else if tasks.length === 0}
				<p class="text-sm text-muted-foreground">No tasks returned for current assignee/org.</p>
			{:else}
				<ul class="space-y-2">
					{#each tasks as task (task.id)}
						<li class="rounded-md border p-3">
							<div class="font-medium">{task.title}</div>
							<div class="text-xs text-muted-foreground">
								{task.status} · {task.id}
							</div>
							{#if task.description}
								<div class="mt-1 text-sm">{task.description}</div>
							{/if}
						</li>
					{/each}
				</ul>
			{/if}
		</div>
	</div>
</div>
