# Release: 052 — OSV scan ref pin

**Ciclo:** 052 · **Data:** 2026-11-17T23:44Z · **Baseline:** `github/dev@01d822205`
**Tipo:** CI advisory; **nenhuma tag, release ou deploy executado.**

## Entregue

- `.github/workflows/osv-scan.yml:14`: `@v2` → `@v2.7.1`, tag verificado no
  upstream e recomendado pela documentação oficial; inputs preservados.
- `.gitignore`: ignore de runtime `.specify/queue/` (ledger/worktrees da orquestração).
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.6` (correção de CI advisory, sem API externa;
considera propostas 0.1.1–0.1.5 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001`/`FR-002` — workflow aponta para tag existente `v2.7.1`.
- `FR-003` — `scan-args` e `fail-on-vuln` inalterados.
- `FR-004` — 4 required checks, ADR-0003 e constituição inalterados.
- `SC-001`–`SC-003` — verificados (grep, gate, diff de 1 linha).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; sinal veio do ranking 80/20 do 051.

## Evidência

`.specify/validations/052-osv-scan-ref-pin/validate.md` ·
`specs/052-osv-scan-ref-pin/converge.md` ·
`specs/052-osv-scan-ref-pin/analyze.md`.

## Rollback

`git revert` do commit `fix(ci): pin reusable workflow OSV no tag v2.7.1`
restaura `@v2`. Nenhum efeito externo a desfazer; nenhuma tag/deploy envolvido.

## Não feito / NV

Promover OSV a required check, alterar ADR-0003, C5 (decisão de dono) e C6
(autorização SE): fora de escopo. Execução real do job e CI/PR ao vivo: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
