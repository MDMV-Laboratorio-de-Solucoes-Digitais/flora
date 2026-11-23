/** Tags WCAG 2.0/2.1/2.2 nível A e AA aplicadas pela varredura axe-core. */
export const WCAG_22_AA_TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa'] as const;

/** Nó afetado em uma violação relatada pelo axe-core. */
export interface AxeViolationNode {
	readonly target: readonly unknown[];
}

/** Violação retornada pelo axe-core (subconjunto estrutural independente do runner). */
export interface AxeViolation {
	readonly id: string;
	readonly impact?: string | null;
	readonly nodes: readonly AxeViolationNode[];
}

/** Resumo estruturado da auditoria WCAG para log, anotação e asserção E2E. */
export interface WcagAuditSummary {
	readonly findings: string[];
	readonly summary: string;
	readonly passed: boolean;
}

/** Formata cada nó violado no padrão legível `id [impacto] [seletor]`. */
export function formatWcagViolations(violations: readonly AxeViolation[]): string[] {
	return violations.flatMap((violation) => {
		const impact = violation.impact ?? 'indefinido';
		if (violation.nodes.length === 0) {
			return [`${violation.id} [${impact}] []`];
		}
		return violation.nodes.map(
			(node) => `${violation.id} [${impact}] ${JSON.stringify(node.target)}`
		);
	});
}

/** Constrói o resumo determinístico da auditoria WCAG 2.2 AA para uma rota. */
export function summarizeWcagAudit(
	route: string,
	violations: readonly AxeViolation[]
): WcagAuditSummary {
	const findings = formatWcagViolations(violations);
	const detail = findings.length === 0 ? 'sem achados' : findings.join('; ');
	return {
		findings,
		summary: `WCAG 2.2 AA em ${route}: ${detail}`,
		passed: violations.length === 0 && findings.length === 0
	};
}
