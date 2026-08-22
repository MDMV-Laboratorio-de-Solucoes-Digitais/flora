import { test, expect } from '@playwright/test';

test.describe('File Upload Flow', () => {
	test('handles large files gracefully', async ({ page }) => {
		await page.goto('/channels');

		// Find the file input
		const fileInput = page.locator('input[type="file"]');

		// Create a mock large file (> 50MB)
		const largeFile = Buffer.alloc(51 * 1024 * 1024, 'a');
		await fileInput.setInputFiles({
			name: 'large_file.txt',
			mimeType: 'text/plain',
			buffer: largeFile
		});

		// Check that the error message is displayed
		await expect(page.locator('.error-message')).toContainText('File too large. Maximum size is 50MB.');
	});

	test('handles network errors during upload gracefully', async ({ page }) => {
		await page.goto('/channels');

		// Mock the API to simulate network failure
		await page.route('/api/v1/files/upload', (route) => {
			route.abort('failed');
		});

		const fileInput = page.locator('input[type="file"]');

		const validFile = Buffer.from('hello world');
		await fileInput.setInputFiles({
			name: 'test.txt',
			mimeType: 'text/plain',
			buffer: validFile
		});

		// Check that the generic error message is displayed
		await expect(page.locator('.error-message')).toContainText('Upload failed.');
	});
});
