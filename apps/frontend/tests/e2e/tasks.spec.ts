import { test, expect } from '@playwright/test';

test.describe('Task Tracking Flow', () => {
	test('can create a task and filter by assignee', async ({ page }) => {
		// Navigate to tasks page
		await page.goto('/tasks');
		await expect(page.locator('h1')).toHaveText(/Tasks/);

		// Fill in new task form
		const titleInput = page.getByPlaceholder('New Task Title');
		const assigneeInput = page.getByPlaceholder('Assignee ID (optional)');
		const addButton = page.getByRole('button', { name: 'Add Task' });

		await titleInput.fill('Implement E2E test');
		await assigneeInput.fill('user-e2e');
		await addButton.click();

		// Ensure task appears in list
		await expect(page.getByText('Implement E2E test')).toBeVisible();
		await expect(page.getByText('Assigned to: user-e2e')).toBeVisible();

		// Change task status
		const statusSelect = page.locator('select').first();
		await statusSelect.selectOption('DONE');
		await expect(page.getByText('DONE').first()).toBeVisible();

		// Filter tasks by assignee
		const filterInput = page.getByPlaceholder('User ID');
		await filterInput.fill('non-existent-user');
		await expect(page.getByText('Implement E2E test')).not.toBeVisible();
		await expect(page.getByText('No tasks found.')).toBeVisible();

		// Show task again
		await filterInput.fill('user-e2e');
		await expect(page.getByText('Implement E2E test')).toBeVisible();
	});
});
