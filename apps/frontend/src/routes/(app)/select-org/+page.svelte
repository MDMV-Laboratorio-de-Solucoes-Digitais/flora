<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { authState } from '$lib/state/AuthState.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	type Organization = {
		id: string;
		name: string;
		slug: string;
		role?: string;
	};

	let organizations = $state<Organization[]>([]);
	let loading = $state(true);
	let error = $state('');
	let createName = $state('');
	let createSlug = $state('');
	let submitting = $state(false);

	function slugify(value: string): string {
		return value
			.toLowerCase()
			.trim()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/(^-|-$)+/g, '');
	}

	$effect(() => {
		if (createName && !createSlug) {
			createSlug = slugify(createName);
		}
	});

	async function loadOrganizations() {
		loading = true;
		error = '';
		try {
			organizations = await ApiClient.request<Organization[]>('/api/v1/organizations');
		} catch (e) {
			error = (e as Error).message || 'Failed to load organizations';
		} finally {
			loading = false;
		}
	}

	async function selectOrganization(organizationId: string) {
		submitting = true;
		error = '';
		try {
			const response = await ApiClient.request<{
				session_token: string;
				user_id: string;
				organization_id: string;
			}>('/api/v1/auth/select-organization', {
				method: 'POST',
				body: JSON.stringify({ organization_id: organizationId })
			});
			authState.setOrganization(response.session_token, response.organization_id);
			await goto('/tasks');
		} catch (e) {
			error = (e as Error).message || 'Failed to select organization';
		} finally {
			submitting = false;
		}
	}

	async function createAndSelect() {
		if (!createName.trim() || !createSlug.trim()) return;
		submitting = true;
		error = '';
		try {
			const created = await ApiClient.request<{ id: string; name: string; slug: string }>(
				'/api/v1/organizations',
				{
					method: 'POST',
					body: JSON.stringify({ name: createName.trim(), slug: createSlug.trim() })
				}
			);
			await selectOrganization(created.id);
		} catch (e) {
			error = (e as Error).message || 'Failed to create organization';
		} finally {
			submitting = false;
		}
	}

	onMount(async () => {
		if (!authState.isAuthenticated || !authState.token) {
			await goto('/');
			return;
		}
		if (authState.organizationId) {
			await goto('/tasks');
			return;
		}
		await loadOrganizations();
	});
</script>

<div class="mx-auto flex w-full max-w-2xl flex-col gap-6 p-6">
	<div>
		<h1 class="text-2xl font-bold">Select organization</h1>
		<p class="text-sm text-muted-foreground">
			Choose an organization to scope your session before using workspaces and tasks.
		</p>
	</div>

	{#if error}
		<div class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive">
			{error}
		</div>
	{/if}

	<div class="space-y-3 rounded-lg border p-4">
		<h2 class="font-semibold">Your organizations</h2>
		{#if loading}
			<p class="text-sm text-muted-foreground">Loading...</p>
		{:else if organizations.length === 0}
			<p class="text-sm text-muted-foreground">No organizations found. Create one below.</p>
		{:else}
			<div class="space-y-2">
				{#each organizations as org (org.id)}
					<div class="flex items-center justify-between rounded-md border p-3">
						<div>
							<div class="font-medium">{org.name}</div>
							<div class="text-xs text-muted-foreground">{org.slug}{org.role ? ` · ${org.role}` : ''}</div>
						</div>
						<Button disabled={submitting} onclick={() => selectOrganization(org.id)}>Select</Button>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="space-y-4 rounded-lg border p-4">
		<h2 class="font-semibold">Create organization</h2>
		<div class="space-y-2">
			<Label for="org-name">Name</Label>
			<Input id="org-name" bind:value={createName} placeholder="Acme Corp" />
		</div>
		<div class="space-y-2">
			<Label for="org-slug">Slug</Label>
			<Input id="org-slug" bind:value={createSlug} placeholder="acme-corp" />
		</div>
		<Button disabled={submitting || !createName.trim() || !createSlug.trim()} onclick={createAndSelect}>
			Create and select
		</Button>
	</div>
</div>
