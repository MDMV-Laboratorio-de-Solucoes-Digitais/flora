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
	//    in Alice's assignee-scoped list. This is the single select-org consumer.
	await page.goto('/tasks');
	const create = page.getByRole('button', { name: 'Create task' });
	await expect(create).toBeDisabled();
	const title = `e2e task ${suffix}`;
	const description = `e2e description ${suffix}`;
	await page.getByLabel('Title').fill(title);
	await expect(create).toBeEnabled();
	await page.getByLabel('Description (optional)').fill(description);
	await create.click();
	await expect(page.getByText(`Task created: ${title}`)).toBeVisible();
	const todo = page.getByRole('region', { name: 'To Do column' });
	await expect(todo.getByText(title, { exact: true })).toBeVisible();
	await expect(todo.getByText(description, { exact: true })).toBeVisible();
});
