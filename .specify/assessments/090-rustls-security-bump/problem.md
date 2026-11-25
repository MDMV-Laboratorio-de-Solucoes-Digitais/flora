# Problem Definition: Advisories de rustls no grafo resolvido

- **Slug**: 090-rustls-security-bump
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O plano de job advisory registra quatro alertas canônicos de Rust no `Cargo.lock` raiz, todos em `rustls`: `RUSTSEC-2026-0285` (medium, correção `0.23.45`), `RUSTSEC-2026-0346` (medium, `0.23.46`), `RUSTSEC-2026-0348` (low, `0.23.46`) e `RUSTSEC-2026-0349` (low, `0.23.46`) (`decision.md:16-21`; `docs/research/2026-11-16-028-advisory-job-plan.md:40`). No snapshot do ciclo, o grafo resolvido travava `0.23.43` (`Cargo.lock:2748`), abaixo das versões corretivas, enquanto a isenção cargo-vet já apontava `0.23.46` (`supply-chain/config.toml:1115-1117`): divergência entre o que é executado e o que foi auditado (`decision.md:23-26`). O risco registrado não é apenas a exposição, mas ampliar a correção para além do lockfile e da isenção — alterando a faixa declarada no manifesto, código, workflows, ADRs ou promovendo a mudança a PR/merge/deploy neste ciclo (`decision.md:28-30`). No checkout corrente a entrada está em `Cargo.lock:2787` com `0.23.46`, sem `0.23.43` residual (`research.md:50`); a equivalência "0.23.46 corrige as quatro" não foi verificada de forma independente e o banco RustSec não foi consultado (`research.md:65`); estado ao vivo de PR/CI/produção é **[NV]**.

## Affected Users & Stakeholders

- **Users**: mantenedor/compliance, que precisa fechar os quatro advisories sem alterar faixa declarada, API, gate ou processo de merge (`research.md:51`).
- **Stakeholders**: dono da política de supply-chain e das isenções cargo-vet; dono do gate autoritativo ADR-0003; revisores de segurança; [ASSUMPTION] consumidores da aplicação que dependem de TLS — não identificados nominalmente nos artefatos.

## Goals

- Eliminar a exposição dos quatro advisories no grafo resolvido travado.
- Fazer convergir o grafo e a exceção auditada na mesma versão corrigida (`decision.md:23-26`).
- Preservar a faixa de compatibilidade declarada no manifesto e a ausência de mudança de API (`decision.md:32-41`).
- Manter o gate determinístico e os testes como verificação autoritativa, sem criar checagem de advisories como required check (`decision.md:49-52`; `research.md:55`).
- Manter a correção confinada ao lockfile e à isenção, com registro documental rastreável.

## Non-Goals

- Alterar `Cargo.toml`, código de aplicação, workflows, ADRs, constituição, `validate/`, `release/` ou `.specify/memory/STATE.md`.
- Promover mudança a PR, merge, tag, deploy ou produção neste ciclo (`decision.md` 090, "Fora").
- Afirmar que a versão corrigida resolve cada advisory sem fonte verificada — o host do banco de advisories não foi consultado (`research.md:65`).
- Afirmar estado de CI do PR, aprovações de maintainer, proteção de branch ou ambiente de produção sem **[NV]** (`decision.md:84-87`).

## Success Metrics

- `rustls` no lockfile na versão corretiva, sem ocorrência residual da versão afetada (baseline: `0.23.43` em `Cargo.lock:2748` no snapshot; `0.23.46` em `Cargo.lock:2787` no checkout — `research.md:50`).
- Isenção cargo-vet alinhada ao lock e ao critério registrado (baseline: divergência `0.23.46` na isenção contra `0.23.43` no grafo — `decision.md:23-26`).
- Gate `check_gate_integrity.py` PASS e `cargo test --workspace` exit 0 com zero falhas (baseline: registrado com 10 pisos/58; checkout com 11/87 — `research.md:58,59`).
- `dependency-review` sem achados (baseline: quatro advisories registrados no plano de job advisory; recontagem ao vivo **[NV]** — `research.md:51`).

## Cost of Inaction

Os quatro advisories permanecem no grafo travado que alimenta builds, e a divergência entre a versão executada e a isenção auditada continua a tornar a auditoria enganosa. Sem alinhamento, uma futura checagem de dependências pode bloquear o merge por um achado já conhecido, e a decisão de relaxar o gate — explicitamente rejeitada em ciclos anteriores — passa a parecer alternativa (`research.md:55`).

## Open Questions

- [NEEDS CLARIFICATION: em qual fonte ou ferramenta foi verificado que `0.23.46` corrige efetivamente as quatro advisories, e quem valida essa equivalência?] (`intake.md:37`; `research.md:65`)
- [NEEDS CLARIFICATION: **[NV]** qual é o estado do CI do PR, das aprovações de maintainer, da proteção de branch e do ambiente de produção?] (`intake.md:38`; `decision.md:84-87`)
- [NEEDS CLARIFICATION: o bump altera dependências transitivas de `rustls` ou outro pacote do grafo, mesmo sem mudança de API, dado que a aresta de `getrandom` em `tempfile` foi normalizada?] (`intake.md:39`; `research.md:59`)
- [NEEDS CLARIFICATION: se `dependency-review` ou o gate acusarem achados após a correção, quem decide o descarte ou o seguimento?] (`intake.md:40`)
- [NEEDS CLARIFICATION: a isenção cargo-vet com `criteria = "safe-to-deploy"` continua adequada para a versão corrigida ou exige reavaliação do critério?] (`intake.md:41`)
- [NEEDS CLARIFICATION: como atualizar referências de linha defasadas (`Cargo.lock:2748` → `:2787`) sem reescrever o registro histórico do ciclo?] (`research.md:69`)
