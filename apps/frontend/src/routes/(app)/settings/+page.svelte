<script lang="ts">
	import OrganizationManager from '$lib/components/organization/OrganizationManager.svelte';
	import RoleManager from '$lib/components/rbac/RoleManager.svelte';
	import ThemeSelector from '$lib/components/settings/ThemeSelector.svelte';
	import WorkspaceCreate from '$lib/components/workspace/WorkspaceCreate.svelte';
	import { t } from '$lib/i18n';
</script>

<div class="mx-auto max-w-4xl space-y-8 p-6">
	<h1 class="mb-4 text-3xl font-bold">{t['settings.title']}</h1>

	<div class="space-y-6">
		<ThemeSelector />
		<OrganizationManager />
		<WorkspaceCreate />
		<RoleManager />
	</div>
</div>
