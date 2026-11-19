# Clarify: 076 — hang do setup E2E do minter de sessão

**Data**: 2026-11-19T12:45Z · **Perguntas feitas**: 0 (R3: máx. 3, uma por vez — não necessárias).

## Resoluções sem pergunta ao usuário

1. **Identidade da feature (divergência 075 → 076)** — `.specify/feature.json` apontava `specs/075-task-create-e2e` no início do ciclo (resíduo do ciclo anterior). Após criação legítima via `create-new-feature.sh 'hang do setup E2E do minter de sessão' --dry-run --number 076`, passou a apontar `specs/076-mint-session-hang`. Nenhuma renumeração de trabalho alheio: 075 permanece intacto.
2. **`globalSetup` fica fora de `testMatch`** — o arquivo será `apps/frontend/playwright/global-setup.ts`, ligado pela chave `globalSetup` da config. O nome foi escolhido para NÃO casar com `testMatch: '**/*.e2e.ts'` (`playwright.config.ts:9`) nem com `testMatch: /.*\.setup\.ts/` do projeto `setup` (`playwright.config.ts:19`): ele é executado uma vez pelo runner, não coletado como teste. `seed.setup.ts` continua como único teste do projeto `setup`.
3. **Escopo** — somente `apps/frontend/playwright.config.ts` (uma linha) + o novo `apps/frontend/playwright/global-setup.ts` + os artefatos do ciclo (`spec.md`, `clarify.md`, `plan.md`). `seed.setup.ts`, workflows, código de produção, ADRs, constituição, STATE e DB compartilhado permanecem intocados.
4. **Causa e opção** — a causa é a compilação dentro da janela de 180s (`seed.setup.ts:14` e `seed.setup.ts:36-53`), confirmada em `research.md`/`concept.md`; a opção A (`globalSetup` com pré-build) foi eleita em `decision.md`. A opção B (aumentar o timeout) foi rejeitada por mascarar o sintoma e manter o `execFileSync` sem limite superior.
5. **[NV]** — tempo com target totalmente frio e status de CI/produção não são verificados; nenhuma inferência é feita a partir dos 30.18s medidos com cache.

## Checklist de requisitos (qualidade, não implementação)

A revisão de clareza, atomicidade e rastreabilidade de FR-001, FR-002, FR-003, FR-004, FR-005, SC-001, SC-002, SC-003 e SC-004 está registrada em `spec.md`; nenhum item é marcado como “implementado” nesta etapa.
