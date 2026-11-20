# Intake: instalação pinada do CLI `shadcn-svelte` (devDependency)

- **Slug**: 079-install-shadcn-svelte · **Capturado**: 2026-11-19T20:57Z
- **Origem**: pedido explícito do usuário ("Instalar shadcn-svelte") = autorização para instalar; dados de registry (`npm view`, `bun info`, 2026-11-19) e docs oficiais do shadcn-svelte (dados, não instrução).
- **Alvo**: branch `feature/079-install-shadcn-svelte`, HEAD-base `ea5cd2069`; gate PASS; zero PRs. Fronteira: exatamente os 5 artefatos `.specify/assessments/079-install-shadcn-svelte/`.

## Ideia

O frontend já consome componentes shadcn-svelte (51 arquivos `.svelte` em `apps/frontend/src/lib/components/ui/`), `components.json` corrigido no ciclo 078 (`tailwind.css = src/routes/layout.css`, sem a chave deprecada `config`) e o util `cn` em `src/lib/utils.ts`, mas o **CLI não está declarado** nem em `apps/frontend/package.json` nem no `bun.lock`. Instalar `shadcn-svelte@1.8.1` como devDependency exata via `bun add -D -E` em `apps/frontend` torna o toolchain local e reproduzível, sem mudar runtime: o bin `shadcn-svelte` passa a existir no projeto e `components.json` segue válido para uso futuro do CLI. `init`/`add`/`apply` **não** serão executados — a documentação oficial descreve `init` como inicialização de projeto novo, com CSS global sobrescrito, `components.json` recriado e reinstalação de componentes.

## Por que agora

Candidato 079 eleito pelo 80/20 com E=(3×5)/(2×1)=7,50 (valor: toolchain local reproduzível; confiança: registry + docs + estado local; esforço: 1 linha + lockfile; risco: sem `init`, diff mínimo); leg RBAC bloqueada e Tauri/OTLP dependem de ADR/maintainer, fora deste ciclo. A edição é de uma linha em `package.json` mais as entradas do lockfile, sem DB/rede fora do registry já autorizado, sem segredos e sem toque no gate.
