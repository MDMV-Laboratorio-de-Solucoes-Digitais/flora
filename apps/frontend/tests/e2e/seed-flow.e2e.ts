import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import type { Page } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { mkdirSync } from 'node:fs';
import path from 'node:path';
import { format, t } from '../../src/lib/i18n';
import { WCAG_22_AA_TAGS, summarizeWcagAudit } from '../../src/lib/utils/a11yAudit';

const MAIN_WORKSPACE = '88888888-8888-8888-8888-888888888888';
const ALICE_ID = '22222222-2222-2222-2222-222222222222';
const ACME_ID = '11111111-1111-1111-1111-111111111111';
const SCREENSHOT_DIR = process.env.E2E_SCREENSHOT_DIR?.trim() ?? '';

function runSql(sql: string): void {
	execFileSync(
		'psql',
		['-v', 'ON_ERROR_STOP=1', '-h', '127.0.0.1', '-U', 'postgres', '-d', 'flora', '-c', sql],
		{ env: { ...process.env, PGPASSWORD: process.env.PGPASSWORD ?? 'postgres' }, stdio: 'pipe' }
	);
}

async function capture(page: Page, filename: string): Promise<void> {
	if (!SCREENSHOT_DIR) return;
	mkdirSync(SCREENSHOT_DIR, { recursive: true });
	await page.screenshot({ path: path.join(SCREENSHOT_DIR, filename), fullPage: true });
}

/** Auditoria WCAG 2.2 AA: registra resumo no log/anotação e falha em caso de violações. */
async function auditAccessibility(page: Page, route: string): Promise<void> {
	const results = await new AxeBuilder({ page }).withTags([...WCAG_22_AA_TAGS]).analyze();
	const { findings, summary } = summarizeWcagAudit(route, results.violations);
	process.stdout.write(`${summary}\n`);
	test.info().annotations.push({ type: 'wcag-axe', description: summary });
	expect(findings, summary).toEqual([]);
}

test('smoke: select-org -> workspace/channel -> message -> task', async ({ page }) => {
	test.setTimeout(180_000);
	const suffix = crypto.randomUUID().slice(0, 8);
	runSql(`DELETE FROM notifications WHERE user_id = '${ALICE_ID}'::uuid;`);

	// 1. select-org proves the auth state was read on the frontend origin with a
	//    nil organization. This rotates the injected jti — never reuse that token.
	await page.goto('/select-org');
	await expect(page.getByRole('heading', { name: t['org.selectTitle'] })).toBeVisible();
	await expect(page.getByText('Acme Corporation')).toBeVisible();
	await expect(page.getByText('acme-corp')).toBeVisible();
	await capture(page, '01-select-org.png');

	// Scope to the Acme row: the seed has more than one organization, so a bare
	// getByRole('button', { name: 'Selecionar' }) is ambiguous in strict mode.
	await page
		.getByText('Acme Corporation')
		.locator('xpath=../..')
		.getByRole('button', { name: t['org.select'] })
		.click();
	await expect(page).toHaveURL(/\/tasks$/);
	await expect(page.getByRole('heading', { name: t['tasks.title'], level: 1 })).toBeVisible();

	await auditAccessibility(page, '/tasks');

	// 2. workspace/channel — one selection only (the jti was rotated above).
	await page.goto('/channels');
	await expect(page.getByRole('heading', { name: t['channels.title'], level: 1 })).toBeVisible();
	await page.getByLabel(t['channels.workspace']).selectOption(MAIN_WORKSPACE);

	await auditAccessibility(page, '/channels');
	await expect(page.getByRole('button', { name: '# general' })).toBeVisible();
	await page.getByRole('button', { name: '# general' }).click();
	await expect(page.getByRole('heading', { name: '# general', level: 2 })).toBeVisible();

	// 3. message — REST upsert is sufficient; WS with a lean backend is NV.
	await expect(page.getByText(/Welcome to Acme Corporation/)).toBeVisible();
	await expect(page.getByText('Alice Admin', { exact: true }).first()).toBeVisible();
	await expect(page.getByText('Bob Builder', { exact: true }).first()).toBeVisible();
	await expect(page.getByText('Usuário 22222222')).toHaveCount(0);
	const message = `e2e smoke ${suffix}`;
	await page.locator('textarea').fill(message);
	await page.getByRole('button', { name: t['chat.send'] }).click();
	await expect(page.getByText(message)).toBeVisible();
	await expect(page.getByText('Usuário 22222222')).toHaveCount(0);
	await capture(page, '03-channels-chat.png');

	// 4. task — "Assign to me" stays checked so the created task is the only one
	//    in Alice's assignee-scoped list. This is the single select-org consumer.
	await page.goto('/tasks');
	const create = page.getByRole('button', { name: t['tasks.create'] });
	await expect(create).toBeDisabled();
	const title = `e2e task ${suffix}`;
	const description = `e2e description ${suffix}`;
	await page.getByLabel(t['tasks.titleLabel']).fill(title);
	await expect(create).toBeEnabled();
	await page.getByLabel(t['tasks.descriptionLabel']).fill(description);
	await create.click();
	await expect(page.getByText(format('tasks.created', { title }))).toBeVisible();
	const todo = page.getByRole('region', { name: `${t['kanban.column']} ${t['kanban.todo']}` });
	await expect(todo.getByText(title, { exact: true })).toBeVisible();
	await expect(todo.getByText(description, { exact: true })).toBeVisible();
	await capture(page, '02-tasks-board.png');

	// 5. files — reuse rotated session to verify /files GET resolution, empty state, and WCAG 2.2 AA compliance.
	const filesListResponsePromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/files?workspace_id=') &&
			response.request().method() === 'GET'
	);
	await page.goto('/files');
	const filesListResponse = await filesListResponsePromise;
	expect(filesListResponse.status()).toBe(200);
	await expect(page.getByRole('heading', { name: t['files.title'], level: 1 })).toBeVisible();
	await expect(page.getByLabel(t['files.workspace'])).toBeEnabled();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await expect(page.getByText(t['files.empty'])).toBeVisible();
	await auditAccessibility(page, '/files');
	await capture(page, '04-files-workspace.png');

	// 6. notifications — honest navigation (no /dms sidebar link or fake AI drawer) and org-scoped /notifications.
	await expect(page.getByRole('link', { name: t['nav.directMessages'] })).toHaveCount(0);
	await expect(page.getByRole('button', { name: t['nav.openAiPanel'] })).toHaveCount(0);

	const notificationsListPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/notifications') && response.request().method() === 'GET'
	);
	await page.getByRole('link', { name: t['nav.notifications'] }).click();
	const notificationsListResponse = await notificationsListPromise;
	expect(notificationsListResponse.status()).toBe(200);
	await expect(page).toHaveURL(/\/notifications$/);
	await expect(
		page.getByRole('heading', { name: t['notifications.title'], level: 1 })
	).toBeVisible();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await expect(page.getByText(t['notifications.empty'])).toBeVisible();
	await auditAccessibility(page, '/notifications');
	await capture(page, '05-notifications-empty.png');

	// 6b. notifications populated — unread badge WCAG 2.5.3 regression guard and read-all transition.
	const targetMention = crypto.randomUUID();
	const targetAssignment = crypto.randomUUID();
	runSql(`
		INSERT INTO notifications (organization_id, user_id, event_type, target_id, target_url, is_read)
		VALUES
			('${ACME_ID}'::uuid, '${ALICE_ID}'::uuid, 'Mention', '${targetMention}'::uuid, '/channels', false),
			('${ACME_ID}'::uuid, '${ALICE_ID}'::uuid, 'Assignment', '${targetAssignment}'::uuid, '/tasks', false);
	`);

	const refreshPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/notifications') && response.request().method() === 'GET'
	);
	await page.getByRole('button', { name: t['notifications.refresh'] }).click();
	const refreshResponse = await refreshPromise;
	expect(refreshResponse.status()).toBe(200);
	await expect(page.getByText(t['notifications.type.Mention'], { exact: true })).toBeVisible();
	await expect(page.getByText(t['notifications.type.Assignment'], { exact: true })).toBeVisible();
	await expect(
		page.getByRole('link', {
			name: `${t['nav.notifications']} 2 ${t['notifications.filterUnread'].toLowerCase()}`
		})
	).toBeVisible();
	await auditAccessibility(page, '/notifications (unread)');
	await capture(page, '06-notifications-unread-and-badge.png');

	const readAllPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/notifications/read-all') &&
			response.request().method() === 'POST'
	);
	await page.getByRole('button', { name: t['notifications.markAllRead'] }).click();
	const readAllResponse = await readAllPromise;
	expect(readAllResponse.status()).toBe(204);
	await expect(page.getByText(t['notifications.read'], { exact: true })).toHaveCount(2);
	await expect(page.getByText(t['notifications.unread'], { exact: true })).toHaveCount(0);
	await expect(page.getByRole('button', { name: t['notifications.markAllRead'] })).toBeDisabled();
	await expect(page.getByRole('link', { name: t['nav.notifications'], exact: true })).toBeVisible();
	await capture(page, '07-notifications-after-read-all.png');

	// 7. dms — direct navigation to /dms shows an honest unavailable message without hardcoded channel errors.
	await page.goto('/dms');
	await expect(page.getByRole('heading', { name: t['dms.title'], level: 1 })).toBeVisible();
	await expect(page.getByText(t['dms.unavailableDescription'])).toBeVisible();
	await expect(page.getByRole('link', { name: t['dms.goToChannels'] })).toBeVisible();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await auditAccessibility(page, '/dms');
	await capture(page, '08-dms-honest-unavailable.png');
});
