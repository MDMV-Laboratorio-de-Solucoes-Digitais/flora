# Evidence — backfill/regularização de concept (54 assessments)

**Data/hora**: 2026-11-25T04:00Z · **Base**: SHA remoto `266437bbfa` · **Worktree**: `.specify/queue/worktrees/assess-shape-backfill-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem `.specify/assessments` | PASS | Bloco A |
| 2 | Contar artefatos | 54 `concept.md` (52 novos + 2 reescritos) + 1 `problem.md` novo (`116`) | PASS | Bloco A |
| 3 | Formato canônico | 54/54 com H1, Slug, Recommended option, 3 opções e campos obrigatórios | PASS | Bloco A |
| 4 | Conteúdo | 3 opções por arquivo; Recommendation, Out of Scope e Assumptions presentes | PASS | Bloco A |
| 5 | Gate determinístico | PASS, 87 contratos, `GATE_RC=0` | PASS | Bloco B |
| 6 | Higiene | `DIFFCHECK_RC=0`; 55 paths no status | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo/worktrees alheias intactos; #410 mesclado por humano, não tocado | PASS | Bloco C |

## Bloco A — verificação determinística (resumo)

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  003-flora-80-20-cycle bytes 4744 options 3
OK  004-seed-runnable-baseline bytes 4403 options 3
OK  005-macro-alternative-mvp bytes 4323 options 3
OK  006-secure-vertical-slice bytes 4449 options 3
OK  007-org-scoped-auth-e2e bytes 4120 options 3
OK  008-select-org-tests-and-dev-smoke bytes 4250 options 3
OK  009-db-smoke-frontend-slice bytes 4584 options 3
OK  010-compose-uuidv7-smoke-frontend-verify bytes 5463 options 3
OK  020-next-vertical-slice bytes 5474 options 3
OK  024-next-vertical-slice bytes 4772 options 3
OK  026-next-vertical-slice bytes 4292 options 3
OK  027-next-vertical-slice bytes 4152 options 3
OK  028-next-vertical-slice bytes 4282 options 3
OK  029-next-vertical-slice bytes 4064 options 3
OK  030-next-vertical-slice bytes 4268 options 3
OK  031-next-vertical-slice bytes 4155 options 3
OK  032-next-vertical-slice bytes 4573 options 3
OK  033-next-vertical-slice bytes 4151 options 3
OK  034-origin-boundary bytes 4923 options 3
OK  040-seed-ui-debloat bytes 4329 options 3
OK  041-rust-dead-crates bytes 4135 options 3
OK  042-real-rbac-list bytes 4040 options 3
OK  048-governance-drift bytes 3419 options 3
OK  049-local-logger bytes 3357 options 3
OK  050-rate-limit-paths bytes 3453 options 3
OK  051-dead-invite-ui bytes 3301 options 3
OK  052-osv-scan-ref-pin bytes 3340 options 3
OK  055-auth-floor-thirteen bytes 3426 options 3
OK  057-dynamic-floor-total bytes 3278 options 3
OK  058-root-doc-index bytes 3025 options 3
OK  060-refresh-verified-counts bytes 3075 options 3
OK  061-dead-circuit-breaker bytes 3126 options 3
OK  062-escalation-register bytes 3038 options 3
OK  063-validate-osv-run bytes 3060 options 3
OK  065-adr-floor-alignment bytes 3099 options 3
OK  066-rbac-role-write bytes 3341 options 3
OK  086-rbac-ci-leg bytes 3767 options 3
OK  087-bootstrap-ci-leg bytes 3626 options 3
OK  088-dynamic-gate-oracle bytes 6597 options 3
OK  089-forgejo-mirror-sync bytes 6721 options 3
OK  090-rustls-security-bump bytes 6697 options 3
OK  091-governance-ci-audit bytes 6816 options 3
OK  092-npm-lock-sync bytes 6435 options 3
OK  093-sveltekit-security-upgrade bytes 6730 options 3
OK  094-visao-so-pme bytes 6451 options 3
OK  095-frontend-deploy bytes 6928 options 3
OK  107-wildcard-one-shot bytes 7455 options 3
OK  108-proptest-invariants bytes 4615 options 3
OK  109-comprehension-briefing bytes 4578 options 3
OK  110-mutation-pilot bytes 4527 options 3
OK  111-wcag-axe bytes 4455 options 3
OK  112-wcag-fixes bytes 4562 options 3
OK  113-bug-status bytes 4483 options 3
OK  116-chat-author-identity bytes 4918 options 3
SHAPE_BACKFILL_OK 54 / 54
116 has problem: True
```

`bytes` = UTF-8 bytes; `options` = contagem de `### Option`. `shape.md`/`define.md` antigos permaneceram intactos.

## Bloco B — gate determinístico

Execução (2026-11-25T03:50:13Z → 04:00:27Z), com `cargo clean` no target da worktree, `TMPDIR` gravável e `CARGO_BUILD_JOBS=2`:

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

Resultado local; checks remotos no head exato do PR são registrados separadamente.

## Bloco C — higiene e preservação

```text
=== STATUS ===
54 concept.md + 1 problem.md (116), paths autorizados
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees alheias intactas; PR #410 (`dev→main`) foi mesclado por `lfgranja` às 02:56:17Z, não por esta sessão. Nenhum merge de `main`, deploy, auto-merge ou force-push executado aqui.
