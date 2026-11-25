# Problem Definition: Verificação de browser e integridade documental do Seed

- **Slug**: 030-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O Seed não dispõe de uma única verificação determinística em navegador que prove o happy path `select-organization → canais/chat → tarefas` de ponta a ponta: os artefatos e2e existentes têm sufixo `.spec.ts` e ficam fora do `testMatch` do Playwright (`**/*.e2e.ts`), portanto nunca executam (research.md:3). Ao mesmo tempo, `select_organization` revoga o `jti` injetado (`auth.rs:384,387` via research.md:3), de modo que qualquer tentativa de smoke com repetições é instável por construção — a última lane de verificação permanece sem evidência utilizável. A documentação também declara estados de verificação que não são sustentados por fatos: prosa de clippy desatualizada em README, `rust.yml` e OBJETIVO, banners obsoletos em snapshots de 029, e artefato de evidência de smoke sem revisão fixada, impressão digital do segredo ou transcript versionado (intake.md:8; research.md:7). O problema subjacente é a ausência de prova reprodutível do fluxo e a desonestidade documental sobre o que foi realmente verificado — não a falta de uma ferramenta específica.

## Affected Users & Stakeholders

- **Users**: maintainer/SE que prepara o Seed e precisa de uma trilha de evidência para o happy path em navegador; desenvolvedor que executa o smoke localmente e depende de configuração estável.
- **Stakeholders**: mantenedores do Flora e revisores de CI — precisam distinguir checks required de evidência meramente informativa, sem solicitar acesso a produção.

## Goals

- Dispor de pelo menos uma execução determinística em navegador que cubra o happy path e gere evidência interpretável, sem depender de segredos estáticos nem de acesso a produção.
- Fazer a documentação descrever com precisão o estado das verificações (clippy, smoke e banners históricos), sem afirmar o que não foi executado.
- Preservar o trabalho paralelo existente e manter verdes os checks required, sem ampliar a superfície de CI.
- Explicitar o limite entre evidência coletada em CI não-required e verificação obrigatória de merge.

## Non-Goals

- Não criar novos serviços, não reverter trabalho paralelo e não acessar produção.
- Não entregar tag/release + SBOM, allowlist CORS, execução de load/restore, promoção de cargo-vet a required ou OTLP neste ciclo (intake.md:10-12; decision.md, "NV / deferred").
- Não converter o smoke em gate de merge nem afirmar cobertura de todos os fluxos do frontend.
- Não atribuir a este ciclo a resolução de ADR-0002 #2 nem de Documentos FASE 7.

## Success Metrics

- Pelo menos um smoke de navegador do happy path executa com `retries: 0` e `workers: 1` e se mantém verde em execuções repetidas (baseline: nenhum smoke executado; `.spec.ts` fora do `testMatch`).
- Artefato de evidência contém revisão fixada, impressão digital do segredo e transcript versionado (baseline: artefato incompleto, research.md:7).
- Prosa de clippy e banners históricos não contêm afirmações desatualizadas em README, `rust.yml`, OBJETIVO e snapshots de 029 (baseline: prosa e banners obsoletos).
- Nenhum check required regrediu e nenhuma alteração reverteu trabalho paralelo (baseline: required checks verdes, auditoria prévia VERIFICADA).

## Cost of Inaction

Sem a lane de navegador, regressões no fluxo de seleção de organização e de tarefas passam despercebidas até a release, e a documentação continua sugerindo verificações que não existem. O custo é confiança falsa no Seed e retrabalho de auditoria; não há evidência de impacto financeiro ou de usuário final anexada a este ciclo [NV].

## Open Questions

- [NEEDS CLARIFICATION: qual critério define que o smoke não-required é "evidência suficiente" para a aprovação do slice?]
- [NEEDS CLARIFICATION: quais seletores semânticos estáveis devem ser congelados, dado que `data-testid` não é necessário para um único smoke?]
- [NEEDS CLARIFICATION: o artefato de evidência deve ser versionado no repositório ou apenas referenciado por caminho externo?]
- [NEEDS CLARIFICATION: quem valida que a correção da prosa de clippy não conflita com edições de outros agentes?]
