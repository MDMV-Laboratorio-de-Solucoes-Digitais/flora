# Decision: Bump de segurança rustls 0.23.43 → 0.23.46 (ciclo 090)

- **Slug**: 090-rustls-security-bump
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/090-rustls-security-bump` @ `824ba00e9`
- **Âncora**: `.specify/feature.json` → `specs/090-rustls-security-bump`
- **Artefatos revisados**: `Cargo.toml:189` | `Cargo.lock:2748` | `supply-chain/config.toml:1115-1117` | spec | plan | tasks

## Contexto / Problema

O plano de job advisory (`docs/research/2026-11-16-028-advisory-job-plan.md`,
linha 40) registra os **4 alertas canônicos de Rust** no `Cargo.lock` raiz —
todos em `rustls`:

| Advisory | Severidade | Versão corretiva | Publicado |
|---|---|---|---|
| `RUSTSEC-2026-0285` | medium | `0.23.45` | 2026-11-02 |
| `RUSTSEC-2026-0346` | medium | `0.23.46` | 2026-11-15 |
| `RUSTSEC-2026-0348` | low | `0.23.46` | 2026-11-15 |
| `RUSTSEC-2026-0349` | low | `0.23.46` | 2026-11-15 |

O lockfile registrava `rustls` **`0.23.43`** em `Cargo.lock:2748` — abaixo
das quatro versões corretivas — enquanto a isenção cargo-vet já apontava
`0.23.46` em `supply-chain/config.toml:1115-1117`. Havia, portanto,
divergência entre o grafo resolvido e a exceção auditada. O risco a evitar é
ampliar o bump para além do lockfile: alterar a faixa declarada no manifest
(`Cargo.toml:189`, `rustls = "0.23"`), tocar código, workflows, ADRs,
constituição, `validate/`, `release/`, `STATE.md` ou promover mudança a
PR/merge/deploy/produção neste ciclo.

## Decisão

Adotar **bump patch** de `rustls` `0.23.43` → `0.23.46` restrito ao grafo
resolvido e à isenção cargo-vet, preservando a faixa de compatibilidade
declarada no manifest:

1. **Lockfile é a fonte da correção**: `Cargo.lock:2748` passa a registrar
   `rustls` `0.23.46`; nenhuma ocorrência de `0.23.43` permanece. As quatro
   advisories são fechadas por uma única versão corretiva compatível
   (`0.23.46` ≥ `0.23.45`), sem mudança de API.
2. **Manifest inalterado**: `Cargo.toml:189` continua `rustls = "0.23"`. O
   requisito declarado já admite o patch; reescrevê-lo seria ampliar o escopo
   sem necessidade e invalidar a premissa de bump somente de lockfile.
3. **Isenção alinhada**: `[[exemptions.rustls]]` em
   `supply-chain/config.toml:1115-1117` passa a `version = "0.23.46"`,
   mantendo `criteria = "safe-to-deploy"`. Lock e auditoria vet passam a
   convergir na mesma versão corrigida.
4. **Verificação autoritativa preservada**: gate
   `scripts/ci/check_gate_integrity.py` permanece PASS (oráculo dinâmico do
   ciclo 088: 10 legs / 58 contratos de banco) e `cargo test --workspace`
   fecha com exit 0 e zero falhas; `dependency-review` fica sem achados.
5. **Diff cirúrgico e fora do gate**: alterações de código limitadas a
   `Cargo.lock` + `supply-chain/config.toml`; documentação limitada aos 4
   artefatos do ciclo 090 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).
   Nenhuma proteção de branch, required check, workflow, ADR, constituição,
   `validate/`, `release/` ou `.specify/memory/STATE.md` é tocada.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | 4 advisories RUSTSEC reais sobre `rustls` no lock raiz, com `0.23.43` abaixo das versões corretivas |
| Força da evidência | strong | `Cargo.lock:2748`, `Cargo.toml:189` e `supply-chain/config.toml:1115-1117` verificáveis; gate PASS e testes exit 0 registrados no handoff |
| Valor vs. inação | strong | Fecha 4 alertas de segurança com um patch compatível, sem mudança de faixa ou de API |
| Viabilidade | strong | Uma linha no lock + uma linha na isenção; nenhuma migração, nenhum gate novo |
| Fit estratégico | strong | Mantém o gate autoritativo (ciclo 088) e a disciplina de diff cirúrgico dos ciclos recentes |
| Risco | strong | Faixa `0.23` preservada, critério `safe-to-deploy` preservado, escopo confinado a 2 arquivos de código + 4 artefatos; estado ao vivo marcado `[NV]` |

## Handoff

**Dentro**: `Cargo.lock`, `supply-chain/config.toml` e os quatro artefatos
090 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).

**Fora**: `Cargo.toml`, `.github/workflows/`, `flora-tests/`, `validate/`,
`release/`, `.specify/memory/STATE.md`, ADRs, constituição, código de
aplicação, PRs, merge, tags, deploy e produção.

**Métricas de sucesso**: leitura determinística de `Cargo.lock:2748` =
`0.23.46` (sem `0.23.43` residual) e de
`supply-chain/config.toml:1115-1117` = `0.23.46` / `safe-to-deploy`; gate
integrity PASS (`all 10 db contract floors intact (58 total)`, exit 0);
`cargo test --workspace` exit 0 com zero falhas; `dependency-review` limpo;
diff de código restrito a 2 arquivos. **[NV]**: CI do PR/merge/produção,
aprovações de maintainer, proteção de branch e ambiente de produção não
verificados neste ciclo; gates, build, testes e formatters não reexecutados
pelos agentes de documentação.
