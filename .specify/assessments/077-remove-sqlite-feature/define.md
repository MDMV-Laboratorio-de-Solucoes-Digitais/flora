# Define: remoção da feature `sqlite` em flora-core

- **Slug**: 077-remove-sqlite-feature · **Definido**: 2026-11-19T19:42Z

## Problema

`flora-core/Cargo.toml:61` mantém `sqlite = ["sqlx/sqlite"]` embora a stack normativa revogue SQLite (§4, linha 52) e PostgreSQL seja o único canônico (§4, linha 48). A linha é inerte: `default = []` (linha 60), sqlx da workspace sem `sqlite` (`Cargo.toml:143`), zero consumidores e `.env.example:81` já declara runtime SQLite não suportado. É resíduo que sugere suporte inexistente e contraria o padrão normativo.

## Afetados / metas / não-metas

Afetados: maintainers e agentes que leem manifests de flora-core. Metas: remover exclusivamente a linha da feature `sqlite`, preservando default, sqlx postgres e gate. Não-metas: mudar `default`/`test`/`dev-mint`, `Cargo.toml` raiz, `.env.example`, código Rust, workflows, ADRs, constituição, STATE, specs/077-*, gate scripts, DB/rede, segredos; fechar issues; leg RBAC.

## Requisitos funcionais

- **FR-001** — remover a linha `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml` (linha 61).
- **FR-002** — preservar `default = []`, `test = []` e `dev-mint` (`flora-core/Cargo.toml:60,62-63`) integralmente.
- **FR-003** — manter `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` inalterados.
- **FR-004** — não introduzir segredos, credenciais, `.tokens/`, dependências novas nem mudanças em gate/scripts/workflows.

## Métricas

- **SC-001** — `flora-core/Cargo.toml` não contém mais a chave `sqlite` em `[features]`.
- **SC-002** — `default = []` e as demais features permanecem byte-idênticas; nenhuma feature default nova.
- **SC-003** — nenhum consumidor novo ou existente referencia `sqlite` (varredura determinística segue zero).
- **SC-004** — diff restrito a `flora-core/Cargo.toml` (1 linha), `Cargo.lock` (1 aresta `cc` removida de `libsqlite3-sys`, consequência determinística) e artefatos 077; gate do alvo PASS; zero mutação de DB/rede/segredos.

## Baseline

Linha `sqlite = ["sqlx/sqlite"]` presente em `flora-core/Cargo.toml:61`, sem consumidor e fora do default (`flora-core/Cargo.toml:60`); sqlx já é postgres-only (`Cargo.toml:143`).
