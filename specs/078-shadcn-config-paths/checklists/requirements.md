# Requirements Quality Checklist — 078

**Feature**: [spec.md](../spec.md) · **Slug**: 078-shadcn-config-paths
**Created**: 2026-11-19T20:25Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T20:25Z, contra `define.md`/`decision.md` (acessados 2026-11-19).

## Completude

- [x] FR-001 define a remoção da chave `"config": "tailwind.config.ts"` de `tailwind` em `apps/frontend/components.json:5` — chave marcada como "DEPRECATED IN TAILWIND v4" no schema oficial (acessado 2026-11-19).
- [x] FR-002 define a correção de `"css": "src/app.css"` (linha 6) para `"css": "src/routes/layout.css"`, caminho que efetivamente importa Tailwind (`layout.css:1`, `+layout.svelte:2`) e já usado por `prettier.config.js:11`.
- [x] FR-003 preserva byte-idênticos `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` (`components.json:1-4,7-16`), mantendo a formatação do Prettier local (tabs, sem trailing commas — `prettier.config.js:6-8`) e aspas duplas obrigatórias em JSON.
- [x] FR-004 proíbe dependências novas, segredos, credenciais, `.tokens/`, mudanças em gate/scripts/workflows e mutação de DB/rede; nenhum outro arquivo além de `components.json` e dos 5 artefatos 078.

## Clareza e consistência

- [x] SC-001 é verificável: `apps/frontend/components.json` não contém mais a chave `tailwind.config`.
- [x] SC-002 é verificável: `tailwind.css` aponta para arquivo existente (`src/routes/layout.css`) que contém `@import 'tailwindcss'` e casa com `prettier.config.js:11`.
- [x] SC-003 é verificável: `python3 -c` com `json.load` valida o JSON; `bunx prettier --check components.json` limpo; `bun run check`, `bun run lint`, `bun run test:unit` e `scripts/ci/check_gate_integrity.py` sem regressão (sem DB/rede).
- [x] SC-004 é verificável: diff restrito a `apps/frontend/components.json` (1 linha removida + 1 valor trocado) e aos 5 artefatos 078; gate do alvo PASS; zero mutação de DB/rede/segredos.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7).
- [x] Evidência determinística (schema e docs oficiais com URL e data de acesso 2026-11-19; linhas coladas; varredura de consumidores) distinguida de `[NV]` para a validação pelo CLI `shadcn-svelte` (não instalado; exigiria rede) e para a observação de `style` deprecado / `cssVariables` não declarado no schema.

## Edge cases

- [x] E se a chave `config` já estiver ausente? FR-001 é idempotente: nada a remover e SC-001 já satisfeito.
- [x] E se `css` já apontar para `src/routes/layout.css`? FR-002 é idempotente: nada a trocar e SC-002 já satisfeito.
- [x] E se `src/routes/layout.css` não existir ou não contiver `@import 'tailwindcss'`? Criar/editar CSS está FORA de escopo; a correção é de metadados e a premissa local é verificada em SC-002 — se mudar, registrar `[NV]` e reavaliar antes de editar.
- [x] E se aparecer um consumidor local dos campos não detectado na varredura determinística? A fronteira permanece explícita; reavaliar em `clarify.md` antes de qualquer edição, sem ampliar escopo automaticamente.
- [x] E se houver risco de DB/rede/segredos? Nenhum comando de DB/rede é executado; todos os gates são locais/offline; a fronteira de paths permanece explícita.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T20:21Z) antes da escrita dos artefatos.
- [x] R3 zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 077→078 documentada via `--dry-run`, sem bloqueio).
- [x] R4 e R7 respeitados na rastreabilidade: FR/SC individuais em `spec.md`, `plan.md` e `tasks.md`, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: `style`/`baseColor`/`cssVariables`/`aliases`/`$schema`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, ADRs, constituição, STATE, `specs/078-*` (exceto artefatos), DB/rede, segredos, fechamento de issues, leg RBAC e instalação do CLI `shadcn-svelte`.
