# Validation: 039 — Restore drill (NV-1 do Seed backup)

**Data:** 2026-11-16/17 · **Baseline:** `github/dev@c177caa46` · **Tipo:** evidência executada,
sem mudança de código/config/workflow. **Escopo:** provar localmente a tese "VPS barata"
(backup/restore verificável) com o runbook `docs/research/2026-11-16-032-restore-drill-runbook.md`
(§3–§6), perfil lean, stack descartável e isolada.

## Gate B1–B9 (todos PASS)

| # | Gate | Evidência | Resultado |
|---|---|---|---|
| B1 | dump não-zero + checksum | `flora_20261117T023331Z.dump` 49K; `sha256sum -c SHA256SUMS_20261117T023331Z` → SUCESSO (dump + globals) | PASS |
| B2 | PG major ≥ 18 | `server_version_num=180007`, `max_connections=100` | PASS |
| B3 | `pg_restore -1` exit 0 | `pg_restore exit=0`; log mostra `creating ROW SECURITY` (policies restauradas) | PASS |
| B4 | 12/12 contagens iguais (pré-smoke) | `counts-source` == `counts-restored` = `2\|3\|3\|3\|2\|4\|4\|3\|2\|1\|0\|1` | PASS |
| B5 | `ANALYZE` sob `ON_ERROR_STOP=1` | `ANALYZE exit=0` | PASS |
| B6 | restored API `/health/ready` 200 + `test-flora.sh` green | `READY_HTTP=200`; `SMOKE_RC=0`; "Smoke completed."; task criada (`01a1e75b-…`) | PASS |
| B7 | RTO/RPO registrados; nenhum token em artefato | `RTO_seconds=97`; `RPO_source_dump_ts=20261117T023331Z`; só `TOKEN_FINGERPRINT` (sha256 12 chars) nos logs | PASS |
| B8 | projeto/volume descartável removido; outros agentes intactos | `down -v` removeu `flora-restore-postgres-1` + volume; `33001 free`; `floraaudit012-*` e `tirateima_fc-db-1` seguem Up/healthy | PASS |
| B9 | caps assertados após `up` | `memory.max=1073741824`, `cpu.max=100000 100000` (não `max`); `NanoCpus=1000000000` | PASS |

## Metodologia executada (R0–R6)

- **R0** — `docker compose -f docker-compose.yml -f "$ART/compose-override.yml" -p flora-restore`:
  `ports: !override 127.0.0.1:55433:5432`. Config render → `name=flora-restore`,
  `volume=flora-restore_flora_postgres_data`, `mem=1073741824`, `cpus=1`. Ambos `!override` e
  `-p` são load-bearing: sem eles, 5432 seria republicado e o volume real compartilhado.
- **R1** — API efêmera na :33001 aplicou migrations; `dev-seed.sql` + `scripts/dev-smoke-session.sql`
  carregados; 13 tabelas públicas no fim.
- **R2** — `pg_dump -Fc -Z 3` + `pg_dumpall --globals-only` (globals só como evidência, não
  restaurados — C8); checksums sha256 verificados.
- **R3** — `createdb -T template0 flora_restore` + `pg_restore -1 --no-owner`; `ANALYZE`.
- **R4** — diff de 12 contagens ANTES do smoke (o smoke rotaciona sessão e cria task — E8).
- **R5** — API sobre `flora_restore`, perfil lean (`FLORA_SEARCH__ENABLED=false`,
  `FLORA_MESSAGING__ENABLED=false`), JWT HS256 fresco com `jti` do seed, sem ecoar token.
  Nota: o `select-organization` rotaciona a sessão; re-apliquei `dev-smoke-session.sql`
  (upsert documentado) antes da execução final — o smoke passou com `SMOKE_RC=0`.
- **R6** — `down -v` apenas do projeto `flora-restore`; override removido; nenhum stray API.

## Artefatos (fora do repo, `$HOME/.local/state/flora/restore-drill/`)

`flora_20261117T023331Z.dump`, `globals_20261117T023331Z.sql`, `SHA256SUMS_20261117T023331Z`,
`counts-source`, `counts-restored`, `restore_t0`, `last_ts`, logs de dump/restore/API e
`restore-smoke-verified.txt`. Nenhum dump, token ou segredo versionado.

## Correções ao runbook 032 (confirmadas na execução)

- O `health/ready` responde 200 mas o corpo JSON não é impresso por `jq -r '.status'` em todas
  as execuções (campo ausente); usei código HTTP como gate.
- `PIPESTATUS` não existe em zsh (usei captura direta de `$?`).
- A sessão pré-seleção é rotacionada pelo smoke; re-seed (upsert) é necessário para re-executar.

## NV que permanecem

- **NV-2** — destino de backup off-host (nenhum host/credencial fornecido).
- **NV-5** — restore em cluster *novo* (variante §5, dois projetos) não executado; o drill usa
  um cluster descartável único ( Newcastle: `flora` → `flora_restore` no mesmo cluster).
- **NV-6 parcial** — path `$ART` em btrfs verificado no preflight; escrita real executada.
- **Carga/OOM e uptime externo** — fora de escopo (exige gerador off-box e host 2 vCPU/4 GB).
- **Caps de CPU/RSS reais sob carga** — não medidos; apenas `memory.max`/`cpu.max` do cgroup.

## Conclusão

O NV-1 do Seed backup ("restore drills não executados", `docs/dev-smoke.md:119`) está
**fechado localmente** com evidência executada, redacted e rastreável. A tese "roda em VPS
barata" agora tem prova de restore (RTO 97 s) com perfil lean e limites de 1 vCPU/1 GB
enforcement confirmado — não apenas planos.
