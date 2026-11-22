import { describe, it, expect } from 'vitest';
import { isValidFileSize, MAX_FILE_SIZE_BYTES } from '../../src/lib/utils/files';

describe('fileUpload', () => {
	it('accepts small files', () => {
		expect(isValidFileSize(10)).toBe(true);
	});

	it('accepts exactly the Seed limit and rejects one byte above it', () => {
		expect(MAX_FILE_SIZE_BYTES).toBe(10 * 1024 * 1024);
		expect(isValidFileSize(MAX_FILE_SIZE_BYTES)).toBe(true);
		expect(isValidFileSize(MAX_FILE_SIZE_BYTES + 1)).toBe(false);
	});

	it('rejects files above the configured limit', () => {
		expect(isValidFileSize(100 * 1024 * 1024)).toBe(false);
	});
});
