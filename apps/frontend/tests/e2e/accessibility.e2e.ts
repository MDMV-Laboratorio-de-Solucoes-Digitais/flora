import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import type { Page } from '@playwright/test';
import { t } from '../../src/lib/i18n';

const WCAG_TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa'];
const ROUTES = ['/tasks', '/channels'] as const;

/** Mensagem determinística em pt-BR: id, impacto e alvo de cada violação. */
function describeViolations(
	route: string,
	violations: { id: string; impact: string | null; nodes: { target: unknown }[] }[]
): string {
	const linhas = violations.flatMap((v) =>
		v.nodes.map((n) => `  - ${v.id} [${v.impact ?? 'indefinido'}] ${JSON.stringify(n.target)}`)
	);
	return `Acessibilidade WCAG 2.2 AA em ${route}:\n${linhas.join('\n')}`;
}

/** A sessão seed começa sem organização; seleciona Acme antes de navegar. */
async function selectOrganization(page: Page): Promise<void> {
	await page.goto('/select-org');
	await expect(page.getByRole('heading', { name: t['org.selectTitle'] })).toBeVisible();
	await page
		.getByText('Acme Corporation')
		.locator('xpath=../..')
		.getByRole('button', { name: t['org.select'] })
		.click();
}

for (const route of ROUTES) {
	test(`acessibilidade: ${route} sem violações WCAG 2.2 AA`, async ({ page }) => {
		await selectOrganization(page);
		await page.goto(route);
		await expect(page).toHaveURL(new RegExp(`${route}$`));

		const results = await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze();

		expect(results.violations, describeViolations(route, results.violations)).toHaveLength(0);
	});
}
