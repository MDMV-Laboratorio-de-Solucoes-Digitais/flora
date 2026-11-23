import { describe, expect, it } from 'vitest';
import {
	WCAG_22_AA_TAGS,
	formatWcagViolations,
	summarizeWcagAudit
} from '../../src/lib/utils/a11yAudit';

describe('a11yAudit', () => {
	it('includes all required WCAG 2.0/2.1/2.2 A and AA tags', () => {
		expect(WCAG_22_AA_TAGS).toEqual(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa']);
	});

	it('returns passed=true and sem achados summary when there are no violations', () => {
		expect(summarizeWcagAudit('/tasks', [])).toEqual({
			findings: [],
			summary: 'WCAG 2.2 AA em /tasks: sem achados',
			passed: true
		});
	});

	it('formats violations across multiple nodes and defaults missing impact to indefinido', () => {
		const violations = [
			{
				id: 'label',
				impact: 'critical',
				nodes: [{ target: ['input[type="file"]'] }]
			},
			{
				id: 'color-contrast',
				impact: null,
				nodes: [{ target: ['a[download]'] }, { target: ['.error-message'] }]
			}
		];

		expect(formatWcagViolations(violations)).toEqual([
			String.raw`label [critical] ["input[type=\"file\"]"]`,
			'color-contrast [indefinido] ["a[download]"]',
			'color-contrast [indefinido] [".error-message"]'
		]);

		expect(summarizeWcagAudit('/files', violations)).toEqual({
			findings: [
				String.raw`label [critical] ["input[type=\"file\"]"]`,
				'color-contrast [indefinido] ["a[download]"]',
				'color-contrast [indefinido] [".error-message"]'
			],
			summary: String.raw`WCAG 2.2 AA em /files: label [critical] ["input[type=\"file\"]"]; color-contrast [indefinido] ["a[download]"]; color-contrast [indefinido] [".error-message"]`,
			passed: false
		});
	});

	it('treats a violation without nodes as failed rather than silently passing', () => {
		expect(
			summarizeWcagAudit('/channels', [{ id: 'landmark-one-main', impact: 'moderate', nodes: [] }])
		).toEqual({
			findings: ['landmark-one-main [moderate] []'],
			summary: 'WCAG 2.2 AA em /channels: landmark-one-main [moderate] []',
			passed: false
		});
	});
});
