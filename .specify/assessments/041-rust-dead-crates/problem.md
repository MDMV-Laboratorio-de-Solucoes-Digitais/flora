# Problem Definition: Peso do grafo de dependências Rust do Seed

- **Slug**: 041-rust-dead-crates
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Como contrapartida backend do debloat de frontend do ciclo 040, o objetivo "VPS barata" esbarra em um grafo Rust e um checkout que carregam quatro dependências (`dotenv`, `argon2`, `bcrypt`, `rust-i18n`) e dois members de workspace órfãos (`flora-tasks`, `flora-files`) sem call sites verificados por grep em `*.rs` e `Cargo.toml`, sem `build.rs`, feature-gate ou reexport (intake.md:10,19; research.md:19; decision.md, "Chosen slice"). O problema não é a remoção em si, mas a confiabilidade da premissa "zero call sites" ao longo do tempo: `argon2` voltou a ter call site e exemption no checkout atual por decisão deliberada do ciclo 096 (research.md:43), o `Cargo.lock` tem 4567 linhas contra as 4529 registradas como resultado histórico do ciclo (research.md:44), e nenhum delta de memória, CPU ou tempo de build foi medido (research.md:13,45). Ao mesmo tempo, as mudanças de maior impacto em segurança — wiring de RbacService e RLS/role least-privilege — foram adiadas por exigirem autoridade do dono/SE (research.md:46). O problema é enxugar o grafo sem generalizar a premissa nem reivindicar ganho além do tamanho de grafo/configuração.

## Affected Users & Stakeholders

- **Users**: maintainer/SE do Flora, que precisa reduzir checkout e grafo em um objetivo de VPS barata; não há evidência de usuário final afetado funcionalmente (research.md:12).
- **Stakeholders**: mantenedores e responsáveis por supply chain — precisam preservar isenções relevantes do cargo-vet; operador de VPS — seria o beneficiário de qualquer ganho operacional, ainda não medido [NV].

## Goals

- Reduzir o peso do grafo de dependências e da configuração do workspace por remoção pura e verificável.
- Manter gates determinísticos idênticos ao baseline e nenhuma mudança de comportamento de código.
- Preservar dependências e isenções que ainda possuem call site, explicitamente `dotenvy` e `jsonwebtoken/rust_crypto`, e não generalizar "zero refs" para crates como `flora-search`.
- Reconhecer reintroduções deliberadas como a do ciclo 096 como parte da premissa, não como exceção silenciosa.

## Non-Goals

- Não afirmar mudança de comportamento, alteração de backend/migration, edição de ADR/constituição, deploy, tag, release ou secrets (decision.md, "Non-goals / non-claims").
- Não reivindicar ganho de performance além de tamanho de grafo/configuração; efeito em memória/CPU/runtime não foi medido.
- Não incluir wiring de RbacService (`revoke_role` não implementado, `PermissionService` fora do `AppState`, último Owner) nem RLS/role least-privilege — ambos dependem de decisão do SE e permanecem [NV] (decision.md, "Deferred").
- Não tratar um crate "aparentemente morto" em um recorte de tempo como prova permanente de ausência de uso (research.md:48).

## Success Metrics

- Delta de 34 pacotes e 369 linhas do `Cargo.lock` com zero adições, com `dotenvy` e `jsonwebtoken/rust_crypto` preservados (baseline: 4898 → 4529 no ciclo; checkout atual com 4567 linhas).
- Gates `fmt`, `clippy -D warnings`, `deny`, `test --workspace` e `vet` idênticos ao baseline, com 46 passed / 43 ignored (baseline: mesma contagem).
- Nenhum uso órfão das dependências e members removidos após a regeneração do `Cargo.lock` (baseline: grep vazio no recorte original).
- Auditoria author-blind executada sobre snapshot congelado, sem falsos positivos de concorrência (baseline: achado F-1 P1 classificado como falso positivo; CI remoto [NV]).

## Cost of Inaction

O grafo continua carregando pacotes sem uso e o checkout permanece mais pesado em um cenário de VPS barata; mais grave, tratar o número histórico do ciclo como estado atual gera afirmações falsas sobre o lockfile e sobre isenções. O custo é peso de checkout e risco de erro de auditoria; qualquer ganho operacional além disso permanece sem medição [NV].

## Open Questions

- [NEEDS CLARIFICATION: como confirmar que as dependências e os members não têm call sites indiretos via `build.rs`, feature-gate ou reexport, e como repetir a checagem após reintroduções como a do ciclo 096?]
- [NEEDS CLARIFICATION: qual efeito mensurável em memória/CPU se espera, além da redução do tamanho do grafo?]
- [NEEDS CLARIFICATION: quem autoriza o papel least-privilege e as políticas de RLS, hoje marcados como [NV] (c041-03)?]
- [NEEDS CLARIFICATION: o que falta para o wiring do RbacService entrar neste escopo, incluindo assign/revoke e proteção de último Owner (c041-02)?]
- [NEEDS CLARIFICATION: a contagem de 46 passed / 43 ignored permanece idêntica após a regeneração do `Cargo.lock` no estado atual?]
- [NEEDS CLARIFICATION: as isenções removidas do cargo-vet afetam `dotenvy` ou `jsonwebtoken/rust_crypto`, que devem ser mantidos?]
