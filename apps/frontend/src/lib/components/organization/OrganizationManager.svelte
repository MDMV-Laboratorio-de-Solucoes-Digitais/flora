<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	let name = $state('');
	let isSubmitting = $state(false);
	let successMessage = $state('');

	async function createOrganization() {
		if (!name.trim()) return;
		isSubmitting = true;
		successMessage = '';
		try {
			await ApiClient.request('/api/v1/organizations', {
				method: 'POST',
				body: JSON.stringify({ name })
			});
			successMessage = `Created organization: ${name}`;
			name = '';
		} catch (error) {
			logger.error('organization', 'Failed to create organization', undefined, error as Error);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="space-y-4 rounded-lg border p-6 shadow-sm">
	<div class="space-y-2">
		<h2 class="text-2xl font-bold tracking-tight">Create Organization</h2>
		<p class="text-muted-foreground text-sm">Set up a new organization.</p>
	</div>

	{#if successMessage}
		<div class="text-sm text-green-600">{successMessage}</div>
	{/if}

	<div class="space-y-2">
		<Label for="org-name">Organization Name</Label>
		<Input id="org-name" bind:value={name} placeholder="Acme Corp" />
	</div>

	<Button disabled={isSubmitting} onclick={createOrganization}>Create</Button>
</div>
