<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authState } from '$lib/state/AuthState.svelte';
	import { logger } from '$lib/utils/logger';
	import { ApiClient } from '$lib/api/client';

	onMount(async () => {
		const parameters = new URLSearchParams(location.search);
		const code = parameters.get('code');
		const state = parameters.get('state');
		const error = parameters.get('error');

		if (error) {
			logger.error('auth', 'OIDC error from provider', { error });
			await goto('/?error=' + encodeURIComponent(error));
			return;
		}

		if (!code || !state) {
			await goto('/?error=missing_code_or_state');
			return;
		}

		if (typeof sessionStorage === 'undefined') {
			await goto('/?error=storage_unavailable');
			return;
		}

		const storedState = sessionStorage.getItem('flora.oauth.state');
		if (storedState && storedState !== state) {
			logger.error('auth', 'OIDC state mismatch');
			await goto('/?error=state_mismatch');
			return;
		}

		const storedRedirectUri = sessionStorage.getItem('flora.oauth.redirect_uri');

		try {
			const redirectUri = storedRedirectUri ?? `${location.origin}/callback`;
			const query = new URLSearchParams({
				code,
				state,
				redirect_uri: redirectUri
			});
			const response = await ApiClient.request<{
				session_token: string;
				user_id: string;
				next_step?: string;
			}>(`/api/v1/auth/callback?${query.toString()}`, {
				method: 'GET'
			});
			sessionStorage.removeItem('flora.oauth.state');
			sessionStorage.removeItem('flora.oauth.redirect_uri');
			authState.login(response.session_token, { id: response.user_id }, null);
			if (response.next_step === 'select_or_create_organization') {
				await goto('/select-org');
			} else {
				await goto('/tasks');
			}
		} catch (error_) {
			logger.error('auth', 'Failed to exchange code', undefined, error_ as Error);
			await goto('/?error=exchange_failed');
		}
	});
</script>

<div class="flex h-screen items-center justify-center">
	<p class="animate-pulse text-muted-foreground">Authenticating...</p>
</div>
