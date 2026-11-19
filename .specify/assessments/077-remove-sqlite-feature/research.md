# Research: remoção da feature `sqlite` em flora-core

- **Coletado**: 2026-11-19T19:42Z · fontes: stack normativa, manifests Cargo, config de ambiente, varredura de consumidores.

## Evidência

- `STACK_TECNOLOGICA_MDMV.md:6` — precedência: a stack prevalece sobre manuais que prescrevem SQLite como padrão local.
- `STACK_TECNOLOGICA_MDMV.md:13` — "Postgres for everything", acesso via `sqlx`, **sem ORM**.
- `STACK_TECNOLOGICA_MDMV.md:48` — PostgreSQL é o único armazenamento canônico; não adicionar SQLite por conveniência.
- `STACK_TECNOLOGICA_MDMV.md:51` — acesso exclusivo via `sqlx`; SeaORM revogado; persistência atrás de traits de repositório.
- `STACK_TECNOLOGICA_MDMV.md:52` — "SQLite: removido do padrão"; necessidade offline decide-se por ADR de projeto, nunca por padrão.
- `flora-core/Cargo.toml:59-63` — bloco `[features]`: `default = []` (60), `sqlite = ["sqlx/sqlite"]` (61), `test = []` (62), `dev-mint = ["dep:clap","dep:anyhow"]` (63).
- `flora-core/Cargo.toml:60` — `default = []`: `sqlite` não é feature padrão; removê-la não altera o default.
- `Cargo.toml:143` — `sqlx = { version = "0.9.0", features = ["runtime-tokio","postgres","uuid","chrono","json"] }`: postgres habilitado, `sqlite` ausente na workspace.
- `.env.example:23-29` — database PostgreSQL obrigatório (`FLORA_DATABASE__POSTGRES_URL`); `.env.example:81` — "SQLite runtime path is not supported by the API (PostgreSQL required)."
- Varredura determinística (`grep` em *.toml/*.rs/*.yml/*.yaml/*.sh por `--features sqlite`, `flora-core/sqlite`, `feature = "sqlite"`, `features = [..sqlite..]`): zero correspondências — nenhum consumidor.

## Riscos / lacunas

A linha apenas repassa `sqlx/sqlite` (`flora-core/Cargo.toml:61`); a remoção é local a um manifest e não toca segredos, `.tokens/`, credenciais, runtime, DB nem rede. Aderência ao gate não depende desta linha; leg RBAC depende de maintainer — fora deste assessment. `[NV]` extensões de arquivo não cobertas pela varredura; nenhum consumidor identificado nas extensões varridas.
