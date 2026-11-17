import { describe, expect, it, vi } from 'vitest';

import { logger } from './logger';

describe('local logger', () => {
	it('keeps entries locally without issuing network requests', () => {
		const fetchSpy = vi.fn();
		vi.stubGlobal('fetch', fetchSpy);

		logger.info('test', 'hello');

		expect(fetchSpy).not.toHaveBeenCalled();
		const entries = logger.snapshot();
		expect(entries.length).toBeGreaterThan(0);
		expect(entries.at(-1)?.message).toBe('hello');

		vi.unstubAllGlobals();
	});

	it('caps the in-memory buffer at 200 entries', () => {
		for (let index = 0; index < 250; index += 1) {
			logger.debug('cap', `entry-${index}`);
		}

		expect(logger.snapshot().length).toBe(200);
	});
});
