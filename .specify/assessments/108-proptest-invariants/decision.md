# Decision: Invariantes de domínio sob entradas geradas

- **Slug**: 108-proptest-invariants
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Invariantes puros de `flora-core` eram verificados só por exemplos fixos; uma string ou offset arbitrário pode conceder permissão indevida ou romper limite sem o gate perceber (problem.md:9; research.md:12-15). |
| Evidence strength | strong | `proptest` já declarado (`Cargo.toml:208`, `flora-core/Cargo.toml:59`, 1.11.0 em `Cargo.lock`), zero `proptest!` no checkout original e registro de 4 propriedades em 3 módulos com `PROPTEST_CASES=500` (research.md:20,25,32). |
| Value vs. inaction | strong | Sem geração de casos, o gate mede contagem de `#[ignore]`, não sensibilidade, dando falsa segurança à autorização autônoma sob ADR-0003 (problem.md:42). |
| Feasibility / appetite | strong | Appetite small: dependência já presente, três invariantes puros, sem DB, sem contrato novo e sem mudança de gate (concept.md:11; decision.md anterior:5). |
| Strategic fit | strong | Item F7 classificado como ADOPT no assessment multiagente e próximo 80/20 da fila após o INBOX (intake.md:5,19). |
| Risk posture | strong | Risco limitado a testes puros e aos lints `deny` do workspace; nenhum comportamento de produção alterado, nenhum required check novo (decision.md anterior:5-7). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: todos os critérios são sustentados pelos artefatos, com evidência local determinística e risco confinado à suíte de testes. A decisão anterior — dependência já presente, três propriedades, validação com `cargo test -p flora-core` e `check_gate_integrity.py`, nenhuma mudança de gate — permanece íntegra (decision.md anterior:5). A promoção do burn-in a gate, a remoção da dependência duplicada e a política de atualização seguem como perguntas em aberto, não como condição de revisão do veredito. Cobertura gerada é evidência local, não garantia externa; nenhum comportamento de produção foi alterado **[NV]** (decision.md anterior:7).

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; nenhuma pergunta impede o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Tornar visíveis contraexemplos para invariantes puros de permissão, paginação e seletor wildcard sob entradas arbitrárias, sem depender de banco.
- **Chosen approach**: Opção A — quatro propriedades em três módulos (`role.rs`, `models/mod.rs`, `config/bootstrap.rs`) com geração de casos reproduzível (`PROPTEST_CASES=500`), respeitando os lints `deny` e sem tocar gate, dependências ou produção.
- **In scope / out of scope**: Inclui asserções de round-trip/não-concessão de `Permission`, limites de `resolved_limit()` em `0..=100` e equivalência de `is_wildcard_selector()` a `trim() == "*"`; exclui DB, migrations, novos required checks, promoção do burn-in a gate, contrato de código e cobertura ampla de tenant/sessão.
- **Success metrics**: 4 propriedades em 3 módulos; 0 contraexemplos em 500 casos; `cargo test -p flora-core` com 55 passed e 4 passed, fmt/check/clippy exit 0; gate PASS sem novos `#[ignore]`.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quais invariantes são puros e não exigem DB?]; [NEEDS CLARIFICATION: `proptest!` dispara lints `panic`?]; [NEEDS CLARIFICATION: `quickcheck` deve ser removido ou permanece por compatibilidade?]; [NEEDS CLARIFICATION: qual política de atualização da dependência?]; [NEEDS CLARIFICATION: `PROPTEST_CASES=500` é burn-in contínuo ou evidência pontual?].
