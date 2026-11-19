# Converge: 077 — remoção da feature `sqlite` em flora-core

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T19:52Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/077-remove-sqlite-feature/decision.md`):
  `E=(2×5)/(1×1)=10,00` — candidato 077 eleito sobre as alternativas Tauri (3,75),
  GIN-JSONB (2,00) e OTLP (1,67); leg RBAC bloqueada por maintainer **[NV]**.
  Veredito **go (scoped), opção A**: remover exclusivamente a linha
  `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml:61`,
  preservando `default = []` (60), `test = []` (62), `dev-mint` (63), o sqlx
  postgres-only da workspace (`Cargo.toml:143`), `.env.example:81` e o gate.
- **Evidência determinística consolidada (orchestrator)**, base `cc421e42d`,
  branch `feature/077-remove-sqlite-feature`, gate PASS, zero PRs:
  - `git diff` = 2 paths: `flora-core/Cargo.toml` (1 linha removida) e
    `Cargo.lock` (1 aresta removida — `cc` de `libsqlite3-sys`).
  - `tomllib` → `[features]` com `default/test/dev-mint` intactos e sem chave
    `sqlite`.
  - `grep` de consumidores → somente menções nos artefatos do ciclo 077;
    zero correspondências no código.
  - `cargo metadata --locked --offline` → exit 0.
  - `cargo tree -p flora-core -e features --offline | grep sqlite` → vazio.
  - `cargo check -p flora-core --offline` → exit 0.
  - `python3 scripts/ci/check_gate_integrity.py` → PASS.
- **Analyzer** (`analyze.md`, READ-ONLY): 1ª execução
  `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; 2ª execução
  `artifacts=12, errors=[], fr=4, sc=4, tasks=4, go=true` (exit 0). FR-001..004,
  SC-001..004 e T001..T004 presentes em `plan.md`/`tasks.md`, únicos e
  sequenciais; nenhuma correção de fonte necessária.
- **Verificador isolado `SqliteVerifier`**: **PASS-WITH-NITS** — escopo correto
  (2 paths), nenhum consumidor de `sqlite` no código, `sqlite_path`/`is_sqlite()`
  fora de escopo (não é regressão deste ciclo), `Cargo.lock` coerente com a
  remoção. **Nit**: o drift de `Cargo.lock` (aresta `cc` removida) não era
  esperado pela spec; **fechado** — agora aceito e documentado em `spec.md`,
  `plan.md` (SC-004) e neste converge como atualização determinística da
  resolução, sem poda forçada do lock.
- **Dívidas e [NV]**:
  - Extensões de arquivo não varridas pela varredura determinística
    (`*.toml/*.rs/*.yml/*.yaml/*.sh`); consumidores em outras extensões são
    **[NV]**.
  - Necessidade real de operação offline — decidiria por ADR de projeto
    (§4, linha 52), não avaliada neste ciclo **[NV]**.
  - Status de CI ao vivo, merge, PRs, Forgejo e produção não verificados **[NV]**.
  - Leg RBAC depende de aprovação de maintainer **[NV]**.
- **Próxima portaria**: emitir validate (R4) e release (SemVer proposto
  `0.2.7 → 0.2.8`) em `.specify/validations/077-remove-sqlite-feature/` e
  `.specify/releases/077-remove-sqlite-feature/`, respectivamente; nenhuma tag,
  deploy ou merge executados neste ciclo — PR a critério do maintainer.

converge written: 1 file; gates/formatters skipped
