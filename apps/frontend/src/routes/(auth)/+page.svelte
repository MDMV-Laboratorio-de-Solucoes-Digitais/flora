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

	let isSetup = $state(false);
	let workspaceName = $state('Flora');
	let adminEmail = $state('admin@flora.local');
	let adminPassword = $state('');
	let email = $state('');
	let password = $state('');
	let errorMessage = $state<string | null>(null);
	let isSubmitting = $state(false);

	type ErrorBody = { error?: unknown; field?: unknown };

	async function readError(response: Response): Promise<ErrorBody> {
		try {
			const body: unknown = await response.json();
			return body && typeof body === 'object' ? (body as ErrorBody) : {};
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
				const body = await readError(response);
				switch (response.status) {
					case 401: {
						errorMessage = 'E-mail ou senha incorretos.';
						break;
					}
					case 429: {
						errorMessage = 'Muitas tentativas. Aguarde um momento e tente novamente.';
						break;
					}
					case 422: {
						errorMessage = 'Verifique os dados informados.';
						break;
					}
					default: {
						errorMessage =
							body.error === 'oidc_error'
								? 'O provedor de identidade não está disponível no momento.'
								: 'Não foi possível entrar. Tente novamente.';
					}
				}
				return;
			}
			const data = await response.json();
			authState.login(
				String(data.session_token),
				{ id: data.user_id },
				data.organization_id ?? null
			);
			await goto(
				data.next_step === 'select_or_create_organization' ? '/select-org' : '/tasks'
			);
		} catch (error_) {
			globalThis.console?.error('Falha ao entrar', error_);
			errorMessage = 'Não foi possível conectar ao servidor. Verifique se a API está em execução e tente novamente.';
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
				await readError(response);
				errorMessage =
					response.status === 429
						? 'Muitas tentativas. Aguarde um momento e tente novamente.'
						: 'Não foi possível iniciar o login com SSO.';
				return;
			}
			const data = await response.json();
			if (!data.authorization_url) {
				errorMessage = 'O provedor de identidade não retornou uma URL de autorização.';
				return;
			}
			if (typeof sessionStorage !== 'undefined' && data.state) {
				sessionStorage.setItem('flora.oauth.state', String(data.state));
				sessionStorage.setItem('flora.oauth.redirect_uri', redirectUri);
			}
			location.assign(String(data.authorization_url));
		} catch (error_) {
			globalThis.console?.error('Falha ao iniciar SSO', error_);
			errorMessage = 'Não foi possível conectar ao servidor. Verifique se a API está em execução e tente novamente.';
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
				errorMessage =
					response.status === 409
						? 'Este workspace já foi configurado.'
						: response.status === 422 && body.field === 'admin_password'
							? 'A senha do administrador deve atender aos requisitos de complexidade (15 a 1024 caracteres).'
							: 'Não foi possível criar o workspace. Verifique os dados e tente novamente.';
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
			errorMessage = 'Não foi possível conectar ao servidor. Verifique se a API está em execução e tente novamente.';
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
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">Primeira configuração</CardTitle>
				<CardDescription>Plante as raízes do seu novo workspace Flora.</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="workspaceName">Nome do workspace</Label>
					<Input id="workspaceName" bind:value={workspaceName} placeholder="Minha equipe" />
				</div>
				<div class="grid gap-2">
					<Label for="adminEmail">E-mail do administrador</Label>
					<Input
						id="adminEmail"
						type="email"
						bind:value={adminEmail}
						placeholder="admin@exemplo.com"
					/>
				</div>
				<div class="grid gap-2">
					<Label for="adminPassword">Senha do administrador</Label>
					<Input
						id="adminPassword"
						type="password"
						bind:value={adminPassword}
						minlength={15}
						placeholder="Mínimo de 15 caracteres"
					/>
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleBootstrap} disabled={isSubmitting}
					>Criar workspace</Button
				>
				<Button variant="ghost" class="w-full" onclick={toggleSetup}>Voltar para o login</Button>
			</CardFooter>
		{:else}
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl font-bold tracking-tight">Bem-vindo ao Flora</CardTitle>
				<CardDescription>Entre com seu e-mail e senha para acessar seu workspace.</CardDescription>
			</CardHeader>
			<CardContent class="grid gap-4">
				<div class="grid gap-2">
					<Label for="loginEmail">E-mail</Label>
					<Input id="loginEmail" type="email" bind:value={email} placeholder="voce@exemplo.com" />
				</div>
				<div class="grid gap-2">
					<Label for="loginPassword">Senha</Label>
					<Input id="loginPassword" type="password" bind:value={password} placeholder="Senha" />
				</div>
			</CardContent>
			<CardFooter class="flex flex-col gap-2">
				<Button class="w-full" onclick={handleLogin} disabled={isSubmitting}>Entrar</Button>
				<Button variant="outline" class="w-full" onclick={handleSso} disabled={isSubmitting}
					>Entrar com SSO</Button
				>
				<div class="mt-4 text-center text-sm text-muted-foreground">
					É uma instalação nova?
					<button onclick={toggleSetup} class="text-primary underline-offset-4 hover:underline">
						Fazer primeira configuração
					</button>
				</div>
			</CardFooter>
		{/if}
		{#if errorMessage}
			<p role="alert" class="px-6 pb-2 text-center text-sm text-destructive">{errorMessage}</p>
		{/if}
	</Card>
</div>
