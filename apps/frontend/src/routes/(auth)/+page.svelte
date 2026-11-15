<script lang="ts">
	import { API_BASE_URL } from '$lib/api/client';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import {
		Card,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		CardFooter
	} from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';

	let isSetup = $state(false); // Simulated state for First Seed Journey
	let workspaceName = $state('Flora');
	let adminEmail = $state('admin@flora.local');

	async function handleLogin() {
		try {
			const redirectUri = encodeURIComponent(`${location.origin}/callback`);
			const response = await fetch(`${API_BASE_URL}/api/v1/auth/login?redirect_uri=${redirectUri}`);
			const data = await response.json();
			if (data.authorization_url) {
				location.assign(data.authorization_url);
			}
		} catch (error) {
			globalThis.console?.error('Failed to start login flow', error);
		}
	}

	function toggleSetup() {
		isSetup = !isSetup;
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/20 px-4">
	{#if isSetup}
		<!-- Journey 1: The "First Seed" (Initial Setup) -->
		<Card
			class="animate-in fade-in zoom-in w-full max-w-md border-primary/20 shadow-lg duration-300"
		>
			<CardHeader class="space-y-1 text-center">
				<div
					class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-primary text-2xl font-bold text-primary-foreground shadow-sm"
				>
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
					<Input
						id="admin-email"
						type="email"
						bind:value={adminEmail}
						placeholder="admin@example.com"
					/>
				</div>
				<div class="space-y-2">
					<Label for="admin-password">Admin Password</Label>
					<Input id="admin-password" type="password" value="********" />
				</div>
				<div class="mt-4 rounded-md border bg-muted/50 p-3 text-xs text-muted-foreground">
					<p class="mb-1 font-medium text-foreground">Local-First Setup</p>
					No AWS keys required. Data will be saved locally to PostgreSQL and RustFS.
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={() => location.assign('/channels')}
					>Initialize Workspace</Button
				>
				<Button variant="ghost" class="w-full text-xs" onclick={toggleSetup}
					>Wait, I already have an account</Button
				>
			</CardFooter>
		</Card>
	{:else}
		<!-- Regular Login -->
		<Card class="w-full max-w-sm shadow-sm">
			<CardHeader class="space-y-1 text-center">
				<div
					class="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-lg bg-primary text-xl font-bold text-primary-foreground"
				>
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
