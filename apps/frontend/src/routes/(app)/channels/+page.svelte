<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { toChannel, type ApiChannel } from '$lib/api/mappers';
	import { authState } from '$lib/state/AuthState.svelte';
	import type { Channel } from '$lib/types/models';
	import ChatLayout from '$lib/components/chat/ChatLayout.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { apiErrorMessage, t } from '$lib/i18n';

	type Workspace = { id: string; name: string };

	let workspaces = $state<Workspace[]>([]);
	let channels = $state<Channel[]>([]);
	let selectedWorkspaceId = $state(authState.workspaceId ?? '');
	let selectedChannelId = $state('');
	let newChannelName = $state('');
	let isLoading = $state(false);
	let isSaving = $state(false);
	let errorMessage = $state('');

	async function loadWorkspaces() {
		const result = await ApiClient.request<Workspace[]>('/api/v1/workspaces');
		workspaces = result;
		if (!selectedWorkspaceId && workspaces.length > 0) {
			selectedWorkspaceId = authState.workspaceId ?? workspaces[0].id;
		}
		if (selectedWorkspaceId) authState.setWorkspace(selectedWorkspaceId);
	}

	async function loadChannels() {
		if (!selectedWorkspaceId) {
			channels = [];
			selectedChannelId = '';
			return;
		}
		const result = await ApiClient.request<ApiChannel[]>(
			`/api/v1/channels?workspace_id=${selectedWorkspaceId}`
		);
		channels = result.map((channel) => toChannel(channel));
		if (channels.every((c) => c.id !== selectedChannelId)) {
			selectedChannelId = channels[0]?.id ?? '';
		}
	}

	async function refresh() {
		isLoading = true;
		errorMessage = '';
		try {
			await loadWorkspaces();
			await loadChannels();
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.loadChannels');
		} finally {
			isLoading = false;
		}
	}

	async function onWorkspaceChange(event: Event) {
		selectedWorkspaceId = (event.currentTarget as HTMLSelectElement).value;
		authState.setWorkspace(selectedWorkspaceId || null);
		selectedChannelId = '';
		try {
			await loadChannels();
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.loadChannels');
		}
	}

	async function createChannel() {
		if (!selectedWorkspaceId || !newChannelName.trim()) return;
		isSaving = true;
		errorMessage = '';
		try {
			const created = await ApiClient.request<ApiChannel>('/api/v1/channels', {
				method: 'POST',
				body: JSON.stringify({
					workspace_id: selectedWorkspaceId,
					name: newChannelName.trim(),
					type: 'public'
				})
			});
			const channel = toChannel(created);
			channels = [...channels, channel].toSorted((a, b) => a.name.localeCompare(b.name));
			selectedChannelId = channel.id;
			newChannelName = '';
		} catch (error) {
			errorMessage = apiErrorMessage(error, 'error.createChannel');
		} finally {
			isSaving = false;
		}
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

	const selectedChannel = $derived(channels.find((c) => c.id === selectedChannelId) ?? null);
</script>

<div class="flex h-full flex-col gap-4 p-6">
	<div class="flex flex-wrap items-end justify-between gap-4">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">{t['channels.title']}</h1>
			<p class="text-sm text-muted-foreground">
				{t['channels.description']}
			</p>
		</div>
		<div class="min-w-56">
			<Label for="workspace">{t['channels.workspace']}</Label>
			<select
				id="workspace"
				class="mt-1 h-10 w-full rounded-md border bg-background px-3 text-sm"
				bind:value={selectedWorkspaceId}
				onchange={onWorkspaceChange}
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
	</div>

	{#if errorMessage}
		<div
			class="rounded-md border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive"
		>
			{errorMessage}
		</div>
	{/if}

	<div class="grid min-h-0 flex-1 gap-4 lg:grid-cols-[280px_minmax(0,1fr)]">
		<section class="flex min-h-0 flex-col rounded-md border bg-card">
			<div class="border-b p-3">
				<Label for="new-channel">{t['channels.newChannel']}</Label>
				<div class="mt-2 flex gap-2">
					<Input
						id="new-channel"
						bind:value={newChannelName}
						placeholder={t['channels.namePlaceholder']}
						disabled={!selectedWorkspaceId || isSaving}
						onkeydown={(event_) => event_.key === 'Enter' && createChannel()}
					/>
					<Button
						onclick={createChannel}
						disabled={!selectedWorkspaceId || !newChannelName.trim() || isSaving}
					>
						{t['channels.create']}
					</Button>
				</div>
			</div>
			<div class="flex-1 overflow-y-auto p-2">
				{#if isLoading}
					<p class="p-2 text-sm text-muted-foreground">{t['channels.loading']}</p>
				{:else if channels.length === 0}
					<p class="p-2 text-sm text-muted-foreground">{t['channels.empty']}</p>
				{:else}
					<div class="space-y-1">
						{#each channels as channel (channel.id)}
							<button
								type="button"
								class="w-full rounded-md px-3 py-2 text-left text-sm transition-colors {selectedChannelId ===
								channel.id
									? 'bg-primary/10 text-foreground'
									: 'hover:bg-muted'}"
								onclick={() => (selectedChannelId = channel.id)}
							>
								# {channel.name}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</section>

		<section class="min-h-0 rounded-md border bg-background">
			<div class="flex h-full flex-col">
				<div class="border-b px-4 py-3">
					<h2 class="font-semibold">
						{selectedChannel ? `# ${selectedChannel.name}` : t['channels.noneSelected']}
					</h2>
				</div>
				<div class="min-h-0 flex-1">
					<ChatLayout channelId={selectedChannelId} />
				</div>
			</div>
		</section>
	</div>
</div>
