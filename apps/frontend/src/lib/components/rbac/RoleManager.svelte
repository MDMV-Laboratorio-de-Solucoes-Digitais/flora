<script lang="ts">
	import { createRole, deleteRole, fetchRoles, updateRole } from '$lib/api/roles';
	import { authState } from '$lib/state/AuthState.svelte';
	import { roleStore } from '$lib/state/RoleStore.svelte';
	import type { RoleItem } from '$lib/types/models';
	import { buildNotificationContextKey } from '$lib/utils/notifications';
	import {
		ROLE_PERMISSION_OPTIONS,
		canModifyRole,
		formatRolePermissions,
		pickEditablePermissions,
		validateRoleForm
	} from '$lib/utils/roles';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { apiErrorMessage, format, t, type TranslationKey } from '$lib/i18n';

	let editingRoleId = $state<string | null>(null);
	let name = $state('');
	let description = $state('');
	let selected = $state<string[]>([]);
	let isMutating = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');
	let observedContextKey: string | null = null;

	function contextKey(): string | null {
		return buildNotificationContextKey(authState);
	}

	function describeError(error: unknown, fallback: TranslationKey): string {
		const apiError = error as { status?: unknown; code?: unknown } | null;
		if (apiError?.status === 403) return t['roles.forbidden'];
		if (apiError?.status === 409 || apiError?.code === 'conflict') return t['roles.duplicateName'];
		return apiErrorMessage(error, fallback);
	}

	function resetForm() {
		editingRoleId = null;
		name = '';
		description = '';
		selected = [];
	}

	function startEditing(role: RoleItem) {
		if (isMutating || !canModifyRole(role)) return;
		editingRoleId = role.id;
		name = role.name;
		description = role.description ?? '';
		selected = pickEditablePermissions(role.permissions);
		errorMessage = '';
		successMessage = '';
	}

	async function loadRoles(shouldForce = false) {
		const key = contextKey();
		if (!key) return;
		roleStore.syncContext(key);
		errorMessage = '';
		try {
			await roleStore.load({ contextKey: key, shouldForce, fetcher: fetchRoles });
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.loadRoles');
		}
	}

	async function submitRole(event: SubmitEvent) {
		event.preventDefault();
		if (isMutating || roleStore.isLoading) return;
		const invalid = validateRoleForm({ name, permissions: selected });
		successMessage = '';
		if (invalid) {
			errorMessage = t[invalid];
			return;
		}
		const key = contextKey();
		if (!key) return;
		isMutating = true;
		errorMessage = '';
		const activeEditId = editingRoleId;
		try {
			if (activeEditId) {
				const updated = await updateRole(activeEditId, {
					name,
					description,
					permissions: selected
				});
				if (contextKey() === key) {
					roleStore.upsert(updated);
					successMessage = format('roles.updated', { name: updated.name });
					resetForm();
				}
			} else {
				const created = await createRole({ name, description, permissions: selected });
				if (contextKey() === key) {
					roleStore.upsert(created);
					successMessage = format('roles.created', { name: created.name });
					resetForm();
				}
			}
		} catch (error) {
			if (contextKey() === key) {
				errorMessage = describeError(error, activeEditId ? 'error.updateRole' : 'error.createRole');
			}
		} finally {
			isMutating = false;
		}
	}

	async function removeRole(id: string, roleName: string) {
		if (isMutating || roleStore.isLoading) return;
		const key = contextKey();
		if (!key) return;
		isMutating = true;
		errorMessage = '';
		successMessage = '';
		try {
			await deleteRole(id);
			if (contextKey() === key) {
				roleStore.remove(id);
				if (editingRoleId === id) resetForm();
				successMessage = format('roles.deleted', { name: roleName });
			}
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.deleteRole');
		} finally {
			isMutating = false;
		}
	}

	$effect(() => {
		const key = contextKey();
		if (!key) {
			observedContextKey = null;
			roleStore.clear();
			resetForm();
			return;
		}
		if (key !== observedContextKey) {
			observedContextKey = key;
			resetForm();
			void loadRoles();
		}
	});
</script>

<section class="space-y-4 rounded-lg border p-6 shadow-sm" aria-labelledby="roles-title">
	<div class="flex flex-wrap items-start justify-between gap-4">
		<div class="space-y-2">
			<h2 id="roles-title" class="text-2xl font-bold tracking-tight">{t['roles.title']}</h2>
			<p class="text-sm text-muted-foreground">{t['roles.description']}</p>
		</div>
		<Button
			variant="outline"
			size="sm"
			disabled={roleStore.isLoading || isMutating}
			onclick={() => loadRoles(true)}
		>
			{t['roles.refresh']}
		</Button>
	</div>

	{#if errorMessage}
		<div role="alert" class="text-sm text-destructive">{errorMessage}</div>
	{/if}
	{#if successMessage}
		<div role="status" class="text-sm text-green-700 dark:text-green-400">{successMessage}</div>
	{/if}

	{#if roleStore.isLoading}
		<p class="text-sm text-muted-foreground">{t['roles.loading']}</p>
	{:else if roleStore.roles.length === 0}
		<p class="text-sm text-muted-foreground">{t['roles.empty']}</p>
	{:else}
		<ul class="space-y-2">
			{#each roleStore.roles as role (role.id)}
				<li class="flex flex-wrap items-start justify-between gap-4 rounded-md border p-3">
					<div class="space-y-1">
						<div class="flex items-center gap-2">
							<span class="font-medium">{role.name}</span>
							<Badge variant={role.isBuiltin ? 'secondary' : 'outline'}>
								{role.isBuiltin ? t['roles.builtin'] : t['roles.custom']}
							</Badge>
						</div>
						{#if role.description}
							<p class="text-sm text-muted-foreground">{role.description}</p>
						{/if}
						<p class="text-xs text-muted-foreground">{formatRolePermissions(role.permissions)}</p>
					</div>
					{#if canModifyRole(role)}
						<div class="flex items-center gap-2">
							<Button
								variant="outline"
								size="sm"
								disabled={isMutating || roleStore.isLoading}
								aria-label={format('roles.edit', { name: role.name })}
								onclick={() => startEditing(role)}
							>
								{t['roles.editShort']}
							</Button>
							<Button
								variant="outline"
								size="sm"
								disabled={isMutating || roleStore.isLoading}
								aria-label={format('roles.delete', { name: role.name })}
								onclick={() => removeRole(role.id, role.name)}
							>
								{t['roles.deleteShort']}
							</Button>
						</div>
					{/if}
				</li>
			{/each}
		</ul>
	{/if}

	<form class="space-y-4 border-t pt-4" onsubmit={submitRole}>
		<h3 class="text-lg font-semibold">
			{editingRoleId ? t['roles.editTitle'] : t['roles.createTitle']}
		</h3>
		<div class="space-y-2">
			<Label for="role-name">{t['roles.name']}</Label>
			<Input id="role-name" bind:value={name} placeholder={t['roles.namePlaceholder']} />
		</div>
		<div class="space-y-2">
			<Label for="role-description">{t['roles.descriptionLabel']}</Label>
			<Input id="role-description" bind:value={description} />
		</div>
		<fieldset class="space-y-2">
			<legend class="text-sm font-medium">{t['roles.permissionsLegend']}</legend>
			<div class="grid grid-cols-2 gap-2 md:grid-cols-3">
				{#each ROLE_PERMISSION_OPTIONS as permission (permission)}
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" value={permission} bind:group={selected} />
						<span>{permission}</span>
					</label>
				{/each}
			</div>
		</fieldset>
		<div class="flex items-center gap-2">
			<Button type="submit" disabled={isMutating || roleStore.isLoading}>
				{editingRoleId ? t['roles.save'] : t['roles.create']}
			</Button>
			{#if editingRoleId}
				<Button type="button" variant="outline" disabled={isMutating} onclick={resetForm}>
					{t['roles.cancel']}
				</Button>
			{/if}
		</div>
	</form>
</section>
