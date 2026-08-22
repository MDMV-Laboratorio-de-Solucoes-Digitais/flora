<script lang="ts">
	import { API_BASE_URL } from '$lib/api/client';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';

	let isSetup = $state(false); // Simulated state for First Seed Journey
	let workspaceName = $state('Flora');
	let adminEmail = $state('admin@flora.local');
	
	async function handleLogin() {
		try {
			const redirectUri = encodeURIComponent(`${window.location.origin}/callback`);
			const res = await fetch(`${API_BASE_URL}/api/v1/auth/login?redirect_uri=${redirectUri}`);
			const data = await res.json();
			if (data.authorization_url) {
				location.assign(data.authorization_url);
			}
		} catch (err) {
			console.error('Failed to start login flow', err);
		}
	}

	function toggleSetup() {
		isSetup = !isSetup;
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/20 px-4">
	{#if isSetup}
		<!-- Journey 1: The "First Seed" (Initial Setup) -->
		<Card class="w-full max-w-md shadow-lg border-primary/20 animate-in fade-in zoom-in duration-300">
			<CardHeader class="space-y-1 text-center">
				<div class="mx-auto mb-4 h-12 w-12 rounded-xl bg-primary text-primary-foreground flex items-center justify-center font-bold text-2xl shadow-sm">
					F
				</div>
				<CardTitle class="text-2xl font-bold tracking-tight">Plant the First Seed</CardTitle>
				<CardDescription>Configure your new Flora Workspace.</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label for="workspace-name">Workspace Name</Label>
					<Input id="workspace-name" bind:value={workspaceName} placeholder="e.g. Acme Corp" />
				</div>
				<div class="space-y-2">
					<Label for="admin-email">Admin Email</Label>
					<Input id="admin-email" type="email" bind:value={adminEmail} placeholder="admin@example.com" />
				</div>
				<div class="space-y-2">
					<Label for="admin-password">Admin Password</Label>
					<Input id="admin-password" type="password" value="********" />
				</div>
				<div class="rounded-md bg-muted/50 p-3 mt-4 text-xs text-muted-foreground border">
					<p class="font-medium text-foreground mb-1">Local-First Setup</p>
					No AWS keys required. Data will be saved locally to PostgreSQL and RustFS.
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={() => location.assign('/channels')}>Initialize Workspace</Button>
				<Button variant="ghost" class="w-full text-xs" onclick={toggleSetup}>Wait, I already have an account</Button>
			</CardFooter>
		</Card>
	{:else}
		<!-- Regular Login -->
		<Card class="w-full max-w-sm shadow-sm">
			<CardHeader class="space-y-1 text-center">
				<div class="mx-auto mb-2 h-10 w-10 rounded-lg bg-primary text-primary-foreground flex items-center justify-center font-bold text-xl">
					F
				</div>
				<CardTitle class="text-2xl font-bold tracking-tight">Flora Workspace</CardTitle>
				<CardDescription>Sign in to access your workspace</CardDescription>
			</CardHeader>
			<CardContent>
				<Button class="w-full" size="lg" onclick={handleLogin}>Login with SSO</Button>
			</CardContent>
			<CardFooter class="flex justify-center border-t p-4">
				<Button variant="link" class="text-xs text-muted-foreground" onclick={toggleSetup}>
					First time here? Run Setup
				</Button>
			</CardFooter>
		</Card>
	{/if}
</div>
