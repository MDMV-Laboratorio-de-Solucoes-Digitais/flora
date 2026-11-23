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

async function settleAnimations(page: Page): Promise<void> {
	await page.evaluate(async () => {
		await Promise.allSettled(
			document.getAnimations({ subtree: true }).map((animation) => animation.finished)
		);
		await new Promise<void>((resolve) =>
			requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
		);
	});
}

async function enableDarkMode(page: Page): Promise<void> {
	await page.emulateMedia({ colorScheme: 'dark' });
	await page.evaluate(() => {
		try {
			localStorage.setItem('flora.theme', 'dark');
		} catch {
			// Ignore storage access errors in restricted contexts.
		}
		document.documentElement.classList.add('dark');
	});
	await settleAnimations(page);
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
	runSql(
		`DELETE FROM notifications WHERE user_id = '${ALICE_ID}'::uuid; DELETE FROM files WHERE organization_id = '${ACME_ID}'::uuid;`
	);

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

	// 5. files — verify initial GET empty state, real multipart upload, and persistence across page.reload().
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

	// 5b. upload real file and reload page to confirm DB-backed persistence in UI.
	const uploadedFileName = 'relatorio-ciclo-117.txt';
	const uploadPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/files/upload') && response.request().method() === 'POST'
	);
	await page.getByLabel(t['file.selectInput']).setInputFiles({
		name: uploadedFileName,
		mimeType: 'text/plain',
		buffer: Buffer.from('evidencia e2e flora ciclo 117', 'utf8')
	});
	const uploadResponse = await uploadPromise;
	expect(uploadResponse.status()).toBe(201);
	await expect(page.getByText(format('files.uploaded', { name: uploadedFileName }))).toBeVisible();
	await expect(page.getByText(uploadedFileName, { exact: true })).toBeVisible();
	await capture(page, '05-files-uploaded.png');

	const reloadFilesPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/files?workspace_id=') &&
			response.request().method() === 'GET'
	);
	await page.reload();
	const reloadFilesResponse = await reloadFilesPromise;
	expect(reloadFilesResponse.status()).toBe(200);
	await expect(page.getByText(uploadedFileName, { exact: true })).toBeVisible();
	await expect(page.getByText(t['files.empty'])).toHaveCount(0);
	await auditAccessibility(page, '/files (populated after reload)');
	await capture(page, '06-files-persisted-after-reload.png');

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
	await capture(page, '07-notifications-empty.png');

	// 6b. notifications populated — 2 unread items, plural sidebar badge, and WCAG 2.2 AA audit.
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
	await page.getByRole('button', { name: t['notifications.refresh'], exact: true }).click();
	const refreshResponse = await refreshPromise;
	expect(refreshResponse.status()).toBe(200);
	await expect(page.getByText(t['notifications.type.Mention'], { exact: true })).toBeVisible();
	await expect(page.getByText(t['notifications.type.Assignment'], { exact: true })).toBeVisible();
	await expect(
		page.getByRole('link', {
			name: `${t['nav.notifications']} 2 ${t['notifications.unreadPlural']}`,
			exact: true
		})
	).toBeVisible();
	await auditAccessibility(page, '/notifications (2 unread)');
	await capture(page, '08-notifications-2-unread.png');

	// 6c. single mark-as-read (POST /api/v1/notifications/{id}/read) + singular "1 não lida" verification.
	const markSinglePromise = page.waitForResponse(
		(response) =>
			/\/api\/v1\/notifications\/[^/]+\/read$/.test(new URL(response.url()).pathname) &&
			response.request().method() === 'POST'
	);
	await page
		.getByRole('button', { name: t['notifications.markRead'], exact: true })
		.first()
		.click();
	const markSingleResponse = await markSinglePromise;
	expect(markSingleResponse.status()).toBe(204);
	await expect(page.getByText(t['notifications.read'], { exact: true })).toHaveCount(1);
	await expect(page.getByText(t['notifications.unread'], { exact: true })).toHaveCount(1);
	await expect(
		page.getByRole('link', {
			name: `${t['nav.notifications']} 1 ${t['notifications.unreadSingular']}`,
			exact: true
		})
	).toBeVisible();
	await auditAccessibility(page, '/notifications (1 unread singular)');
	await capture(page, '09-notifications-1-unread-singular.png');

	// 6d. filter "Não lidas", bulk "Marcar todas como lidas", empty unread state, and switch to "Todas".
	await page.getByRole('button', { name: t['notifications.filterUnread'], exact: true }).click();
	await expect(page.getByText(t['notifications.unread'], { exact: true })).toHaveCount(1);
	await expect(page.getByText(t['notifications.read'], { exact: true })).toHaveCount(0);
	await capture(page, '10-notifications-filter-unread.png');

	const readAllPromise = page.waitForResponse(
		(response) =>
			response.url().includes('/api/v1/notifications/read-all') &&
			response.request().method() === 'POST'
	);
	await page.getByRole('button', { name: t['notifications.markAllRead'] }).click();
	const readAllResponse = await readAllPromise;
	expect(readAllResponse.status()).toBe(204);
	await expect(page.getByText(t['notifications.emptyUnread'])).toBeVisible();
	await expect(page.getByRole('button', { name: t['notifications.markAllRead'] })).toBeDisabled();
	await expect(page.getByRole('link', { name: t['nav.notifications'], exact: true })).toBeVisible();
	await capture(page, '11-notifications-empty-unread-filter.png');

	await page.getByRole('button', { name: t['notifications.filterAll'], exact: true }).click();
	await expect(page.getByText(t['notifications.read'], { exact: true })).toHaveCount(2);
	await expect(page.getByText(t['notifications.unread'], { exact: true })).toHaveCount(0);
	await capture(page, '12-notifications-all-read.png');

	// 6e. click "Abrir destino" safely navigates to /channels or /tasks, then return to /notifications.
	await page.getByRole('link', { name: t['notifications.openTarget'] }).first().click();
	await expect(page).toHaveURL(/\/(channels|tasks)$/);
	await page.goto('/notifications');
	await expect(page.getByText(t['notifications.read'], { exact: true })).toHaveCount(2);

	// 6f. collapsed sidebar (isSidebarOpen === false) accessible icon-only navigation + WCAG audit.
	await page.getByRole('button', { name: t['nav.closeSidebar'], exact: true }).click();
	await settleAnimations(page);
	await expect(page.getByRole('button', { name: t['nav.openSidebar'], exact: true })).toBeVisible();
	await expect(page.getByRole('link', { name: t['nav.channels'], exact: true })).toBeVisible();
	await expect(page.getByRole('link', { name: t['nav.tasks'], exact: true })).toBeVisible();
	await expect(page.getByRole('link', { name: t['nav.files'], exact: true })).toBeVisible();
	await expect(page.getByRole('link', { name: t['nav.notifications'], exact: true })).toBeVisible();
	await expect(page.getByRole('button', { name: t['nav.myAccount'], exact: true })).toBeVisible();
	await auditAccessibility(page, '/notifications (sidebar collapsed)');
	await capture(page, '13-sidebar-collapsed.png');
	await page.getByRole('button', { name: t['nav.openSidebar'], exact: true }).click();
	await settleAnimations(page);

	// 7. dms — direct navigation to /dms shows an honest unavailable message without hardcoded channel errors.
	await page.goto('/dms');
	await expect(page.getByRole('heading', { name: t['dms.title'], level: 1 })).toBeVisible();
	await expect(page.getByText(t['dms.unavailableDescription'])).toBeVisible();
	await expect(page.getByRole('link', { name: t['dms.goToChannels'] })).toBeVisible();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await auditAccessibility(page, '/dms');
	await capture(page, '14-dms-honest-unavailable.png');

	// 8. dark mode (.dark) — persistent dark theme across navigations, unread cards, and WCAG 2.2 AA audits.
	await enableDarkMode(page);
	await expect
		.poll(() => page.evaluate(() => document.documentElement.classList.contains('dark')))
		.toBe(true);
	await auditAccessibility(page, '/dms (dark mode)');
	await capture(page, '15-dms-dark-mode.png');

	// Re-arm one unread notification so .dark audits both unread styling and collapsed unread aria-label.
	runSql(
		`UPDATE notifications SET is_read = false, updated_at = NOW() WHERE user_id = '${ALICE_ID}'::uuid AND target_id = '${targetMention}'::uuid;`
	);
	await page.goto('/notifications');
	await expect
		.poll(() => page.evaluate(() => document.documentElement.classList.contains('dark')))
		.toBe(true);
	await settleAnimations(page);
	await expect(page.getByText(t['notifications.type.Mention'], { exact: true })).toBeVisible();
	await expect(page.getByText(t['notifications.unread'], { exact: true })).toBeVisible();
	await expect(
		page.getByRole('link', {
			name: `${t['nav.notifications']} 1 ${t['notifications.unreadSingular']}`,
			exact: true
		})
	).toBeVisible();
	await auditAccessibility(page, '/notifications (dark mode)');
	await capture(page, '16-notifications-dark-mode.png');

	// Verify collapsed sidebar aria-label with unread count > 0, then expand cleanly.
	await page.getByRole('button', { name: t['nav.closeSidebar'], exact: true }).click();
	await settleAnimations(page);
	await expect(
		page.getByRole('link', {
			name: `${t['nav.notifications']} (1 ${t['notifications.unreadSingular']})`,
			exact: true
		})
	).toBeVisible();
	await page.getByRole('button', { name: t['nav.openSidebar'], exact: true }).click();
	await settleAnimations(page);

	await page.goto('/tasks');
	await expect
		.poll(() => page.evaluate(() => document.documentElement.classList.contains('dark')))
		.toBe(true);
	await settleAnimations(page);
	await expect(page.getByRole('heading', { name: t['tasks.title'], level: 1 })).toBeVisible();
	await expect(page.getByText(title, { exact: true })).toBeVisible();
	await auditAccessibility(page, '/tasks (dark mode)');
	await capture(page, '17-tasks-dark-mode.png');

	await page.evaluate(() => {
		try {
			localStorage.removeItem('flora.theme');
		} catch {
			// Ignore storage cleanup errors.
		}
		document.documentElement.classList.remove('dark');
	});
});
