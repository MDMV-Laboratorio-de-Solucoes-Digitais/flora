<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { ApiClient } from '$lib/api/client';
	import { authState } from '$lib/state/AuthState.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { apiErrorMessage, t } from '$lib/i18n';

	type Organization = {
		id: string;
		name: string;
		slug: string;
		role?: string;
	};

	let organizations = $state<Organization[]>([]);
	let isLoading = $state(true);
	let errorMessage = $state('');
	let createName = $state('');
	let createSlug = $state('');
	let isSubmitting = $state(false);

	function slugify(value: string): string {
		return value
			.toLowerCase()
			.trim()
			.replaceAll(/[^a-z0-9]+/g, '-')
			.replaceAll(/(^-|-$)+/g, '');
	}

	$effect(() => {
		if (createName && !createSlug) {
			createSlug = slugify(createName);
		}
	});

	async function loadOrganizations() {
		isLoading = true;
		errorMessage = '';
		try {
			organizations = await ApiClient.request<Organization[]>('/api/v1/organizations');
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.loadOrganizations');
			isLoading = false;
		}
	}

	async function selectOrganization(organizationId: string) {
		isSubmitting = true;
		errorMessage = '';
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
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.selectOrganization');
			isSubmitting = false;
		}
	}

	async function createAndSelect() {
		if (!createName.trim() || !createSlug.trim()) return;
		isSubmitting = true;
		errorMessage = '';
		try {
			const created = await ApiClient.request<{ id: string; name: string; slug: string }>(
				'/api/v1/organizations',
				{
					method: 'POST',
					body: JSON.stringify({ name: createName.trim(), slug: createSlug.trim() })
				}
			);
			await selectOrganization(created.id);
		} catch (error_) {
			errorMessage = apiErrorMessage(error_, 'error.createOrganization');
			isSubmitting = false;
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
		<h1 class="text-2xl font-bold">{t['org.selectTitle']}</h1>
		<p class="text-sm text-muted-foreground">
			{t['org.selectDescription']}
		</p>
	</div>

	{#if errorMessage}
		<div
			class="rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive"
		>
			{errorMessage}
		</div>
	{/if}

	<div class="space-y-3 rounded-lg border p-4">
		<h2 class="font-semibold">{t['org.yourOrganizations']}</h2>
		{#if isLoading}
			<p class="text-sm text-muted-foreground">{t['org.loading']}</p>
		{:else if organizations.length === 0}
			<p class="text-sm text-muted-foreground">{t['org.empty']}</p>
		{:else}
			<div class="space-y-2">
				{#each organizations as org (org.id)}
					<div class="flex items-center justify-between rounded-md border p-3">
						<div>
							<div class="font-medium">{org.name}</div>
							<div class="text-xs text-muted-foreground">
								{org.slug}{org.role ? ` · ${org.role}` : ''}
							</div>
						</div>
						<Button disabled={isSubmitting} onclick={() => selectOrganization(org.id)}
							>{t['org.select']}</Button
						>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="space-y-4 rounded-lg border p-4">
		<h2 class="font-semibold">{t['org.createTitle']}</h2>
		<div class="space-y-2">
			<Label for="org-name">{t['org.name']}</Label>
			<Input
				id="org-name"
				bind:value={createName}
				placeholder={t['common.organizationPlaceholder']}
			/>
		</div>
		<div class="space-y-2">
			<Label for="org-slug">{t['org.slug']}</Label>
			<Input id="org-slug" bind:value={createSlug} placeholder={t['common.slugPlaceholder']} />
		</div>
		<Button
			disabled={isSubmitting || !createName.trim() || !createSlug.trim()}
			onclick={createAndSelect}
		>
			{t['org.createAndSelect']}
		</Button>
	</div>
</div>
