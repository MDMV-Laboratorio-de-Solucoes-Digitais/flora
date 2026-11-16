import { expect, request, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

const REPO_ROOT = path.resolve(import.meta.dirname, '../../../..');
const AUTH_FILE = path.resolve(import.meta.dirname, '../../playwright/.auth/alice.json');
const API_ORIGIN = process.env.E2E_API_ORIGIN ?? 'http://127.0.0.1:3000';
const FRONTEND_ORIGIN = process.env.E2E_FRONTEND_ORIGIN ?? 'http://127.0.0.1:4173';
const ACME_ID = '11111111-1111-1111-1111-111111111111';

test('seed database and mint browser auth state', async () => {
	// The gated minter may need a first-time build of its optional deps on CI.
	test.setTimeout(180_000);
	// The API webServer applies migrations on boot; seed reference data and the
	// pre-selection session with the documented compose credentials.
	for (const sql of ['dev-seed.sql', 'scripts/dev-smoke-session.sql']) {
		execFileSync(
			'psql',
			[
				'-v',
				'ON_ERROR_STOP=1',
				'-h',
				'127.0.0.1',
				'-U',
				'postgres',
				'-d',
				'flora',
				'-f',
				path.resolve(REPO_ROOT, sql)
			],
			{ env: { ...process.env, PGPASSWORD: process.env.PGPASSWORD ?? 'postgres' }, stdio: 'pipe' }
		);
	}

	execFileSync(
		'cargo',
		[
			'run',
			'-q',
			'-p',
			'flora-core',
			'--bin',
			'flora-mint-session',
			'--features',
			'dev-mint',
			'--',
			'--storage-state',
			AUTH_FILE,
			'--frontend-origin',
			FRONTEND_ORIGIN
		],
		{ cwd: REPO_ROOT, stdio: 'pipe' }
	);

	// Positive drift guard: wrong secret, expired token or a missing session row
	// must fail setup before the browser smoke starts.
	const stored = JSON.parse(readFileSync(AUTH_FILE, 'utf8')) as {
		origins: Array<{ localStorage: Array<{ name: string; value: string }> }>;
	};
	const raw = stored.origins[0]?.localStorage.find((item) => item.name === 'flora.auth.v1')?.value;
	expect(raw, 'storageState must contain flora.auth.v1').toBeTruthy();
	const token = (JSON.parse(raw as string) as { token: string }).token;
	const api = await request.newContext();
	const response = await api.get(`${API_ORIGIN}/api/v1/organizations`, {
		headers: { Authorization: `Bearer ${token}` }
	});
	expect(response.status(), 'minted token must be accepted by the API').toBe(200);
	const orgs = (await response.json()) as Array<{ id: string }>;
	expect(orgs).toEqual(expect.arrayContaining([expect.objectContaining({ id: ACME_ID })]));
	await api.dispose();
});
