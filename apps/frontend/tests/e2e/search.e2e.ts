import { expect, test } from '@playwright/test';
import { t } from '../../src/lib/i18n/pt-BR';

const SEARCH_ROUTE = /\/api\/v1\/search(?:\?|$)/;

test.describe('Search UI', () => {
	test('renders entity type and snippet from the API', async ({ page }) => {
		await page.route(SEARCH_ROUTE, (route) =>
			route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({
					query: 'dashboard',
					results: [
						{
							type: 'task',
							id: '44444444-5555-6666-7777-888888888888',
							snippet: 'Design new dashboard UI'
						}
					],
					source: 'postgres'
				})
			})
		);

		await page.goto('/channels');
		const box = page.getByRole('searchbox', { name: t['search.ariaLabel'] });
		await box.fill('dashboard');

		await expect(page.getByText(`1 ${t['search.result_one']}`)).toBeVisible();
		await expect(page.getByText(t['search.type.task'], { exact: true })).toBeVisible();
		await expect(page.getByText('Design new dashboard UI')).toBeVisible();
	});

	test('shows an empty state for a query with no matches', async ({ page }) => {
		await page.route(SEARCH_ROUTE, (route) =>
			route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify({ query: 'nothing', results: [], source: 'postgres' })
			})
		);

		await page.goto('/channels');
		await page.getByRole('searchbox', { name: t['search.ariaLabel'] }).fill('nothing');
		await expect(page.getByText(t['search.noResults'].replace('{query}', 'nothing'))).toBeVisible();
	});
});
