<script lang="ts">
	import { goto } from '$app/navigation';
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
	import { authState } from '$lib/state/AuthState.svelte';

	let isSetup = $state(false); // Simulated state for First Seed Journey
	let workspaceName = $state('Flora');
	let adminEmail = $state('admin@flora.local');
	let errorMessage = $state<string | null>(null);
	let isSubmitting = $state(false);

	async function readErrorDetail(response: Response): Promise<string> {
		try {
			const body: unknown = await response.json();
			if (body && typeof body === 'object') {
				const detail =
					(body as { message?: unknown; error?: unknown }).message ??
					(body as { error?: unknown }).error;
				if (typeof detail === 'string' && detail.trim().length > 0) return detail.trim();
			}
		} catch {
			// Non-JSON error body: fall back to the generic message.
		}
		return '';
	}

	async function handleLogin() {
		if (isSubmitting) return;
		isSubmitting = true;
		errorMessage = null;
		try {
			const redirectUri = `${location.origin}/callback`;
			const response = await fetch(
				`${API_BASE_URL}/api/v1/auth/login?redirect_uri=${encodeURIComponent(redirectUri)}`
			);
			if (!response.ok) {
				const detail = await readErrorDetail(response);
				errorMessage =
					response.status === 429
						? 'Too many attempts. Please wait a minute before trying again.'
						: `Unable to start sign-in.${detail ? ` ${detail}` : ''}`;
				return;
			}
			const data = await response.json();
			if (data.authorization_url) {
				if (typeof sessionStorage !== 'undefined' && data.state) {
					sessionStorage.setItem('flora.oauth.state', String(data.state));
					sessionStorage.setItem('flora.oauth.redirect_uri', redirectUri);
				}
				location.assign(String(data.authorization_url));
				return;
			}
			errorMessage =
				'Unable to start sign-in. The identity provider returned no authorization URL.';
		} catch (error_) {
			globalThis.console?.error('Failed to start login flow', error_);
			errorMessage = 'Unable to reach the server. Check that the API is running and try again.';
		} finally {
			isSubmitting = false;
		}
	}

	async function handleBootstrap() {
		if (isSubmitting) return;
		isSubmitting = true;
		errorMessage = null;
		try {
			const response = await fetch(`${API_BASE_URL}/api/v1/bootstrap`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					organization_name: workspaceName,
					admin_email: adminEmail
				})
			});
			if (!response.ok) {
				const detail = await readErrorDetail(response);
				errorMessage = `Unable to create workspace.${detail ? ` ${detail}` : ''}`;
				return;
			}
			const data = await response.json();
			authState.login(
				String(data.session_token),
				{ id: data.user_id },
				data.organization_id ?? null
			);
			await goto('/tasks');
		} catch (error_) {
			globalThis.console?.error('Failed to bootstrap workspace', error_);
			errorMessage = 'Unable to reach the server. Check that the API is running and try again.';
		} finally {
			isSubmitting = false;
		}
	}

	function toggleSetup() {
		isSetup = !isSetup;
		errorMessage = null;
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
				<Button class="w-full" onclick={handleBootstrap} disabled={isSubmitting}
					>Sprout Workspace</Button
				>
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
				<Button class="w-full" onclick={handleLogin} disabled={isSubmitting}>Sign In</Button>

				<div class="mt-4 text-center text-sm text-muted-foreground">
					Is this a fresh install?
					<button onclick={toggleSetup} class="text-primary underline-offset-4 hover:underline">
						Start First Seed Journey
					</button>
				</div>
			</CardFooter>
		{/if}
		{#if errorMessage}
			<p role="alert" class="px-6 pb-2 text-center text-sm text-destructive">{errorMessage}</p>
		{/if}
	</Card>
</div>
