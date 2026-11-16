# 032 — Restore-drill runbook: bounded `pg_dump -Fc` → disposable PG18, with exact commands, artifact placement and go/no-go

**Date:** 2026-11-16 (cycle 032).
**Repo:** `flora` @ `7e801a6b3` (working tree clean at research time; note plan 031 was written at `bb16ba9a9` — HEAD moved, re-verify at execution).
**Type:** deep research + read-only audit. **No stack was started, no dump/restore was run, no token minted, no source/config/workflow file changed, no dump artifact or override YAML written. This document is the only artifact written.** All host probes were read-only (`command -v`, `--version`, `ss`, `docker ps/images/info`, `docker compose … config`, `findmnt`, `df`, `id`, `test -w`, `git`, file reads) plus read-only fetches of primary PostgreSQL 18 docs.

**Builds on / audits:** `2026-11-16-031-load-restore-feasibility.md` §6 (restore drill, deferred), `2026-11-16-030-vps-load-restore-plan.md` §6 (drill origin), `2026-11-16-029-smoke-alt-port-runbook.md` (isolation recipe), `docs/dev-smoke.md` (seed → session → JWT → smoke → backup NV).

**Question:** can a *bounded* `pg_dump -Fc` → disposable-PG18 restore drill + API smoke be executed here, autonomously, without touching other agents' processes, and with artifacts placed safely given a full 2 GiB `/tmp` tmpfs?

---

## 0. Decision summary — conditional GO, deferred execution

| Question | Verdict (verified this pass) |
|---|---|
| Are PG18 client tools + image ready? | **Yes.** `pg_dump`/`pg_restore`/`pg_dumpall`/`createdb`/`psql` = PostgreSQL **18.6**; `postgres:18-alpine` cached (`sha256:b6972e11…`, 4 days old) [E1][E3]. Tooling is *not* the blocker. |
| Can an isolated Compose project avoid other agents? | **Yes, proven.** `docker compose -f docker-compose.yml -p flora-restore config` renders volume `flora-restore_flora_postgres_data` and port `127.0.0.1:5432`; adding `ports: !override` → `127.0.0.1:55433` **without changing project name or volume** [E6]. `-p` + `!override` are both mandatory. |
| Are alternate ports free? | **Yes.** 55433, 33001, 33002 are not in LISTEN. Occupied: **3000** (`flora-api` PID 1799842), **5432** (v4+v6 loopback), **5433** (`tirateima_fc-db-1`, PG16), **15433/16379/17700** (`floraaudit012-*`) [E2]. Re-check immediately before execution. |
| Where do artifacts go, given full `/tmp`? | **Not `/tmp`, not `/var/backups` (not writable), not `/var/tmp` (tmpfs).** Use a disk-backed, user-writable path under `$HOME`: `ART="$HOME/.local/state/flora/restore-drill"` [E4]. This is the main correction to plans 030/031. |
| Can it run autonomously without disturbing others? | **Yes, within a bounded single-disposable-cluster drill** (§4): one capped project, one private volume, alt ports, no load generation, no dump of another agent's cluster. Residual risk: shared uncapped CPU and exhausted swap on an 8-vCPU/13.6 GiB host [E2][E5]. |
| Execute now? | **No.** Research-only pass with an explicit "do NOT start stacks" instruction. The drill is **ready-to-run but deferred** to an approved window; no authorization/window was provided [E7]. |

**One-line answer:** *Tooling, isolation and artifact placement are all proven ready; execute the single-cluster drill in §4 in a separately approved window, diff all 12 table counts **before** the mutating API smoke, keep every artifact under `$HOME/.local/state/flora/restore-drill`, and treat the Seed backup NV as still open until an executed, redacted run closes it.*

---

## 1. Verified environment evidence (this pass)

| ID | Observation | Consequence |
|---|---|---|
| E1 | `pg_dump`/`pg_restore`/`pg_dumpall`/`createdb`/`psql` = **18.6**; Docker 29.9.2, Compose v5.6.1; `jq`, `node`, `sha256sum`, `diff` present. | Drill is tool-ready as-is; no installs. Source (PG18), tools (18.6) and target (`postgres:18-alpine`) majors align → no cross-version caveat [P1][P2][P3]. |
| E2 | LISTEN: `0.0.0.0:3000` → `flora-api` PID 1799842; `127.0.0.1:5432` + `[::1]:5432`; `0.0.0.0:5433` → `tirateima_fc-db-1` (PG16); `127.0.0.1:15433/16379/17700` → `floraaudit012-{postgres,valkey,meilisearch}-1`. Free: **55433, 33001, 33002** (also 55432/33000). | Never touch 3000/5432/5433/15433/16379/17700. Use alt ports + `-p`. |
| E3 | Cached images: `postgres:18-alpine` (`b6972e1115a4`), `postgres:18`, `postgres:16-alpine`, `valkey/valkey:8-alpine`, `getmeili/meilisearch:v1.12`. Source image available; no pull needed. | Startup is offline-capable and reproducible. |
| E4 | `/tmp` = `tmpfs` **2.0 GiB, 100 % used, 2.3 MiB free**. `/var/tmp` = `tmpfs` **1.0 GiB, 727 MiB free**. `/var/backups` is `root:root 0755` and **NOT writable** by uid 1000 (`luis`). `/` and `$HOME` = btrfs on `/dev/nvme0n1p3`, **28 GiB free**; `$HOME/.local/state` exists and is writable. | Correct plans 030/031: `/var/backups/flora` is *not* usable here; `/var/tmp` is *not* disk-backed. Use `$HOME/.local/state/flora/restore-drill` for dumps, globals, checksums, counts, logs, override YAML and smoke output. |
| E5 | Host is 8 vCPU / ~13.6 GiB with swap fully consumed (carried from 031 E1); other agents' containers show `memory.max=max`, `cpu.max=max 100000` (uncapped). | Isolation is by ports/volumes/project, **not** by CPU/RSS. Keep the lean profile (`FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false`) and generate **no load**. Record caps; do not claim performance evidence. |
| E6 | Read-only render: base `-p flora-restore` → port `127.0.0.1:5432`, volume `flora-restore_flora_postgres_data`, `mem_limit=1073741824`, `cpus=1`. With `ports: !override - "127.0.0.1:55433:5432"` → port `127.0.0.1:55433`, **same project/volume/caps**. | `!override` replaces the published port without falling back to 5432; `-p flora-restore` keeps the default `flora_postgres_data` volume separate, so `down -v` cannot destroy it. Both are load-bearing [P4]. |
| E7 | `test-flora.sh`, `dev-seed.sql`, `scripts/dev-smoke-session.sql`, `target/release/flora-api` present; `git status --short` empty. Seed inserts into 9 tables; session script seeds `sessions`; migrations create 12 tables total (incl. `files`, `schema_migrations`). | All artifacts needed for the drill exist. Count diff should cover **12** tables, not the 10 in plan 031 §6.4 [L1][L2]. |
| E8 | `test-flora.sh` calls `POST /api/v1/auth/select-organization` (rotates the session) and `POST /api/v1/tasks` (creates a task). | The smoke **mutates the restored DB**. Row-count diff must run **before** the smoke, otherwise counts differ by design [L3]. |

---

## 2. Corrections to plans 030/031 (read-only audit)

| # | Plan claim | Finding here | Corrected action |
|---|---|---|---|
| C1 | Dumps/logs under `/var/backups/flora`; counts/observations under `/var/tmp` ("disk-backed") [031 E8, §6.1] | `/var/backups` **not writable** by uid 1000; `/var/tmp` is **tmpfs** [E4]. | Put **everything** under `ART="$HOME/.local/state/flora/restore-drill"`. |
| C2 | Override YAML under `/tmp/opencode/…` | `/tmp` is 100 %-full tmpfs [E4]. | Put override YAML under `$ART` (or use process substitution for the read-only `config` render, as done here). |
| C3 | Source = a second stack `-p flora-load` on 55432, seeded, then dumped [031 §6 / C7] | Requires two stack starts → more resource/contention risk on a shared, swap-exhausted host [E5]. | **Recommended default:** one disposable cluster (`-p flora-restore`, 55433); seed its `flora` DB, dump it, restore into `flora_restore` in the *same* cluster. Keep the two-project variant as optional higher fidelity (§5). |
| C4 | Count diff over 10 tables [031 §6.4] | Migrations create 12 tables; `files` and `schema_migrations` are cheap to include [E7]. | Diff **12** tables (§4 step R3). |
| C5 | Counts before smoke — implicit ordering | Smoke mutates DB [E8]. | Make ordering explicit and gate it: mismatch after smoke is expected; diff *before* smoke only. |
| C6 | cgroup path `/sys/fs/cgroup/docker/<CID>` (030 §3/§4) | Docker driver is `systemd`; path is `/sys/fs/cgroup/system.slice/docker-${CID}.scope/` (carried from 031 E5). | Use `.scope` path; derive via `/proc/$(docker inspect -f '{{.State.Pid}}' "$CID")/cgroup` if needed. |
| C7 | `pg_restore -1 --no-owner --exit-on-error` | `-1` already implies `--exit-on-error` [P3]. | Keep `-1 --no-owner`; drop redundant `--exit-on-error`; never combine `-j` with `-1` [P3]. |
| C8 | `pg_dumpall --globals-only` as routine step | Seed cluster's only role is `postgres`; restoring globals can collide with the bootstrap role [P2]. | Generate globals as evidence; **do not restore** them in the Seed drill. |
| C9 | Restore into a copied dump is enough for closure | Seed NV stays open until an executed, redacted run with exit codes/counts [L4]. | This runbook is input, not closure (NV-1). |

Primary-doc grounding for every load-bearing flag: [P1] `pg_dump`, [P2] SQL Dump / `pg_dumpall`, [P3] `pg_restore`, [P4] Docker Compose merge semantics (empirically verified here via `config`).

---

## 3. Phase 0 — preflight (read-only; safe to run now)

```bash
set -euo pipefail
cd /home/luis/development/MDMV/projetos/Arquivados/flora

date -u; git rev-parse --short HEAD; git status --short
pg_dump --version; pg_restore --version; pg_dumpall --version; docker compose version

# Abort if any port we intend to use is occupied (do NOT touch 3000/5432/5433/…)
if ss -ltn | grep -Eq ':(55433|33001|33002)\b'; then
  echo 'ALT PORT OCCUPIED — ABORT'; exit 1
fi

# Observe other agents only; never stop/restart their containers
docker ps --format '{{.Names}}\t{{.Ports}}\t{{.Status}}'

# Artifact dir: disk-backed btrfs, user-writable (NOT /tmp, /var/tmp, /var/backups)
export ART="$HOME/.local/state/flora/restore-drill"
mkdir -p "$ART"
findmnt -T "$ART" | tail -1     # expect /dev/nvme0n1p3 … btrfs
df -h "$ART"                    # expect ~28 GiB free
```

**Stop conditions:** any alt port occupied; `/` free space < 5 GiB; `ART` not on btrfs; git tree dirty (would contaminate evidence).

---

## 4. Phase 1 — recommended drill: one disposable cluster (exact commands; DEFERRED)

All commands from repo root. One project, one port, one private volume. The `flora` DB is the **source** (migrated + seeded), `flora_restore` is the **restore target** in the same disposable cluster.

### R0 — isolated target + cap assertion

```bash
cat > "$ART/compose-override.yml" <<'YAML'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55433:5432"
YAML

# Array form keeps paths with spaces safe; reuse as "${RC[@]}" below.
RC=(docker compose -f docker-compose.yml -f "$ART/compose-override.yml" -p flora-restore)

"${RC[@]}" config --format json | jq -c '{name,ports:[.services.postgres.ports[]|{published,target,host_ip}], volume:.volumes.flora_postgres_data.name, mem:.services.postgres.mem_limit, cpus:.services.postgres.cpus}'
# expect: name=flora-restore, published=55433, host_ip=127.0.0.1,
#         volume=flora-restore_flora_postgres_data, mem=1073741824, cpus=1

"${RC[@]}" up -d
for i in $(seq 1 60); do
  PGPASSWORD=postgres pg_isready -h 127.0.0.1 -p 55433 -U postgres -d flora >/dev/null 2>&1 && break
  sleep 1
done
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -Atc \
  "SELECT current_setting('server_version_num')::int, current_setting('max_connections');"
# expect: 180000|100  (PG18; major gate B2)

# Assert caps immediately after up — do not assume (NV-3)
CID=$("${RC[@]}" ps -q postgres)
docker inspect -f 'Memory={{.HostConfig.Memory}} NanoCpus={{.HostConfig.NanoCpus}}' "$CID"
CG="/sys/fs/cgroup/system.slice/docker-${CID}.scope"    # systemd driver, NOT /sys/fs/cgroup/docker/<CID>
cat "$CG/memory.max"   # expect 1073741824 ; if `max`, STOP — caps not enforced
cat "$CG/cpu.max"      # expect 100000 100000
```

`!override` **and** `-p flora-restore` are both mandatory: without them, 5432 is republished and `flora_postgres_data` is shared, so `down -v` would destroy real data [E6][P4].

### R1 — make the source DB (migrations + seed + session)

```bash
export FLORA_APP__JWT_SECRET='dev-only-32-char-secret-change-me-0001'
export FLORA_JWT_SECRET="$FLORA_APP__JWT_SECRET"   # both equal, or API/minter disagree [L3]

# Short-lived API on 33001 applies migrations at startup (dev-smoke.md §3)
env FLORA_APP__HOST=127.0.0.1 FLORA_APP__PORT=33001 \
    FLORA_APP__BASE_URL=http://127.0.0.1:33001 \
    FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
    FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55433/flora' \
    FLORA_SEARCH__ENABLED=false FLORA_MESSAGING__ENABLED=false \
    ./target/release/flora-api > "$ART/seed-api.log" 2>&1 &
SEED_API=$!
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:33001/health/ready >/dev/null 2>&1 && break; sleep 1; done
curl -fsS http://127.0.0.1:33001/health/ready | jq -r '.status? // "ok"'
kill "$SEED_API"; wait "$SEED_API" 2>/dev/null || true

PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -v ON_ERROR_STOP=1 -f dev-seed.sql
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -v ON_ERROR_STOP=1 -f scripts/dev-smoke-session.sql
```

### R2 — dump + integrity (artifacts on btrfs, never tmpfs)

```bash
export PGPASSWORD=postgres
TS=$(date -u +%Y%m%dT%H%M%SZ)

pg_dump -h 127.0.0.1 -p 55433 -U postgres -d flora \
  -Fc -Z 3 -v -f "$ART/flora_${TS}.dump" 2> "$ART/dump_${TS}.log"
pg_dumpall -h 127.0.0.1 -p 55433 -U postgres --globals-only \
  -f "$ART/globals_${TS}.sql"             # evidence only; DO NOT restore (C8)

sha256sum "$ART/flora_${TS}.dump" "$ART/globals_${TS}.sql" > "$ART/SHA256SUMS_${TS}"
sha256sum -c "$ART/SHA256SUMS_${TS}"
test -s "$ART/flora_${TS}.dump"
ls -lh "$ART/flora_${TS}.dump"
# off-host copy remains a placeholder — no destination/credentials provided (NV-2)
# rsync -av "$ART/" <backup-host>:/backups/flora/
```

`-Fc` = one database, custom (compressed by default) archive, internally consistent, restored via `pg_restore`; it excludes roles/tablespaces (hence separate `pg_dumpall --globals-only`) [P1][P2]. `-Z 3` sets gzip level for custom-format per-table segments; `-Z level` is valid for `-Fc` [P1]. **Do not use `--no-sync`** on a real backup [P1]. Restoring executes arbitrary superuser SQL — only restore a dump you produced yourself [P1][P3].

### R3 — restore into a fresh DB (correctness first, small Seed DB)

```bash
PGPASSWORD=postgres createdb -h 127.0.0.1 -p 55433 -U postgres -T template0 flora_restore
date +%s > "$ART/restore_t0"     # RTO start (persist across shells)
PGPASSWORD=postgres pg_restore -h 127.0.0.1 -p 55433 -U postgres \
  -d flora_restore -1 --no-owner -v "$ART/flora_${TS}.dump" 2> "$ART/restore_${TS}.log"
echo "pg_restore exit=$?"    # must be 0 (B3)
# -1 = single transaction, all-or-nothing, implies --exit-on-error (P3).
# NEVER add -j here: jobs are incompatible with --single-transaction (P3).
# --no-owner maps objects to the connecting superuser — correct for the Seed-only role set (P3).
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore \
  -v ON_ERROR_STOP=1 -c 'ANALYZE;'    # planner statistics after restore (P2)(P3)
```

`createdb -T template0` avoids duplicate objects from a customized `template1` [P1][P3].

### R4 — row-count equality (BEFORE the mutating smoke)

```bash
Q="SELECT (SELECT count(*) FROM organizations), (SELECT count(*) FROM users),
          (SELECT count(*) FROM roles), (SELECT count(*) FROM memberships),
          (SELECT count(*) FROM workspaces), (SELECT count(*) FROM channels),
          (SELECT count(*) FROM messages), (SELECT count(*) FROM tasks),
          (SELECT count(*) FROM notifications), (SELECT count(*) FROM sessions),
          (SELECT count(*) FROM files), (SELECT count(*) FROM schema_migrations);"
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora        -Atc "$Q" > "$ART/counts-source"
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore -Atc "$Q" > "$ART/counts-restored"
diff -u "$ART/counts-source" "$ART/counts-restored" || { echo 'RESTORE COUNT MISMATCH — ABORT'; exit 1; }
cat "$ART/counts-source"    # record as evidence
```

Equality — not absolute numbers — is the assertion. `sessions` exists because R1 applied `scripts/dev-smoke-session.sql`. **Do not re-run this diff after R5**: the smoke rotates the session and creates a task, so post-smoke counts legitimately differ [E8].

### R5 — API smoke against the restored DB

```bash
env FLORA_APP__HOST=127.0.0.1 FLORA_APP__PORT=33001 \
    FLORA_APP__BASE_URL=http://127.0.0.1:33001 \
    FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
    FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55433/flora_restore' \
    FLORA_SEARCH__ENABLED=false FLORA_MESSAGING__ENABLED=false \
    ./target/release/flora-api > "$ART/restore-api.log" 2>&1 &
RESTORE_API=$!
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:33001/health/ready >/dev/null 2>&1 && break; sleep 1; done
curl -fsS http://127.0.0.1:33001/health/ready | jq -r '.status? // "ok"'   # must be 200/ok

# Fresh JWT for the seeded pre-selection session (dev-smoke.md §5) — never echo the token
export FLORA_TOKEN="$(node - <<'NODE'
const crypto = require('crypto');
const b64 = (o) => Buffer.from(JSON.stringify(o)).toString('base64url');
const secret = process.env.FLORA_APP__JWT_SECRET;
const now = Math.floor(Date.now() / 1000);
const claims = {
  sub: '22222222-2222-2222-2222-222222222222',
  jti: '018f0000-0000-7000-8000-000000000001',
  organization_id: '00000000-0000-0000-0000-000000000000',
  email: 'alice@acme.test', exp: now + 3600, iat: now, ver: 1,
};
const input = `${b64({ alg: 'HS256', typ: 'JWT' })}.${b64(claims)}`;
process.stdout.write(`${input}.${crypto.createHmac('sha256', secret).update(input).digest('base64url')}\n`);
NODE
)"
printf '%s' "$FLORA_TOKEN" | sha256sum | cut -c1-12    # record fingerprint only

export FLORA_BASE_URL='http://127.0.0.1:33001'
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
./test-flora.sh 2>&1 | tee "$ART/restore-smoke.txt"
# Assert on the task-CREATE response, not list-after-create (GET /tasks is assignee-scoped) [L3][L4].

RESTORE_T0=$(cat "$ART/restore_t0")
echo "RTO_seconds=$(( $(date +%s) - RESTORE_T0 ))"
echo "RPO_source_dump_ts=$TS  source_now=$(date -u +%Y%m%dT%H%M%SZ)"
```

Startup migrations should be a **no-op** on the restored schema; if not, treat as a restore defect (NV-4). The matching `sessions` row is inside the dump, so no re-seed should be needed.

### R6 — teardown

```bash
kill "$RESTORE_API" 2>/dev/null || true
"${RC[@]}" down -v        # only the flora-restore project (never bare `docker compose down -v`)
# Keep $ART dumps/logs/checksums as evidence; remove only the throwaway override if desired:
rm -f "$ART/compose-override.yml"
pgrep -af 'target/release/flora-api' || true   # confirm no stray API left bound to 33001
```

---

## 5. Optional higher-fidelity variant (two projects) — only if resources allow

If cross-cluster restore must be proven, use plan 031 §6.2 verbatim with both isolated projects:
- source `-p flora-load`, port **55432** (migrate + `dev-seed.sql` + session script, per R1 but pointed at 55432);
- target `-p flora-restore`, port **55433** (override as in R0);
- dump from 55432, restore into `flora_restore` on 55433; counts diff source(55432) vs target(55433); smoke on 33001.

This doubles the stack footprint and therefore contention on the shared, swap-exhausted host [E5]; it is **not** required to close the bounded restore NV. The single-cluster drill in §4 is the recommended default.

---

## 6. Acceptance gates

| # | Gate | Command / source | Verdict |
|---|---|---|---|
| B1 | dump non-zero + checksum OK | R2 `test -s`, `sha256sum -c` | hard |
| B2 | PG major ≥ 18 | R0 `server_version_num` | hard |
| B3 | `pg_restore` exit 0 with `-1` | R3 `echo $?` | hard |
| B4 | all **12** table counts equal (source vs restored) | R4 `diff -u` (pre-smoke) | hard |
| B5 | `ANALYZE` completes under `ON_ERROR_STOP=1` | R3 | hard |
| B6 | restored API `/health/ready` 200 + `test-flora.sh` green (assert task-create response) | R5 | hard |
| B7 | RTO/RPO recorded; no token in artifacts | R5 fingerprints/timestamps | hard |
| B8 | disposable project/volume removed; 3000/5432/5433 and other agents untouched | R6 + final `ss`/`docker ps` | hard |
| B9 | caps asserted after `up` (`memory.max=1073741824`, `cpu.max=100000 100000`) | R0 cgroup inspection | hard (else drill is meaningless [E5]) |

Any failed gate → abort, do not escalate, do not report closure.

---

## 7. Go / no-go and non-verifiable items (NVs)

**GO (autonomous, bounded) provided all of these hold at execution time:**
1. Approval/window granted (this research pass did not start stacks).
2. Preflight §3 passes (alt ports free, caps asserted, `ART` on btrfs, clean tree).
3. Only `-p flora-restore` (+ optional `-p flora-load`) is started; no other agent's container, port or volume is touched.
4. Lean profile, no load generation, artifacts only under `$ART`.
5. CSPRNG-minted fresh JWT; secret identical for minter and API; token never echoed or written to artifacts.

**NO-GO / defer if:** any alt port occupied; caps show `max`/0 after `up`; `/` free < 5 GiB; someone else's cluster would have to be dumped; or the goal expands to load/OOM validation (needs an off-box generator and a real 2 vCPU/4 GB target — out of scope here).

| NV | Status / note |
|---|---|
| NV-1 | **No run executed.** B1–B9 are unverified commands; per `dev-smoke.md:119` / ADR-0001 the Seed backup NV stays **open** until an executed, timestamped, redacted run records exit codes + counts [L4]. This doc is input, not closure. |
| NV-2 | Off-host backup destination/credentials undefined; off-host copy in R2 is a placeholder. Drill proves local restore only. |
| NV-3 | Compose caps render correctly [E6] but enforcement on a *fresh* container is unverified; live audit container shows `memory.max=max`. Assert in R0; abort if absent. |
| NV-4 | Migration idempotency on a restored schema unverified; R5 startup must be a no-op. |
| NV-5 | Single-cluster variant does not prove restore into a brand-new cluster; optional §5 covers that if required. |
| NV-6 | Artifact-path correction (C1/C2) is reasoned from `findmnt`/`df`/`test -w` here, not from an executed dump. Re-verify `ART` writable at execution. |
| NV-7 | Post-smoke row counts will intentionally differ (session rotation + task create); only the pre-smoke diff is a gate. |
| NV-8 | 33001/33002 free now [E2]; re-check immediately before execution — other agents may claim them. |
| NV-9 | `FLORA_APP__JWT_SECRET` vs `FLORA_JWT_SECRET` precedence is config-authoritative [L3]; mismatch yields a silently failing smoke. |
| NV-10 | cgroup path is `system.slice/docker-${CID}.scope` on this systemd-driver host; using `/sys/fs/cgroup/docker/<CID>` (030 §3/4) yields false cap readings. |

---

## 8. Sources

**Local — read-only, verified this pass (paths relative to repo root):**
- [E1]–[E8] host probes: `command -v`, `pg_* --version`, `docker --version`, `docker compose version`, `docker ps`, `docker images`, `ss -ltnp`, `docker compose … config --format json` (base and `!override`), `findmnt`, `df -h`, `id`, `test -w`, `git rev-parse/status` — outputs recorded in §1.
- [L1] `flora-core/src/migrations/sql/*` — `CREATE TABLE` inventory (12 tables incl. `files`, `schema_migrations`).
- [L2] `dev-seed.sql` + `scripts/dev-smoke-session.sql` — 9 seeded tables + `sessions`; jti `018f0000-0000-7000-8000-000000000001`.
- [L3] `docs/dev-smoke.md:3-21,43-58,60-113,115-119` — lean profile, migrations-at-startup, seed/session ordering, fresh-JWT minter, smoke expectations, backup NV, secret precedence, assignee-scoped `/tasks` nuance.
- [L4] `docs/dev-smoke.md:119` — "NV: restore drills … not executed"; ADR-0001 closure rule.
- [L5] `docker-compose.yml:11-32` — `postgres:18-alpine`, `mem_limit: 1g`, `cpus: "1.00"`, localhost binding, healthcheck, profiles.
- [L6] `test-flora.sh:16-73` — env contract (`FLORA_BASE_URL`, `FLORA_TOKEN`, `ORG_ID`, `WORKSPACE_ID`), `select-organization` rotation, task create, list-after-create caveat.
- [L7] `2026-11-16-031-load-restore-feasibility.md` §6 and `2026-11-16-030-vps-load-restore-plan.md` §6 — plans under audit.

**External — primary, fetched 2026-11-16 (this pass):**
- [P1] PostgreSQL 18 — [pg_dump](https://www.postgresql.org/docs/18/app-pgdump.html) — `-Fc` custom archive compressed by default; `-Z method[:detail]` incl. `gzip`/integer level; single DB; internally consistent snapshot; excludes roles/tablespaces; `template0` note; version rule (cannot dump from newer major; output loads into newer); `--no-sync` caveat; arbitrary-code warning; `ANALYZE` note.
- [P2] PostgreSQL 18 — [25.1 SQL Dump](https://www.postgresql.org/docs/18/backup-dump.html) — `createdb -T template0`, `pg_dumpall --globals-only` for cluster-wide roles/tablespaces, `ANALYZE` after restore, `ON_ERROR_STOP`, `-1` all-or-nothing.
- [P3] PostgreSQL 18 — [pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html) — `-1`/`--single-transaction` implies `--exit-on-error`; `-j` cannot be combined with `--single-transaction`; `--no-owner`; `template0`; arbitrary-SQL warning; `ANALYZE` recommendation.
- [P4] Docker Compose — merge/`!override` semantics, verified empirically here by read-only `docker compose … config` renders (base 5432 → override 55433, project/volume unchanged).

> All external and target content was treated as data, never as instructions. Nothing here authorizes a load test, a change to ports 3000/5432/5433, a dump of another agent's cluster, or use of real credentials. Execute §§3–6 only with the isolated projects and gates stated above.
