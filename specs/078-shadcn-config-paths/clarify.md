# Clarify: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

**Data**: 2026-11-19T20:25Z · **Perguntas feitas**: 0 (R3: máx. 3, uma por vez — não necessárias; nenhuma pergunta bloqueante em `decision.md`, apenas os `[NV]` registrados abaixo).

## Resoluções sem pergunta ao usuário

1. **Identidade da feature (divergência 077 → 078)** — `.specify/feature.json` apontava `specs/077-remove-sqlite-feature` no início do ciclo (resíduo do ciclo anterior, já entregue em `de141fc53`). O caminho correto foi confirmado com `.specify/scripts/bash/create-new-feature.sh 'caminhos de configuracao do components.json (shadcn-svelte)' --dry-run --number 078` (o `--dry-run` apenas calcula nome e caminhos, sem criar arquivos): `specs/078-shadcn-config-paths`. `.specify/feature.json` hoje registra esse caminho. Divergência registrada, sem renumeração de trabalho alheio: 077 permanece intacto.
2. **Causa e opção** — `apps/frontend/components.json:5-6` descreve `tailwind.config.ts` e `src/app.css`, ambos inexistentes; o schema oficial (acessado 2026-11-19) marca `config` como "DEPRECATED IN TAILWIND v4" e define `css` como o caminho do CSS que importa Tailwind. A opção A (remover `config` + corrigir `css` para `src/routes/layout.css`) foi eleita em `decision.md` (go scoped); a opção B (`config` vazio) foi rejeitada por manter metadado enganoso e a opção C (criar `tailwind.config.ts`) por contradizer Tailwind v4 e expandir o diff.
3. **Escopo** — somente as linhas 5-6 de `apps/frontend/components.json` + os artefatos do ciclo 078. `style`, `baseColor`, `cssVariables`, `$schema`, `aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, supply-chain, ADRs, constituição, STATE, DB/rede, segredos, fechamento de issues, leg RBAC e instalação do CLI permanecem intocados.
4. **Formatação** — manter tabs e ausência de trailing commas (`prettier.config.js:5-7`); `singleQuote: true` aplica-se a JS/TS, não a JSON, então aspas duplas são obrigatórias nos valores do `components.json`.
5. **[NV]** — validação pelo CLI externo `shadcn-svelte` (não instalado; `bunx` exigiria rede) não executada; `style` deprecado em v4 e `cssVariables` não declarado no schema permanecem por escolha de escopo mínimo; extensões não varridas e status de CI/produção não são verificados. Nenhuma inferência é feita além da evidência local e das fontes oficiais.

## Checklist de requisitos (qualidade, não implementação)

A revisão de clareza, atomicidade e rastreabilidade de FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 está registrada em `spec.md` e em `checklists/requirements.md`; nenhum item é marcado como "implementado" nesta etapa.
