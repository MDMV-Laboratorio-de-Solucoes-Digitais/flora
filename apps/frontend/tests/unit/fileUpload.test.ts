import { describe, it, expect } from 'vitest';
import { isValidFileSize } from '../../src/lib/utils/files';

describe('fileUpload', () => {
	it('should validate file size correctly', () => {
		expect(isValidFileSize(10)).toBe(true);
		expect(isValidFileSize(100 * 1024 * 1024)).toBe(false);
	});
	it('should fail for files > 50MB', () => {
		expect(isValidFileSize(51 * 1024 * 1024)).toBe(false);
	});
});
