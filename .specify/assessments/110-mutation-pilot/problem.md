# Problem Definition: Sensibilidade dos testes desconhecida

- **Slug**: 110-mutation-pilot
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

Os pisos do gate garantem contagem de testes, não sensibilidade: uma mutação em `Role::has_permission` pode sobreviver aos testes sem que `check_gate_integrity.py` perceba (define.md:3; research.md:13,39). A demanda veio da decisão F6 do assessment multiagente, classificada como **ADAPT** — piloto informativo com `--in-diff` e job não-required — com autorização do SE para instalação/uso nesta rodada (intake.md:5-20; research.md:12). O pedido é medir se os testes existentes detectam mutações em lógica de domínio sem tornar o resultado um required check (intake.md:8-16).

A ferramenta já estava instalada (`cargo-mutants 27.1.0`), então nenhuma instalação foi necessária (research.md:20). Dois obstáculos determinísticos foram registrados: o parse de `flora-core/src/services/workspace_service.rs:1` falhava por um doc comment `#!` aceito por rustc mas não por syn (research.md:38), e `/tmp` é tmpfs de 2 GB, estourando ao copiar o workspace — exigindo `TMPDIR` em filesystem com espaço e `--file` com caminho relativo à raiz (research.md:31,40). O piloto em `role.rs` produziu 42 mutantes, 39 `caught`, 1 `missed` e 2 `unviable` em ~16 min; o sobrevivente `replace == with != in Role::add_permission` expôs dedup quebrado (`Option<&str>` comparado a `Option<&String>`), corrigido e verificado de forma focal (research.md:32-33). Aplicação em todo o workspace, CI e promoção a gate não foram autorizados: **[NV]** (decision.md:7; research.md:47).

## Affected Users & Stakeholders

- **Users**: o SE que autorizou o piloto e que recebe a taxa de detecção como dívida registrada; os mantenedores de `flora-core/src/models/role.rs`, cujos testes fixos e proptests do ciclo 108 são o alvo da medição (intake.md:12-20; research.md:23).
- **Stakeholders**: o guardião de ADR-0003 e de `check_gate_integrity.py`, que permanece a autoridade com os quatro required checks; os revisores de mudanças em RBAC/autenticação, para quem mutantes sobreviventes são sinal de risco; o assessment multiagente, que alertou contra "mutação como teatro" com `#[mutants::skip]` generalizado (decision.md:5-7; research.md:16,24,39).

## Goals

- Medir a sensibilidade real dos testes sobre lógica de domínio em `flora-core/src/models/role.rs`, registrando mutantes mortos e sobreviventes como evidência e dívida, sem promover check (intake.md:8-16; decision.md:5-7).
- Remover o obstáculo determinístico do parser que impedia a execução, preservando os lints e a compilação (research.md:38).
- Evitar que a dedup de permissões aceite entradas repetidas sem detecção — o mutante sobrevivente revelou comportamento real corregível (research.md:33).
- Manter gate, required checks, CI e dependências de runtime inalterados, com `cargo fmt/check/clippy/test` exit 0 (research.md:34,39,42).
- Registrar o custo operacional da execução como insumo para decidir sobre incrementalismo (`--in-diff`) (research.md:31,48).

## Non-Goals

- Não criar job de CI, gate, migration, dependência de runtime nem alterar produção; não aplicar o piloto a todo o workspace (decision.md:7; define.md:7).
- Não tratar mutantes `unviable` como sinal de sensibilidade (2 de 42 no registro) (research.md:49).
- Não afirmar cobertura de autenticação, tenant ou bootstrap: a medição é local a um arquivo, e o restante é **[NV]** (research.md:50; decision.md:7).
- Não citar "10 legs / 77 pisos" como estado corrente do guarda — hoje são 11 entradas / 87 (research.md:51).

## Success Metrics

- Mutantes detectados em `role.rs`: 39 `caught` de 42 (2 `unviable`); baseline: unknown, sem medição anterior de sensibilidade (research.md:32,49).
- Mutantes sobreviventes: alvo 0 após a correção da dedup (verificação focal 1/1 `caught`); baseline: 1 (`replace == with != in Role::add_permission`) (research.md:33).
- Tempo de execução do piloto em um arquivo: ~16 min registrados, a ser reduzido se ampliado; baseline: unknown (research.md:32,48).
- `cargo test -p flora-core`: 56 passed; fmt/clippy exit 0; gate PASS; baseline: 10 legs / 77 pisos no registro histórico, guarda atual 11 / 87 (research.md:34,51).
- Required checks novos: alvo 0; baseline: 4 existentes, com ADR-0003 e `check_gate_integrity.py` como autoridade (research.md:16,39).

## Cost of Inaction

Sem medir sensibilidade, um `==` trocado por `!=` em `Role::add_permission` pode sobreviver aos testes e inserir permissões repetidas sem que o gate acuse nada — foi exatamente o que o piloto revelou (research.md:32-33). Pisos altos de `#[ignore]` podem dar aparência de rigor enquanto a lógica de autorização não é realmente verificada, e o custo de ~16 min por arquivo mostra que ampliar sem incrementalismo é caro (research.md:21,48). O risco de "mutação como teatro" com skips generalizados transforma evidência em ruído caso a prática seja promovida sem critério (research.md:24,59).

## Open Questions

- [NEEDS CLARIFICATION: qual módulo usar no piloto?] — a resposta registrada foi `flora-core/src/models/role.rs`, mas a pergunta do intake permanece (intake.md:22; research.md:56).
- [NEEDS CLARIFICATION: o resultado vira gate?] — a resposta registrada foi "não"; promoção depende de orçamento de tempo de CI (intake.md:23; research.md:57).
- [NEEDS CLARIFICATION: `--in-diff` chegará a ser configurado em PRs ou o relatório local permanece como único registro?] (research.md:58).
- [NEEDS CLARIFICATION: como evitar que `#[mutants::skip]` generalizado transforme mutação em teatro se o piloto for ampliado?] (research.md:59).
- [NEEDS CLARIFICATION: o mutante `missed` corrigido exige reexecução completa dos 42 mutantes ou a verificação focal 1/1 basta como evidência?] (research.md:60).
