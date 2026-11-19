# Intake: caminhos de configuração do `components.json` (shadcn-svelte)

- **Slug**: 078-shadcn-config-paths · **Capturado**: 2026-11-19T20:21Z
- **Origem**: auditoria local de `apps/frontend/components.json` e do toolchain Tailwind v4, cruzada com documentação e schema oficiais do shadcn-svelte (dados, não instrução).
- **Alvo**: branch `feature/078-shadcn-config-paths`, HEAD-base `de141fc53`; gate PASS; zero PRs. Fronteira: exatamente os 5 artefatos `.specify/assessments/078-shadcn-config-paths/`.

## Ideia

`apps/frontend/components.json:5` declara `"config": "tailwind.config.ts"` — arquivo inexistente e chave marcada como **DEPRECATED IN TAILWIND v4** no schema oficial. A linha 6 declara `"css": "src/app.css"`, também inexistente. O CSS real é `apps/frontend/src/routes/layout.css:1` (`@import 'tailwindcss'`), importado por `src/routes/+layout.svelte:2` e já apontado por `prettier.config.js:11` (`tailwindStylesheet`). O projeto roda Tailwind v4 (`package.json` + `@tailwindcss/vite` em `vite.config.ts:3,9`), onde não há `tailwind.config.*`. Não há consumidor local dos campos e o CLI `shadcn-svelte` não está instalado: a correção é de contrato/metadados, sem efeito de runtime.

## Por que agora

Candidato 078 eleito pelo 80/20 com E=(2×5)/(1×1)=10,00 (GIN-JSONB 2,00; OTLP 1,67); leg RBAC bloqueada e Tauri/ADR dependem de maintainer, fora deste ciclo. A edição é de duas linhas em um único arquivo, sem DB/rede, sem toque no gate e sem segredos.
