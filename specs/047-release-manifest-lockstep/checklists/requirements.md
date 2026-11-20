# Checklist: 047 — Revisão de qualidade dos requisitos

**Data:** 2026-11-17T13:46Z · **Nota:** `[x]` = requisito revisado quanto a clareza,
testabilidade e rastreabilidade. **Não** significa implementação concluída.

- [x] FR-001 — contagem 6 é verificável por `cargo metadata --no-deps` + `jq -e`.
- [x] FR-002 — lockstep de versão permanece como cláusula independente.
- [x] FR-003 — escopo limitado a `.github/workflows/release.yml`.
- [x] FR-004 — nenhum required check alterado; `check_gate_integrity.py` como prova.
- [x] SC-001 — critério com comando executável e saída colável.
- [x] SC-002 — gate autoritativo do alvo identificado pelo nome.
- [x] SC-003 — teto de diff explícito (≤2 linhas).
- [x] Cada user story é independentemente testável (P1 cobre o defect inteiro).
- [x] Não-objetivos e `[NV]` declarados.
- [x] Identidade 047 justificada por inventário, não por nome de branch.

## Revisão do checklist (executada)

- Cobertura FR: 4/4 requisitos com critério de aceite verificável.
- Cobertura SC: 3/3 critérios com comando ou limite objetivo.
- Ambiguidades remanescentes: nenhuma (`clarify.md` sem `NEEDS CLARIFICATION`).
- Resultado: **aprovado para `tasks`**. Não é declaração de implementação.
