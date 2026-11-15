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
			const redirectUri = `${location.origin}/callback`;
			const response = await fetch(
				`${API_BASE_URL}/api/v1/auth/login?redirect_uri=${encodeURIComponent(redirectUri)}`
			);
			const data = await response.json();
			if (data.authorization_url) {
				if (typeof sessionStorage !== 'undefined' && data.state) {
					sessionStorage.setItem('flora.oauth.state', String(data.state));
					sessionStorage.setItem('flora.oauth.redirect_uri', redirectUri);
				}
				location.assign(String(data.authorization_url));
			}
		} catch (error_) {
			globalThis.console?.error('Failed to start login flow', error_);
		}
	}

	function toggleSetup() {
		isSetup = !isSetup;
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/40 p-4">
	<Card class="w-full max-w-md shadow-lg">
		{#if isSetup}
			<!-- First Seed Journey UI -->
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">First Seed Journey</CardTitle>
				<CardDescription>Plant the roots of your new Flora workspace.</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="workspaceName">Workspace Name</Label>
					<Input id="workspaceName" bind:value={workspaceName} placeholder="My Awesome Team" />
				</div>
				<div class="grid gap-2">
					<Label for="adminEmail">Admin Email</Label>
					<Input
						id="adminEmail"
						type="email"
						bind:value={adminEmail}
						placeholder="admin@example.com"
					/>
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleLogin}>Sprout Workspace</Button>
				<Button variant="ghost" class="w-full" onclick={toggleSetup}>Back to Login</Button>
			</CardFooter>
		{:else}
			<!-- Standard Login UI -->
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">Welcome to Flora</CardTitle>
				<CardDescription>Enter your credentials to access your workspace.</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="email">Email</Label>
					<Input id="email" type="email" placeholder="m@example.com" />
				</div>
				<div class="grid gap-2">
					<Label for="password">Password</Label>
					<Input id="password" type="password" />
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleLogin}>Sign In</Button>

				<div class="mt-4 text-center text-sm text-muted-foreground">
					Is this a fresh install?
					<button onclick={toggleSetup} class="text-primary underline-offset-4 hover:underline">
						Start First Seed Journey
					</button>
				</div>
			</CardFooter>
		{/if}
	</Card>
</div>
