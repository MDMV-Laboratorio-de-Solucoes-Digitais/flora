# Implementation Plan: 077 — remoção da feature `sqlite` em flora-core

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/077-remove-sqlite-feature/{intake,research,define,concept,decision}.md` (go scoped, opção A).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`.

## Passos

1. **T001** Remover exatamente uma linha de `flora-core/Cargo.toml`: `sqlite = ["sqlx/sqlite"]` (linha 61), deixando `[features]` com `default = []` (60), `test = []` (62) e `dev-mint = ["dep:clap", "dep:anyhow"]` (63) byte-idênticos. Validar a estrutura com `tomllib`: chave `sqlite` ausente e demais valores inalterados. *(FR-001, FR-002, SC-001, SC-002)*
2. **T002** Confirmar que `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` permanecem inalterados, e que a varredura determinística por `grep` segue com zero consumidores de `sqlite` (`--features sqlite`, `flora-core/sqlite`, `feature = "sqlite"`, `features = [..sqlite..]` em `*.toml/*.rs/*.yml/*.yaml/*.sh`). *(FR-003, SC-003)*
3. **T003** Verificar coerência offline do manifest e do lock, sem rede/DB: `cargo metadata --locked --offline` (nenhuma mudança de `Cargo.lock` exigida — `sqlx-sqlite` permanece dependência opcional) e `cargo tree -p flora-core --offline` sem `sqlx-sqlite` no grafo ativo. Confirmar que nenhum segredo, `.tokens/`, dependência nova, gate/script ou workflow foi tocado. *(FR-004, SC-003, SC-004)*
4. **T004** Executar os gates determinísticos abaixo (`cargo check -p flora-core --offline`, `check_gate_integrity.py`, auditoria de diff), confirmar gate do alvo PASS e registrar os `[NV]` em `spec.md`/`clarify.md`. *(FR-004, SC-002, SC-004)*

## Gates determinísticos

```
# 1. estrutura de [features] via tomllib (stdlib, sem rede)
python3 - <<'PY'
import tomllib
with open('flora-core/Cargo.toml', 'rb') as f:
    feat = tomllib.load(f)['features']
assert 'sqlite' not in feat, 'sqlite ainda presente'
assert feat['default'] == [], feat['default']
assert feat['test'] == [], feat['test']
assert feat['dev-mint'] == ['dep:clap', 'dep:anyhow'], feat['dev-mint']
print('OK features:', feat)
PY

# 2. zero consumidores e preservação de sqlx postgres-only / .env.example (grep)
! grep -RInE --include='*.toml' --include='*.rs' --include='*.yml' --include='*.yaml' --include='*.sh' \
    -- '--features sqlite|flora-core/sqlite|feature = "sqlite"|features = \[[^]]*sqlite' .
! grep -n 'sqlite' flora-core/Cargo.toml
test "$(sed -n '143p' Cargo.toml | grep -c 'postgres')" -eq 1
sed -n '143p' Cargo.toml | grep -qv 'sqlite'
test "$(sed -n '81p' .env.example | grep -c 'SQLite runtime path is not supported')" -eq 1

# 3. coerência de manifest/lock e grafo, sem rede (Cargo.lock inalterado é esperado)
cargo metadata --locked --offline --no-deps --format-version 1 >/dev/null
! cargo tree -p flora-core --offline | grep -i 'sqlx-sqlite'

# 4. compilação do alvo e integridade de gate
cargo check -p flora-core --offline
python3 scripts/ci/check_gate_integrity.py

# 5. auditoria de escopo do diff (base cc421e42d)
git diff --name-only cc421e42d..HEAD
```

A auditoria de escopo espera somente `flora-core/Cargo.toml` e os artefatos de `specs/077-remove-sqlite-feature/`; qualquer outro path invalida SC-004. Opcionalmente, `cargo metadata --locked --offline` sem `--no-deps` confirma que o lock permanece consistente.

## Rollback

Reverter o commit que introduziu T001, reintroduzindo `sqlite = ["sqlx/sqlite"]` em `flora-core/Cargo.toml:61` e restaurando o manifest original; nenhum `Cargo.toml` raiz, `.env.example`, código Rust, workflow, gate/script, `deny.toml`, ADR, constituição, STATE, DB, rede ou segredo é modificado. Os artefatos do ciclo 077 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.
