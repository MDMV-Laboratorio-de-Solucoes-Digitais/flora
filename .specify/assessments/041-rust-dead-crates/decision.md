# Decision: 041 — Enxugamento de dependências Rust

- **Slug**: 041-rust-dead-crates
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | adequate | Há cruza e members sem call sites no recorte original, mas a premissa "zero call sites" não é estável: `argon2` voltou a ter call site e exemption no checkout atual por decisão do ciclo 096 (intake.md:10; research.md:43; problem.md:9). |
| Evidence strength | adequate | Mapeamento por grep, gates determinísticos e validate com auditoria author-blind, mas confiança overall `medium`, `Cargo.lock` com 4567 linhas contra 4529 históricas e falso positivo P1 por concorrência em árvore em edição (research.md:5,44,47). |
| Value vs. inaction | adequate | Reduz grafo, checkout e configuração por remoção pura, mas nenhum delta de memória/CPU/runtime foi medido — qualquer ganho operacional além do tamanho é inferência (research.md:13,31; problem.md:37-39). |
| Feasibility / appetite | adequate | Medium: exige revalidação em snapshot congelado e contagem do lockfile refeita no estado atual, sem reaproveitar o número histórico (concept.md:11-12,29). |
| Strategic fit | strong | Contrapartida backend do debloat do ciclo 040 para o objetivo "VPS barata", sem mudança de comportamento (research.md:11,21; concept.md:12). |
| Risk posture | adequate | Remover algo que voltou a ser usado e generalizar "aparentemente morto" a crates preservados como `flora-search`; a auditoria só é confiável sobre snapshot congelado (research.md:47-48; concept.md:12-13). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`, com uma ressalva explícita: a afirmação "zero call sites" do recorte aprovado foi contradita no tempo — `argon2` voltou a ter call site em `flora-core/src/services/password_service.rs:10-13` e exemption ativa (ciclo 096), e o `Cargo.lock` atual tem 4567 linhas contra as 4529 registradas (research.md:36-44). Isso não inverte o veredito porque o próprio `concept.md` trata a premissa como algo a revalidar, não como verdade permanente, e recomenda a Option A desde que a verificação rode sobre snapshot congelado e a contagem seja refeita no estado atual (concept.md:10-12,29). O que seria errado é reaproveitar o número histórico como estado atual — isso segue registrado como `[NV]`. O chamado `RbacService` e RLS/least-privilege seguem diferidos por exigirem autoridade do SE (decision.md antigo:11-17).

## If needs-clarification

N/A — veredito `go`. A ressalva sobre a premissa "zero call sites" é tratada como revalidação no handoff, não como pergunta bloqueante.

## If go — Handoff to `/speckit.specify`

- **Problem**: Grafo Rust e checkout carregam dependências e members sem uso no recorte original, mas a premissa de ausência de uso precisa ser revalidada no estado atual por causa de reintroduções deliberadas.
- **Chosen approach**: Option A — remoção pura auditada em snapshot congelado (`concept.md:9-12,29`).
- **In scope / out of scope**: In — remoção das 4 dependências registradas (`dotenv`, `argon2`, `bcrypt`, `rust-i18n`) e dos 2 members órfãos (`flora-tasks`, `flora-files`), no recorte aprovado, com verificação antes/depois sobre snapshot congelado, preservando `dotenvy`, `jsonwebtoken/rust_crypto`, `flora-search` e as isenções ainda necessárias; gates `fmt`, `clippy -D warnings`, `deny`, `test --workspace` (46 passed / 43 ignored) e `vet` idênticos ao baseline. Out — mudança de comportamento de código, backend/migration, ADR/constituição, required-check/proteções, deploy/tag/release/secrets; qualquer afirmação de performance além de tamanho de grafo/configuração; wiring do `RbacService` (`revoke_role` não implementado, `PermissionService` fora do `AppState`, último Owner) e RLS/role least-privilege (app conecta como superuser `postgres`, `FORCE` sem policies zeraria linhas) — ambos `[NV]` por autoridade do SE (decision.md antigo:5-22; research.md:33-34).
- **Success metrics**: Delta de 34 pacotes / 369 linhas do `Cargo.lock` com zero adições **no estado atual revalidado** (número histórico 4898→4529 não é estado atual; checkout com 4567 linhas), com `dotenvy`/`jsonwebtoken/rust_crypto` preservados; gates idênticos ao baseline com 46 passed / 43 ignored; nenhum uso órfão após regeneração; auditoria author-blind sem falsos positivos de concorrência (baseline: achado F-1 P1 classificado como falso positivo; CI remoto [NV] — problem.md:30-35; research.md:35,38,44).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: como confirmar call sites indiretos via `build.rs`/feature-gate/reexport e repetir a checagem após reintroduções como a do ciclo 096]; [NEEDS CLARIFICATION: efeito mensurável em memória/CPU além do tamanho do grafo]; [NEEDS CLARIFICATION: quem autoriza papel least-privilege e políticas de RLS (c041-03)]; [NEEDS CLARIFICATION: o que falta para o wiring do `RbacService`, incluindo assign/revoke e proteção de último Owner (c041-02)]; [NEEDS CLARIFICATION: a contagem 46/43 permanece idêntica após regeneração no estado atual]; [NEEDS CLARIFICATION: as isenções removidas afetam `dotenvy` ou `jsonwebtoken/rust_crypto`?]; [NV: Forgejo/remote CI]; ADR-0003 mantém merge como decisão de máquina sob 4 required checks verdes (decision.md antigo:24-26).
