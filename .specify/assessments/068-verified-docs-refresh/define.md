# Define: 068 — refresh verificado de documentação

## Problema

Leitores e agentes usam `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md` e `ROADMAP.md` como retrato do estado do repositório. Hoje esses retratos divergem do filesystem em até 21 ciclos e em seis contagens, e um comentário em código descreve handlers que já são reais. Isso corrói a confiança na documentação e pode levar a decisões baseadas em estado obsoleto.

## Requisitos funcionais

- **FR-001**: atualizar a contagem de `docs/research/index.md` e sua tabela de contagens para os valores obtidos por `find` em 2026-11-19, preservando a dívida de indexação retroativa 047+ como registro append-only.
- **FR-002**: acrescentar a `docs/PROGRESSO.md` uma nota datada de estado verificado com contagens por `find`, gate autoritativo, âncora local/remota e **[NV]** explícito, sem reescrever notas anteriores.
- **FR-003**: registrar em `OBJETIVO.md` e `ROADMAP.md` que o intervalo verificado chega ao ciclo 067, por apêndice datado, sem alterar o corpo histórico.
- **FR-004**: corrigir o comentário obsoleto em `flora-api/src/routes/rbac.rs:4-5` para refletir o CRUD real implementado no ciclo 066, sem tocar na lógica.
- **FR-005**: registrar a execução do ciclo em `.specify/memory/STATE.md` e nos artefatos SDD (assess/spec/validate/release), com convenção de commits de um arquivo por commit.

## Critérios de sucesso

- **SC-001**: todas as contagens publicadas coincidem com `find` executado na worktree (58/280, 55, 54, 54, 4, 137, 6).
- **SC-002**: os gates do alvo permanecem verdes: `check_gate_integrity.py` PASS (local + `--remote`), fmt/clippy/deny/build/test e frontend check/lint/test/build exit 0.
- **SC-003**: nenhuma alteração em `.github/workflows/`, migrations, ADRs, constituição, required checks ou produção; `git diff` limitado aos paths declarados.

## Fora de escopo

Indexação retroativa de 047–067 (candidato autônomo seguinte, E=3.75), emenda da constituição (exige maintainer), contrato RBAC vácuo, remoção do stub `AuthService`, e qualquer merge sem CI verde observada ao vivo.
