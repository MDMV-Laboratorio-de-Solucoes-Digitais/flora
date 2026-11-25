# Decision: Remoção da feature `sqlite` em flora-core

- **Slug**: 077-remove-sqlite-feature
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `flora-core/Cargo.toml:61` declara `sqlite = ["sqlx/sqlite"]` embora a stack normativa revogue SQLite (§4, linha 52) e fixe PostgreSQL como único canônico (`intake.md:9`, `research.md:8-11`). |
| Evidence strength | strong | `default = []` (`:60`), sqlx postgres-only em `Cargo.toml:143`, `.env.example:81` nega runtime SQLite e varredura determinística em `*.toml/*.rs/*.yml/*.yaml/*.sh` encontra zero consumidores (`research.md:12-17`). |
| Value vs. inaction | strong | Inação mantém feature enganosa que contradiz `.env.example:81` e a precedência normativa (`STACK_TECNOLOGICA_MDMV.md:6`) (`concept.md:12-14`). |
| Feasibility / appetite | strong | Opção A: remover uma linha, sem DB/rede, sem mudança de default e sem segredos — apetite trivial (`concept.md:5-8`). |
| Strategic fit | strong | Alinhamento direto à stack normativa (§1 linha 13; §4 linhas 48, 51-52) (`research.md:8-11`). |
| Risk posture | strong | Diff de uma linha; `default`, sqlx e gate intactos, nenhuma credencial ou `.tokens/` envolvida (`define.md:11,19`, `research.md:18`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. O candidato 077 foi eleito pelo 80/20 com `E=(2×5)/(1×1)=10,00`, acima de Tauri (3,75), GIN-JSONB (2,00) e OTLP (1,67); o leg RBAC permanece bloqueado por maintainer (`intake.md:10`, `decision anterior:5`). A opção B deixa a superfície enganosa no manifest e a C é desproporcional para uma linha inerte, exigindo toque em supply-chain/gate (`concept.md:11-19`). Extensões de arquivo não cobertas pela varredura são `[NV]`; necessidade offline real seria decidida por ADR de projeto, nunca por padrão (`research.md:18`, `concept.md:25`).

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; a necessidade hipotética de operação offline não é bloqueante e seguiria ADR próprio (§4, linha 52).
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: feature `sqlite` inerte em `flora-core/Cargo.toml:61` sugere suporte que a stack normativa revoga e o runtime não aceita.
- **Chosen approach**: Opção A — deletar a linha 61 de `flora-core/Cargo.toml`, deixando `[features]` com `default = []`, `test = []` e `dev-mint` (`concept.md:5-8`).
- **In scope / out of scope**: dentro = `flora-core/Cargo.toml` + 5 artefatos do ciclo 077 (e a consequência determinística em `Cargo.lock`, uma aresta `cc` removida de `libsqlite3-sys`, cf. SC-004); fora = `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, `STATE.md`, `specs/077-*`, DB/rede, segredos, fechamento de issues e leg RBAC (`concept.md:21-23`, `define.md:11`, `define.md:25`).
- **Success metrics**: FR-001..FR-004 de `define.md:13-19`; SC-001 sem chave `sqlite` em `[features]`; SC-002 `default = []` e demais features byte-idênticas; SC-003 varredura segue zero consumidores; SC-004 diff restrito a `flora-core/Cargo.toml` (1 linha), `Cargo.lock` (1 aresta) e artefatos 077, gate PASS e zero mutação de DB/rede/segredos (`define.md:20-25`).
- **Carried-forward open questions**: nenhuma bloqueante; `[NV]` para extensões de arquivo não varridas; eventual necessidade offline decidida por ADR de projeto (§4, linha 52).
