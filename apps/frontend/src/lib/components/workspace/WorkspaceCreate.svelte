<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { t } from '$lib/i18n';

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
		<h2 class="text-2xl font-bold tracking-tight">{t['workspace.createTitle']}</h2>
		<p class="text-sm text-muted-foreground">{t['workspace.createDescription']}</p>
	</div>

	<div class="space-y-2">
		<Label for="workspace-name">{t['workspace.name']}</Label>
		<Input
			id="workspace-name"
			bind:value={name}
			placeholder={t['common.organizationPlaceholder']}
		/>
	</div>
	<Button disabled={isSubmitting} onclick={createWorkspace}>{t['workspace.create']}</Button>
</div>
