# Decision: Piloto informativo de mutation testing

- **Slug**: 110-mutation-pilot
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Pisos do gate garantem contagem de testes, não sensibilidade: uma mutação em `Role::has_permission` pode sobreviver sem que `check_gate_integrity.py` perceba (problem.md:9; F6 ADAPT em intake.md:5). |
| Evidence strength | strong | Piloto em `role.rs`: 42 mutantes, 39 `caught`, 1 `missed` (dedup quebrado corrigido), 2 `unviable`, ~16 min; ferramenta já instalada (cargo-mutants 27.1.0) (research.md:20,32-33). |
| Value vs. inaction | strong | O sobrevivente revelou defeito real de dedup (`Option<&str>` vs `Option<&String>`) que os pisos não viam; sem medição, a dívida de sensibilidade permanece invisível (problem.md:42). |
| Feasibility / appetite | adequate | Appetite small, mas com obstáculos operacionais registrados: parser `syn` em `workspace_service.rs:1`, `/tmp` tmpfs de 2 GB e ~16 min por arquivo (research.md:31,38,40). |
| Strategic fit | strong | Enquadramento F6 ADAPT com autorização do SE para instalação/uso nesta rodada; ADR-0003 e `check_gate_integrity.py` permanecem a autoridade (intake.md:5; decision.md anterior:5). |
| Risk posture | adequate | Nenhum required check novo e nenhuma aplicação ao workspace, mas há risco de "mutação como teatro" com `#[mutants::skip]` generalizado e custo de execução não calibrado (problem.md:43). |

## Verdict & Rationale

O veredito histórico **`go`** é preservado: o piloto é informativo, local e compatível com o regime ADAPT; a evidência obtida gerou aprendizado acionável sem alterar governança. Mantêm-se integralmente a decisão anterior — executar o piloto em `role.rs`, registrar taxa de mutantes mortos e sobreviventes como dívida, nenhum required check novo, com ADR-0003 e `check_gate_integrity.py` como autoridade (decision.md anterior:5) — e a ressalva de que aplicação a todo o workspace e uso em CI não estão autorizados neste ciclo (decision.md anterior:7). Mutantes `unviable` não contam como sensibilidade; cobertura de autenticação, tenant ou bootstrap permanece **[NV]**.

## If needs-clarification

- **Blocking questions**: N/A — veredito `go` preservado; as incertezas de escopo não bloqueiam o handoff.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Medir se os testes existentes detectam mutações em lógica de domínio sem tornar o resultado um required check.
- **Chosen approach**: Opção A — piloto informativo em `flora-core/src/models/role.rs` com `--in-diff`, job não-required, sobreviventes registrados como dívida e mutantes inviáveis excluídos do cálculo de sensibilidade.
- **In scope / out of scope**: Inclui remoção do obstáculo do parser, `TMPDIR` com espaço e `--file` relativo à raiz, e correção focal do sobrevivente; exclui job de CI, gate, required checks, aplicação ao workspace, dependências de runtime, produção e exclusões generalizadas que virem teatro.
- **Success metrics**: 39 `caught` de 42 (2 `unviable`); 0 sobreviventes após a correção da dedup (verificação focal 1/1 `caught`); ~16 min por arquivo registrados; `cargo test -p flora-core` com 56 passed, fmt/clippy exit 0, gate PASS; 0 required checks novos.
- **Carried-forward open questions**: [NEEDS CLARIFICATION: qual módulo usar no piloto?]; [NEEDS CLARIFICATION: o resultado vira gate?]; [NEEDS CLARIFICATION: `--in-diff` será configurado em PRs?]; [NEEDS CLARIFICATION: como evitar `#[mutants::skip]` generalizado se o piloto for ampliado?]; [NEEDS CLARIFICATION: a verificação focal 1/1 basta ou é preciso reexecutar os 42 mutantes?].
