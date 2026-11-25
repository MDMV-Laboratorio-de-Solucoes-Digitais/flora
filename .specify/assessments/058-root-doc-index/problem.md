# Problem Definition: Índice único de documentação na raiz

- **Slug**: 058-root-doc-index
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

Não há um único ponto de entrada na raiz que conecte visão, roadmap, estado verificado, índice de research e ADRs: a descoberta depende de conhecer os nomes dos arquivos. `test -f index.md` retornou ausente e `ls -1 *.md` listou apenas `AGENTS.md`, `OBJETIVO.md`, `README.md`, `ROADMAP.md` e `guide-to-strict-rust.md` (gitignored), embora o Bloco 4 da orquestração liste `index.md` junto dos demais (intake.md:9-15; research.md:5-6). Os pontos de entrada agora existem — `docs/PROGRESSO.md` e `docs/research/index.md` vieram do ciclo 056 — mas nenhum índice único os conecta; `README.md` descreve o projeto, não o estado verificado (intake.md:11-15; research.md:7-10). Ranqueamento 80/20 = 2.5 (intake.md:24-28).

## Affected Users & Stakeholders

- **Users**: pessoas que chegam ao repositório e precisam encontrar direção, estado verificado e ADRs sem conhecer previamente os nomes dos arquivos (intake.md:9-15; research.md:7-10).
- **Stakeholders**: mantenedores que publicam estado verificado em `PROGRESSO.md` e precisam de um ponto de entrada estável sem duplicar conteúdo (`decision.md:9-13`).

## Goals

- Criar `index.md` na raiz com links relativos válidos para `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `README.md`, `AGENTS.md` e `docs/adr/` (`define.md:7-8`).
- Declarar data de verificação, âncora e **[NV]** para o estado remoto (`define.md:9`; `shape.md:5-7`).
- Não duplicar o conteúdo dos documentos linkados nem suas contagens (`define.md:10`; `shape.md:11`).
- Manter o índice enxuto (≤ 40 linhas) e validar os links por verificador isolado (`define.md:17-19`).

## Non-Goals

- Não editar `README.md`, ADRs, constituição, código, workflows, C5/C6 ou deploy (`define.md:13-14`; `decision.md:12-13`).
- Não criar novos índices em `docs/` — já existem (`define.md:15`).
- Não transformar o índice em resumo exaustivo de specs/assessments nem duplicar contagens de `PROGRESSO.md` (`research.md:20-21`; `shape.md:12`).
- Não alterar decisões de autorização, CI ou governança (`decision.md:11`).

## Success Metrics

- Todos os links relativos resolvem para arquivos/diretórios existentes (baseline: `index.md` ausente — define.md:17; intake.md:9-11).
- Índice com ≤ 40 linhas (baseline: arquivo inexistente — define.md:18; shape.md:5-7).
- Verificador isolado confirma cobertura dos pontos de entrada (baseline: nenhum índice único para verificar — define.md:19).
- Estado verificado e **[NV]** distintos no texto (baseline: `README.md` não declara estado verificado — research.md:7; decision.md:9).
- Gates do alvo permanecem verdes (baseline: gate PASS 8 pisos/50, total derivado de `FLOORS` — intake.md:19; define.md:20).

## Cost of Inaction

Descoberta de documentos continua dependente de conhecimento prévio dos nomes de arquivos; estado verificado, research index e ADRs ficam invisíveis para quem começa no README. O custo é de navegação e confiança documental, não de runtime — nenhum código ou gate é alterado (intake.md:11-15; shape.md:15-17).

## Open Questions

- [NEEDS CLARIFICATION: além dos sete alvos listados, algum outro ponto de entrada deve constar no índice?] (`define.md:7-8`; `research.md:14`)
- [NEEDS CLARIFICATION: qual âncora/data exata deve ser impressa no cabeçalho?] (`define.md:9`; `intake.md:5`)
- [NEEDS CLARIFICATION: estado remoto/CI e aprovação para corrigir ADRs — quando serão observados?] (`decision.md:17`; **[NV]**)
