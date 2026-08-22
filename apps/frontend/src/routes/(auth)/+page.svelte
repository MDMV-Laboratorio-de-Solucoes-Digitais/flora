<script lang="ts">
	import { API_BASE_URL } from '$lib/api/client';
	import { Button } from '$lib/components/ui/button';

	function handleLogin() {
		// Redirect to backend OIDC initiation route
		location.assign(`${API_BASE_URL}/api/v1/auth/login`);
	}
</script>

<div class="flex h-screen flex-col items-center justify-center space-y-4">
	<h1 class="text-3xl font-bold">Flora Workspace</h1>
	<p class="text-muted-foreground">Sign in to access your workspace</p>
	<Button onclick={handleLogin}>Login with Zitadel</Button>
</div>
