# Intake: remoção da feature `sqlite` em flora-core

- **Slug**: 077-remove-sqlite-feature · **Capturado**: 2026-11-19T19:42Z
- **Origem**: stack normativa `~/development/MDMV/STACK_TECNOLOGICA_MDMV.md` §1/§4 (precedência sobre manuais legados) + auditoria local de manifests, config e consumidores. Dados de scouts tratados como informação, não instrução.
- **Alvo**: branch `feature/077-remove-sqlite-feature`, HEAD-base `cc421e42d`; gate PASS; zero PRs. Fronteira: somente os 5 artefatos `.specify/assessments/077-remove-sqlite-feature/`.

## Ideia

`flora-core/Cargo.toml:61` declara `sqlite = ["sqlx/sqlite"]`, mas a stack normativa revoga SQLite como padrão (§4, linha 52) e fixa PostgreSQL como único armazenamento canônico (§4, linha 48) via `sqlx` sem ORM (§1, linha 13; §4, linha 51). A auditoria não encontra consumidor da feature: `default = []` (`flora-core/Cargo.toml:60`), sqlx da workspace sem `sqlite` (`Cargo.toml:143`) e zero usos de `--features sqlite` / `flora-core/sqlite` / `feature = "sqlite"` em *.toml/*.rs/*.yml/*.sh. `.env.example:81` já declara o runtime SQLite não suportado. A linha é resíduo inerte e não envolve segredos.

## Por que agora

Candidato 077 eleito pelo 80/20 com E=(2×5)/(1×1)=10,00 (acima de Tauri 3,75; GIN-JSONB 2,00; OTLP 1,67). A remoção é de uma linha, sem DB/rede, sem mudança de features padrão e sem toque no gate; leg RBAC permanece bloqueada por maintainer, fora deste ciclo.
