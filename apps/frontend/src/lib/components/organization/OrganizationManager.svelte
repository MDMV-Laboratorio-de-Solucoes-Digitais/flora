<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { format, t } from '$lib/i18n';

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
			successMessage = format('org.created', { name });
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
		<h2 class="text-2xl font-bold tracking-tight">{t['org.managerTitle']}</h2>
		<p class="text-sm text-muted-foreground">{t['org.managerDescription']}</p>
	</div>

	{#if successMessage}
		<div class="text-sm text-green-600">{successMessage}</div>
	{/if}

	<div class="space-y-2">
		<Label for="org-name">{t['org.managerName']}</Label>
		<Input id="org-name" bind:value={name} placeholder={t['common.organizationPlaceholder']} />
	</div>

	<Button disabled={isSubmitting} onclick={createOrganization}>{t['org.create']}</Button>
</div>
