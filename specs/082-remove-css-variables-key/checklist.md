# Quality Checklist: 082 — remoção da chave `cssVariables` de `components.json`

**Purpose**: revisão de qualidade dos requisitos FR-001, FR-002, FR-003, FR-004 e dos critérios SC-001, SC-002, SC-003, SC-004, além das portarias do ciclo 082, antes da implementação.
**Created**: 2026-11-19T22:36Z
**Feature**: [spec.md](spec.md) · [define.md](../../.specify/assessments/082-remove-css-variables-key/define.md) · [decision.md](../../.specify/assessments/082-remove-css-variables-key/decision.md)

> Itens gerados sem marcações; `[ ]` = aguardando revisão de qualidade de requisitos, **não** implementação.

**Review Ownership**: artefato de revisão de qualidade de requisitos, de responsabilidade do revisor. Marcar `[x]` somente quando o critério de qualidade estiver satisfeito.
**Marker Semantics**: `[ ]` = critério gerado e pendente de revisão; `[x]` = revisado e satisfeito quanto à qualidade dos requisitos. Nenhum marcador indica implementação concluída. A revisão de referência está em `checklists/requirements.md`.

## Requisitos

- [x] FR-001 — remoção da linha `apps/frontend/components.json:7` (`"cssVariables": true`) do objeto `tailwind`, chave ausente em todo o schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `additionalProperties = false`) e descartada por `$strip` no CLI `shadcn-svelte@1.8.1` (`schema-Bq2uExa4.d.mts:407-414`), especificada?
- [x] FR-002 — ajuste da linha `apps/frontend/components.json:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula da chave excluída), preservando o valor `zinc` e gerando JSON válido sem trailing comma (`prettier.config.js:7`); SC-001 depende de FR-001 + FR-002, especificada?
- [x] FR-003 — byte-identicidade de `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), do valor de `tailwind.baseColor` (`:6`) e de todo o bloco `aliases` (`:9-14`), com formatação Prettier local (tabs em `prettier.config.js:5`, aspas duplas em JSON, sem trailing comma em `:7`) e sem remover `style` (declarado no schema, apenas deprecado em v4), especificada?
- [x] FR-004 — diff rastreado restrito a `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada) + 5 artefatos `.specify/assessments/082-remove-css-variables-key/`, sem outro arquivo de código/config/doc/gate/CI/lockfile/`node_modules/`, com base `8f24519fe` preservada, especificada?

## Critérios de sucesso

- [x] SC-001 — `python3 -c` com `json.load` em `apps/frontend/components.json` retorna `set(obj["tailwind"]) == {"css", "baseColor"}` → `True` e `cssVariables` ausente em todos os níveis (só passa após T001 e T002; `[NV]` até a edição), verificável?
- [x] SC-002 — `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` sem `apps/frontend/components.json` nem arquivo de código/script/CI, apenas registros históricos em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*`, com `$schema`, `style`, `baseColor`, `css` e `aliases` byte-idênticos, verificável?
- [x] SC-003 — `cd apps/frontend && bunx prettier --check components.json` com exit 0 e `bun run check`/`bun run lint`/`bun run test:unit -- --run`/`bun run build` (`package.json:8,11,13,15`; `test:unit` é `vitest`, `package.json:15`, que exige `--run` para execução única) sem regressão, `python3 scripts/ci/check_gate_integrity.py` PASS, nenhum comando lendo `cssVariables` em runtime (`[NV]` até a execução), verificável?
- [x] SC-004 — `git diff --name-only` com somente `apps/frontend/components.json` e os 5 artefatos 082, zero mutação de gate/CI/scripts/workflows/lockfiles/`node_modules/`/ADRs/specs numeradas/DB/rede/segredos, zero PRs e base `8f24519fe` preservada, verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` (2026-11-19T22:31Z) antes de `specify`, com E=(2×5)/(1×1)=5,0 e opção A (remover `:7` e ajustar a vírgula em `:6` em `apps/frontend/components.json`)?
- [x] R3 — zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 081→082 via `--dry-run`, sem bloqueio)?
- [x] R4 e R7 — rastreabilidade individual (FR-001..FR-004 e SC-001..SC-004 em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; SC-001 em T001 e T002) registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/078-*`/`.specify/assessments/078-*`, `specs/082-*` (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles fora de escopo?
- [x] `[NV]` — gates pós-edição, validação online e consumidores externos de `components.json` separados da evidência determinística da linha colada, do schema oficial com URL/data, do parser `$strip`, da varredura de consumidores zero e de `prettier.config.js:5-7`?

## Notes

- Marcar `[x]` somente após revisão confirmar o critério de qualidade de requisitos; itens em aberto permanecem com `[ ]`.
- Este `checklist.md` é gerenciado por `/speckit-checklist`; `checklists/requirements.md` tem ciclo de vida próprio, mantido por `/speckit-specify` e `/speckit-clarify`.
- Registrar achados inline; vincular à evidência em `.specify/assessments/082-remove-css-variables-key/`.
