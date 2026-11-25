# Evidence — backfill de intake (13 assessments)

**Data/hora**: 2026-11-25T01:52:17Z · **Base**: `github/dev@7ac614425` · **Worktree**: `.specify/queue/worktrees/assess-intake-backfill-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem para `.specify/assessments` | PASS | Bloco A |
| 2 | Contar novos intake | `INTAKE_BACKFILL_OK 13 / 13` | PASS | Bloco A |
| 3 | Validar estrutura SpecKit | 13/13 com título, seções e ≥4 `[NEEDS CLARIFICATION:` | PASS | Bloco A |
| 4 | Conteúdo não genérico | 2.230–2.876 bytes; cita `decision.md`/`specs/<slug>` | PASS | Bloco A |
| 5 | Gate determinístico | ver Bloco B | ver Bloco B | Bloco B |
| 6 | `git diff --check` | `DIFFCHECK_RC=0`; status com apenas paths autorizados | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo e worktrees alheias intactos; PR #410 não mesclado | PASS | Bloco C |

## Bloco A — verificação determinística

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  040-seed-ui-debloat bytes 2325 unknowns 6
OK  041-rust-dead-crates bytes 2230 unknowns 6
OK  042-real-rbac-list bytes 2229 unknowns 6
OK  086-rbac-ci-leg bytes 2730 unknowns 5
OK  087-bootstrap-ci-leg bytes 2740 unknowns 5
OK  088-dynamic-gate-oracle bytes 2805 unknowns 5
OK  089-forgejo-mirror-sync bytes 2608 unknowns 5
OK  090-rustls-security-bump bytes 2548 unknowns 5
OK  091-governance-ci-audit bytes 2862 unknowns 6
OK  092-npm-lock-sync bytes 2685 unknowns 6
OK  093-sveltekit-security-upgrade bytes 2827 unknowns 6
OK  094-visao-so-pme bytes 2575 unknowns 7
OK  095-frontend-deploy bytes 2876 unknowns 6
INTAKE_BACKFILL_OK 13 / 13
```

## Bloco B — gate determinístico

Execução (2026-11-25T01:51:56Z → 02:01:29Z), com `TMPDIR=/home/luis/development/MDMV/projetos/flora/.specify/queue/tmp-intake-backfill`:

```text
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (87 total)
GATE_RC=0
```

Resultado local à worktree isolada; checks remotos no head exato do PR permanecem `[NV]` até observação ao vivo.

## Bloco C — higiene e preservação

```text
=== DIFF CHECK ===
DIFFCHECK_RC=0
=== STATUS ===
?? .specify/assessments/040-seed-ui-debloat/intake.md
?? .specify/assessments/041-rust-dead-crates/intake.md
?? .specify/assessments/042-real-rbac-list/intake.md
?? .specify/assessments/086-rbac-ci-leg/intake.md
?? .specify/assessments/087-bootstrap-ci-leg/intake.md
?? .specify/assessments/088-dynamic-gate-oracle/intake.md
?? .specify/assessments/089-forgejo-mirror-sync/intake.md
?? .specify/assessments/090-rustls-security-bump/intake.md
?? .specify/assessments/091-governance-ci-audit/intake.md
?? .specify/assessments/092-npm-lock-sync/intake.md
?? .specify/assessments/093-sveltekit-security-upgrade/intake.md
?? .specify/assessments/094-visao-so-pme/intake.md
?? .specify/assessments/095-frontend-deploy/intake.md
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees `github-workflows-2026-11-24` e `speckit-1.4.3-upgrade` intactas; nenhum merge, deploy, auto-merge ou force-push executado.
