<script lang="ts">
	import {
		addOrganizationMember,
		assignRoleToMember,
		fetchOrganizationMembers,
		fetchRoles,
		revokeRoleFromMember
	} from '$lib/api/roles';
	import { authState } from '$lib/state/AuthState.svelte';
	import { memberStore } from '$lib/state/MemberStore.svelte';
	import { roleStore } from '$lib/state/RoleStore.svelte';
	import type { OrganizationMemberItem } from '$lib/types/models';
	import { canManageMember, validateAddMemberForm } from '$lib/utils/members';
	import { buildNotificationContextKey } from '$lib/utils/notifications';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { apiErrorMessage, format, t, type TranslationKey } from '$lib/i18n';

	let addEmail = $state('');
	let addRoleId = $state('');
	let draftRoleIds = $state<Record<string, string>>({});
	let isMutating = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');
	let observedContextKey: string | null = null;

	const currentUserId = $derived(typeof authState.user?.id === 'string' ? authState.user.id : null);
	const defaultAddRoleId = $derived(
		roleStore.roles.find((role) => role.name === 'Member')?.id ?? roleStore.roles[0]?.id ?? ''
	);
	const effectiveAddRoleId = $derived(
		roleStore.roles.some((role) => role.id === addRoleId) ? addRoleId : defaultAddRoleId
	);

	function contextKey(): string | null {
		return buildNotificationContextKey(authState);
	}

	function describeError(error: unknown, fallback: TranslationKey): string {
		const apiError = error as { status?: unknown; code?: unknown } | null;
		if (apiError?.status === 403) return t['members.forbidden'];
		if (apiError?.status === 404) return t['members.notFound'];
		if (apiError?.status === 409 || apiError?.code === 'conflict') return t['members.duplicate'];
		return apiErrorMessage(error, fallback);
	}

	function draftRoleFor(member: OrganizationMemberItem): string {
		const candidate = draftRoleIds[member.userId];
		if (candidate && roleStore.roles.some((role) => role.id === candidate)) {
			return candidate;
		}
		return member.roleId;
	}

	async function loadMembers(shouldForce = false) {
		const key = contextKey();
		if (!key) return;
		memberStore.syncContext(key);
		errorMessage = '';
		try {
			await Promise.all([
				memberStore.load({ contextKey: key, shouldForce, fetcher: fetchOrganizationMembers }),
				roleStore.roles.length === 0
					? roleStore.load({ contextKey: key, fetcher: fetchRoles })
					: Promise.resolve(true)
			]);
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.loadMembers');
		}
	}

	async function assignMemberRole(member: OrganizationMemberItem) {
		if (isMutating || memberStore.isLoading || !canManageMember(member, currentUserId)) return;
		const targetRoleId = draftRoleFor(member);
		if (!targetRoleId || targetRoleId === member.roleId) return;
		const key = contextKey();
		if (!key) return;

		isMutating = true;
		errorMessage = '';
		successMessage = '';
		try {
			await assignRoleToMember(targetRoleId, member.userId);
			if (contextKey() === key) {
				const roleName =
					roleStore.roles.find((role) => role.id === targetRoleId)?.name ?? member.roleName;
				memberStore.updateRoleLocal(member.userId, targetRoleId, roleName);
				draftRoleIds = { ...draftRoleIds, [member.userId]: targetRoleId };
				successMessage = format('members.roleUpdated', {
					name: member.displayName,
					role: roleName
				});
			}
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.assignRole');
		} finally {
			isMutating = false;
		}
	}

	async function removeMember(member: OrganizationMemberItem) {
		if (isMutating || memberStore.isLoading || !canManageMember(member, currentUserId)) return;
		const key = contextKey();
		if (!key) return;

		isMutating = true;
		errorMessage = '';
		successMessage = '';
		try {
			await revokeRoleFromMember(member.roleId, member.userId);
			if (contextKey() === key) {
				memberStore.remove(member.userId);
				successMessage = format('members.removed', { name: member.displayName });
			}
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.revokeRole');
		} finally {
			isMutating = false;
		}
	}

	async function submitAddMember(event: SubmitEvent) {
		event.preventDefault();
		if (isMutating || memberStore.isLoading) return;
		const roleId = effectiveAddRoleId;
		const invalid = validateAddMemberForm({ email: addEmail, roleId });
		successMessage = '';
		if (invalid) {
			errorMessage = t[invalid];
			return;
		}
		const key = contextKey();
		if (!key) return;

		isMutating = true;
		errorMessage = '';
		try {
			const created = await addOrganizationMember({ email: addEmail, roleId });
			if (contextKey() === key) {
				memberStore.upsert(created);
				draftRoleIds = { ...draftRoleIds, [created.userId]: created.roleId };
				addEmail = '';
				successMessage = format('members.added', { name: created.displayName });
			}
		} catch (error) {
			if (contextKey() === key) errorMessage = describeError(error, 'error.addMember');
		} finally {
			isMutating = false;
		}
	}

	$effect(() => {
		const key = contextKey();
		if (!key) {
			observedContextKey = null;
			memberStore.clear();
			draftRoleIds = {};
			addEmail = '';
			errorMessage = '';
			successMessage = '';
			return;
		}
		if (key !== observedContextKey) {
			observedContextKey = key;
			draftRoleIds = {};
			addEmail = '';
			errorMessage = '';
			successMessage = '';
			void loadMembers();
		}
	});
</script>

<section class="space-y-4 rounded-lg border p-6 shadow-sm" aria-labelledby="members-title">
	<div class="flex flex-wrap items-start justify-between gap-4">
		<div class="space-y-2">
			<h2 id="members-title" class="text-2xl font-bold tracking-tight">{t['members.title']}</h2>
			<p class="text-sm text-muted-foreground">{t['members.description']}</p>
		</div>
		<Button
			variant="outline"
			size="sm"
			disabled={memberStore.isLoading || isMutating}
			onclick={() => loadMembers(true)}
		>
			{t['members.refresh']}
		</Button>
	</div>

	{#if errorMessage}
		<div role="alert" class="text-sm text-destructive">{errorMessage}</div>
	{/if}
	{#if successMessage}
		<div role="status" class="text-sm text-green-700 dark:text-green-400">{successMessage}</div>
	{/if}

	{#if memberStore.isLoading}
		<p class="text-sm text-muted-foreground">{t['members.loading']}</p>
	{:else if memberStore.members.length === 0}
		<p class="text-sm text-muted-foreground">{t['members.empty']}</p>
	{:else}
		<ul class="space-y-2">
			{#each memberStore.members as member (member.userId)}
				<li class="flex flex-wrap items-center justify-between gap-4 rounded-md border p-3">
					<div class="space-y-1">
						<div class="flex flex-wrap items-center gap-2">
							<span class="font-medium">{member.displayName}</span>
							<Badge variant="outline">{member.roleName}</Badge>
							{#if !canManageMember(member, currentUserId)}
								<Badge variant="secondary">{t['members.you']}</Badge>
							{/if}
						</div>
						<p class="text-sm text-muted-foreground">{member.email}</p>
					</div>

					{#if canManageMember(member, currentUserId)}
						{@const draftRoleId = draftRoleFor(member)}
						<div class="flex flex-wrap items-center gap-2">
							<label for={`member-role-${member.userId}`} class="sr-only">
								{format('members.roleLabel', { name: member.displayName })}
							</label>
							<select
								id={`member-role-${member.userId}`}
								class="h-8 rounded-lg border border-input bg-background px-2.5 text-sm text-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none"
								value={draftRoleId}
								disabled={isMutating || memberStore.isLoading || roleStore.roles.length === 0}
								onchange={(event) => {
									draftRoleIds = { ...draftRoleIds, [member.userId]: event.currentTarget.value };
								}}
							>
								{#each roleStore.roles as role (role.id)}
									<option value={role.id}>{role.name}</option>
								{/each}
							</select>
							<Button
								type="button"
								variant="outline"
								size="sm"
								aria-label={format('members.saveRoleAria', { name: member.displayName })}
								disabled={isMutating || memberStore.isLoading || draftRoleId === member.roleId}
								onclick={() => assignMemberRole(member)}
							>
								{t['members.saveRole']}
							</Button>
							<Button
								type="button"
								variant="outline"
								size="sm"
								aria-label={format('members.removeAria', { name: member.displayName })}
								disabled={isMutating || memberStore.isLoading}
								onclick={() => removeMember(member)}
							>
								{t['members.remove']}
							</Button>
						</div>
					{/if}
				</li>
			{/each}
		</ul>
	{/if}

	<form class="space-y-4 border-t pt-4" onsubmit={submitAddMember}>
		<h3 class="text-lg font-semibold">{t['members.addTitle']}</h3>
		<div class="grid gap-4 md:grid-cols-2">
			<div class="space-y-2">
				<Label for="member-email">{t['members.emailLabel']}</Label>
				<Input
					id="member-email"
					type="email"
					bind:value={addEmail}
					placeholder={t['members.emailPlaceholder']}
				/>
			</div>
			<div class="space-y-2">
				<Label for="member-add-role">{t['members.roleSelectLabel']}</Label>
				<select
					id="member-add-role"
					class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-xs focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none"
					value={effectiveAddRoleId}
					disabled={isMutating || roleStore.roles.length === 0}
					onchange={(event) => {
						addRoleId = event.currentTarget.value;
					}}
				>
					{#each roleStore.roles as role (role.id)}
						<option value={role.id}>{role.name}</option>
					{/each}
				</select>
			</div>
		</div>
		<Button type="submit" disabled={isMutating || memberStore.isLoading || !effectiveAddRoleId}>
			{t['members.addSubmit']}
		</Button>
	</form>
</section>
