# Evidence — assessment harness Sanderson (flora)

**Data/hora**: 2026-11-25T01:23:56Z · **Superfície**: CLI/artefatos versionados · **Base**: `github/dev@0ef79ec60` · **Worktree**: `.specify/queue/worktrees/sanderson-selfevolve-20261124`

Roteiro: `.specify/validations/avaliacao-harness-sanderson-flora/test-roadmap.md`. Capturas são saídas de terminal coladas abaixo; não há UI de produto alterada, portanto não há captura de tela (justificativa registrada conforme 0.3).

| Passo | Ação | Saída observada | Resultado | Captura |
|---|---|---|---|---|
| 1 | `Path.resolve()` em `.specify`, `.specify/assessments` e diretório do slug | `is_dir=True is_symlink=False inside=True` para os três | PASS | Bloco A |
| 2 | `stat` dos cinco artefatos | `intake.md 8914`, `research.md 34201`, `define.md 3377`, `shape.md 4323`, `decision.md 15785` bytes | PASS | Bloco A |
| 3 | Asserções de estrutura do intake | `intake-structure: OK` | PASS | Bloco A |
| 4 | Contagem/validação da matriz | `decision-rows=34 invalid=0` | PASS | Bloco A |
| 5 | Veredito e fontes | `verdict-go=True`, `research-urls=90`, cobertura e injeções presentes | PASS | Bloco A |
| 6 | Correções factuais e injeções | seções presentes em `research.md`/`decision.md`/brief | PASS | Bloco A |
| 7 | `python3 scripts/ci/check_gate_integrity.py` | PASS na 2ª execução com `TMPDIR` gravável; 1ª FAIL por `/tmp` 100% (`cargo` rc=101) | PASS | Bloco B |
| 8 | `git diff --check` e `git status --short` | `DIFFCHECK_RC=0`; 12 paths versionados no status final | PASS | Bloco C |
| 9 | Preservação alheia | checkout compartilhado sujo e worktrees alheias inalterados; ledger local da sessão | PASS | Bloco C |
| 10 | PR #410 não mesclado | `410 OPEN main<-0ef79ec... https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/410` | PASS | Bloco D |

## Bloco A — verificação determinística (2026-11-25T01:00:21Z)

```text
.specify is_dir= True is_symlink= False inside= True
.specify/assessments is_dir= True is_symlink= False inside= True
.specify/assessments/avaliacao-harness-sanderson-flora is_dir= True is_symlink= False inside= True
intake.md exists= True bytes= 8914
research.md exists= True bytes= 34201
define.md exists= True bytes= 3377
shape.md exists= True bytes= 4323
decision.md exists= True bytes= 15785
intake-structure: OK
decision-rows= 34 invalid= 0 verdict-go= True
research-urls= 90 coverage= True injections= True
VERIFY_RC=0
```

## Bloco B — gate determinístico do ALVO

Primeira execução (2026-11-25T01:00:14Z → 01:09:24Z): `gate-integrity: FAIL`; `cargo --ignored --list failed (rc=101)` e `discovered no tests (empty oracle)`, com 0 contratos encontrados em todos os 11 pisos. Causa ambiental: `/tmp` estava em 100% de uso (`df -h /tmp` → `2.0G 2.0G 0 100%`); nenhum arquivo de código foi alterado entre execuções.

Segunda execução (2026-11-25T01:10:15Z → 01:23:42Z), com `TMPDIR=/home/luis/development/MDMV/projetos/flora/.specify/queue/tmp-sanderson`:

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

Causa da correção: `/tmp` (tmpfs 2 GB) estava saturado por worktrees temporárias de outras sessões; redirecionar `TMPDIR` para diretório gitignored com espaço disponível restaurou o oráculo. Ambas as execuções são locais à worktree isolada; não substituem checks remotos do PR, que permanecem `[NV]` até observação ao vivo no head exato.

## Bloco C — higiene e preservação

```text
=== DIFF CHECK ===
DIFFCHECK_RC=0
=== STATUS FINAL (2026-11-25T01:23:56Z, após correções da ACR) ===
AM .specify/assessments/avaliacao-harness-sanderson-flora/decision.md
A  .specify/assessments/avaliacao-harness-sanderson-flora/define.md
A  .specify/assessments/avaliacao-harness-sanderson-flora/intake.md
A  .specify/assessments/avaliacao-harness-sanderson-flora/research.md
A  .specify/assessments/avaliacao-harness-sanderson-flora/shape.md
M  .specify/memory/STATE.md
A  .specify/validations/avaliacao-harness-sanderson-flora/evidence.md
A  .specify/validations/avaliacao-harness-sanderson-flora/test-roadmap.md
M  docs/PROGRESSO.md
A  docs/governance/2026-11-25-avaliacao-harness-sanderson-flora.md
A  docs/research/2026-11-24-sanderson-selfevolving-tools.md
M  docs/research/index.md
```

Snapshot anterior (apenas 3 paths untracked) registrava o estado antes da conclusão do assessment; o snapshot acima é o estado final. `git diff --check` e `git diff --cached --check` retornaram RC 0. Baseline do Bloco 0.b preservado: checkout compartilhado em `fix/promo-355-conflict` com arquivos sujos de outras sessões; worktrees `github-workflows-2026-11-24` e `speckit-1.4.3-upgrade` intactas.

## Bloco D — escalonamento

```text
410 OPEN main<-0ef79ec60378612ba7d26bcf4ca206b51ed50e11 https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/410
```

Nenhum merge de `main`, deploy, auto-merge, force-push ou alteração de proteção foi executado. PR #410 permanece escalado ao SWE; a promoção `dev→main` é decisão humana. A revisão agêntica independente é registrada em `.specify/queue/sanderson-acr.md` (runtime gitignored): `changes_requested` com 1 achado HIGH, 1 MEDIUM e 1 LOW, todos corrigidos neste arquivo, em `decision.md` e no snapshot do Bloco C antes do commit final.
