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
				const redirectUri = encodeURIComponent(`${window.location.origin}/callback`);
				const response = await ApiClient.request<{
					session_token: string;
					user_id: string;
				}>(`/api/v1/auth/callback?code=${code}&redirect_uri=${redirectUri}`, {
					method: 'GET'
				});
				authState.login(response.session_token, { id: response.user_id });
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
