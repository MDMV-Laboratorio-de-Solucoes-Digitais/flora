# Decision: 047 — Lockstep de manifests de release

- **Slug**: 047-release-manifest-lockstep
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md | decision.md (antigo) — sem `problem.md`/`concept.md` no baseline; equivalentes locais em `define.md`/`shape.md`

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `.github/workflows/release.yml:53-61` exige `length == 8` enquanto `cargo metadata` retorna 6 manifests `flora*`, e o teste vermelho local executou com `RED_TEST_EXIT=1` (intake.md:7-15; research.md:5-11). |
| Evidence strength | strong | Evidência primária local executada: comando idêntico ao do workflow contra `/tmp/flora-metadata-047.json`, único consumidor do literal `8` em `.github`, `check_gate_integrity.py` não cobre release (research.md:5-19). |
| Value vs. inaction | strong | A asserção de versão em lockstep fica inalcançável antes de qualquer publicação; correção de 1 linha lidera o ranqueamento 80/20 com C1 = 20.0 (intake.md:5,17-21; define.md:5-8). |
| Feasibility / appetite | strong | Uma linha funcional alterada + comentário, sem abstração nova, reversível por `git revert` do commit único (shape.md:5-7,18-21; define.md:26-27). |
| Strategic fit | adequate | Fecha uma falha de release de forma estrita, mas não resolve as dívidas de fase 043–046, registradas como fora deste ciclo (intake.md:22; define.md:22). |
| Risk posture | strong | Sem superfície HTTP, autoria ou dependências; nenhum estado externo criado e reversão trivial (shape.md:18-21). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: o scorecard é `strong` em validade, evidência executada, valor, apetite e risco, com fit `adequate` pelo escopo estrito. A forma escolhida em `shape.md` — trocar `8` por `6` mantendo a cláusula de lockstep e acrescentar comentário ligando o número aos membros do workspace — é a única que restaura o objetivo do passo sem enfraquecer a proteção; derivar a contagem dinamicamente adicionaria superfície de falha, restaurar `flora-tasks`/`flora-files` reintroduziria crates removidos e remover a checagem enfraqueceria a garantia (shape.md:10-16). A identidade 047 é preservada porque 043–046 já estão ocupados por validations/releases embora `specs/` pare em 042 (decision.md antigo:8-9).

## If needs-clarification

N/A — veredito `go`. Execução real do workflow em GitHub Actions, gatilho de tag e proteção de branch seguem `[NV]`, fora do escopo; nenhuma pergunta bloqueante.

## If go — Handoff to `/speckit.specify`

- **Problem**: O workflow de release falha deterministicamente antes de publicar porque usa 8 manifests onde o workspace real tem 6, tornando inalcançável a verificação de que todos os crates estão na versão da tag.
- **Chosen approach**: Forma escolhida em `shape.md` — trocar o literal `8` por `6` na asserção `jq -e`, preservando a condição de lockstep e acrescentando comentário curto; sem abstração nova (`shape.md:5-8`).
- **In scope / out of scope**: In — apenas a asserção de contagem em `.github/workflows/release.yml` (uma linha funcional + comentário) e nota append-only separada em `OBJETIVO.md`/`ROADMAP.md`. Out — criar/remover crates ou alterar `Cargo.toml`, disparar tag/release/deploy/push/auto-merge, tocar constituição, ADRs ou qualquer outro `.github/workflows/*` além de `release.yml`, corrigir dívidas de fase 043–046 (decision.md antigo:11-17; define.md:17-22).
- **Success metrics**: `jq -e` com o número corrigido retorna exit 0 localmente com versões iguais; `check_gate_integrity.py` continua PASS; diff revisável de 1 linha + comentário (baseline: `RED_TEST_EXIT=1`, `length == 8`, gate integrity PASS — intake.md:12-15; define.md:24-27).
- **Carried-forward open questions**: [NV: execução real do workflow em GitHub Actions, gatilho de tag e permissões de publicação]; [NV: proteção de branch]; FR-002 (lockstep intacto) e FR-003 (comando executável localmente) permanecem requisitos de verificação (define.md:12-14; research.md:24-26). Merge permanece decisão de máquina sob 4 required checks verdes (ADR-0003).
