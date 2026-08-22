<script lang="ts">
	import { ApiClient } from '$lib/api/client';
	import { logger } from '$lib/utils/logger';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	let email = $state('');
	let role = $state('Member');
	let isSubmitting = $state(false);
	let successMessage = $state('');

	async function inviteUser() {
		if (!email.trim()) return;
		isSubmitting = true;
		successMessage = '';
		try {
			await ApiClient.request('/api/v1/users/invite', {
				method: 'POST',
				body: JSON.stringify({ email, role })
			});
			successMessage = `Invited ${email} as ${role}`;
			email = '';
		} catch (error) {
			logger.error('users', 'Failed to invite user', undefined, error as Error);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="space-y-4 rounded-lg border p-6 shadow-sm">
	<div class="space-y-2">
		<h2 class="text-2xl font-bold tracking-tight">Invite User</h2>
		<p class="text-muted-foreground text-sm">Invite a new user to your workspace.</p>
	</div>

	{#if successMessage}
		<div class="text-sm text-green-600">{successMessage}</div>
	{/if}

	<div class="space-y-2">
		<Label for="user-email">Email Address</Label>
		<Input id="user-email" type="email" bind:value={email} placeholder="user@example.com" />
	</div>

	<div class="space-y-2">
		<Label for="user-role">Role</Label>
		<select
			id="user-role"
			bind:value={role}
			class="border-input bg-background ring-offset-background focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none"
		>
			<option value="Admin">Admin</option>
			<option value="Member">Member</option>
			<option value="Guest">Guest</option>
		</select>
	</div>

	<Button disabled={isSubmitting} onclick={inviteUser}>Send Invitation</Button>
</div>
