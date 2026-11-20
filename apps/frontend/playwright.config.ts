import { defineConfig, devices } from '@playwright/test';

const FRONTEND_ORIGIN = process.env.E2E_FRONTEND_ORIGIN ?? 'http://127.0.0.1:4173';
const API_ORIGIN = process.env.E2E_API_ORIGIN ?? 'http://127.0.0.1:3000';
const AUTH_FILE = 'playwright/.auth/alice.json';

export default defineConfig({
	testDir: './tests/e2e',
	testMatch: '**/*.e2e.ts',
	globalSetup: './playwright/global-setup.ts',
	forbidOnly: !!process.env.CI,
	// `select-organization` revokes the injected jti and rotates the session, so a
	// retry would reuse a revoked token. Keep this smoke strictly one-shot.
	retries: 0,
	workers: 1,
	globalTimeout: 10 * 60 * 1000,
	reporter: [['html', { open: 'never' }], ['list']],
	use: { baseURL: FRONTEND_ORIGIN, trace: 'on-first-retry' },
	projects: [
		{ name: 'setup', testMatch: /.*\.setup\.ts/ },
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'], storageState: AUTH_FILE },
			dependencies: ['setup']
		}
	],
	webServer: [
		{
			name: 'api',
			command: 'cargo run -q -p flora-api',
			cwd: '../..',
			url: `${API_ORIGIN}/health`,
			timeout: 120_000,
			reuseExistingServer: !process.env.CI
		},
		{
			name: 'web',
			command: 'npm run build && npm run preview -- --host 127.0.0.1 --port 4173',
			url: FRONTEND_ORIGIN,
			timeout: 120_000,
			reuseExistingServer: !process.env.CI
		}
	]
});
