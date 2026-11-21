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

const STATUS_PATH = '/api/v1/bootstrap/status';

function statusResponse(isAvailable: boolean): Response {
	return Response.json(
		{ available: isAvailable },
		{ status: 200, headers: { 'cache-control': 'no-store' } }
	);
}

type FetchSpy = { mock: { calls: Array<[unknown, RequestInit?]> } };

function stubFetch(
	handler: (url: string, init?: RequestInit) => Response | Promise<Response>
): FetchSpy {
	const mock = vi.fn((input: RequestInfo | URL, init?: RequestInit) =>
		handler(String(input), init)
	);
	vi.stubGlobal('fetch', mock);
	return mock;
}

function callTo(spy: FetchSpy, path: string, isExact = false): [unknown, RequestInit?] {
	const call = spy.mock.calls.find(([url]) =>
		isExact ? String(url).endsWith(path) : String(url).includes(path)
	);
	if (!call) throw new Error(`fetch não foi chamado para ${path}`);
	return call;
}

function wasStatusFetched(spy: FetchSpy): boolean {
	return spy.mock.calls.some(([url]) => String(url).includes(STATUS_PATH));
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
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(false);
			return jsonResponse(
				{
					session_token: 'jwt-local',
					expires_in: 3600,
					user_id: 'user-teste',
					next_step: 'select_or_create_organization'
				},
				200
			);
		});

		render(LoginPage);
		await page.getByPlaceholder('voce@exemplo.com').fill('luis.granja@mdmv.com.br');
		await page.getByPlaceholder('Senha').fill('senha-muito-forte-123');
		await page.getByRole('button', { name: 'Entrar' }).click();

		await vi.waitFor(() => expect(authState.token).toBe('jwt-local'));
		expect(authState.isAuthenticated).toBe(true);
		expect(localStorage.getItem('flora.auth.v1')).toContain('jwt-local');
		expect(goto).toHaveBeenCalledWith('/select-org');

		const [url, init] = callTo(fetchSpy, '/api/v1/auth/password/login');
		expect(String(url)).toContain('/api/v1/auth/password/login');
		expect(init?.method).toBe('POST');
		expect(JSON.parse(String(init?.body))).toEqual({
			email: 'luis.granja@mdmv.com.br',
			password: 'senha-muito-forte-123'
		});
	});

	it('mostra mensagem genérica em pt-BR para credenciais inválidas', async () => {
		stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(false);
			return jsonResponse({ error: 'invalid_credentials', message: 'Invalid credentials' }, 401);
		});

		render(LoginPage);
		await page.getByPlaceholder('voce@exemplo.com').fill('luis.granja@mdmv.com.br');
		await page.getByPlaceholder('Senha').fill('senha-errada-123');
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect.element(page.getByText(/e-mail ou senha incorretos/i)).toBeInTheDocument();
		expect(authState.isAuthenticated).toBe(false);
		expect(goto).not.toHaveBeenCalled();
	});

	it('explica a limitação de tentativas em pt-BR', async () => {
		stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(false);
			return jsonResponse({ error: 'rate_limited' }, 429);
		});

		render(LoginPage);
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect.element(page.getByText(/muitas tentativas/i)).toBeInTheDocument();
	});

	it('explica falhas de conexão em pt-BR', async () => {
		stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(false);
			throw new Error('offline');
		});

		render(LoginPage);
		await page.getByRole('button', { name: 'Entrar' }).click();

		await expect
			.element(page.getByText(/não foi possível conectar ao servidor/i))
			.toBeInTheDocument();
	});

	it('cria um workspace e envia a senha do administrador', async () => {
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(true);
			return jsonResponse(
				{
					session_token: 'jwt-teste',
					expires_in: 3600,
					user_id: 'user-teste',
					organization_id: 'org-teste',
					workspace_id: 'workspace-teste'
				},
				201
			);
		});

		render(LoginPage);
		await page.getByRole('button', { name: 'Fazer primeira configuração' }).click();
		await page.getByPlaceholder('Mínimo de 15 caracteres').fill('SenhaAdministrador@123');
		await page.getByRole('button', { name: 'Criar workspace' }).click();

		await vi.waitFor(() => expect(authState.organizationId).toBe('org-teste'));
		expect(authState.isAuthenticated).toBe(true);
		expect(localStorage.getItem('flora.auth.v1')).toContain('jwt-teste');
		expect(goto).toHaveBeenCalledWith('/tasks');

		const [url, init] = callTo(fetchSpy, '/api/v1/bootstrap', true);
		expect(String(url)).toContain('/api/v1/bootstrap');
		expect(init?.method).toBe('POST');
		expect(JSON.parse(String(init?.body))).toEqual({
			organization_name: 'Flora',
			admin_email: 'admin@flora.local',
			admin_password: 'SenhaAdministrador@123'
		});
	});

	it('mostra conflito em pt-BR quando o workspace já existe', async () => {
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(true);
			return jsonResponse({ error: 'conflict', message: 'bootstrap already completed' }, 409);
		});

		render(LoginPage);
		await page.getByRole('button', { name: 'Fazer primeira configuração' }).click();
		await page.getByPlaceholder('Mínimo de 15 caracteres').fill('SenhaAdministrador@123');
		await page.getByRole('button', { name: 'Criar workspace' }).click();

		await expect.element(page.getByText(/workspace já foi configurado/i)).toBeInTheDocument();
		await expect
			.element(page.getByRole('button', { name: 'Fazer primeira configuração' }))
			.not.toBeInTheDocument();
		expect(authState.isAuthenticated).toBe(false);
		expect(goto).not.toHaveBeenCalled();
		expect(wasStatusFetched(fetchSpy)).toBe(true);
	});

	it('mostra o link de primeira configuração somente quando a instalação está vazia', async () => {
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(true);
			return jsonResponse({}, 200);
		});

		render(LoginPage);

		await expect
			.element(page.getByRole('button', { name: 'Fazer primeira configuração' }))
			.toBeInTheDocument();
		await vi.waitFor(() => expect(wasStatusFetched(fetchSpy)).toBe(true));
		const [url, init] = callTo(fetchSpy, STATUS_PATH);
		expect(String(url)).toContain(STATUS_PATH);
		expect(init?.method ?? 'GET').toBe('GET');
		expect(init?.headers).toBeUndefined();
	});

	it('oculta o link quando a instalação já foi configurada', async () => {
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) return statusResponse(false);
			return jsonResponse({}, 200);
		});

		render(LoginPage);

		await vi.waitFor(() => expect(wasStatusFetched(fetchSpy)).toBe(true));
		await expect
			.element(page.getByRole('button', { name: 'Fazer primeira configuração' }))
			.not.toBeInTheDocument();
	});

	it.each([404, 405, 503])(
		'oculta o link quando o status responde %i (fail closed)',
		async (status) => {
			const fetchSpy = stubFetch((url) => {
				if (url.includes(STATUS_PATH)) return jsonResponse({ error: 'unavailable' }, status);
				return jsonResponse({}, 200);
			});

			render(LoginPage);

			await vi.waitFor(() => expect(wasStatusFetched(fetchSpy)).toBe(true));
			await expect
				.element(page.getByRole('button', { name: 'Fazer primeira configuração' }))
				.not.toBeInTheDocument();
		}
	);

	it('oculta o link quando a consulta de status falha na rede', async () => {
		const fetchSpy = stubFetch((url) => {
			if (url.includes(STATUS_PATH)) throw new Error('offline');
			return jsonResponse({}, 200);
		});

		render(LoginPage);

		await vi.waitFor(() => expect(wasStatusFetched(fetchSpy)).toBe(true));
		await expect
			.element(page.getByRole('button', { name: 'Fazer primeira configuração' }))
			.not.toBeInTheDocument();
	});
});
