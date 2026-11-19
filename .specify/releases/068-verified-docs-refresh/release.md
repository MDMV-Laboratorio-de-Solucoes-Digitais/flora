# Release: 068 — refresh verificado de documentação

**Ciclo:** 068 · **Data:** 2026-11-19T02:10Z · **Baseline:** `github/dev@cbad031b5`
**Tipo:** documentação de estado + comentário de fonte. **Nenhuma tag, release, deploy ou produção.**

## Entregue

- `docs/research/index.md` — contagens atualizadas por `find` e cifras antigas preservadas em nota append-only; dívida de indexação 047+ mantida.
- `docs/PROGRESSO.md` — nota datada de estado verificado (contagens, gate 8 pisos/51, âncora local = remota, 0 PRs, 4 runs verdes, **[NV]**), mais nota corretiva sobre o snapshot das 01:58Z.
- `OBJETIVO.md` e `ROADMAP.md` — apêndices datados com intervalo verificado 001–067; corpo histórico intacto.
- `flora-api/src/routes/rbac.rs` — comentário `//!` alinhado ao CRUD real do ciclo 066; nenhuma linha executável alterada.
- Artefatos completos: assess (5), spec (8), validate (R4 aprovado), release (R7) e `STATE.md` (decisão 80/20 + execução).

## SemVer

**Proposta:** patch `0.2.1` → `0.2.2` (documentação e comentário, sem mudança de API externa). As propostas 0.1.x/0.2.0/0.2.1 de ciclos anteriores permanecem não aplicadas. **Nenhuma tag criada.**

## Changelog (IDs)

- `FR-001` — contagens do índice corrigidas e histórico preservado.
- `FR-002` — nota datada em `PROGRESSO.md`, com **[NV]** explícito e ressalva de snapshot.
- `FR-003` — intervalo 001–067 registrado em `OBJETIVO.md` e `ROADMAP.md`.
- `FR-004` — comentário de `rbac.rs` coerente com o ciclo 066.
- `FR-005` — rastreabilidade em `STATE.md`, commits de um arquivo por commit (18 commits no total).
- `SC-001` — contagens conferidas por `find` e pelo verificador isolado.
- `SC-002` — gates do alvo verdes (local + `--remote`, Rust e frontend), saídas em `validate.md`.
- `SC-003` — 18 paths declarados; zero mudança em workflows, migrations, ADRs, constituição ou produção.
- `T001`–`T009` — executados; `T009` (PR/merge/higiene) conduzido no Bloco 7.
- Bug: nenhum em `.specify/bugs/`; origem é a decisão autônoma 80/20 em `STATE.md`.

## Evidência

`.specify/validations/068-verified-docs-refresh/validate.md` ·
`specs/068-verified-docs-refresh/converge.md` ·
`specs/068-verified-docs-refresh/analyze.md` ·
verificador isolado `VEREDITO: PASS`.

## Rollback

Reverter os commits do PR em `dev/` com um novo commit de reversão (sem force-push e sem reescrita de histórico). Todas as mudanças são documentação e um comentário; não há dados, schema, required checks, deploy ou estado de produção a restaurar.
