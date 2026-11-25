# Evidence — revisita canônica de decision.md (62 assessments)

**Data/hora**: 2026-11-25T04:49Z · **Base**: SHA remoto `f13d321c7a2b` · **Worktree**: `.specify/queue/worktrees/assess-decide-revisit-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem `.specify/assessments` | PASS | Bloco A |
| 2 | Contar artefatos | 62 `decision.md` reescritos + 4 pré-requisitos (114/115) | PASS | Bloco A |
| 3 | Formato canônico | 62/62 com H1, Slug, Decided, Verdict, 6 critérios e seções condicionais | PASS | Bloco A |
| 4 | Conteúdo | 62 × `go`; Evidence strength `strong`/`adequate` em todos; veredito histórico preservado | PASS | Bloco A |
| 5 | Gate determinístico | PASS, 87 contratos, `GATE_RC=0` | PASS | Bloco B |
| 6 | Higiene | `DIFFCHECK_RC=0`; 66 paths no status | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo/worktrees alheias intactos; #410 mesclado por humano, não tocado | PASS | Bloco C |

## Bloco A — verificação determinística (resumo)

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  006-secure-vertical-slice bytes 4363 rows 6 verdict go
OK  007-org-scoped-auth-e2e bytes 4521 rows 6 verdict go
OK  024-next-vertical-slice bytes 4467 rows 6 verdict go
OK  026-next-vertical-slice bytes 4446 rows 6 verdict go
OK  027-next-vertical-slice bytes 4276 rows 6 verdict go
OK  028-next-vertical-slice bytes 4528 rows 6 verdict go
OK  029-next-vertical-slice bytes 4251 rows 6 verdict go
OK  030-next-vertical-slice bytes 4659 rows 6 verdict go
OK  031-next-vertical-slice bytes 4596 rows 6 verdict go
OK  032-next-vertical-slice bytes 4873 rows 6 verdict go
OK  033-next-vertical-slice bytes 4511 rows 6 verdict go
OK  034-origin-boundary bytes 4772 rows 6 verdict go
OK  040-seed-ui-debloat bytes 4831 rows 6 verdict go
OK  041-rust-dead-crates bytes 5546 rows 6 verdict go
OK  042-real-rbac-list bytes 4877 rows 6 verdict go
OK  047-release-manifest-lockstep bytes 4275 rows 6 verdict go
OK  048-governance-drift bytes 4624 rows 6 verdict go
OK  049-local-logger bytes 4600 rows 6 verdict go
OK  050-rate-limit-paths bytes 4104 rows 6 verdict go
OK  051-dead-invite-ui bytes 3786 rows 6 verdict go
OK  052-osv-scan-ref-pin bytes 3825 rows 6 verdict go
OK  053-db-gate-uncovered-legs bytes 4205 rows 6 verdict go
OK  054-db-env-fail-closed bytes 4210 rows 6 verdict go
OK  055-auth-floor-thirteen bytes 3906 rows 6 verdict go
OK  056-docs-research-index bytes 3876 rows 6 verdict go
OK  057-dynamic-floor-total bytes 3930 rows 6 verdict go
OK  058-root-doc-index bytes 3953 rows 6 verdict go
OK  059-db-tests-fail-local bytes 3303 rows 6 verdict go
OK  060-refresh-verified-counts bytes 3343 rows 6 verdict go
OK  061-dead-circuit-breaker bytes 3370 rows 6 verdict go
OK  062-escalation-register bytes 3089 rows 6 verdict go
OK  063-validate-osv-run bytes 3367 rows 6 verdict go
OK  064-n3-org-scoped-roles bytes 3792 rows 6 verdict go
OK  065-adr-floor-alignment bytes 3294 rows 6 verdict go
OK  066-rbac-role-write bytes 3628 rows 6 verdict go
OK  067-c6-canary-rls-design bytes 4223 rows 6 verdict go
OK  068-verified-docs-refresh bytes 4159 rows 6 verdict go
OK  069-research-index-truth bytes 3009 rows 6 verdict go
OK  070-rbac-contract bytes 3033 rows 6 verdict go
OK  071-dead-auth-service bytes 3089 rows 6 verdict go
OK  072-verified-state-refresh bytes 3412 rows 6 verdict go
OK  074-post-merge-closure bytes 3478 rows 6 verdict go
OK  075-task-create-e2e bytes 3357 rows 6 verdict go
OK  076-mint-session-hang bytes 3508 rows 6 verdict go
OK  077-remove-sqlite-feature bytes 3690 rows 6 verdict go
OK  094-visao-so-pme bytes 4955 rows 6 verdict go
OK  107-wildcard-one-shot bytes 4324 rows 6 verdict go
OK  108-proptest-invariants bytes 3878 rows 6 verdict go
OK  109-comprehension-briefing bytes 3942 rows 6 verdict go
OK  110-mutation-pilot bytes 3885 rows 6 verdict go
OK  111-wcag-axe bytes 4110 rows 6 verdict go
OK  112-wcag-fixes bytes 4063 rows 6 verdict go
OK  113-bug-status bytes 4001 rows 6 verdict go
OK  114-wcag-ci-guard bytes 4472 rows 6 verdict go
OK  115-files-list-persistence bytes 4496 rows 6 verdict go
OK  116-chat-author-identity bytes 4614 rows 6 verdict go
OK  117-notifications-ui-and-honest-navigation bytes 4696 rows 6 verdict go
OK  118-oidc-callback-mock-idp-exchange bytes 4540 rows 6 verdict go
OK  119-rbac-roles-admin-ui bytes 4524 rows 6 verdict go
OK  120-hitl-receipt-http-endpoint bytes 4472 rows 6 verdict go
OK  121-settings-theme-selector bytes 4233 rows 6 verdict go
OK  122-org-members-rbac-management bytes 4579 rows 6 verdict go
DECIDE_REVISIT_OK 62 / 62 verdicts {'go': 62}
114/115 problem.md and concept.md created from define.md/shape.md equivalents
```

`rows` = linhas do scorecard; `bytes` = UTF-8 bytes. Vereditos históricos preservados; nenhuma mudança de veredito.

## Bloco B — gate determinístico

Execução (2026-11-25T04:38:21Z → 04:48:58Z), com `cargo clean` no target da worktree, `TMPDIR` gravável e `CARGO_BUILD_JOBS=2`:

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
62 decision.md reescritos + 4 artefatos de pre-requisito, paths autorizados
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees alheias intactas; PR #410 (`dev→main`) mesclado por `lfgranja` às 02:56:17Z, não por esta sessão. Nenhum merge de `main`, deploy, auto-merge ou force-push executado aqui.
