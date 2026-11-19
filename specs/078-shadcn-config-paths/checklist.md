# Quality Checklist: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 078, antes da implementação.
**Created**: 2026-11-19T20:25Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/078-shadcn-config-paths/define.md) · [decision.md](../../.specify/assessments/078-shadcn-config-paths/decision.md)

**Note**: checklist gerado a partir do contexto da feature; os itens abaixo aguardam revisão de qualidade de requisitos, **não** implementação.
**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[x]` = critério revisado e satisfeito quanto à qualidade dos requisitos; não significa implementação concluída.

## Requisitos

- [x] FR-001 — remoção da chave `config` (`"config": "tailwind.config.ts"`) de `tailwind` em `apps/frontend/components.json:5` (deprecada em Tailwind v4 no schema oficial) especificada?
- [x] FR-002 — correção de `"css": "src/app.css"` (linha 6) para `"css": "src/routes/layout.css"` especificada?
- [x] FR-003 — preservação byte-idêntica de `$schema`, `style`, `baseColor`, `cssVariables` e `aliases`, com tabs, sem trailing commas e aspas duplas em JSON, especificada?
- [x] FR-004 — proibição de dependências novas, segredos, `.tokens/`, gate/scripts/workflows e mutação de DB/rede, com fronteira restrita a `components.json` + 5 artefatos 078, especificada?

## Critérios de sucesso

- [x] SC-001 — ausência da chave `tailwind.config` em `apps/frontend/components.json` verificável?
- [x] SC-002 — `tailwind.css` aponta para `src/routes/layout.css` existente, com `@import 'tailwindcss'`, e casa com `prettier.config.js:11` verificável?
- [x] SC-003 — `json.load`, `bunx prettier --check components.json`, `bun run check`, `bun run lint`, `bun run test:unit` e `scripts/ci/check_gate_integrity.py` sem regressão (sem DB/rede) verificável?
- [x] SC-004 — diff restrito a `apps/frontend/components.json` (1 linha removida + 1 valor trocado) + 5 artefatos 078, gate do alvo PASS e zero mutação de DB/rede/segredos verificável?

## Portarias

- [x] R1 — `go (scoped)` registrado em `decision.md` (2026-11-19T20:21Z) antes de `specify`?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 077→078 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR/SC em `spec.md`, `plan.md` e `tasks.md`) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — `style`/`baseColor`/`cssVariables`/`aliases`/`$schema`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, ADRs, constituição, STATE, `specs/078-*` (exceto artefatos), DB/rede, segredos, issues, leg RBAC e instalação do CLI fora de escopo?
- [x] `[NV]` — validação pelo CLI `shadcn-svelte` (não instalado; exigiria rede) e observações de `style` deprecado / `cssVariables` não declarado no schema separadas da evidência determinística?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/078-shadcn-config-paths/`.
