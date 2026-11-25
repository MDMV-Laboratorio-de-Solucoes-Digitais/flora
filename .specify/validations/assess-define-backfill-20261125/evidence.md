# Evidence — backfill/regularização de problem definition (42 assessments)

**Data/hora**: 2026-11-25T03:33Z · **Base**: `github/dev@df75b1bcd` · **Worktree**: `.specify/queue/worktrees/assess-define-backfill-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem `.specify/assessments` | PASS | Bloco A |
| 2 | Contar artefatos | 30 criações + 5 reescritas + 7 criações = 42 slugs | PASS | Bloco A |
| 3 | Formato canônico | 42/42 com H1, Slug e 7 seções obrigatórias | PASS | Bloco A |
| 4 | Conteúdo | Users/Stakeholders, baseline, Cost of Inaction e `[NEEDS CLARIFICATION:]` presentes | PASS | Bloco A |
| 5 | Gate determinístico | PASS na 3ª tentativa; duas falhas ambientais documentadas | PASS | Bloco B |
| 6 | Higiene | `DIFFCHECK_RC=0`; 42 paths no status | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo/worktrees alheias intactos; PR #410 não mesclado | PASS | Bloco C |

## Bloco A — verificação determinística

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  003-flora-80-20-cycle bytes 4940 q 3
OK  004-seed-runnable-baseline bytes 5024 q 3
OK  005-macro-alternative-mvp bytes 5092 q 3
OK  006-secure-vertical-slice bytes 5491 q 3
OK  007-org-scoped-auth-e2e bytes 5309 q 3
OK  008-select-org-tests-and-dev-smoke bytes 5253 q 3
OK  009-db-smoke-frontend-slice bytes 5569 q 3
OK  010-compose-uuidv7-smoke-frontend-verify bytes 6951 q 4
OK  020-next-vertical-slice bytes 5409 q 3
OK  021-next-vertical-slice bytes 5807 q 4
OK  022-next-vertical-slice bytes 5547 q 4
OK  023-next-vertical-slice bytes 5701 q 4
OK  024-next-vertical-slice bytes 5407 q 4
OK  026-next-vertical-slice bytes 5085 q 4
OK  027-next-vertical-slice bytes 5002 q 4
OK  028-next-vertical-slice bytes 5193 q 5
OK  029-next-vertical-slice bytes 5266 q 5
OK  030-next-vertical-slice bytes 4387 q 4
OK  031-next-vertical-slice bytes 3798 q 4
OK  032-next-vertical-slice bytes 4358 q 4
OK  033-next-vertical-slice bytes 4013 q 4
OK  034-origin-boundary bytes 4792 q 5
OK  040-seed-ui-debloat bytes 4855 q 6
OK  041-rust-dead-crates bytes 4995 q 6
OK  042-real-rbac-list bytes 4819 q 6
OK  086-rbac-ci-leg bytes 4850 q 6
OK  087-bootstrap-ci-leg bytes 5263 q 6
OK  088-dynamic-gate-oracle bytes 5202 q 6
OK  089-forgejo-mirror-sync bytes 5102 q 6
OK  090-rustls-security-bump bytes 5263 q 6
OK  091-governance-ci-audit bytes 5137 q 6
OK  092-npm-lock-sync bytes 4685 q 6
OK  093-sveltekit-security-upgrade bytes 5193 q 7
OK  094-visao-so-pme bytes 4952 q 6
OK  095-frontend-deploy bytes 5107 q 7
OK  107-wildcard-one-shot bytes 5728 q 5
OK  108-proptest-invariants bytes 5665 q 5
OK  109-comprehension-briefing bytes 5577 q 5
OK  110-mutation-pilot bytes 5969 q 5
OK  111-wcag-axe bytes 5768 q 5
OK  112-wcag-fixes bytes 5610 q 5
OK  113-bug-status bytes 6105 q 5
DEFINE_BACKFILL_OK 42 / 42
```

`bytes` = UTF-8 bytes; `q` = contagem de `[NEEDS CLARIFICATION:]` em Open Questions. Arquivos `define.md` dos 7 slugs recentes permaneceram intactos.

## Bloco B — gate determinístico

1ª tentativa (2026-11-25T02:57:50Z → 03:02:04Z): `gate-integrity: FAIL`; `cargo --ignored --list failed (rc=101)` por `cc-rs`/`aws-lc-sys` SIGABRT sob pressão de memória (`free -h`: 379 MiB livres, swap 6,5/8 GiB; load ~16). Nenhum arquivo de código foi alterado.

2ª tentativa (2026-11-25T03:02:21Z → 03:14:10Z), com `CARGO_BUILD_JOBS=1`: `FAIL`; erro extraído: `failed to build archive .../chrono-tz...rlib: failed to rename archive file: No space left on device (os error 28)`. Btrfs em `/home` com metadata 5,49/6,00 GiB e nenhum chunk livre.

3ª tentativa (2026-11-25T03:18:11Z → 03:32:52Z), após `cargo clean` no `target/` desta worktree (3.189 arquivos / 817,5 MiB removidos) e `CARGO_BUILD_JOBS=2`:

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

As duas falhas são ambientais (OOM/ENOSPC em btrfs), não regressão de código: o diff é exclusivamente documental e o gate passou no mesmo checkout após limpeza do target. Checks remotos no head exato do PR são registrados separadamente.

## Bloco C — higiene e preservação

```text
=== STATUS (42 paths) ===
M/?? em .specify/assessments/<42 slugs>/problem.md
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees alheias intactas; nenhum merge, deploy, auto-merge ou force-push executado.
