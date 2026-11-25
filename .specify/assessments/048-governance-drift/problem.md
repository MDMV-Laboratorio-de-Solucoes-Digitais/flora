# Problem Definition: Governança alinhada ao checkout executável

- **Slug**: 048-governance-drift
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`AGENTS.md:4` afirma 7 membros no workspace (`flora-tasks` e `flora-files` inclusive) e `AGENTS.md:7` fixa a toolchain em `1.98.0`, enquanto `Cargo.toml:19` declara `rust-version = "1.98.1"` e `:26-32` lista 5 membros mais o pacote raiz `flora` — 6 manifests, confirmados por `cargo metadata --no-deps` (intake.md:9-12; research.md:5-10). Como `AGENTS.md` é a instrução governante lida por agentes antes de qualquer tarefa, o erro propaga-se para backlog, verificação e decisões de governança. `flora-tasks`/`flora-files` existem apenas como módulos de rota em `flora-api/src/routes/`, não como crates; o bug `rust-toolchain-pin-bump-1-98-1` documenta UB em 1.98.0 e registra o pin 1.98.1 em 12 pontos como `applied`/`verified` (research.md:11). O teste vermelho executado encontrou 2 linhas desatualizadas; ranqueamento 80/20 = 15.0 (intake.md:14-22).

## Affected Users & Stakeholders

- **Users**: agentes (humanos ou automáticos) que leem `AGENTS.md` antes de uma tarefa e derivam inventário de crates/toolchain do texto, em vez das fontes executáveis (`research.md:20-22`).
- **Stakeholders**: mantenedores do repositório e responsáveis por backlog, gates e decisões de governança, que precisam que a instrução de partida seja factual sem enfraquecer regras (`decision.md:7-9`).

## Goals

- Fazer `AGENTS.md` descrever os 5 membros reais e o pacote raiz `flora` e declarar toolchain `1.98.1` (`define.md:7-9`).
- Corrigir apenas os fatos, sem alterar regra, permissão ou gate (`define.md:10`; `decision.md:7`).
- Anexar notas datadas de estado verificado em `OBJETIVO.md`/`ROADMAP.md`, preservando o histórico (`define.md:11`; `shape.md:5-7`).
- Verificar a correção por `grep` de ausência, `cargo metadata` e gates do alvo (`define.md:20-23`).

## Non-Goals

- Não alterar constituição, ADRs, workflows, gates, código Rust ou frontend (`define.md:14-16`; `decision.md:11-13`).
- Não reintroduzir `flora-tasks`/`flora-files` como crates nem mudar `Cargo.toml` para coincidir com o texto antigo (`shape.md:10-11`).
- Não tocar PR #262, branches alheias ou trabalho em voo da factory; sem merge, auto-merge, tag, deploy ou force-push (`intake.md:31-33`; `decision.md:12-14`).
- Não reescrever `AGENTS.md` inteiro, para não perder regras úteis (`shape.md:12`).

## Success Metrics

- `grep` por `flora-tasks|flora-files|1.98.0` em `AGENTS.md` → 0 ocorrências (baseline: 2 linhas desatualizadas — intake.md:24-25).
- `cargo metadata --no-deps` → 6 pacotes `flora*`, conferindo com a lista escrita (baseline: texto com 7 membros e nenhum pacote raiz explícito — research.md:9).
- Notas datadas presentes em `OBJETIVO.md`/`ROADMAP.md`, com histórico anterior preservado (baseline: sem nota de estado verificado para este ciclo — define.md:11).
- Gates do alvo e `check_gate_integrity.py` permanecem verdes (baseline: gate verde; mudança é documental — intake.md:33; shape.md:20).
- CI e branch protection ao vivo: **[NV]** — não verificadas nesta sessão (research.md:24; decision.md:17).

## Cost of Inaction

Enquanto `AGENTS.md` descrever um workspace inexistente, cada agente que começa uma tarefa herda um inventário e uma toolchain errados; decisões de backlog e verificação podem continuar apontando para crates removidos e para a versão associada a UB documentada. A correção é de baixo custo e reversível; o estado remoto segue **[NV]**.

## Open Questions

- [NEEDS CLARIFICATION: a edição de `AGENTS.md` passa por revisão deliberada antes de ser considerada concluída?] (`shape.md:15-16`)
- [NEEDS CLARIFICATION: `OBJETIVO.md` e `ROADMAP.md` são os únicos documentos de governança que devem receber a nota datada?] (`define.md:11`)
- [NEEDS CLARIFICATION: algum fluxo externo ainda depende dos nomes `flora-tasks`/`flora-files` como documentation?] (`research.md:12`)
- [NEEDS CLARIFICATION: CI e branch protection no alvo serão observadas em qual PR?] (`decision.md:17`; **[NV]**)
