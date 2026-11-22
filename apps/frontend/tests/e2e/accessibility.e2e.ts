import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

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

for (const route of ROUTES) {
	test(`acessibilidade: ${route} sem violações WCAG 2.2 AA`, async ({ page }) => {
		await page.goto(route);
		await expect(page).toHaveURL(new RegExp(`${route}$`));

		const results = await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze();

		expect(results.violations, describeViolations(route, results.violations)).toHaveLength(0);
	});
}
