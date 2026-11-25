# Roteiro de testes — ciclo 126 quota agregada transacional

**Slug**: `126-aggregate-quota` · **Data**: 2026-11-25 · **Base**: `github/dev@cbf39dbb22c0` · **Branch**: `fix/126-aggregate-quota`

## Escopo / IDs

Decisão em `.specify/memory/STATE.md:1395-1399` (impacto4 × confiança4 × fechamento3 × esforço⁻¹2 × risco⁻¹4 = 384/3125). Consumidor: membro/admin que faz uploads sucessivos no mesmo workspace e precisa que `SUM(size_bytes)` ativo + novo arquivo respeite a quota agregada de organização/workspace (P0 `ROADMAP.md:72`, `OBJETIVO.md:69`).

Superfícies: `FileRepository::create_with_quota(file, org_limit_bytes, workspace_limit_bytes)` com transação e `pg_advisory_xact_lock` (`flora-core`), ambos os caminhos de escrita (`upload_file` e `create_file` em `FileService`), contrato DB via HTTP (201 → 413 `{error:"file_too_large"}`), piso `files` 7→8 em lockstep e job `db-tests` do CI. Fora de escopo: restore de soft-deleted sem bytes, purge/storage cleanup, deep-link de busca, frontend, migrations, deploy/promoção.

Contrato compartilhado: a rejeição de quota é `Error::FileTooLarge` mapeado para `StatusCode::PAYLOAD_TOO_LARGE` com `{"error":"file_too_large", ...}`; nenhuma migration nova.

## Pré-condições

- Worktree isolada `.specify/queue/worktrees/lightsout-126`, branch `fix/126-aggregate-quota`, base `github/dev@cbf39dbb22c0` (PR #433 já merged em `main`).
- Postgres com migrations aplicadas, disponível em `FLORA_DATABASE__POSTGRES_URL` (fallback `DATABASE_URL`); contrato DB é `#[ignore]` e usa storage temporário único em `/tmp/flora-test-storage-<uuid>`.
- Tokens JWT sintéticos gerados pelos helpers de teste; nenhuma credencial real em artefato.
- Gate autoritativo: `python3 scripts/ci/check_gate_integrity.py`; piso `files` 7→8 em lockstep com `.github/workflows/rust.yml`, total esperado 88→89.
- Este worker é edit-only: não executa `cargo test`, `cargo check`, fmt, clippy, gate, CI, commit, push, merge ou deploy. Resultados ficam `[NV]` no `evidence.md` até o orquestrador executar.

## Passos

1. **Núcleo transacional** — adicionar `create_with_quota(file, org_limit_bytes, workspace_limit_bytes)` ao trait `FileRepository` e ao `PgFileRepository`, somando `size_bytes` de linhas ativas com `pg_advisory_xact_lock` antes do `INSERT`; `FileService::upload_file` e `create_file` passam os limites de `Config::storage`. Superfícies: `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/file_repository.rs`, `flora-core/src/services/file_service.rs`. Comando de verificação (orquestrador): `cargo check -p flora-core -p flora-api`. Automatizável: sim. Captura: terminal → `evidence.md` Bloco A.
2. **Contrato DB** — `db_aggregate_quota_rejects_second_upload` em `flora-tests/src/contract/files.rs`: quota org/workspace = 10 bytes via override de config em `db_state`, dois uploads de 6 bytes (6+6=12 > 10); esperado 201 no primeiro, 413 + `error == "file_too_large"` no segundo; `find_by_workspace_id` devolve 1 linha ativa; `.tmp` com 0 blobs e diretório `files` com 1 blob. Comando: `FLORA_DATABASE__POSTGRES_URL=<url> cargo test -p flora-tests contract::files::db_files::db_aggregate_quota_rejects_second_upload -- --ignored --test-threads=1`. Automatizável: sim (depende de Postgres). Captura: terminal → Bloco B; se o Postgres local estiver contaminado, registrar `blocked` e usar `db-tests` remoto como autoridade.
3. **Formatação e lints** — `cargo fmt --all -- --check` e `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`. Automatizável: sim. Captura: terminal → Bloco C. Nenhum `unwrap`/`expect` em código de produção; o contrato usa apenas `?`/`ok_or_else`.
4. **Floors em lockstep** — `FLOORS["files"] = 8` e `run_leg files contract::files::db_ 8` em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`. Comando: `python3 scripts/ci/check_gate_integrity.py` (com `TMPDIR` gravável). Esperado: gate verde com 89 pisos — **registrado como `[NV]` até verificação pelo orquestrador**. Automatizável: sim. Captura: terminal → Bloco D.
5. **CI remoto** — required checks no head exato do PR (`rust-build`, `db-tests` em especial). **Esperado: verde; `[NV]` até observação ao vivo pelo orquestrador**; nenhum PASS local é afirmado sem execução. Automatizável: não por este worker. Captura: URL do run → Bloco E.
6. **Higiene** — `git diff --check` e `git status --short`, conferindo apenas os paths do ciclo (3 arquivos core, contrato de arquivos, 2 arquivos de gate, artefatos de validação, `docs/PROGRESSO.md`, `.specify/memory/STATE.md`). Automatizável: sim. Captura: terminal → Bloco F.

## Rollback

Reverter os paths do ciclo via commit reverso explícito (trait/implementação/serviço, contrato DB, floors e workflow, documentos de validação e notas). Nenhuma migration, frontend, config de produção, credencial ou deploy é alterado. Se o job `db-tests` falhar, reverificar o lockstep `files` 8 e a contagem de 89 pisos antes de qualquer promoção; o blob temporário do teste é descartável em `/tmp/flora-test-storage-*`.
