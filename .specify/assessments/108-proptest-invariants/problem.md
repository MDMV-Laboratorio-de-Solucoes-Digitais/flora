# Problem Definition: Invariantes de domínio sem cobertura por entradas arbitrárias

- **Slug**: 108-proptest-invariants
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

Invariantes puros de `flora-core` — round-trip de `Permission` (`flora-core/src/models/role.rs:66`), limites de `Pagination` (`flora-core/src/models/mod.rs:58`) e equivalência do seletor wildcard a `trim() == "*"` (`flora-core/src/config/bootstrap.rs:59`) — eram verificados por exemplos fixos; entradas arbitrárias podem revelar concessão indevida de permissão ou limites incorretos de paginação (define.md:3-5; research.md:12,15,22). A dependência `proptest` já estava declarada (`Cargo.toml:208`, `flora-core/Cargo.toml:59`, resolvida em 1.11.0 em `Cargo.lock:2370-2372`), mas o checkout original registrava zero `proptest!` (research.md:20). A dor não é contagem de testes: o gate compara contagens exatas de `#[ignore]` e não mede a sensibilidade das asserções (research.md:21,39).

A demanda veio da decisão F7 do assessment multiagente (**ADOPT**), com o gatilho "INBOX concluído; próximo 80/20 da fila", e os critérios observáveis citam `resolved_limit()` em `0..=100` independente do offset e não-concessão por string arbitrária (intake.md:5-20; research.md:13-15). Os lints do workspace fixam `unwrap_used`, `expect_used` e `panic` em `deny` (`Cargo.toml:77-79`), restringindo como a evidência pode ser escrita (research.md:38). Cobertura gerada é evidência local, não garantia externa; nenhum comportamento de produção foi alterado **[NV]** (decision.md:7).

## Affected Users & Stakeholders

- **Users**: o SE que priorizou o item F7 após o INBOX e os mantenedores que escrevem e revisam testes de `Role::has_permission`, paginação e seletor de recuperação (intake.md:18-20; research.md:13-15).
- **Stakeholders**: o guardião do gate ADR-0003 e de `check_gate_integrity.py`, que continua como autoridade e não deve ver pisos nem required checks alterados; os revisores das mudanças de autorização, para quem a evidência de não-concessão é insumo; o assessment multiagente que classificou o item como ADOPT (decision.md:5-7; research.md:16,30,50).

## Goals

- Tornar visíveis contraexemplos para os invariantes de permissão, paginação e seletor wildcard sob entradas arbitrárias, sem dependência de DB (research.md:15,22-23).
- Preservar o contrato de não-concessão: strings arbitrárias não podem ampliar `Role::has_permission` (define.md:5; research.md:15).
- Manter `resolved_limit()` dentro de `0..=100` e independente do offset, e `is_wildcard_selector()` equivalente a `trim() == "*"` (research.md:15).
- Respeitar os lints `deny` do workspace ao escrever a evidência (research.md:38).
- Manter pisos e required checks inalterados, com burn-in reprodutível registrado (`PROPTEST_CASES=500`) (research.md:39,41).

## Non-Goals

- Não alterar DB, gate, required checks, dependências novas, contrato de código ou produção (define.md:7; decision.md:7).
- Não substituir revisão humana nem contratos DB: a cobertura de propriedades é evidência local (research.md:50).
- Não citar o diagnóstico pré-implementação "zero `proptest!`" como estado corrente — hoje há três módulos `proptests` (research.md:20,47).

## Success Metrics

- Invariantes com cobertura por geração de casos: alvo 4 propriedades em 3 módulos (`role.rs`, `models/mod.rs`, `config/bootstrap.rs`); baseline: 0 `proptest!` no checkout do artefato original (research.md:20,25).
- Round-trip de `Permission` e não-concessão por strings arbitrárias: alvo 0 contraexemplos em 500 casos; baseline: unknown (research.md:15).
- `resolved_limit()` em `0..=100` e `is_wildcard_selector()` equivalente a `trim()`: alvo 0 contraexemplos; baseline: exemplos fixos, sem geração (research.md:15,22).
- `cargo test -p flora-core` e `PROPTEST_CASES=500`: registro de 55 passed e 4 passed, fmt/check/clippy exit 0; baseline: unknown (handoff registrado, não reexecutado — research.md:32).
- Gate PASS sem novos `#[ignore]`: baseline: 10 legs / 77 pisos no artefato do ciclo; o guarda atual tem 11 entradas / 87, portanto a citação antiga é histórica (research.md:21,49).

## Cost of Inaction

Sem entradas geradas, uma string inesperada pode conceder permissão indevida ou um offset extremo pode produzir limite incorreto sem que os exemplos fixos percebam (define.md:3; research.md:12-15). O gate pode continuar PASS porque mede contagens de `#[ignore]`, não sensibilidade (research.md:21,39), dando falsa segurança à autorização autônoma sob ADR-0003. A dependência `proptest` permanece ociosa enquanto `quickcheck` segue declarada por duplicidade (research.md:20,48), e a dívida de invariantes não verificadas se acumula em domínios sensíveis como RBAC.

## Open Questions

- [NEEDS CLARIFICATION: quais invariantes são puros e não exigem DB?] — a lista registrada cobre os escolhidos, não necessariamente todos os possíveis (intake.md:24; research.md:55).
- [NEEDS CLARIFICATION: `proptest!` dispara lints `panic`?] — os lints estão em `deny`, sem medição isolada da macro; `prop_assert!` foi a resposta prática (intake.md:24; research.md:56).
- [NEEDS CLARIFICATION: `quickcheck` deve ser removido do workspace ou permanece por compatibilidade?] (research.md:57).
- [NEEDS CLARIFICATION: qual política de atualização adotar para `proptest`, dado lock em 1.11.0 e upstream mais recente?] (research.md:58).
- [NEEDS CLARIFICATION: `PROPTEST_CASES=500` é burn-in contínuo ou apenas evidência pontual do ciclo?] (research.md:59).
