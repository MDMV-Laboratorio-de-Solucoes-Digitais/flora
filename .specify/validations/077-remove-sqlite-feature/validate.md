# Validation: 077 — remoção da feature `sqlite` em flora-core

**Data**: 2026-11-19T19:52Z · **Branch**: `feature/077-remove-sqlite-feature` · **Baseline**: `dev@cc421e42d`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff --name-only cc421e42d` | `Cargo.lock` · `flora-core/Cargo.toml` — exatamente 2 arquivos, 2 remoções |
| `tomllib` em `flora-core/Cargo.toml` → `[features]` | `OK features: {'default': [], 'test': [], 'dev-mint': ['dep:clap', 'dep:anyhow']}` — chave `sqlite` ausente |
| varredura de consumidores (`grep` em `*.toml/*.rs/*.yml/*.yaml/*.sh`) | zero correspondências no código; apenas menções nos artefatos do ciclo 077 |
| `cargo metadata --locked --offline` | `exit 0` — nenhuma alteração adicional de `Cargo.lock` exigida |
| `cargo tree -p flora-core -e features --offline \| grep -i sqlite` | vazio — `sqlx-sqlite` fora do grafo ativo |
| `cargo check -p flora-core --offline` | `exit 0` |
| `python3 scripts/ci/check_gate_integrity.py` | `PASS` — 4 required checks, `exit 0` |
| `analyze.md` — 1ª execução | `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}` · `exit=0` |
| `analyze.md` — 2ª execução | `artifacts=12, errors=[], fr=4, sc=4, tasks=4, go=true` · `exit=0` |
| Verificador isolado `SqliteVerifier` | **PASS-WITH-NITS** — escopo correto, nenhum consumidor, `sqlite_path`/`is_sqlite()` fora de escopo (não regressão), lock coerente; nit (drift do lock) aceito e documentado nos artefatos → **fechado** |

### Saídas coladas

Diff observado contra a base `cc421e42d` (2 arquivos):

```
flora-core/Cargo.toml | 1 -   remoção de: sqlite = ["sqlx/sqlite"]   (linha 61)
Cargo.lock            | 1 -   remoção de: "cc",                       (aresta de libsqlite3-sys)
2 files changed, 2 deletions(-)
```

Segunda execução do analisador (`/tmp/flora-sdd-077/analyze_077.py`):

```
{"artifacts": 12, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
exit=0
```

## Fronteira

Somente `flora-core/Cargo.toml` (1 linha) e `Cargo.lock` (1 aresta) mudaram no código; todo o restante são artefatos do ciclo 077. `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, `.specify/memory/STATE.md`, worktree 073, DB/rede e `.tokens/` permanecem intocados. `sqlite_path`/`is_sqlite()` em `database.rs` ficam fora de escopo (não são regressão deste ciclo).

## [NV]

Extensões de arquivo não cobertas pela varredura determinística; necessidade real de operação offline (decidiria por ADR de projeto, §4 linha 52); CI ao vivo, Forgejo, aprovações, merge e produção. Nenhuma inferência além da evidência local.
