# 037 — Rust toolchain pin bump 1.98.0 → 1.98.1: decisão 80/20 (pesquisa multilíngue)

**Data:** 2026-11-16. **Tipo:** research + decisão; acompanha o bug flow `.specify/bugs/rust-toolchain-pin-bump-1-98-1/`.
**Escopo:** repositório `flora` (Arquivados), branch `dev` @ `f12a328c4` (pós-ciclo 036).
**Método:** seis subagentes em segundo plano — (1) verificação de fontes primárias EN/PT, (2) corroboração multilíngue ES/FR/DE/ZH/RU/JA/KO, (3) auditoria de estado local e candidatos, (4) procedimento operacional da Dark Factory, (5) descoberta de skills (`npx skills find` / `npx skills use`), (6) execução real dos gates. Skills aplicadas: `research`, `source-driven-development`, `doubt-driven-development`, `mdmv-rust-thin`, `verification-before-completion`, `security-and-hardening`, `discernment-nudge`.
**NV:** branch protections, required checks e merges no remoto não foram verificados ao vivo neste ciclo.

## 1) Decisão

**Próximo ciclo 80/20 (037): substituir o pin exato `1.98.0` por `1.98.1` em CI/workflows, Dockerfile e MSRV declarado.** O ciclo 036 já fechou o bug do gate de testes (`cargo test --workspace`, `rust.yml:35`); o próximo maior retorno com menor risco é remover uma UB documentada no compilador exatamente pinned pelo repositório.

Custo: 13 tokens em 4 arquivos. Risco: medido como baixo (patch release sem novos lints; gates executados verdes sob 1.98.1 e 1.99.1). Autorização: edição, verificação e commit local são autônomas; push/PR/merge exigem autorização explícita (guardrail `OBJETIVO.md:94`).

## 2) Por que este e não outro candidato

| Candidato | Alcance | Impacto | Confiança | Esforço | Veredito |
|---|---:|---:|---:|---:|---|
| **A. Pin 1.98.0 → 1.98.1** | todos os builds CI/Docker | 3 | 100% | ~0,01 | **escolhido** |
| B. Mínimos de contagem nos 6 legs `db-tests` | 1 gate | 2 | 100% | 0,2 | bom 2º lugar (bug de filtro que casa 0 testes e sai 0) |
| C. Refrescar docs de planejamento obsoletos | 1 processo | 0,5 | 100% | ~0,01 | higiene; sem efeito em produto |
| D. Required checks / branch protection | governança | 2 | 80% | — | decisão humana, autorização-flagged |
| E. `docker-staging.yml:43` (`secrets` em `if:`) | deploy | 3 | 100% | 0,05 | **bloqueado**: dispara GHCR + webhook Coolify |

O A vence porque a falha foi reproduzida e documentada upstream (rust-lang/rust#161441): `1.98.0` gera vtable de trait object com ponteiro nulo onde deveria haver ponteiro de função. Em código Rust com dyn dispatch (presente no workspace — Axum, SQLx, traits de repositório/serviço), isso é UB com possíveis segfaults. A correção é uma point release cuja única mudança funcional é esse fix.

## 3) Evidência de fontes primárias (verificada)

- **Rust 1.98.1** (blog oficial, 2026-09-03): «Rust 1.98.1 fixes a miscompilation in vtable generation. In Rust 1.98.0, in some circumstances, rustc would incorrectly generate a trait object vtable with a null pointer where a function pointer should be. This leads to undefined behavior in the emitted code.» Único item da release. https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/
- **Issue raiz:** rust-lang/rust#161441 (P-critical, I-miscompile, A-dyn-trait, regression stable→stable): 1.97.1 OK, 1.98.0 SIGSEGV, nightly OK; fix em PR #161555.
- **Linha do tempo:** 1.98.0 (2026-08-20) → **1.98.1 (2026-09-03)** → 1.99.0 (2026-10-01) → 1.99.1 (2026-10-15) → 1.100.0 (2026-11-12, estável atual). https://blog.rust-lang.org/releases/
- **1.99.1** corrige três regressões *diferentes* (wcslen, FFI bool x86_64, rust-lld macOS) — não é o fix desta UB. https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/
- **1.100.0** adiciona lint `unused_dependencies` warn-by-default, mudança de layout de build-dir e never-type (esta última só afeta editions ≤2021; Flora é 2024). Risco desproporcional para um repo arquivado com `-D warnings`. https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/
- **Política semver do Cargo:** patch release = apenas bug fixes; «introducing new lints» é mudança de minor. https://doc.rust-lang.org/cargo/reference/semver.html
- **`rust-version` é piso (MSRV), não pin exato.** https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
- **Rustup overrides:** `channel` aceita `major.minor.patch`; pinar é prática documentada. https://rust-lang.github.io/rustup/overrides.html
- **Tag Docker `rust:1.98.1-slim`** existe e está `active` (amd64 `sha256:fce8354…`, último push 2026-09-26), verificado na API do Docker Hub em 2026-11-16. A tag flutuante `rust:1.98-slim` já resolvia para 1.98.1; fixá-la removes ambiguidade sem risco de tag inexistente.

## 4) Corroboração multilíngue

| Idioma | Achado | Natureza |
|---|---|---|
| **ES** | CodemaDev: 1.98.1 é point fix da vtable nula; recomenda inventariar pins, reconstruir artefatos e **não misturar** o fix com bump não relacionado. | Republica o blog oficial + inferência editorial local (NV como endosso oficial) |
| **FR** | Nenhuma fonte FR encontrada; apenas hits EN (rustup book, Swatinem, Effective Rust). | Sem corroboração independente |
| **DE** | Falhas de ferramenta nas consultas; nenhuma fonde obtida. | **Lacuna de pesquisa** |
| **ZH** | blog.path-finder.jp: drift de binários cargo/rustc por cache de CI quebra builds; recomenda pin coordenado e logar versões no início do CI. | Independente operacionalmente, mas fonte possivelmente gerada por IA — NV |
| **RU** | Digitable: custo concreto do `-D warnings` em `stable` flutuante (novo lint pinta vermelho um commit intocado); duas saídas — pin exato **ou** separar `clippy::correctness` bloqueante de gate informativo. | **Insight independente útil** |
| **JA** | tsumikasane.net: 1.98.1 patch, 0 breaking changes, atualizar pin de CI e rodar testes de dynamic dispatch. | Agregação de notas EN — NV |
| **KO** (bônus) | Daewook Kwon: usar o repro upstream como *compiler canary job*, forçar fronteiras de dynamic dispatch no e2e de release e manter differential stable/beta/N-1. | **Insight independente** |
| **ADRs reais** | QOR-MATRIX ADR-072 pinou 1.98.1 porque 1.99.0 Clippy disparou `clone_on_copy` em macro SDK e quebrou `-D warnings`; scitadel #229 corrigiu drift local 1.95 vs CI 1.98 pinando 1.98.1. | Evidência empírica de que 1.99.x gera atrito de lint |

**Conclusão multilíngue:** nenhuma voz independente recomenda 1.100.0 para este caso; o consenso (inclusive ADRs de terceiros) é patch exato 1.98.1 quando o objetivo é remover a UB sem introduzir churn de lint.

## 5) Execução e verificação (evidência real)

Gates executados no checkout, **antes** da edição (baseline) e **depois** da edição sob `+1.98.1`, registrados em `fix.md`/`test.md` do bug flow:

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace          # 104 passed, 44 ignored, 0 failed
cargo +1.98.1 <mesmos gates>    # idsêntico: 104 passed, 44 ignored, 0 failed
```

A instalação de `1.98.1` com `--profile minimal` não traz `rustfmt`/`clippy`; os componentes foram adicionados explicitamente. **Caveat operacional:** os steps de setup do CI já executam `rustup component add rustfmt clippy` — portanto o gate não é afetado.

Controles advisory da Dark Factory (aplicáveis, não gates do Flora):
- `python3 <DF>/scripts/policy/check_policy.py --command "cargo build" --policy <DF>/scripts/policy/policy.yaml` → `allow`.
- `check_secrets_hygiene.py` redirecionado para escanear o Flora: 5 findings, todos fixtures/docs — nenhum nos 4 arquivos de pin.

Os demais gates do hub (`check_production_gate.py`, `hub_docs_gate.py`, `check_sdd_completeness.py`, `check_policy_drift.py`, holdout/telemetry/runner) são hub-bound e **inaplicáveis** ao Flora (ver `2026-11-16-036-dark-factory-tools-applicability.md`).

## 6) Fontes

Primárias: Rust blog 1.98.1 / 1.99.1 / 1.100.0 / releases; rust-lang/rust#161441 e PR #161555; Rustup overrides; Cargo manifest `rust-version`; Cargo semver; Docker Hub API `library/rust:1.98.1-slim`.
Secundárias/independentes: CodemaDev (ES), Digitable (RU), blog.path-finder.jp (ZH), Daewook Kwon (KO), tsumikasane.net (JA), QOR-MATRIX ADR-072, scitadel#229, Swatinem, Effective Rust.
Internas: `docs/research/2026-11-16-036-rust-pin-bump.md`, `…-036-docker-tag-check.md`, `…-036-dark-factory-tools-applicability.md`, `…-036-authorization-matrix.md`, `…-035-global-progress-and-80-20.md`, `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `AGENTS.md`.
