# Release: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

**Ciclo**: 078 · **Data**: 2026-11-19T20:38Z · **Branch**: `feature/078-shadcn-config-paths` · **Baseline**: `de141fc53`
**Tipo**: correção de contrato de configuração (metadado de CLI inerte). **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/components.json:5` — linha `"config": "tailwind.config.ts",` removida de `tailwind` (chave "DEPRECATED IN TAILWIND v4" no schema oficial, acessado 2026-11-19).
- `apps/frontend/components.json:6` — valor trocado de `"css": "src/app.css"` (inexistente) para `"css": "src/routes/layout.css"` (CSS real, com `@import 'tailwindcss'` em `layout.css:1` e casa com `prettier.config.js:11`).
- `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` byte-idênticos; tabs, sem trailing commas, aspas duplas (formatação do Prettier local preservada).
- Artefatos completos: assess (5), spec, clarify, plan, checklist/tasks, analyze, converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.2.8` → `0.2.9` (correção de metadado de configuração inerte, sem mudança de API, comportamento, runtime ou dependência ativa). Nenhuma tag criada.

## Changelog (FR / SC / T)

- `FR-001` chave `tailwind.config` removida (deprecada no v4); `FR-002` `tailwind.css` → `src/routes/layout.css`, arquivo existente com `@import 'tailwindcss'` e alinhado a `prettier.config.js:11`; `FR-003` `$schema`/`style`/`baseColor`/`cssVariables`/`aliases` e formatação Prettier preservados byte-a-byte; `FR-004` zero dependências, segredos, credenciais, `.tokens/`, mudanças de gate/scripts/workflows e mutação de DB/rede.
- `SC-001`..`SC-004` verificados por `json.load`, `prettier --check`, `bun run check`, `bun run lint`, `bun run test:unit` (42/42), `check_gate_integrity.py` (PASS), varredura de consumidores (zero correspondências) e auditoria de diff (1 arquivo: 1 remoção + 1 troca).
- `T001`..`T004` executados; verificador isolado `ComponentsVerifier` **PASS** (diff exato, schema oficial satisfeito, CSS real carregado, zero consumidores locais); nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/078-shadcn-config-paths/validate.md` · `specs/078-shadcn-config-paths/converge.md` · `specs/078-shadcn-config-paths/analyze.md`.

## Rollback

Reverter, em `dev/`, exclusivamente o commit que tocou `apps/frontend/components.json` — reintroduzindo `"config": "tailwind.config.ts"` na linha 5 e restaurando `"css": "src/app.css"` na linha 6 — com novo commit de reversão, sem force-push ou reescrita de histórico. Nenhum outro path é modificado: `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, ADRs, constituição, STATE, specs 077 e demais artefatos, DB/rede, `.tokens/` e segredos permanecem intactos. Os artefatos do ciclo 078 são documentação e podem ser revertidos em bloco.

## [NV]

CLI externo `shadcn-svelte` não instalado (exigiria rede) e validação de schema em runtime não executada; `style` deprecado no Tailwind v4 e `cssVariables` não declarado no schema, preservados por escopo mínimo; extensões de arquivo não varridas na busca de consumidores; CI ao vivo, Forgejo, aprovações, merge, produção e leg RBAC/maintainer: **[NV]**.
