<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	let name = $state('');
	let isSubmitting = $state(false);

	async function createWorkspace() {
		if (!name.trim()) return;
		isSubmitting = true;
		try {
			await ApiClient.request('/api/v1/workspaces', {
				method: 'POST',
				body: JSON.stringify({ name })
			});
			name = '';
			// Optionally redirect or update a store
		} catch (error) {
			logger.error('workspace', 'Failed to create workspace', undefined, error as Error);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="space-y-4 rounded-lg border p-6 shadow-sm">
	<div class="space-y-2">
		<h2 class="text-2xl font-bold tracking-tight">Create Workspace</h2>
		<p class="text-muted-foreground text-sm">Set up a new workspace for your team.</p>
	</div>

	<div class="space-y-2">
		<Label for="workspace-name">Workspace Name</Label>
		<Input id="workspace-name" bind:value={name} placeholder="Acme Corp" />
	</div>
	<Button disabled={isSubmitting} onclick={createWorkspace}>Create</Button>
</div>
