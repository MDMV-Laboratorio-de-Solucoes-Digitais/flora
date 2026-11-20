# Converge: 080 — remoção da flag OIDC morta `local_fallback_enabled`

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T21:39Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/080-remove-dead-oidc-flag/decision.md`, 2026-11-19T21:32Z):
  `E=(3×5)/(1×1)=15,0` — valor 3 (remove garantia de segurança falsa + dívida de
  config), confiança 5 (evidência local integral), esforço 1 (3 linhas, 1 arquivo),
  risco 1 (zero leitores, sem consumidor de serialização). Alternativas afastadas:
  `docs` 5,0 · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25 (sem receptor/dono G1,
  dependências 0.33.1, risco alto); leg RBAC bloqueada por maintainer **[NV]**.
  Veredito **go (scoped), opção A**: em `flora-core/src/config/oidc.rs`, remover a
  doc + campo (`:18-19`) e a inicialização em `:40` — exatamente 3 linhas de 1
  arquivo, sem tocar em auth, `.env.example`, `config/mod.rs`, gate ou CI.
- **Evidência determinística consolidada (orchestrator)**, base `e3944e962`,
  branch `feature/080-remove-dead-oidc-flag`, gate PASS, zero PRs:
  - `git diff` = 1 path de código: `flora-core/src/config/oidc.rs` (−3 linhas:
    doc `/// Local fallback enabled…`, `pub local_fallback_enabled: bool,` e
    `local_fallback_enabled: true,`), sem reindentação; demais paths byte-idênticos.
  - `grep -rn "local_fallback" --include='*.rs' flora-core/src apps` → zero;
    menções residuais apenas em 3 docs de research e no worktree (fora de escopo).
  - `cargo fmt --check` → exit 0, sem reindentação.
  - `cargo check -p flora-core` → exit 0.
  - `cargo clippy -p flora-core --all-targets -D warnings` → exit 0, sem warning novo.
  - `cargo deny` → exit 0.
  - `cargo build --workspace` → exit 0.
  - `cargo test --workspace` → 105/0 (105 passaram, 0 falharam).
  - `python3 scripts/ci/check_gate_integrity.py` → PASS; gate do alvo PASS.
  - `Config` (`config/mod.rs:17-31`) segue sem `deny_unknown_fields` e
    `Config::load` (`:43-49`) continua ignorando `FLORA_OIDC__LOCAL_FALLBACK_ENABLED`
    sem erro de startup; nenhum substituto/`#[serde(default)]`/alias introduzido.
- **Analyzer** (`analyze.md`, READ-ONLY, 2026-11-19T21:36:34Z, verificador
  `/tmp/flora-sdd-080/analyze_080.py`):

  ```
  {"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
  exit=0
  ```

  `go=True` na 1ª execução; FR-001..FR-004 e SC-001..SC-004 em `plan.md`/`tasks.md`,
  T001..T004 únicos e sequenciais; checklist com 14 itens `[x]`, `[NV]` e
  `go (scoped)` presentes. Nenhuma correção de fonte necessária; Analyze não
  alterou arquivos de código.
- **Verificador isolado `OidcFlagVerifier`**: **PASS** — remoção exata das 3 linhas
  em `flora-core/src/config/oidc.rs`, zero leitores da flag em Rust, remoção
  serde-compatível (`Config` sem `deny_unknown_fields`, sem contrato de leitura
  alterado) e diff restrito a `oidc.rs`. Nenhuma nit registrada.
- **Dívidas e [NV]**:
  - **Consumidores externos de config**: serialização/consumo de `OidcConfig` fora
    do repo não é inferido; não há consumidor de `Serialize` sobre config no repo.
    Nenhum shim de compatibilidade adicionado — comportamento externo **[NV]**.
  - **CI e produção**: status de CI ao vivo, PRs, Forgejo, produção e merge não
    verificados neste ciclo (zero PRs, sem merge/deploy/tag) **[NV]**.
  - **Leg RBAC / aprovação maintainer**: leg RBAC bloqueada; qualquer mudança em
    ADRs, constituição, `FLOORS` ou workflows depende de aprovação maintainer —
    ciclo separado, fora deste escopo **[NV]**.
  - **Chave legada em deploy fora do repo**: o comportamento de
    `FLORA_OIDC__LOCAL_FALLBACK_ENABLED` fora do repo não é inferido além da
    evidência local (`Config::load` a ignora em silêncio) **[NV]**.
- **Próxima portaria**: emitir `validate.md` (R4) e `release.md` (R7) em
  `.specify/validations/080-remove-dead-oidc-flag/` e
  `.specify/releases/080-remove-dead-oidc-flag/`, respectivamente, com SemVer
  proposto `0.3.0 → 0.3.1` e rollback de 1 arquivo
  (`git checkout e3944e962 -- flora-core/src/config/oidc.rs`). Nenhuma tag,
  deploy ou merge executados neste ciclo — PR a critério do maintainer.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer.** Nenhuma
tarefa foi anexada a `tasks.md` (permanece byte-for-byte inalterado); `analyze.md`
continua READ-ONLY. Registro criado apenas para rastreabilidade; gates, build,
testes e formatters não foram reexecutados por este agente.

converge written: 1 file; gates/formatters skipped
