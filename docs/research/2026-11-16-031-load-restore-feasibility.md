# 031 — Load/restore feasibility in the current workspace: what can actually be executed safely here

**Date:** 2026-11-16 (cycle 031).  
**Repo:** `flora` @ `bb16ba9a9` (working tree clean at research time).  
**Type:** research/read-only audit. **No stack was started, no load was generated, no dump/restore was run, no token minted, no source/config/workflow file changed. This document is the only artifact written.** All host probes were read-only (`ss`, `docker ps/inspect/info`, `docker compose config`, `cat` of cgroup files, `command -v`, `df`, `free`, `nproc`, `systemctl --user is-system-running`).  
**Builds on / audits:** `2026-11-16-030-vps-load-restore-plan.md` (plan under audit), `2026-11-16-029-smoke-alt-port-runbook.md` (isolation recipe), `docs/dev-smoke.md` (seed/session/JWT/smoke/backup NV).  
**Question:** given the tools and live processes actually present here, what load/restore work can be executed safely without disturbing other agents, and what must be deferred?

---

## 0. Decision summary

| Question | Answer (verified this pass) |
|---|---|
| Is the plan's host assumption true? | **No.** This is an **8 vCPU / ~13.6 GiB** Fedora host (kernel 7.2.11), not 2 vCPU / 4 GB. Swap is **8.0 GiB used / 208 KiB free**. Only ~2.9 GiB is currently *available*. The plan's CPU-threshold and cap rationale does not transfer verbatim [E1][E2]. |
| Are k6 / hey / wrk / vegeta available? | **No — all four missing.** No cached binaries found. `go`, `gcc`, `make`, `cargo`, `dnf`, `npm` are present, so installation is *possible* but is a system/network change, not a research-pass action [E3]. |
| Are PG18 client tools available? | **Yes.** `psql`/`pg_dump`/`pg_restore`/`pg_dumpall`/`createdb` = PostgreSQL **18.6**, matching the `postgres:18-alpine` image already cached locally [E3][E8]. The restore drill is tool-ready. |
| Is cgroup v2 available? | **Yes** (`cgroup2fs`; `cpuset cpu io memory hugetlb pids rdma misc dmem`; `user@1000.slice` has `cpu io memory pids` delegated). **But** Docker's driver is **systemd**, so container cgroups are `/sys/fs/cgroup/system.slice/docker-<CID>.scope/`, **not** `/sys/fs/cgroup/docker/<CID>/` as plan 030 assumes [E4][E5]. |
| Are caps enforced? | **Not proven; currently false for the other agent's container.** `floraaudit012-postgres-1` has `HostConfig.Memory=0`, `NanoCpus=0`, and `memory.max=max`, `cpu.max=max 100000`. Yet a fresh `config` render for `-p flora-load` / `-p flora-restore` shows `mem_limit=1 GiB`, `cpus=1`. So rendered config is correct but **must be verified after `up`**, never assumed [E5][E6]. |
| Are alt ports free? | **Yes.** 55432 / 55433 / 33000 / 33001 are not in LISTEN. Occupied: **3000** (`flora-api` PID 1799842), **5432** (v4+v6 loopback), **5433** (`tirateima_fc-db-1`, PG16), **15433/16379/17700** (`floraaudit012-*`). A `-p flora-load` / `-p flora-restore` project with `ports: !override` is technically possible [E2][E7]. |
| Is there a safe off-host generator? | **Not in evidence.** No second host was provided. Running a generator on this box invalidates any load claim; a bounded *liveness smoke* is still possible here but is **not** a load/OOM test [E9][E10]. |
| **Execute now?** | **Only the read-only preflight and this document — both done.** A bounded curl smoke and the restore drill are **ready-to-run but deferred**: both require starting an isolated stack, which is outside this research-only pass, and the load claim additionally requires an off-box generator + authorization. |

**One-line answer:** *Tooling for the restore drill is ready (PG18.6 + cached image, isolated project renders correctly); tooling for load is not (no generator binary, no second host); the host is 8 vCPU/13.6 GiB with swap exhausted and other agents' workloads sharing the same uncapped CPUs — so execute nothing beyond read-only evidence now, run the bounded curl smoke and restore drill only in a separately approved execution window, and keep the full 2 vCPU/4 GB load/OOM NV open.*

---

## 1. Environment reality (verified this pass)

| ID | Observation | Consequence |
|---|---|---|
| E1 | `nproc`=8; `docker info` `NCPU`=8, `MemTotal`=14575771648 B ≈ 13.6 GiB; `free -h`: 13 GiB total, 10 GiB used, 162 MiB free, 3.7 GiB buff/cache, **2.9 GiB available**; swap 8.0 GiB total, **8.0 GiB used**, 208 KiB free; kernel `7.2.11-200.fc44.x86_64`. | The "2 vCPU / 4 GB VPS" target does not exist here. A 1 GiB PG cap + 768 MiB API cap + ≥1 GiB OS reserve ≈ 2.75 GiB fits only in *available*, with no swap headroom. Any spike risks reclaim/OOM affecting other agents. |
| E2 | LISTEN: `127.0.0.1:5432` + `[::1]:5432`; `0.0.0.0:3000` → `flora-api` PID 1799842; `0.0.0.0:5433` → `tirateima_fc-db-1` (PG16); `127.0.0.1:15433/16379/17700` → `floraaudit012-{postgres,valkey,meilisearch}-1`. 55432/55433/33000/33001 free. | Do not touch 3000/5432/5433/15433/16379/17700. Use alt ports with `!override` **and** `-p`. |
| E3 | Missing: `k6`, `hey`, `wrk`, `vegeta`. Present: `jq`, `psql`, `pg_dump`, `pg_restore`, `pg_dumpall`, `createdb`, `docker`, `node`, `bun`, `curl` 8.18, `ss`, `rsync`, `ssh`, `systemd-run`, `choom`, `go`, `gcc`, `make`, `cargo`, `dnf`, `npm`. | Restore drill is executable as-is. Load needs either an install (network/system change) or a curl-based bounded smoke substitute. |
| E4 | `/proc/self/cgroup` → `0::/user.slice/user-1000.slice/user@1000.service/app.slice/kitty-...scope`; `/sys/fs/cgroup/cgroup.controllers` = `cpuset cpu io memory hugetlb pids rdma misc dmem`; `user@1000.service/cgroup.controllers` = `cpu io memory pids`; `systemctl --user is-system-running` → **degraded**. | cgroup v2 with delegation exists, but `systemd-run --user --scope -p MemoryMax=768M` is **unverified** on this degraded user manager (NV-4). Verify before relying on it. |
| E5 | `docker info`: cgroup driver `systemd`, `SecurityOptions=["name=seccomp,profile=builtin","name=cgroupns"]` → **rootful** Docker 29.9.2 / Compose v5.6.1. Running container cgroup: `/system.slice/docker-<CID>.scope` (confirmed via `/proc/<PID>/cgroup`), with `memory.max=max`, `cpu.max=max 100000`. | Plan 030 §§3/4 cgroup paths (`/sys/fs/cgroup/docker/<CID>`) are **wrong here**. Use `/sys/fs/cgroup/system.slice/docker-${CID}.scope/`. |
| E6 | `floraaudit012-postgres-1` (started with the repo compose + `/tmp/opencode/flora-audit-ports.yml`, which only overrides ports) shows `Memory=0`, `NanoCpus=0` — caps **not applied**. But read-only `config` renders for new projects give: `flora-load` → ports `55432`, `mem=1073741824`, `cpus=1`, volume `flora-load_flora_postgres_data`; `flora-restore` → ports `55433`, same caps, volume `flora-restore_flora_postgres_data`. | The 1 GiB/1 CPU caps in `docker-compose.yml:30-32` render correctly; the live old container predates or bypassed them. A fresh `up` should apply them, but assert `docker inspect` + `memory.max` before any measurement. |
| E7 | `git status --short` empty; HEAD `bb16ba9a9`; `postgres:18-alpine` image present (`sha256:b6972e11…`, created 2026-11-12). | A clean, reproducible isolated run is possible if an execution window is granted. |
| E8 | `/` = 236 GiB, 28 GiB free (89 % used); **`/tmp` is `tmpfs` 2.0 GiB with 91 MiB free (96 % used)**. | Do **not** put dump files, JSON summaries or observation logs in `/tmp`. Use `/var/backups/flora` or `/var/tmp` (disk-backed). Tiny override YAML files in `/tmp/opencode` are fine. |
| E9 | `/tmp/opencode/flora-audit-ports.yml` belongs to another agent's audit (project `floraaudit012`); its containers are Up 20 h. `tirateima_fc-db-1` is Up 2 days. | Shared CPU/memory are *not* isolated even when ports/volumes are. A load run can perturb other agents' work. |
| E10 | No second host, no off-host backup destination, no written approval, no test window were provided in this task. | Full load/OOM validation and off-host retention remain NV (see §8). |

---

## 2. Tool-availability audit (exact inventory)

```
k6        MISSING      hey       MISSING      wrk       MISSING      vegeta    MISSING
jq        /usr/bin/jq  psql      /usr/bin/psql (18.6)    pg_dump   18.6        pg_restore 18.6
pg_dumpall 18.6        createdb  18.6         docker    29.9.2       compose  v5.6.1
node      /usr/bin/node           bun       /home/luis/.bun/bin/bun  curl  8.18.0
ss        /usr/bin/ss  rsync     /usr/bin/rsync          ssh       /usr/bin/ssh
systemd-run /usr/bin/systemd-run  choom     /usr/bin/choom
go        /usr/bin/go  gcc       /usr/lib64/ccache/gcc   make      /usr/bin/make
cargo     /home/luis/.local/bin/cargo         dnf       /usr/bin/dnf
```

No `k6`/`hey`/`wrk`/`vegeta` binary exists in `/usr/local/bin`, `/usr/bin`, `/home/luis/go/bin`, `/home/luis/.local/bin` or `/tmp/opencode`. Installation routes exist (`go install …`, `dnf`, release binaries), but each needs network access and changes the host; none was attempted (research-only). **The only load-capable tool already present is `curl`** (plus `xargs`/`awk`/`sort` from coreutils), which supports a *bounded, closed-loop smoke* but not saturation/OOM validation.

---

## 3. Read-only audit corrections to plan 030

| # | Plan 030 claim | Finding here | Corrected action |
|---|---|---|---|
| C1 | cgroup path `/sys/fs/cgroup/docker/$(docker inspect -f '{{.Id}}' "$CID")` (§3, §4 Step 2) | Docker driver is `systemd`; real path is `/sys/fs/cgroup/system.slice/docker-${CID}.scope/` [E5]. | Replace both occurrences with the `.scope` path; add a fallback `cat /proc/<PID>/cgroup` derivation. |
| C2 | k6 script includes `sleep(1)` at end of each `constant-arrival-rate` iteration (§4 Step 3) | k6 primary doc: *"Don't put sleep at the end of an iteration. The arrival-rate executors already pace the iteration rate"* [P1]. | Remove `sleep(1)`; add `discardResponseBodies: true`. Recompute VU budget (no 1 s pause ⇒ fewer VUs needed for same rate). |
| C3 | §018 correction "pool = 10, not 20" | Confirmed by repo code; keep [L1][L2]. | Saturation assertion remains clients ≤ `max_connections − 3`. |
| C4 | Host premise "2 vCPU / 4 GB VPS" | Host is 8 vCPU / 13.6 GiB with swap exhausted [E1]. | Do not quote p95/CPU thresholds as 2 vCPU/4 GB evidence here; re-baseline if executed on this host, or provision a real 2/4 target. |
| C5 | A8 "CPU < 70–80 %" | Meaningless on 8 shared cores with other agents' processes; `cpu.max=max` on their containers. | Record CPU% and `cpu.stat` throttling, but do not use as a pass gate on this host. |
| C6 | Dump/restore artifacts in `/var/backups/flora`, override YAML in `/tmp` | `/tmp` is 2 GiB tmpfs, 91 MiB free [E8]. | Keep that split; never write dumps or `k6 --out json` to `/tmp` on this host. |
| C7 | Restore source = `flora-load` PG on 55432 | That DB does not exist yet; the only live PG18 is another agent's `floraaudit012` on 15433. | If a drill is executed, first create **and seed** `-p flora-load` (migrations + `dev-seed.sql` + `scripts/dev-smoke-session.sql`). Never dump the other agent's DB without coordination. |
| C8 | `pg_dumpall --globals-only` as routine step | Seed cluster's only role is `postgres`; restoring globals into a fresh cluster can collide with the bootstrap role. | Generate globals as evidence, but **do not restore it** for the Seed drill; document as a production-only step [P2]. |
| C9 | `pg_restore -1 --no-owner --exit-on-error` | `-1` already implies `--exit-on-error` [P3]; harmless but redundant. `-j` is incompatible with `-1`. | Keep `-1 --no-owner`; note that `-j 2` must **not** be combined with `-1`. |
| C10 | 030 §7 deferral reasons | Still valid, plus new hard blockers: no tool, host mismatch, swap exhausted, systemd cgroup path, degraded user manager, `/tmp` full. | See §7. |

---

## 4. Execute-now scope (read-only, already completed)

These are the only actions safe to perform in this research pass, and they were performed:

```bash
# tool/version inventory
for t in k6 hey wrk vegeta jq psql pg_dump pg_restore pg_dumpall createdb docker node curl ss; do
  command -v "$t" || echo "$t MISSING"
done
docker --version; docker compose version; psql --version

# live-process / port reality (no mutation)
ss -ltnp | grep -E ':(3000|5432|5433|15433|16379|17700|55432|55433|33000|33001)\b'
docker ps --format 'table {{.Names}}\t{{.Image}}\t{{.Ports}}\t{{.Status}}'

# cgroup + enforcement reality
stat -fc %T /sys/fs/cgroup; cat /sys/fs/cgroup/cgroup.controllers
docker info --format '{{.CgroupDriver}} {{json .SecurityOptions}} {{.NCPU}} {{.MemTotal}}'
CID=$(docker inspect -f '{{.Id}}' floraaudit012-postgres-1)
cat /proc/$(docker inspect -f '{{.State.Pid}}' floraaudit012-postgres-1)/cgroup
cat /sys/fs/cgroup/system.slice/docker-${CID}.scope/memory.max
docker inspect -f 'Memory={{.HostConfig.Memory}} NanoCpus={{.HostConfig.NanoCpus}}' floraaudit012-postgres-1

# read-only compose renders for isolated projects (no up)
docker compose -f docker-compose.yml -f /tmp/opencode/flora-feasibility-load-override.yml \
  -p flora-load config --format json | jq -c '{name,ports,mem:.services.postgres.mem_limit,cpus:.services.postgres.cpus,volume:.volumes.flora_postgres_data.name}'
```

Result: every item in §1/§2 is evidence-backed; **nothing was started, minted, dumped or restored.**

---

## 5. Exact commands — bounded smoke-level load (NOT a full load test; deferred)

Purpose: prove the isolated stack + auth path are wired and that a small fixed-concurrency burst does not break readiness. It is **closed-loop and on-host**, so it cannot find the saturation knee, cannot validate p99 under overload, and **must not be reported as 2 vCPU/4 GB load validation**. Use it only as a pre-flight to the (deferred) real k6 run.

### 5.1 Preconditions before any execution

```bash
# Re-check ports immediately before the window; abort if any alt port is occupied
ss -ltnp | grep -E ':(55432|55433|33000|33001)\b' && { echo 'ALT PORT OCCUPIED — ABORT'; exit 1; }
# Confirm no other agent's process is touched
docker ps --format '{{.Names}}' | grep -E '^(floraaudit012|tirateima_fc)' || true   # observe only
```

### 5.2 Start the isolated target (DEFERRED — do not run in this pass)

Use plan 030 §4 Step 1 verbatim, with the corrected cgroup path from C1, project `-p flora-load`, port 55432, API on 33000 (`FLORA_APP__HOST=127.0.0.1`, `FLORA_APP__PORT=33000`, `FLORA_APP__BASE_URL=http://127.0.0.1:33000`), lean profile (`FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false`), migrations + `dev-seed.sql` + `scripts/dev-smoke-session.sql`, then an explicit wait loop on `/health/ready` (029 B3).

**Verify caps immediately after `up`** (this is the step plan 030 leaves implicit):

```bash
DC="docker compose -f docker-compose.yml -f /tmp/opencode/flora-feasibility-load-override.yml -p flora-load"
CID=$($DC ps -q postgres)
echo "cgroup path: /sys/fs/cgroup/system.slice/docker-${CID}.scope"
docker inspect -f 'Memory={{.HostConfig.Memory}} NanoCpus={{.HostConfig.NanoCpus}}' "$CID"
cat /sys/fs/cgroup/system.slice/docker-${CID}.scope/memory.max      # expect 1073741824
cat /sys/fs/cgroup/system.slice/docker-${CID}.scope/cpu.max         # expect 100000 100000
# If either shows `max`/0, STOP: caps are not enforced; the drill is meaningless [E6].
```

### 5.3 Mint a fresh JWT (dev-smoke §5) — keep in env, never echo

```bash
export FLORA_APP__JWT_SECRET='dev-only-32-char-secret-change-me-0001'
export FLORA_JWT_SECRET="$FLORA_APP__JWT_SECRET"     # both equal, or API/minter disagree (029 B4)
export ORG_ID='11111111-1111-1111-1111-111111111111'
export BASE='http://127.0.0.1:33000'
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
# Do NOT `echo "$FLORA_TOKEN"`. Record only a fingerprint if needed:
printf '%s' "$FLORA_TOKEN" | sha256sum | cut -c1-12
```

### 5.4 Sampler (disk-backed output, corrected cgroup path)

```bash
SMOKE_DIR=/var/tmp/flora-smoke-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p "$SMOKE_DIR"
API_PID=<pid of the isolated flora-api on 33000>          # set from Step 5.2, never PID 1799842
CID=$($DC ps -q postgres)
CG="/sys/fs/cgroup/system.slice/docker-${CID}.scope"
( while true; do
    date -u +%FT%TZ
    docker stats --no-stream --format '{{.Name}} {{.MemUsage}} {{.CPUPerc}}' | grep '^flora-load-' || true
    ps -o pid=,rss=,comm= -p "$API_PID" || true
    PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -Atc \
      "SELECT 'clients='||count(*) FILTER (WHERE backend_type='client backend')||
              ' active='||count(*) FILTER (WHERE state='active')||
              ' max='||current_setting('max_connections') FROM pg_stat_activity;"
    printf 'memory.events '; cat "$CG/memory.events" 2>/dev/null | tr '\n' ' '; echo
    printf 'memory.current '; cat "$CG/memory.current" 2>/dev/null
    sleep 5
  done ) > "$SMOKE_DIR/observations.txt" 2>&1 &
SAMPLER=$!
```

### 5.5 Bounded burst: 4 workers × 15 requests = 60 per endpoint (~≤30 s wall clock)

```bash
seq 1 60 | xargs -P4 -I{} sh -c '
  curl -sS -o /dev/null --max-time 3 -w "%{http_code} %{time_total}\n" \
    -H "Authorization: Bearer $FLORA_TOKEN" \
    -H "X-Organization-ID: $ORG_ID" \
    "$BASE/api/v1/workspaces"
' > "$SMOKE_DIR/workspaces.tsv"

# readiness probe from the same loop footprint (no auth header needed)
seq 1 20 | xargs -P4 -I{} sh -c '
  curl -sS -o /dev/null --max-time 3 -w "%{http_code} %{time_total}\n" "$BASE/health/ready"
' > "$SMOKE_DIR/ready.tsv"

awk '{print $2}' "$SMOKE_DIR/workspaces.tsv" | sort -n > "$SMOKE_DIR/times"
N=$(wc -l < "$SMOKE_DIR/times")
P95=$(awk -v n="$N" 'NR==int(n*0.95)+1{print $1}' "$SMOKE_DIR/times")
NON200=$(awk '$1!=200{c++} END{print c+0}' "$SMOKE_DIR/workspaces.tsv")
READY_BAD=$(awk '$1!=200{c++} END{print c+0}' "$SMOKE_DIR/ready.tsv")
echo "n=$N p95=${P95}s non200=$NON200 ready_non200=$READY_BAD"
test "$NON200" -eq 0 -a "$READY_BAD" -eq 0 -a "${P95%.*}" -le 0 || echo 'SMOKE GATE: inspect observations.txt'
```

**Smoke-level gates (weaker than §5 of 030; do not close the NV):** 60/60 HTTP 200 on `/api/v1/workspaces`; `/health/ready` 200 throughout; p95 < 0.300 s; zero `oom`/`oom_kill` in `memory.events`; DB clients ≤ 27 and no acquire-timeout in the API log; row counts (§6.4) unchanged after the burst. Anything else → abort, do not escalate to the full load run.

### 5.6 The real (still deferred) k6 run — corrected k6 script

Install k6 **on a separate host** first (`go install go.k6.io/k6@latest` or release binary), then:

```js
import http from 'k6/http';
import { check } from 'k6';
const B = __ENV.BASE_URL;
const H = { Authorization: `Bearer ${__ENV.FLORA_TOKEN}`,
            'X-Organization-ID': __ENV.ORG_ID };
export const options = {
  discardResponseBodies: true,                       // C2: primary-doc recommendation
  scenarios: { api: { executor: 'constant-arrival-rate',
    rate: Number(__ENV.RATE || 5), timeUnit: '1s', duration: __ENV.DUR || '30s',
    preAllocatedVUs: 5, maxVUs: 40 } },              // open-loop; no sleep() [P1]
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<300', 'p(99)<1000'],
    checks: ['rate==1'],
  },
};
export default function () {
  let r = http.get(`${B}/health/ready`, { tags: { ep: 'ready' } });
  check(r, { 'ready 200': x => x.status === 200 });
  r = http.get(`${B}/api/v1/workspaces`, { headers: H, tags: { ep: 'workspaces' } });
  check(r, { 'workspaces 200': x => x.status === 200 });
  // C2: intentionally NO sleep() — arrival rate already paces iterations
}
```

```bash
BASE_URL="http://<VPS>:33000" FLORA_TOKEN="$FLORA_TOKEN" ORG_ID="$ORG_ID" \
  RATE=5 DUR=30s k6 run --out json=/var/tmp/k6-smoke.json k6-flora-load.js
echo "k6 exit=$?"   # 0 pass, 99 threshold violation [P1][P4]
```

### 5.7 Teardown

```bash
kill "$SAMPLER" 2>/dev/null || true
$DC down -v           # -p flora-load keeps default flora volume safe
kill "$API_PID" 2>/dev/null || true
rm -f /tmp/opencode/flora-feasibility-load-override.yml
```

---

## 6. Exact commands — `pg_dump -Fc` restore drill into disposable PG18 (deferred)

Tooling is ready here (PG18.6 clients [E3], cached `postgres:18-alpine` [E7]). The only reason this is deferred is that it requires starting a disposable stack, which is outside a research-only pass, plus an off-host destination for the retention half of the NV. Use the **isolated `flora-load` source** from §5.2 — never the other agent's 15433 cluster.

### 6.1 R0 — dump + integrity (artifacts on disk, not `/tmp`)

```bash
export PGPASSWORD=postgres
TS=$(date -u +%Y%m%dT%H%M%SZ)
mkdir -p /var/backups/flora
pg_dump -h 127.0.0.1 -p 55432 -U postgres -d flora \
  -Fc -Z 3 -v -f /var/backups/flora/flora_${TS}.dump 2>/var/backups/flora/dump_${TS}.log
pg_dumpall -h 127.0.0.1 -p 55432 -U postgres --globals-only \
  -f /var/backups/flora/globals_${TS}.sql            # evidence only; do not restore in Seed drill (C8)
sha256sum /var/backups/flora/flora_${TS}.dump /var/backups/flora/globals_${TS}.sql \
  > /var/backups/flora/SHA256SUMS_${TS}
sha256sum -c /var/backups/flora/SHA256SUMS_${TS}
test -s /var/backups/flora/flora_${TS}.dump
# off-host copy is a PLACEHOLDER: no destination/credentials were provided (NV-6)
# rsync -av /var/backups/flora/ <backup-host>:/backups/flora/
```

`-Fc` is one database, custom (compressed) format, internally consistent; it excludes roles/tablespaces, hence the separate `pg_dumpall --globals-only` [P2][P5]. `-Z 3` selects gzip level for custom archives; `-Z zstd[:level]` is also accepted by PG18 [P5]. Do not use `--no-sync` on a real backup [P5].

### 6.2 R1 — disposable PG18 on a separate project/port/volume

```bash
cat > /tmp/opencode/flora-feasibility-restore-override.yml <<'YAML'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55433:5432"
YAML
RC="docker compose -f docker-compose.yml -f /tmp/opencode/flora-feasibility-restore-override.yml -p flora-restore"
$RC config --format json | jq -c '{name, ports:[.services.postgres.ports[]|{published,target}], volume:.volumes.flora_postgres_data.name}'
$RC up -d
until PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -tAc 'SELECT 1' >/dev/null 2>&1; do sleep 1; done
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -tAc \
  "SELECT current_setting('server_version_num')::int >= 180000, current_setting('max_connections');"
# Both `!override` and `-p flora-restore` are mandatory: otherwise 5432 is republished
# and `flora_postgres_data` is shared, so `down -v` would destroy real data [029 B2].
```

### 6.3 R2 — restore (correctness first, small Seed DB)

```bash
PGPASSWORD=postgres createdb -h 127.0.0.1 -p 55433 -U postgres -T template0 flora_restore
PGPASSWORD=postgres pg_restore -h 127.0.0.1 -p 55433 -U postgres \
  -d flora_restore -1 --no-owner -v /var/backups/flora/flora_${TS}.dump
# -1 = single transaction, all-or-nothing, implies --exit-on-error (C9) [P3].
# Do NOT add -j: jobs are incompatible with --single-transaction [P3].
# --no-owner maps objects to the connecting superuser (correct for the Seed-only role set) [P3].
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore \
  -v ON_ERROR_STOP=1 -c 'ANALYZE;'
```

`createdb -T template0` avoids duplicate objects from a customized `template1`; `ANALYZE` gives the planner usable statistics [P2][P3]. Restoring executes arbitrary superuser SQL — only restore a dump you produced yourself [P3].

### 6.4 R3 — row-count equality (source vs restored)

```bash
Q="SELECT (SELECT count(*) FROM organizations), (SELECT count(*) FROM users),
          (SELECT count(*) FROM roles), (SELECT count(*) FROM memberships),
          (SELECT count(*) FROM workspaces), (SELECT count(*) FROM channels),
          (SELECT count(*) FROM messages), (SELECT count(*) FROM tasks),
          (SELECT count(*) FROM notifications), (SELECT count(*) FROM sessions);"
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -Atc "$Q" > /var/tmp/counts-source
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore -Atc "$Q" > /var/tmp/counts-restored
diff -u /var/tmp/counts-source /var/tmp/counts-restored || { echo 'RESTORE COUNT MISMATCH'; exit 1; }
```

Equality — not absolute numbers — is the assertion. `sessions` is present because §5.2 applied `scripts/dev-smoke-session.sql`.

### 6.5 R4 — application-level verification on the restored DB (optional but required for full closure)

```bash
env FLORA_APP__HOST=127.0.0.1 FLORA_APP__PORT=33001 \
    FLORA_APP__BASE_URL=http://127.0.0.1:33001 \
    FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
    FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55433/flora_restore' \
    FLORA_SEARCH__ENABLED=false FLORA_MESSAGING__ENABLED=false \
    ./target/release/flora-api > /var/backups/flora/restore-api.log 2>&1 &
RESTORE_API=$!
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:33001/health/ready >/dev/null 2>&1 && break; sleep 1; done
export FLORA_BASE_URL='http://127.0.0.1:33001'
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
./test-flora.sh 2>&1 | tee /var/backups/flora/restore-smoke.txt
```

Startup migrations should be a no-op on the restored schema; if not, treat as a restore defect (030 NV-8). Assert on the task-**create** response, not list-after-create (`dev-smoke.md` §6). Record **RTO** (timer from `pg_restore` start to green smoke) and **RPO** (`TS` of dump vs source `now()`).

### 6.6 R5 — cleanup

```bash
$RC down -v                 # only the flora-restore project
kill "$RESTORE_API" 2>/dev/null || true
rm -f /tmp/opencode/flora-feasibility-restore-override.yml /var/tmp/counts-source /var/tmp/counts-restored
```

Never run a bare `docker compose down -v` from the repo root [029 B2].

---

## 7. Acceptance thresholds (adapted for this host)

| # | Gate | Command / source | Applies to | Note |
|---|---|---|---|---|
| S1 | 60/60 workspaces 200; ready 200 throughout | §5.5 TSVs | bounded smoke | Hard gate; abort on any non-200 |
| S2 | p95 < 0.300 s | `sort -n` + awk on `time_total` | bounded smoke | Weak evidence; closed-loop |
| S3 | `oom=0`, `oom_kill=0` | `memory.events` in `system.slice/docker-${CID}.scope` | both | Use corrected path (C1) |
| S4 | DB clients ≤ 27, zero acquire timeouts | `pg_stat_activity` + API log | both | `max_connections=30` minus 3 reserved |
| A1 | p95 < 300 ms, p99 < 1000 ms | k6 thresholds (corrected script §5.6) | full load only | Cannot be asserted by curl |
| A2 | error rate < 1 %, target 0; no 503 | k6 `http_req_failed`, `checks` | full load only | Any 5xx = hard fail |
| A3 | achieved rate within ±10 % of target | k6 `http_reqs.rate` | full load only | Guards generator bottleneck |
| A5 | API RSS ≤ 1.2× warm baseline and < cap | `ps`/`/proc` sampler | full load only | Cap value unverified here (NV-4) |
| A6 | PG `memory.current` < 900 MiB of 1 GiB | cgroup `memory.current`/`stat` | both | Verify `memory.max` first (E6) |
| A7 | zero `oom_kill`; 30 s recovery after breakpoint | `memory.events`, `journalctl -k` | full load only | Optional breakpoint run |
| A8 | CPU < 70–80 % | `docker stats`, `cpu.stat` | **not a gate here** | 8 shared cores; record only (C5) |
| A9 | > 20 % free on data volume | `df -h` | both | `/` already 89 % used (E8) |
| A10 | timestamped, redacted evidence with exit codes | artifacts under `/var/backups/flora`, `/var/tmp` | both | No token in artifacts |
| B1 | dump non-zero, `sha256sum -c` OK | §6.1 | restore | Primary integrity gate |
| B2 | PG major ≥ 18 | `server_version_num` | restore | Confirmed tool/image availability |
| B3 | `pg_restore` exit 0 | §6.3 | restore | `-1` implies `--exit-on-error` |
| B4 | row counts equal for all 10 tables | §6.4 `diff` | restore | Equality, not absolute values |
| B5 | `ANALYZE` completes | `ON_ERROR_STOP=1` | restore | — |
| B6 | restored API `/health/ready` 200 + smoke green | §6.5 | restore | Record RTO/RPO here |
| B7 | RTO/RPO recorded | timestamps | restore | Still NV until executed |
| B8 | disposable project/volume removed | §6.6 | restore | No impact on `flora`/`flora-load` |

---

## 8. Execute now vs defer — explicit decision

### EXECUTE NOW (done in this pass)
1. Read-only host/tool/cgroup/port/compose inventory (§1–§2, §4).
2. Read-only audit of plan 030 and correction register (§3).
3. This document with ready-to-run commands and gates (§§5–7).

### DEFER — bounded smoke + restore drill (ready, but not executed)
**Reasons:** (a) both require starting an isolated stack; this is a research-only pass with an explicit "do NOT start stacks" instruction; (b) swap is fully consumed and only ~2.9 GiB is available, so even a capped stack adds real OOM risk to other agents' workloads sharing the same uncapped CPUs [E1][E9]; (c) caps are not yet proven on a fresh container [E6]; (d) no off-host backup destination exists for the retention half of the NV [E10]; (e) no authorization/time window for HTTP traffic, even loopback against a new process [E9][E10].

**Preconditions to lift the defer:** approval + window; re-run the port check; start `-p flora-load`, verify caps via `docker inspect` + `memory.max`; run §5.3–5.5 with artifacts under `/var/tmp`; then §6.1–6.6. The bounded smoke result stays **smoke evidence only** — it does not close the load NV.

### DEFER — full 2 vCPU/4 GB load + OOM validation (hard defer)
**Reasons:** (a) no generator binary installed and no second host — running k6 here measures contention, not the service [E3][E9][P4][P6]; (b) this host is not 2 vCPU/4 GB [E1]; (c) closed-loop curl cannot expose the saturation knee, so a "fast" result would be false [P6][P7]; (d) no written approval for load against a shared box. Provision an off-box generator + a real 2/4 target, install k6 there, use the corrected §5.6 script and plan 030 §§4–5 gates.

### Do NOT do
- Touch PID 1799842 (port 3000), 5432, `tirateima_fc-db-1` (5433), or `floraaudit012-*` (15433/16379/17700).
- Use `/sys/fs/cgroup/docker/<CID>` on this host (C1).
- Put dumps/JSON/logs in `/tmp` (E8).
- Dump the other agent's PG18 as a "source" without coordination (C7).
- Restore `globals.sql` into the Seed target (C8); combine `-j` with `-1` (C9).

---

## 9. Multilingual research basis

| Lang | Sources | Takeaway |
|---|---|---|
| EN (primary) | k6 constant-arrival-rate + thresholds [P1][P4], PostgreSQL 18 `pg_dump`/`pg_restore`/backup-dump [P2][P3][P5], Linux cgroup v2 [P7] — fetched/carried from 030. | Normative flags, exit codes, cgroup semantics. All load-bearing commands in §§5–6 are grounded here. |
| de (new fetch this pass) | S-EDV "Lasttest mit k6: Wann bricht ein kleiner VPS ein?" (netcup 2 cores/4 GB, no swap) [P6]; apidog DE k6 guide [P4]; Server Room/Primcast generator-sizing guides [P8]. | Every source independently says the generator must be a **second machine** (running k6 on the target shifts the measurement), fixed-rate `constant-arrival-rate` beats fixed-VU for saturation, keep the generator ≤ ~80 % CPU, disable swap on the generator, and thresholds with non-zero exit make CI gating possible. Confirms 030's deferral logic and strengthens the "no second host" blocker. |
| pt-BR | Carried from 030 [S16] (StreetHosting, MelhorVPS): generator on another machine, p95/CPU<70 %/no waiting pool/0 errors, ramp 1→10→25 VUs. | No new pt-BR fetch this pass: the free search integration quota was exhausted (NV-9). Convention carried forward, not treated as new primary evidence. |
| es | Carried from 030 [S20] (jacar.es): Compose limits honoured without Swarm, verify with `docker stats`/`inspect`. | Matches E6: rendered config is right, live enforcement must be checked. |
| fr | Carried from 030 §2 [L8] (016 cycle): 25 %/75 % memory heuristics, `work_mem` multiplication, off-host copies, regular restore tests. | No new FR fetch (NV-10); inherited, not authority for flags. |
| zh-CN | Carried from 030 [S18]: `hey -c/-z`, `k6 -u/-d`, `wrk -t/-c/-d --latency`, `vegeta -rate`; `wrk -t` ≈ 1–3× cores. | Tool comparison supports keeping `-t2` if wrk is ever installed here. |
| ja | Carried from 030 [S19] (DevelopersIO, MMM, NIFTY): smoke → load → soak, record 0 % errors + p95, authorized environments only. | Justifies §5 as a smoke pre-flight and the deferral of soak/breakpoint. |

> Non-EN material is operational guidance; exact flags and thresholds were verified against EN primary docs [P1]–[P5][P7]. No target content was treated as instructions; nothing here authorizes load against production or any other agent's process.

---

## 10. NV register (carried + new)

- **NV-1 (carried):** no load generator host provisioned; all rates/ports remain illustrative.
- **NV-2 (carried):** `/metrics` is static; no app-level latency/pool/RSS telemetry.
- **NV-3 (carried):** FR coverage inherited from 016, not re-fetched.
- **NV-4 (new):** `systemd --user` reports **degraded**; `systemd-run --user --scope -p MemoryMax=768M` enforcement unverified.
- **NV-5 (new):** host is 8 vCPU/13.6 GiB with swap fully used — the 2 vCPU/4 GB target and its thresholds cannot be validated here.
- **NV-6 (carried/updated):** off-host backup destination and credentials undefined; off-host copy remains a placeholder.
- **NV-7 (new):** no load tool binary installed (k6/hey/wrk/vegeta all missing); installation route exists but not exercised.
- **NV-8 (new):** Compose caps render correctly for new projects but are **not enforced** on the live 20 h-old audit container; must be re-verified after any `up`.
- **NV-9 (new):** pt-BR search quota exhausted on this pass; pt-BR basis inherited from 030 only.
- **NV-10 (carried):** migration idempotency on a restored dump (§6.5) not verified.
- **NV-11 (new):** `/tmp` is a 96 %-full 2 GiB tmpfs; artifact paths must stay under `/var/backups/flora` or `/var/tmp`.
- **NV-12 (new):** plan 030's k6 script used `sleep()` with `constant-arrival-rate`, contrary to k6 primary docs; corrected in §5.6 but not runtime-verified.
- **NV-13 (overall):** per ADR-0001 / `dev-smoke.md:119`, the Seed NV stays **open** until an executed, timestamped, redacted run records exit codes + counts + cgroup evidence. This document is input, not closure.

---

## 11. Sources

**Local — read-only, verified this pass (paths relative to repo root):**
- [E1]–[E10] host probes: `nproc`, `free -h`, `docker info`, `ss -ltnp`, `docker ps`, `docker inspect`, `/proc/<PID>/cgroup`, `/sys/fs/cgroup/system.slice/docker-<CID>.scope/*`, `command -v`, `df -h`, `git status`, `docker compose … config` — all outputs recorded in §1.
- [L1] `flora-core/src/database.rs:11-25`, [L2] `flora-core/src/config/database.rs:12-70` — pool wiring/defaults (via 030).
- [L3] `docker-compose.yml:11-32` — `postgres:18-alpine`, `mem_limit: 1g`, `cpus: "1.00"`, localhost binding.
- [L4] `docs/dev-smoke.md:7-21,43-58,104-128` — lean profile, seed/session ordering, fresh-JWT minter, smoke expectations, backup NV.
- [L5] `docs/research/2026-11-16-029-smoke-alt-port-runbook.md` §§1–3 — `!override`, `-p` isolation, wait loop, fresh JWT, redaction, rate-limit/task-list nuances.
- [L6] `/tmp/opencode/flora-audit-ports.yml` — another agent's port-only override (read-only observation).

**External — primary, fetched 2026-11-16 (this pass or carried from 030):**
- [P1] Grafana k6 — [constant-arrival-rate executor](https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/constant-arrival-rate/) — open model, fractional spacing, `preAllocatedVUs`/`maxVUs`, **no `sleep()` at end of iteration** (fetched this pass).
- [P2] PostgreSQL 18 — [25.1 SQL Dump](https://www.postgresql.org/docs/18/backup-dump.html) and [pg_dump](https://www.postgresql.org/docs/18/app-pgdump.html) — `pg_dumpall --globals-only`, `template0`, `ANALYZE`, `-Fc`, trust warning.
- [P3] PostgreSQL 18 — [pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html) — `-1` implies `--exit-on-error`; `-j` incompatible with `-1`; `--no-owner`; `template0`; arbitrary-SQL warning; `ANALYZE` note (fetched this pass).
- [P4] Grafana k6 — [Thresholds](https://grafana.com/docs/k6/latest/using-k6/thresholds/) and [API load testing](https://grafana.com/docs/k6/latest/testing-guides/api-load-testing/) — `rate<0.01`, `p(95)`, exit 99 on violation; carried from 030 and corroborated by [P6].
- [P5] PostgreSQL 18 — [pg_dump](https://www.postgresql.org/docs/18/app-pgdump.html), [runtime-config-connection](https://www.postgresql.org/docs/18/runtime-config-connection.html), [resource consumption](https://www.postgresql.org/docs/18/runtime-config-resource.html) — `-Z`, `--no-sync`, `max_connections` ~100 default, `work_mem` multiplication.
- [P6] de: [S-EDV — Lasttest mit k6 auf kleinem VPS](https://s-edv.com/anleitungen/lasttest-k6-vps-wordpress-cache-grenze) (fetched this pass) — second-machine generator, 2-core/4 GB breakpoint, open vs closed loop, `vmstat` sampling.
- [P7] Linux kernel — [Control Group v2](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html) — `memory.current/high/max/events`, `anon` vs `file`, `cpu.stat`.
- [P8] de/en operator guides: [Server Room](https://serverroom.net/use-cases/load-testing-server), [Primcast](http://primcast.com/workloads/load-testing-server) — generator ≤80 % CPU, 1–5 MB/VU, disable swap on generator (fetched this pass).
- [S16]–[S21] carried from 030 §8 (pt-BR, es, fr, zh-CN, ja, operator/de guides).

*Final note: no stack, load, dump, restore, token or secret was created. All commands in §§4–7 are ready-to-run instructions for a separately approved execution window, not a record of execution. Do not use them against the running 3000/5432 services or any other agent's project.*
