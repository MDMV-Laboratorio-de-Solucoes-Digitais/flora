# Validation: 067 — C6 canário RLS

**Data:** 2026-11-18T15:59Z · **Baseline:** `github/dev@9d5f6656f`
**Tipo:** documentação de desenho autorizada; sem código/gate/produção.

## Gates e verificações (saídas coladas)

| Gate | Resultado |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`; 4 required checks; `all 8 db contract floors intact (51 total)`; exit 0 |
| `check_gate_integrity.py --remote` | `dev`/`main`: live required contexts match; exit 0; `[NV]` runner/Forgejo |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished dev profile ... in 14.64s`; exit 0 |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`; exit 0 (apenas warnings de licenças não-encontradas) |
| `cargo build --workspace` | `Finished dev profile ... in 6m 10s`; exit 0 |
| `cargo test --workspace` | `106 passed, 52 ignored (13 suites)`; exit 0 |
| `bun run check` (apps/frontend) | `svelte-check found 0 errors and 0 warnings`; exit 0 |
| `bun run lint` | `All matched files use Prettier code style!`; exit 0 |
| `bun run test:unit -- --run --project server` | `Test Files 12 passed`, `Tests 42 passed`; exit 0 |
| `bun run build` | exit 0 |
| Verificador isolado (2 passagens) | PASS-WITH-NITS → **PASS** após 7 correções; nenhuma contradição nova |
| `analyze.md` (READ-ONLY, reexecutado) | `missing artifacts: none`; FR/SC 11; T 9 únicos; `go=True`; `[NV] marked` |

**Nota de tooling:** a primeira execução de `cargo test --workspace` encerrou 101
com log vazio (transiente de redirecionamento); a reexecução incremental saiu 0
com a contagem acima. Nenhum teste foi pulado ou relaxado. O wrapper `cargo-queue`
de outra sessão manteve o `flock` por ~17 min; usei a saída documentada
`CARGO_QUEUE_OFF=1`, que não toca em trabalho alheio. `node_modules` frontend foi
reutilizado por symlink a partir do worktree principal (diretório gerado).

## Fronteira verificada

- `git diff --name-only -- flora-core/src flora-api/src .github/workflows Cargo.toml docs/adr .specify/memory/constitution.md` → **vazio**.
- `CREATE POLICY` em runtime: apenas o exemplo **comentado** pré-existente em
  `20240818000001_init_schema.sql:214`; `FORCE` 0; `set_config` 0.
- Nenhuma credencial, deploy, tag, required check ou produção alterado.

## SDD

- R1 `go (scoped)` em `decision.md`, registrado antes do BLOCO 6.
- FR-001..FR-007 e SC-001..SC-004 formulados e evidenciados (checklist revisado).
- T001..T008 concluídos; T009 (PR/CI/merge/limpeza) conduzido no Bloco 7.
- `converge.md` com entrada 1 append-only; nenhuma renumeração.

## R4 — Veredito

**Approved (evidência local determinística; verificação de CI ao vivo no Bloco 7).**
R4 aprovado antes de R7.

## NV

Status final de CI/PR, merge e proteções após o merge, Forgejo, produção, prova
A/B em app real e itens (a)–(d) do ADR-0006: **[NV]** até evidência colada no Bloco 7.
