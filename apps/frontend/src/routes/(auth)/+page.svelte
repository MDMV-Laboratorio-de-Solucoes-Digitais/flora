<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
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
	import { backendErrorMessage, t, type BackendErrorBody } from '$lib/i18n';

	let canBootstrap = $state(false);
	let workspaceName = $state('Flora');
	let adminEmail = $state('admin@flora.local');
	let adminPassword = $state('');
	let email = $state('');
	let password = $state('');
	let errorMessage = $state<string | null>(null);
	let isSubmitting = $state(false);

	onMount(() => {
		void (async () => {
			try {
				const response = await fetch(`${API_BASE_URL}/api/v1/bootstrap/status`);
				if (!response.ok) {
					canBootstrap = false;
					return;
				}
				const data: unknown = await response.json();
				canBootstrap =
					typeof data === 'object' &&
					data !== null &&
					(data as { available?: unknown }).available === true;
			} catch (error_) {
				globalThis.console?.error('Falha ao consultar status da instalação', error_);
				canBootstrap = false;
			}
		})();
	});

	async function readError(response: Response): Promise<BackendErrorBody> {
		try {
			const body: unknown = await response.json();
			return body && typeof body === 'object' ? (body as BackendErrorBody) : {};
		} catch {
			return {};
		}
	}

	async function handleLogin() {
		if (isSubmitting) return;
		isSubmitting = true;
		errorMessage = null;
		try {
			const response = await fetch(`${API_BASE_URL}/api/v1/auth/password/login`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email, password })
			});
			if (!response.ok) {
				errorMessage = backendErrorMessage(
					await readError(response),
					response.status,
					'error.loginFailed'
				);
				return;
			}
			const data = await response.json();
			authState.login(
				String(data.session_token),
				{ id: data.user_id },
				data.organization_id ?? null
			);
			await goto(data.next_step === 'select_or_create_organization' ? '/select-org' : '/tasks');
		} catch (error_) {
			globalThis.console?.error('Falha ao entrar', error_);
			errorMessage = t['error.serverUnavailable'];
		} finally {
			isSubmitting = false;
		}
	}

	async function handleSso() {
		if (isSubmitting) return;
		isSubmitting = true;
		errorMessage = null;
		try {
			const redirectUri = `${location.origin}/callback`;
			const response = await fetch(
				`${API_BASE_URL}/api/v1/auth/login?redirect_uri=${encodeURIComponent(redirectUri)}`
			);
			if (!response.ok) {
				const body = await readError(response);
				errorMessage = backendErrorMessage(body, response.status, 'error.ssoFailed');
				return;
			}
			const data = await response.json();
			if (!data.authorization_url) {
				errorMessage = t['error.oidcNoAuthorizationUrl'];
				return;
			}
			if (typeof sessionStorage !== 'undefined' && data.state) {
				sessionStorage.setItem('flora.oauth.state', String(data.state));
				sessionStorage.setItem('flora.oauth.redirect_uri', redirectUri);
			}
			location.assign(String(data.authorization_url));
		} catch (error_) {
			globalThis.console?.error('Falha ao iniciar SSO', error_);
			errorMessage = t['error.serverUnavailable'];
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
					admin_email: adminEmail,
					admin_password: adminPassword
				})
			});
			if (!response.ok) {
				const body = await readError(response);
				if (response.status === 409) canBootstrap = false;
				errorMessage = backendErrorMessage(body, response.status, 'error.bootstrapFailed');
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
			globalThis.console?.error('Falha ao criar workspace', error_);
			errorMessage = t['error.serverUnavailable'];
		} finally {
			isSubmitting = false;
		}
	}

	function toggleSetup() {
		canBootstrap = !canBootstrap;
		errorMessage = null;
	}
</script>

<div class="flex h-screen items-center justify-center bg-muted/40 p-4">
	<Card class="w-full max-w-md shadow-lg">
		{#if canBootstrap}
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">{t['auth.setup.title']}</CardTitle>
				<CardDescription>{t['auth.setup.description']}</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="workspaceName">{t['auth.setup.workspaceName']}</Label>
					<Input
						id="workspaceName"
						bind:value={workspaceName}
						placeholder={t['auth.setup.workspacePlaceholder']}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="adminEmail">{t['auth.setup.adminEmail']}</Label>
					<Input
						id="adminEmail"
						type="email"
						bind:value={adminEmail}
						placeholder={t['auth.setup.adminEmailPlaceholder']}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="adminPassword">{t['auth.setup.adminPassword']}</Label>
					<Input
						id="adminPassword"
						type="password"
						bind:value={adminPassword}
						minlength={15}
						placeholder={t['auth.setup.adminPasswordPlaceholder']}
					/>
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleBootstrap} disabled={isSubmitting}
					>{t['auth.setup.submit']}</Button
				>
				<Button variant="ghost" class="w-full" onclick={toggleSetup}
					>{t['auth.setup.backToLogin']}</Button
				>
			</CardFooter>
		{:else}
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">{t['auth.login.title']}</CardTitle>
				<CardDescription>{t['auth.login.description']}</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="loginEmail">{t['auth.login.email']}</Label>
					<Input
						id="loginEmail"
						type="email"
						bind:value={email}
						placeholder={t['auth.login.emailPlaceholder']}
					/>
				</div>
				<div class="grid gap-2">
					<Label for="loginPassword">{t['auth.login.password']}</Label>
					<Input
						id="loginPassword"
						type="password"
						bind:value={password}
						placeholder={t['auth.login.passwordPlaceholder']}
					/>
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleLogin} disabled={isSubmitting}
					>{t['auth.login.submit']}</Button
				>
				<Button variant="outline" class="w-full" onclick={handleSso} disabled={isSubmitting}
					>{t['auth.login.sso']}</Button
				>
				{#if canBootstrap}
					<div class="mt-4 text-center text-sm text-muted-foreground">
						{t['auth.login.newInstall']}
						<button onclick={toggleSetup} class="text-primary underline-offset-4 hover:underline">
							{t['auth.login.firstSetup']}
						</button>
					</div>
				{/if}
			</CardFooter>
		{/if}
		{#if errorMessage}
			<p role="alert" class="px-6 pb-2 text-center text-sm text-destructive">{errorMessage}</p>
		{/if}
	</Card>
</div>
