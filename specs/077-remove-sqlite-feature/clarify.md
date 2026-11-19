# Clarify: 077 — remoção da feature `sqlite` em flora-core

**Data**: 2026-11-19T19:44Z · **Perguntas feitas**: 0 (R3: máx. 3, uma por vez — não necessárias; nenhuma pergunta carry-forward em `decision.md`).

## Resoluções sem pergunta ao usuário

1. **Identidade da feature (divergência 076 → 077)** — `.specify/feature.json` apontava `specs/076-mint-session-hang` no início do ciclo (resíduo do ciclo anterior). Após criação legítima via `create-new-feature.sh 'remoção da feature sqlite em flora-core' --dry-run --number 077`, passou a apontar `specs/077-remove-sqlite-feature`. Divergência registrada, sem renumeração de trabalho alheio: 076 permanece intacto.
2. **Causa e opção** — a linha `flora-core/Cargo.toml:61` é resíduo inerte: `default = []` (linha 60), sqlx postgres-only (`Cargo.toml:143`), zero consumidores e `.env.example:81` já nega runtime SQLite. A opção A (remover a linha) foi eleita em `decision.md` (go scoped); a opção B (documentar) foi rejeitada por manter a superfície enganosa e a opção C (`deny.toml`) por exigir toque em supply-chain/gate, fora de escopo.
3. **Escopo** — somente a linha 61 de `flora-core/Cargo.toml` + os artefatos do ciclo 077. `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, worktree 073, DB/rede e segredos permanecem intocados; `sqlite_path`/`is_sqlite()` em `database.rs` ficam fora de escopo.
4. **`Cargo.lock`** — `sqlx-sqlite` segue presente como dependência opcional do sqlx (`Cargo.lock:3176,3247,3320`); a remoção da feature não exige mudança de lock, verificável por `cargo metadata --locked --offline`. Podar o lock seria toque fora da fronteira.
5. **[NV]** — extensões de arquivo não varridas, necessidade real de operação offline (decidiria por ADR de projeto, §4 linha 52) e status de CI/produção não são verificados; nenhuma inferência é feita além da evidência local.

## Checklist de requisitos (qualidade, não implementação)

A revisão de clareza, atomicidade e rastreabilidade de FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 está registrada em `spec.md` e em `checklists/requirements.md`; nenhum item é marcado como "implementado" nesta etapa.
