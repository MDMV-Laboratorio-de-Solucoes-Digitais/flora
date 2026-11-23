<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { authState } from '$lib/state/AuthState.svelte';
	import { fileStore } from '$lib/state/FileStore.svelte';
	import FileUpload from '$lib/components/files/FileUpload.svelte';
	import FilePreview from '$lib/components/files/FilePreview.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { apiErrorMessage, format, t } from '$lib/i18n';
	import { filesQueryPath } from '$lib/utils/files';
	import type { FileAttachment } from '$lib/types/models';

	type Workspace = { id: string; name: string };

	let workspaces = $state<Workspace[]>([]);
	let selectedWorkspaceId = $state('');
	let isLoading = $state(true);
	let isLoadingFiles = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');
	let loadSeq = 0;

	async function loadWorkspaces() {
		workspaces = await ApiClient.request<Workspace[]>('/api/v1/workspaces');
		if (!selectedWorkspaceId) {
			const preferred =
				workspaces.find((workspace) => workspace.id === authState.workspaceId) ?? workspaces[0];
			if (preferred) {
				selectedWorkspaceId = preferred.id;
				authState.setWorkspace(preferred.id);
			}
		}
	}

	async function loadFiles(workspaceId: string) {
		if (!workspaceId) {
			loadSeq += 1;
			isLoadingFiles = false;
			fileStore.clear();
			return;
		}
		const seq = ++loadSeq;
		isLoadingFiles = true;
		errorMessage = '';
		try {
			const items = await ApiClient.request<FileAttachment[]>(filesQueryPath(workspaceId));
			if (seq === loadSeq) {
				fileStore.setFiles(items);
			}
		} catch (error) {
			if (seq === loadSeq) {
				errorMessage = apiErrorMessage(error, 'error.loadFiles');
			}
		} finally {
			if (seq === loadSeq) {
				isLoadingFiles = false;
			}
		}
	}

	async function handleWorkspaceChange(event: Event) {
		const target = event.currentTarget as HTMLSelectElement;
		selectedWorkspaceId = target.value;
		successMessage = '';
		errorMessage = '';
		if (selectedWorkspaceId) {
			authState.setWorkspace(selectedWorkspaceId);
		}
		fileStore.clear();
		await loadFiles(selectedWorkspaceId);
	}

	function onUploadComplete(file: FileAttachment) {
		fileStore.addFile(file);
		successMessage = format('files.uploaded', { name: file.fileName });
		errorMessage = '';
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
		isLoading = true;
		try {
			await loadWorkspaces();
			if (selectedWorkspaceId) {
				await loadFiles(selectedWorkspaceId);
			} else {
				fileStore.clear();
			}
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.loadWorkspaces');
		} finally {
			isLoading = false;
		}
	});
</script>

<div class="mx-auto flex w-full max-w-3xl flex-col gap-6 p-6">
	<div>
		<h1 class="text-2xl font-bold">{t['files.title']}</h1>
		<p class="text-sm text-muted-foreground">{t['files.description']}</p>
	</div>

	{#if errorMessage}
		<div
			role="alert"
			class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive"
		>
			{errorMessage}
		</div>
	{/if}
	{#if successMessage}
		<div class="rounded-md border border-primary/30 bg-primary/5 p-3 text-sm">{successMessage}</div>
	{/if}

	<div class="space-y-4 rounded-lg border p-4">
		<div class="space-y-2">
			<Label for="files-workspace">{t['files.workspace']}</Label>
			<select
				id="files-workspace"
				class="flex h-9 w-full rounded-md border bg-background px-3 text-sm"
				bind:value={selectedWorkspaceId}
				onchange={handleWorkspaceChange}
				disabled={isLoading || workspaces.length === 0}
			>
				{#if workspaces.length === 0}
					<option value="">{t['channels.noWorkspaces']}</option>
				{:else}
					{#each workspaces as workspace (workspace.id)}
						<option value={workspace.id}>{workspace.name}</option>
					{/each}
				{/if}
			</select>
		</div>
		<FileUpload workspaceId={selectedWorkspaceId} {onUploadComplete} />
	</div>

	<div class="space-y-2">
		<div class="flex items-center justify-between">
			<h2 class="font-semibold">{t['files.title']}</h2>
			<Button
				variant="outline"
				size="sm"
				disabled={isLoading || isLoadingFiles || !selectedWorkspaceId}
				onclick={() => loadFiles(selectedWorkspaceId)}
			>
				{t['files.refresh']}
			</Button>
		</div>
		{#if isLoadingFiles}
			<p class="text-sm text-muted-foreground">{t['files.loading']}</p>
		{:else if fileStore.files.length === 0}
			<p class="text-sm text-muted-foreground">{t['files.empty']}</p>
		{:else}
			{#each fileStore.files as file (file.id)}
				<FilePreview {file} />
			{/each}
		{/if}
	</div>

	<Button variant="outline" onclick={() => goto('/tasks')}>{t['nav.tasks']}</Button>
</div>
