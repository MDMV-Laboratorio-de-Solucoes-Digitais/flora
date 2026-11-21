import { goto } from '$app/navigation';
import { authState } from '$lib/state/AuthState.svelte';
import { page } from 'vitest/browser';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';

import LoginPage from './+page.svelte';

vi.mock('$app/navigation', () => ({ goto: vi.fn() }));

function jsonResponse(body: unknown, status: number): Response {
	return Response.json(body, { status });
}

describe('(auth)/+page.svelte — login local em pt-BR', () => {
	beforeEach(() => {
		authState.logout();
		localStorage.clear();
		vi.clearAllMocks();
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('envia e-mail e senha para o endpoint local e guarda a sessão', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				jsonResponse(
					{
						session_token: 'jwt-local',
						expires_in: 3600,
						user_id: 'user-teste',
						next_step: 'select_or_create_organization'
					},
					200
				)
			)
		);

		render(LoginPage);
		await page.getByPlaceholder('voce@exemplo.com').fill('luis.granja@mdmv.com.br');
		await page.getByPlaceholder('Senha').fill('senha-muito-forte-123');
		await page.getByRole('button', { name: 'Entrar' }).click();

		await vi.waitFor(() => expect(authState.token).toBe('jwt-local'));
		expect(authState.isAuthenticated).toBe(true);
		expect(localStorage.getItem('flora.auth.v1')).toContain('jwt-local');
		expect(goto).toHaveBeenCalledWith('/select-org');

		const [url, init] = vi.mocked(fetch).mock.calls[0];
		expect(String(url)).toContain('/api/v1/auth/password/login');
		expect(init?.method).toBe('POST');
		expect(JSON.parse(String(init?.body))).toEqual({
			email: 'luis.granja@mdmv.com.br',
			password: 'senha-muito-forte-123'
		});
	});

	it('mostra mensagem genérica em pt-BR para credenciais inválidas', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					jsonResponse({ error: 'invalid_credentials', message: 'Invalid credentials' }, 401)
				)
		);

		render(LoginPage);
		await page.getByPlaceholder('voce@exemplo.com').fill('luis.granja@mdmv.com.br');
		await page.getByPlaceholder('Senha').fill('senha-errada-123');
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect.element(page.getByText(/e-mail ou senha incorretos/i)).toBeInTheDocument();
		expect(authState.isAuthenticated).toBe(false);
		expect(goto).not.toHaveBeenCalled();
	});

	it('explica a limitação de tentativas em pt-BR', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({ error: 'rate_limited' }, 429)));

		render(LoginPage);
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect.element(page.getByText(/muitas tentativas/i)).toBeInTheDocument();
	});

	it('explica falhas de conexão em pt-BR', async () => {
		vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('offline')));

		render(LoginPage);
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect
			.element(page.getByText(/não foi possível conectar ao servidor/i))
			.toBeInTheDocument();
	});

	it('cria um workspace e envia a senha do administrador', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				jsonResponse(
					{
						session_token: 'jwt-teste',
						expires_in: 3600,
						user_id: 'user-teste',
						organization_id: 'org-teste',
						workspace_id: 'workspace-teste'
					},
					201
				)
			)
		);

		render(LoginPage);
		await page.getByRole('button', { name: 'Fazer primeira configuração' }).click();
		await page.getByPlaceholder('Mínimo de 15 caracteres').fill('SenhaAdministrador@123');
		await page.getByRole('button', { name: 'Criar workspace' }).click();

		await vi.waitFor(() => expect(authState.organizationId).toBe('org-teste'));
		expect(authState.isAuthenticated).toBe(true);
		expect(localStorage.getItem('flora.auth.v1')).toContain('jwt-teste');
		expect(goto).toHaveBeenCalledWith('/tasks');

		const [url, init] = vi.mocked(fetch).mock.calls[0];
		expect(String(url)).toContain('/api/v1/bootstrap');
		expect(init?.method).toBe('POST');
		expect(JSON.parse(String(init?.body))).toEqual({
			organization_name: 'Flora',
			admin_email: 'admin@flora.local',
			admin_password: 'SenhaAdministrador@123'
		});
	});

	it('mostra conflito em pt-BR quando o workspace já existe', async () => {
		vi.stubGlobal(
			'fetch',
			vi
				.fn()
				.mockResolvedValue(
					jsonResponse({ error: 'conflict', message: 'bootstrap already completed' }, 409)
				)
		);

		render(LoginPage);
		await page.getByRole('button', { name: 'Fazer primeira configuração' }).click();
		await page.getByPlaceholder('Mínimo de 15 caracteres').fill('SenhaAdministrador@123');
		await page.getByRole('button', { name: 'Criar workspace' }).click();

		await expect.element(page.getByText(/workspace já foi configurado/i)).toBeInTheDocument();
		expect(authState.isAuthenticated).toBe(false);
		expect(goto).not.toHaveBeenCalled();
	});
});
