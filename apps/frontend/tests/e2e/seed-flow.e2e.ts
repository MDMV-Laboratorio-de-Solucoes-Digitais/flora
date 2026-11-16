import { expect, test } from '@playwright/test';

const MAIN_WORKSPACE = '88888888-8888-8888-8888-888888888888';

test('smoke: select-org -> workspace/channel -> message -> task', async ({ page }) => {
	const suffix = crypto.randomUUID().slice(0, 8);

	// 1. select-org proves the auth state was read on the frontend origin with a
	//    nil organization. This rotates the injected jti — never reuse that token.
	await page.goto('/select-org');
	await expect(page.getByRole('heading', { name: 'Select organization' })).toBeVisible();
	await expect(page.getByText('Acme Corporation')).toBeVisible();
	await expect(page.getByText('acme-corp')).toBeVisible();
	// Scope to the Acme row: the seed has more than one organization, so a bare
	// getByRole('button', { name: 'Select' }) is ambiguous in strict mode.
	await page
		.getByText('Acme Corporation')
		.locator('xpath=../..')
		.getByRole('button', { name: 'Select' })
		.click();
	await expect(page).toHaveURL(/\/tasks$/);
	await expect(page.getByRole('heading', { name: 'Tasks', level: 1 })).toBeVisible();

	// 2. workspace/channel — one selection only (the jti was rotated above).
	await page.goto('/channels');
	await expect(page.getByRole('heading', { name: 'Channels', level: 1 })).toBeVisible();
	await page.getByLabel('Workspace').selectOption(MAIN_WORKSPACE);
	await expect(page.getByRole('button', { name: '# general' })).toBeVisible();
	await page.getByRole('button', { name: '# general' }).click();
	await expect(page.getByRole('heading', { name: '# general', level: 2 })).toBeVisible();

	// 3. message — REST upsert is sufficient; WS with a lean backend is NV.
	await expect(page.getByText(/Welcome to Acme Corporation/)).toBeVisible();
	const message = `e2e smoke ${suffix}`;
	await page.locator('textarea').fill(message);
	await page.getByRole('button', { name: 'Send' }).click();
	await expect(page.getByText(message)).toBeVisible();

	// 4. task — "Assign to me" stays checked so the created task is the only one
	//    in Alice's assignee-scoped list.
	await page.goto('/tasks');
	const title = `e2e task ${suffix}`;
	await page.getByLabel('Title').fill(title);
	await page.getByRole('button', { name: 'Create task' }).click();
	await expect(page.getByText(`Task created: ${title}`)).toBeVisible();
	await expect(page.getByText(title, { exact: true })).toBeVisible();
});
