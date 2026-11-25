# Decision: 048 — Governança alinhada ao executável

- **Slug**: 048-governance-drift
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `AGENTS.md:4` afirma 7 crates (inclui `flora-tasks`/`flora-files` inexistentes) e `:7` fixa toolchain `1.98.0`, enquanto `Cargo.toml:19` declara `1.98.1` e `:26-32` lista 5 membros + raiz — a instrução governante induz decisões erradas (intake.md:7-12; research.md:5-19; problem.md:9). |
| Evidence strength | strong | Teste vermelho executado por `grep`, `cargo metadata --no-deps` com 6 pacotes e bug `rust-toolchain-pin-bump-1-98-1` `applied`/`verified` com UB documentada em 1.98.0 e pin 1.98.1 em 12 pontos (research.md:5-13; intake.md:20-23). |
| Value vs. inaction | strong | Correção de baixo custo e reversível: enquanto o texto estiver errado, cada agente herda inventário e toolchain errados para backlog, verificação e governança (problem.md:38-40). |
| Feasibility / appetite | strong | Small: duas linhas de `AGENTS.md` e notas append-only em `OBJETIVO.md`/`ROADMAP.md`, sem código executável (concept.md:11; shape.md:5-7). |
| Strategic fit | adequate | Alinha a fonte que orienta agentes ao checkout executável e preserva a Decisão A, com escopo deliberadamente estreito (decision.md antigo:12-18; concept.md:12). |
| Risk posture | adequate | Risco operacional baixo, mas risco de governança: o texto governa agentes e exige revisão deliberada; CI e branch protection ao vivo `[NV]` (shape.md:15-19; research.md:24). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: validade, evidência executada, valor e apetite são `strong`, fit e risco `adequate` pelo caráter governante do arquivo. A Option A de `concept.md` — correção factual mínima com notas datadas — atende aos FR de `define.md` (inventário real, toolchain correta, nenhuma regra enfraquecida, notas append-only) e respeita os delimitadores do `decision.md` anterior; a Option C inverteria a autoridade ao fazer o código coincidir com um texto errado e reintroduziria a versão associada a UB (concept.md:27-29; shape.md:10-13). A revisão deliberada da edição e o estado remoto seguem como carried-forward, não bloqueantes do recorte.

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante do recorte aprovado.

## If go — Handoff to `/speckit.specify`

- **Problem**: `AGENTS.md` descreve um workspace inexistente (7 crates, toolchain 1.98.0) e, por ser a instrução lida antes de qualquer tarefa, propaga inventário e versão errados para backlog, verificação e decisões de governança.
- **Chosen approach**: Option A — correção factual mínima com notas datadas (`concept.md:9-12,29`; forma em `shape.md:5-7`).
- **In scope / out of scope**: In — editar duas linhas de `AGENTS.md` (inventário de 5 membros + pacote raiz `flora` = 6 manifests; toolchain `1.98.1`) e anexar notas datadas de estado verificado em `OBJETIVO.md`/`ROADMAP.md` sem apagar histórico. Out — constituição, ADRs, workflows, gates, código Rust/frontend, reintroduzir `flora-tasks`/`flora-files` como crates ou mudar `Cargo.toml` para coincidir com o texto, reescrever `AGENTS.md` inteiro, tocar PR #262, branches alheias ou trabalho da factory, merge/auto-merge/tag/deploy/force-push (decision.md antigo:10-15; define.md:16-20; shape.md:10-13).
- **Success metrics**: `grep` por `flora-tasks|flora-files|1.98.0` em `AGENTS.md` retorna 0 ocorrências; `cargo metadata --no-deps` confere com a lista escrita (6 pacotes); notas datadas presentes com histórico preservado; gates do alvo e `check_gate_integrity.py` verdes; CI/branch protection ao vivo `[NV]` (baseline: 2 linhas desatualizadas, texto com 7 membros, sem nota para este ciclo — problem.md:30-36).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: a edição de `AGENTS.md` passa por revisão deliberada antes de ser considerada concluída?]; [NEEDS CLARIFICATION: `OBJETIVO.md`/`ROADMAP.md` são os únicos documentos a receber a nota datada?]; [NEEDS CLARIFICATION: algum fluxo externo ainda depende dos nomes `flora-tasks`/`flora-files` como documentação?]; [NEEDS CLARIFICATION: CI e branch protection no alvo serão observadas em qual PR? — `[NV]`]; Decisão A (mantida) continua autoritativa (decision.md antigo:16-18).
