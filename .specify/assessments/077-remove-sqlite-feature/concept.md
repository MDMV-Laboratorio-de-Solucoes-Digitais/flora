# Shape: remoção da feature `sqlite` em flora-core

- **Slug**: 077-remove-sqlite-feature · **Recomendado**: Opção A (remover a linha) · **Data**: 2026-11-19T19:42Z

## Opção A — remover `sqlite = ["sqlx/sqlite"]` (recomendada)

- **Sketch**: deletar a linha 61 de `flora-core/Cargo.toml`, deixando `[features]` com `default = []`, `test = []` e `dev-mint`. Appetite: trivial (uma linha).
- **Trade-offs**: alinha o manifest à stack normativa (§4, linha 52) sem tocar dependências, default, sqlx postgres-only (`Cargo.toml:143`) nem gate. Zero consumidores verificados; `default = []` (linha 60) garante que nada muda para quem já compila. Risco: nenhum identificado — a feature não é consumida.
- **Rabbit holes**: mexer em `Cargo.toml` raiz, `.env.example`, `default` ou sqlx; "documentar em vez de remover"; adicionar regra de banimento.

## Opção B — manter e documentar

- **Sketch**: preservar a linha e acrescentar nota de que SQLite é legacy/não suportado (cf. `.env.example:81`).
- **Trade-offs**: zero risco de compilação, mas deixa no manifest uma feature que a stack revoga (§4, linha 52) e que o runtime não aceita — documentação não remove a superfície enganosa. Contradiz a precedência normativa (`STACK_TECNOLOGICA_MDMV.md:6`).

## Opção C — banir via `deny.toml`

- **Sketch**: adicionar regra de banimento (e.g., `bans.deny`) para impedir reintrodução de `sqlx/sqlite`.
- **Trade-offs**: defesa em profundidade útil como medida separada, mas desproporcional aqui: exige toque em supply-chain/gate (fora de escopo), não remove a feature morta e pode exigir exceções em `Cargo.toml:143`. Custo > benefício para uma linha inerte.

## Fora de escopo

`Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, supply-chain/`deny.toml`, ADRs, constituição, STATE, specs/077-*, DB/rede, segredos, fechamento de issues, leg RBAC.

## Premissas

`default = []` (`flora-core/Cargo.toml:60`); sqlx é postgres-only (`Cargo.toml:143`); varredura de consumidores retornou zero; `.env.example:81` já nega runtime SQLite. Se houver necessidade real de operação offline, decide-se por ADR de projeto (§4, linha 52), nunca por padrão.
