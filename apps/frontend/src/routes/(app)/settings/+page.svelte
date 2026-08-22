<script lang="ts">
	import OrganizationManager from '$lib/components/organization/OrganizationManager.svelte';
	import WorkspaceCreate from '$lib/components/workspace/WorkspaceCreate.svelte';
	import UserManager from '$lib/components/users/UserManager.svelte';
</script>

<div class="mx-auto max-w-4xl space-y-8 p-6">
	<h1 class="mb-4 text-3xl font-bold">Settings & Administration</h1>

	<div class="grid gap-6 md:grid-cols-2">
		<div class="space-y-6">
			<OrganizationManager />
			<WorkspaceCreate />
		</div>
		<div>
			<UserManager />
		</div>
	</div>
</div>
