<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { authState } from '$lib/state/AuthState.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import KanbanBoard from '$lib/components/tasks/KanbanBoard.svelte';
	import { statusPatchBody, statusPatchEndpoint } from '$lib/components/tasks/status-patch.svelte';
	import { apiErrorMessage, format, t } from '$lib/i18n';

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
	let shouldAssignToMe = $state(true);
	let isLoading = $state(true);
	let isSubmitting = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');

	function currentUserId(): string | null {
		const id = authState.user?.id;
		return typeof id === 'string' ? id : null;
	}

	async function loadWorkspaces() {
		workspaces = await ApiClient.request<Workspace[]>('/api/v1/workspaces');
		if (!selectedWorkspaceId) {
			const stored = authState.workspaceId;
			const preferred = workspaces.find((workspace) => workspace.id === stored) ?? workspaces[0];
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
		isLoading = true;
		errorMessage = '';
		try {
			await loadWorkspaces();
			await loadTasks();
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.loadTasks');
		} finally {
			isLoading = false;
		}
	}

	async function createTask() {
		if (!selectedWorkspaceId || !title.trim()) return;
		isSubmitting = true;
		errorMessage = '';
		successMessage = '';
		try {
			const body: Record<string, unknown> = {
				title: title.trim(),
				workspace_id: selectedWorkspaceId
			};
			if (description.trim()) body.description = description.trim();
			const uid = currentUserId();
			if (shouldAssignToMe && uid) body.assignee_id = uid;
			const created = await ApiClient.request<ApiTask>('/api/v1/tasks', {
				method: 'POST',
				body: JSON.stringify(body)
			});
			successMessage = format('tasks.created', { title: created.title });
			title = '';
			description = '';
			await loadTasks();
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.createTask');
		} finally {
			isSubmitting = false;
		}
	}

	async function changeStatus(id: string, status: string) {
		const patch = statusPatchBody(status);
		if (!patch) {
			errorMessage = format('tasks.unsupportedStatus', { status });
			return;
		}
		errorMessage = '';
		try {
			await ApiClient.request<ApiTask>(statusPatchEndpoint(id), {
				method: 'PATCH',
				body: patch.body
			});
			await loadTasks();
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.changeTaskStatus');
		}
	}

	function onWorkspaceChange(event: Event) {
		const value = (event.currentTarget as HTMLSelectElement).value;
		selectedWorkspaceId = value;
		authState.setWorkspace(value || null);
	}

	onMount(async () => {
		if (!authState.isAuthenticated || !authState.token) {
			await goto('/');
			return;
		}
		if (!authState.organizationId) {
			await goto('/select-org');
			return;
		}
		await refresh();
	});
</script>

<div class="flex h-full flex-col gap-4 p-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">{t['tasks.title']}</h1>
			<p class="text-sm text-muted-foreground">
				{t['tasks.description']}
			</p>
		</div>
		<Button variant="outline" onclick={refresh} disabled={isLoading}>{t['tasks.refresh']}</Button>
	</div>

	{#if errorMessage}
		<div
			class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive"
		>
			{errorMessage}
		</div>
	{/if}
	{#if successMessage}
		<div class="rounded-md border border-green-500/30 bg-green-500/5 p-3 text-sm text-green-700">
			{successMessage}
		</div>
	{/if}

	<div class="grid gap-4 lg:grid-cols-2">
		<div class="space-y-4 rounded-lg border p-4">
			<h2 class="font-semibold">{t['tasks.createPanelTitle']}</h2>
			<div class="space-y-2">
				<Label for="workspace">{t['tasks.workspace']}</Label>
				<select
					id="workspace"
					class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm"
					value={selectedWorkspaceId}
					onchange={onWorkspaceChange}
					disabled={isLoading || workspaces.length === 0}
				>
					{#if workspaces.length === 0}
						<option value="">{t['tasks.noWorkspaces']}</option>
					{:else}
						{#each workspaces as workspace (workspace.id)}
							<option value={workspace.id}>{workspace.name}</option>
						{/each}
					{/if}
				</select>
			</div>
			<div class="space-y-2">
				<Label for="title">{t['tasks.titleLabel']}</Label>
				<Input id="title" bind:value={title} placeholder={t['tasks.titlePlaceholder']} />
			</div>
			<div class="space-y-2">
				<Label for="desc">{t['tasks.descriptionLabel']}</Label>
				<Input id="desc" bind:value={description} placeholder={t['tasks.descriptionPlaceholder']} />
			</div>
			<label class="flex items-center gap-2 text-sm">
				<input type="checkbox" bind:checked={shouldAssignToMe} />
				{t['tasks.assignToMe']}
			</label>
			<Button disabled={isSubmitting || !title.trim() || !selectedWorkspaceId} onclick={createTask}>
				{t['tasks.create']}
			</Button>
		</div>

		<div class="space-y-3 rounded-lg border p-4">
			<h2 class="font-semibold">{t['tasks.listTitle']}</h2>
			{#if isLoading}
				<p class="text-sm text-muted-foreground">{t['tasks.loading']}</p>
			{:else if tasks.length === 0}
				<p class="text-sm text-muted-foreground">{t['tasks.empty']}</p>
			{:else}
				<KanbanBoard {tasks} onStatusChange={changeStatus} />
			{/if}
		</div>
	</div>
</div>
