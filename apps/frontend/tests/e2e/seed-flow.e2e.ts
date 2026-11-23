import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import type { Page } from '@playwright/test';
import { format, t } from '../../src/lib/i18n';
import { WCAG_22_AA_TAGS, summarizeWcagAudit } from '../../src/lib/utils/a11yAudit';

const MAIN_WORKSPACE = '88888888-8888-8888-8888-888888888888';

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

	// 1. select-org proves the auth state was read on the frontend origin with a
	//    nil organization. This rotates the injected jti — never reuse that token.
	await page.goto('/select-org');
	await expect(page.getByRole('heading', { name: t['org.selectTitle'] })).toBeVisible();
	await expect(page.getByText('Acme Corporation')).toBeVisible();
	await expect(page.getByText('acme-corp')).toBeVisible();
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
});
