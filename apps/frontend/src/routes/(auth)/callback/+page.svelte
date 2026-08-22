<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authState } from '$lib/state/AuthState.svelte';
	import { logger } from '$lib/utils/logger';
	import { ApiClient } from '$lib/api/client';

	onMount(async () => {
		const parameters = new URLSearchParams(location.search);
		const code = parameters.get('code');
		const error = parameters.get('error');

		if (error) {
			logger.error('auth', 'OIDC error from Zitadel', { error });
			await goto('/login?error=' + error);
			return;
		}

		if (code) {
			try {
				const response = await ApiClient.request<{
					token: string;
					user: import('$lib/types/models').User;
				}>('/api/v1/auth/callback', {
					method: 'POST',
					body: JSON.stringify({ code })
				});
				authState.login(response.token, response.user as unknown as Record<string, unknown>);
				await goto('/channels');
			} catch (error_) {
				logger.error('auth', 'Failed to exchange code', undefined, error_ as Error);
				await goto('/login?error=exchange_failed');
			}
		} else {
			await goto('/login');
		}
	});
</script>

<div class="flex h-screen items-center justify-center">
	<p class="text-muted-foreground animate-pulse">Authenticating...</p>
</div>
