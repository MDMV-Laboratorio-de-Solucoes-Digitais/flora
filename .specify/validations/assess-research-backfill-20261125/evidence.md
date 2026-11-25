# Evidence — backfill/regularização de research (18 assessments)

**Data/hora**: 2026-11-25T02:25Z · **Base**: `github/dev@e88dc8e4d` · **Worktree**: `.specify/queue/worktrees/assess-research-backfill-20261125`

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | Resolver paths | `.specify dir True symlink False inside True`; idem `.specify/assessments` | PASS | Bloco A |
| 2 | Contar artefatos | 13 novos + 5 modificados = 18 slugs | PASS | Bloco A |
| 3 | Formato canônico | 18/18 com H1, 8 seções e Slug exato | PASS | Bloco A |
| 4 | Evidência | 7–13 fontes por arquivo; `Evidence Against` e `[NEEDS CLARIFICATION:]` presentes | PASS | Bloco A |
| 5 | Gate determinístico | ver Bloco B | ver Bloco B | Bloco B |
| 6 | Higiene | `DIFFCHECK_RC=0`; status com apenas paths autorizados | PASS | Bloco C |
| 7 | Preservação alheia | checkout sujo/worktrees alheias intactos; PR #410 não mesclado | PASS | Bloco C |

## Bloco A — verificação determinística

```text
.specify dir True symlink False inside True
.specify/assessments dir True symlink False inside True
OK  040-seed-ui-debloat bytes 13388 sources 12
OK  041-rust-dead-crates bytes 13240 sources 10
OK  042-real-rbac-list bytes 12684 sources 9
OK  086-rbac-ci-leg bytes 11967 sources 9
OK  087-bootstrap-ci-leg bytes 11875 sources 9
OK  088-dynamic-gate-oracle bytes 12378 sources 8
OK  089-forgejo-mirror-sync bytes 12207 sources 8
OK  090-rustls-security-bump bytes 12278 sources 10
OK  091-governance-ci-audit bytes 11056 sources 10
OK  092-npm-lock-sync bytes 9605 sources 9
OK  093-sveltekit-security-upgrade bytes 11682 sources 13
OK  094-visao-so-pme bytes 9543 sources 9
OK  095-frontend-deploy bytes 10554 sources 10
OK  107-wildcard-one-shot bytes 15288 sources 8
OK  108-proptest-invariants bytes 14379 sources 8
OK  109-comprehension-briefing bytes 13270 sources 8
OK  110-mutation-pilot bytes 13689 sources 7
OK  113-bug-status bytes 13971 sources 9
RESEARCH_BACKFILL_OK 18 / 18
```

`bytes` medidos com `len(text.encode("utf-8"))`; `sources` = bullets em `## Sources`. Nenhum host não-allowlisted foi buscado; quando encontrado, registrado como `auto-refused`.

## Bloco B — gate determinístico

Execução (2026-11-25T02:24:18Z → 02:29:26Z), com `TMPDIR=/home/luis/development/MDMV/projetos/flora/.specify/queue/tmp-research-backfill`:

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

Resultado local à worktree; checks remotos no head exato do PR permanecem `[NV]` até observação ao vivo.

## Bloco C — higiene e preservação

```text
=== STATUS (18 paths) ===
 M .specify/assessments/107-wildcard-one-shot/research.md
 M .specify/assessments/108-proptest-invariants/research.md
 M .specify/assessments/109-comprehension-briefing/research.md
 M .specify/assessments/110-mutation-pilot/research.md
 M .specify/assessments/113-bug-status/research.md
?? .specify/assessments/040-seed-ui-debloat/research.md
?? .specify/assessments/041-rust-dead-crates/research.md
?? .specify/assessments/042-real-rbac-list/research.md
?? .specify/assessments/086-rbac-ci-leg/research.md
?? .specify/assessments/087-bootstrap-ci-leg/research.md
?? .specify/assessments/088-dynamic-gate-oracle/research.md
?? .specify/assessments/089-forgejo-mirror-sync/research.md
?? .specify/assessments/090-rustls-security-bump/research.md
?? .specify/assessments/091-governance-ci-audit/research.md
?? .specify/assessments/092-npm-lock-sync/research.md
?? .specify/assessments/093-sveltekit-security-upgrade/research.md
?? .specify/assessments/094-visao-so-pme/research.md
?? .specify/assessments/095-frontend-deploy/research.md
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Baseline preservado: checkout compartilhado em `fix/promo-355-conflict`, worktrees alheias intactas; nenhum merge, deploy, auto-merge ou force-push executado.
