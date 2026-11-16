# 030 — 2 vCPU / 4 GB VPS load + OOM + `pg_dump -Fc` restore drill: minimal verifiable plan (Seed NV closure candidate)

**Date:** 2026-11-16 (cycle 030).  
**Repo:** `flora` (working directory `projetos/Arquivados/flora`).  
**Type:** research/design only. **No stack was started, no load was generated, no dump/restore was executed, no config/workflow/source file was changed. This document is the only artifact written.** Read-only file reads and primary-doc fetches were performed. Execution of §§4–6 is explicitly **deferred** (reasons in §7).  
**Builds on:** `2026-11-16-016-postgres-vps-backup-tuning.md` (tuning/backup baseline), `2026-11-16-029-smoke-alt-port-runbook.md` (isolated PG18 + alt-port API recipe, fresh JWT, project isolation).  
**Question:** What is the cheapest, highest-reliability way to close Flora's remaining Seed validation NV — *"2vCPU/4GB load/OOM validation not executed"* and *"restore drills not executed"* (`docs/dev-smoke.md:119`) — on a 2 vCPU / 4 GB VPS, with exact commands and acceptance thresholds grounded in primary docs?

---

## 0. Decision & evidence summary

| Item | Answer (this pass) |
|---|---|
| Tool shortlist | **k6** (primary: pass/fail thresholds + *open-loop* constant-arrival-rate, which exposes saturation), **hey** (fast one-line smoke/load with `-H`, CSV), **wrk** (raw throughput ceiling, `--latency`), **vegeta** (open-loop constant-rate, pipeable JSON). All four are static binaries / single installs; none needs an agent on the target [S9][S10][S11][S12]. |
| Critical tool caveat | `hey`, `wrk` (stock) and `ab` are **closed-loop**: when the server slows, offered load drops and overload is hidden. Use `k6 constant-arrival-rate` or `vegeta -rate` to find the saturation knee [S13]. This matters for an OOM drill — a closed-loop tool can report "fast" while the DB is queueing. |
| Generator placement | **Off-box, mandatory.** Run k6/hey/wrk/vegeta on a second small host or workstation. If the generator shares the 2 vCPU / 4 GB target it steals the CPU/RSS you are trying to measure and the result is invalid [S16][S17]. If no second host exists → defer (NV-1). |
| Effective app pool (repo truth) | `flora-core/src/database.rs:16-24` **now wires config into the pool**: `max_connections(config.database.max_connections)`; `flora-core/src/config/database.rs:22-33` default = **10** (min 2, acquire timeout 30 s, idle 600 s). This **corrects** the stale 016 doc claim of a hardcoded pool of 20 [L1][L2][L3]. Effective DB client count ≈ 10 per API process. |
| Fetchable metrics gap | `GET /metrics` is a **static two-line placeholder** (`flora_api_up`, `flora_api_info`) — no latency, RSS, or pool metrics [L4]. Therefore all four required measurements must come from the **load tool + OS/cgroup + `pg_stat_activity`**, not from Flora itself (NV-2). |
| Safe caps (start, then measure) | PG container already capped `mem_limit: 1g`, `cpus: "1.00"` [L5]; add PG params from 016 (`shared_buffers=512MB`, `work_mem=4MB`, `max_connections=30`) [S4][S5]; cap the API process at **~768 MB** with `systemd-run --scope -p MemoryMax=768M`; leave **≥1 GB** for kernel/page cache/SSH. **Verify enforcement** — do not assume it [S7][S14]. |
| OOM detection | cgroup `memory.events` counters `max`/`oom`/`oom_kill`, `memory.current`, `memory.peak`; kernel log `Killed process`; container exit 137. `memory.high` throttles without killing; `memory.max` invokes the OOM killer [S7]. `oom_score_adj=-1000` disables OOM killing for a task (use deliberately for SSH, never casually for PG) [S8]. |
| Restore drill | `pg_dump -Fc -Z 3` (compressed, consistent, one DB) + `pg_dumpall --globals-only` (roles) + `sha256sum` + off-host copy; restore into a **disposable, isolated** PG18 Compose project (`-p flora-restore`, `ports: !override` → 55433) using `createdb -T template0` + `pg_restore -1 --no-owner --exit-on-error`; `ANALYZE`; compare row counts; boot a fresh API on port 33001 against the restored DB and run the existing smoke [S1][S2][S3][L6]. |
| Acceptance (headline) | p95 < 300 ms, p99 < 1000 ms, error rate < 1 % (target 0), `/health/ready` stays 200, DB client backends ≤ `max_connections - 3` with zero acquire timeouts, API RSS after 10-min soak ≤ 1.2× warm baseline and below cap, PG `memory.current` < 900 MB of 1 GB, **zero** `oom_kill` events, restore row counts equal to source, restored API smoke green. Full table in §5. |
| Defer? | **Plan produced; execution deferred.** See §7 for gating reasons (no second host, occupied host ports, no off-host storage credentials, cap values need empirical confirmation, admin approval for load against a shared VPS). |

**One-line closure:** *Run off-box k6 at fixed arrival rate against a config-wired API (pool 10) and a capped PG18, sample `pg_stat_activity` + cgroup `memory.events` + RSS while thresholds gate p95/errors, then restore a `pg_dump -Fc` into an isolated disposable PG18 and prove row counts + API smoke — record RTO/RTO and redacted evidence; do not claim closure from this document alone.*

---

## 1. Repo cross-check (read-only evidence)

| Area | Verified state | Implication |
|---|---|---|
| Compose caps | `docker-compose.yml:30-32`: PG `mem_limit: 1g`, `cpus: "1.00"` with comment "Guardrails for ~2 vCPU / 4 GB Seed hosts (enforcement depends on runtime; verify with `docker stats`)"; Valkey `384m`/`0.50`; Meili `768m`/`0.75`; all profile-gated or lean-core [L5]. | Baseline caps exist for PG. When realtime/search profiles are enabled the sum (1G+384M+768M ≈ 2.15G) plus API + OS is still under 4 GB but leaves little headroom — keep them disabled for this drill. |
| Pool default & wiring | `config/database.rs:12-33` defaults `max_connections=10`, `min_connections=2`, `acquire_timeout_secs=30`, `idle_lifetime_secs=600`; `validate()` enforces >0 and min ≤ max [L2]. `flora-core/src/database.rs:16-24` uses those values [L1]. | Effective pool = 10 (not 20). Saturation threshold should be expressed against 10 app connections, while PG `max_connections` should be ≥ app pool + admin reserve. |
| Backup section | `docs/dev-smoke.md:115-119`: "schedule off-host `pg_dump -Fc` (and globals if needed)… periodically restore into a disposable PG18 instance, run migrations/smoke, and verify data"; **"NV: restore drills, off-host retention automation, and 2vCPU/4GB load/OOM validation not executed in this cycle"** [L6]. | This doc is the executable candidate for exactly that NV; closure still requires an executed, recorded run. |
| Load targets | `health.rs`: `/health` JSON, `/health/ready` executes `SELECT 1` on the pool (503 on failure), `/health/live` bare 200; `/metrics` and `/llms.txt` are static [L4]. `llms.txt` manifest lists `/api/v1/workspaces`, `/api/v1/tasks`, etc. as auth + `X-Organization-ID` endpoints [L4]. | Use `/health/ready` (exercises pool) and one authenticated **read** endpoint for load. Avoid POST endpoints in load (mutations/side effects) to keep the drill deterministic. |
| Fixtures | `dev-seed.sql` inserts into `organizations`, `users`, `roles`, `memberships`, `workspaces`, `channels`, `messages`, `tasks`, `notifications` [L7]; `scripts/dev-smoke-session.sql` seeds the matching `sessions` row [L6]. | Restore verification can compare exact row counts across those tables against the source DB. |
| Prior NVs | 016 NV-5 (Compose limit enforcement), NV-3 (stale pool claim), NV-2 (no benchmarks) [L3]. | §3/§5 of this doc make enforcement and pool measurement explicit; the stale pool claim is corrected here. |

---

## 2. Multilingual research basis (EN, pt-BR, es, de, fr, zh-CN, ja)

Primary-source fetches were made directly to PostgreSQL, the Linux kernel/man-pages, Docker and the tool projects. Localized sources were used for **operational convention**, never as authority for exact flags.

| Lang | Sources consulted | Takeaway carried into the plan |
|---|---|---|
| EN | PostgreSQL 18 docs: SQL Dump, `pg_dump`, `pg_restore`, Connections, Resource Consumption, Cumulative Statistics [S1]–[S6]; Linux cgroup v2 + `proc_pid_oom_score_adj(5)` [S7][S8]; hey/wrk/vegeta READMEs, k6 thresholds/API-load docs [S9]–[S13]; Docker Compose resource reference [S14]. | Normative flags, cgroup semantics, threshold syntax, open- vs closed-loop distinction. |
| pt-BR | StreetHosting "Como escolher VPS para API" and MelhorVPS "Teste de carga com k6 em Cloud Server" [S16]. | Explicit rule: run the generator **from another machine**; checklist = p95 within goal, CPU < 70 % at peak, no sustained swap, DB pool with no requests waiting, zero errors; ramp 1 → 10 → 25 VUs; 2 vCPU/4 GB as a reasonable small-generator/target baseline. Mirrors §3/§5 exactly. |
| es | jacar.es "Límites de recursos y logs en Docker" [S20]. | `deploy.resources.limits` (cpus/memory/pids) is honoured by `docker compose up` **without Swarm**; `mem_limit`/`cpus` shortcuts are equivalent; over-memory → OOM kill, over-CPU → throttle; verify with `docker stats` / `docker inspect`. |
| de | S-EDV "Lasttest mit k6: Wann bricht ein kleiner VPS ein?" (netcup 2 cores/4 GB, no swap) [S17]; vds.guru DE `pg_dump` guide [S21]. | Measured 2-core/4 GB breakpoint example; recommends fixed-rate `constant-arrival-rate` over `ab`/`wrk` because fixed-connection tools hide overload; `vmstat 1` sampling on target; weekly disposable-DB restore with `pg_restore -j 2 --no-owner` + row count; `-Z zstd` for dumps. |
| fr | Prior-cycle FR operator references already captured in `2026-11-16-016-postgres-vps-backup-tuning.md` §2 (D-Open, DreamHost FR, Simafri, VDS.guru FR, Space-Node FR) [L8]. | 25 %/75 % memory heuristics, `work_mem` multiplication warning, low `max_connections` + pooling, off-host copies and regular restore tests — consistent with EN primary docs. **NV-3:** no new FR primary fetch in this pass. |
| zh-CN | javalover123/腾讯云 "HTTP接口性能压力测试" tool comparison [S18]. | Practical flags for `hey -c/-z`, `k6 -u/-d`, `wrk -t/-c/-d --latency`, `vegeta -rate/-workers/-max-workers`; `wrk` threads ≈ 1–3× cores to avoid context switching (use `-t2` on this VPS). |
| ja | DevelopersIO k6 ramping/spike/soak article, MMM "k6を使ったAPI負荷試験入門", NIFTY engineering [S19]. | Soak (long, steady) detects memory leaks/degradation; spike tests survival; smoke first, then load; record error rate 0 % + p95 vs goal; run only on authorized environments. Adopted as the smoke→load→soak sequence in §4. |

> **NV-4 (coverage):** non-EN material is operator guidance, not the owning primary project; all load-bearing flags and thresholds were verified against EN primary docs [S1]–[S14]. FR is carried forward from cycle 016 rather than re-fetched.

---

## 3. Cheap tool selection and safe caps

### 3.1 Tool choice (all primary-doc verified)

| Tool | Install (cheap) | Load model | Output / gate | Use here |
|---|---|---|---|---|
| **k6** | single static binary / `k6` repo | open-loop `constant-arrival-rate` or VU stages | **thresholds set exit code** (0 pass, non-zero fail); `http_req_duration` `p(95)`/`p(99)`, `http_req_failed` rate [S12] | **Primary** — codifies acceptance and exposes saturation. |
| **hey** | `go install` / release binary | fixed concurrency (`-c`), count (`-n`) or duration (`-z`), optional `-q` QPS/**worker** | summary + `-o csv`; `-H` repeats for Bearer/org headers [S9] | Quick pre-flight at known concurrency. Note `-q` is per worker: `-c 4 -q 5` ≈ 20 QPS. |
| **wrk** | `make` from source (C) | closed-loop, threads × kept-open connections | `--latency` percentiles, RPS, `--timeout` [S10] | Raw ceiling only; `-t2 -c20`; `--latency` mandatory. `-t` > cores adds context switches [S18]. |
| **vegeta** | release binary / `go install` | **open-loop** `-rate=N/1s` (0 = unlimited, use with `-max-workers`) | `attack | report -type=json` gives `latencies.p95/p99`, `status_codes`, `errors`; `-header` repeat [S11] | Alternative open-loop if k6 is unavailable. |

**Decision:** use k6 for the gated run; keep a 30 s `hey` smoke and one `wrk --latency` ceiling run as cheap cross-checks. Do **not** use `ab` (closed-loop, single-threaded) for the OOM/saturation question [S13].

### 3.2 Resource budget on 2 vCPU / 4 GB (starting point — measure, then adjust)

| Component | Cap | Knobs | Why |
|---|---|---|---|
| Postgres (container) | `mem_limit: 1g`, `cpus: "1.00"` (already in compose [L5]) | `shared_buffers=512MB`, `effective_cache_size=1536MB` (planner hint only), `work_mem=4MB`, `maintenance_work_mem=128MB`, `max_connections=30`, `random_page_cost=1.1`, `max_wal_size=1GB`, `checkpoint_completion_target=0.9` | PG docs: ~25 % of RAM for `shared_buffers` on ≥1 GB dedicated, unlikely >40 % better [S5]; `work_mem` is **per sort/hash node and per session** so keep 4 MB [S5]; `max_connections` sizes shared memory and defaults to 100 — lower it for a small host and keep admin slots [S4]. |
| API process | ~768 MB hard cap, ~1 CPU | `systemd-run --scope -p MemoryMax=768M -p CPUQuota=100% ...` (or a second Compose service with `mem_limit: 768m`); run **release** binary (`panic=abort`, stripped) | App RSS is the OOM suspect under load; `/metrics` cannot report it [L4], so cap + sample externally. |
| OS / page cache / SSH | **≥1 GB left uncapped** | no swap growth; keep `sshd` OOM-protected if tuning `oom_score_adj` | cgroup memory accounting distinguishes `anon` (unreclaimable working set) from `file` (page cache) [S7]; a high `file` value is not imminent OOM. |
| Optional profiles | **disabled** | `FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false` [L6] | Frees 384M+768M during the drill. |

**Enforcement must be proven, not assumed.** For Compose, both `deploy.resources.limits` and the `mem_limit`/`cpus` shortcuts are applied by `docker compose up` on a normal host (Compose v2/v5); only *other* `deploy` keys (placement, replicas) need Swarm [S14]. Flora uses the shortcut form [L5]. Verify:

```bash
docker compose ps -q postgres | xargs -r docker inspect --format '{{.HostConfig.Memory}} {{.HostConfig.NanoCpus}}'
docker stats --no-stream --format '{{.Name}} {{.MemUsage}} {{.CPUPerc}}'
# cgroup truth (PG container id) — path depends on cgroup driver/version (v2 vs v1, docker vs system.slice) [S7]:
CID=$(docker compose ps -q postgres)
cat /sys/fs/cgroup/docker/$(docker inspect -f '{{.Id}}' "$CID")/memory.max
cat /sys/fs/cgroup/docker/$(docker inspect -f '{{.Id}}' "$CID")/memory.events
```

If the limit column shows full host RAM, stop — the drill is meaningless until caps apply [S14]. If the cgroup path above does not exist, locate it with `cat /proc/$(docker inspect -f '{{.State.Pid}}' "$CID")/cgroup` before reading the files. Memory over the cap → kill (exit 137); CPU over the cap → throttle, not kill [S20].

**OOM tuning (only if needed):** cgroup `memory.high` throttles with heavy reclaim and **never** invokes the OOM killer; `memory.max` is the hard limit that does [S7]. `memory.oom.group=1` makes the cgroup indivisible to the OOM killer (avoids partial kills) [S7]. `oom_score_adj` range is −1000…+1000, with −1000 disabling OOM killing for that task [S8]. **Risk:** protecting the wrong PID merely moves the kill to SSH or another tenant — apply narrowly and record it.

---

## 4. Load + OOM drill (exact commands; NOT executed in this pass)

All commands from repo root. Reuse 029 isolation: Compose project `flora-load`, PG host port **55432**, API port **33000** [L9]. **Generator runs on a separate host** (`LOADER`), target on the VPS.

### Step 0 — preflight (evidence, no mutation)

```bash
date -u; git rev-parse HEAD; git status --short
docker compose version; command -v k6 hey wrk vegeta jq psql node
ss -ltnp | grep -E ':3000|:5432|:55432|:33000' || true   # re-check; 029 found 3000/5432 occupied [L9]
free -m; df -h / /var/lib/docker; swapon --show || true
```

### Step 1 — start isolated target (PG18 + API), cap and assert

```bash
cat > /tmp/flora-load-override.yml <<'YAML'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55432:5432"
    mem_limit: 1g
    cpus: "1.00"
    command:
      - postgres
      - -c
      - shared_buffers=512MB
      - -c
      - work_mem=4MB
      - -c
      - maintenance_work_mem=128MB
      - -c
      - max_connections=30
      - -c
      - random_page_cost=1.1
YAML
DC="docker compose -f docker-compose.yml -f /tmp/flora-load-override.yml -p flora-load"
$DC config --format json | jq -c '{name, ports:[.services.postgres.ports[]|{published,target}], mem:.services.postgres.mem_limit, cpus:.services.postgres.cpus}'
$DC up -d
until PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -tAc 'SELECT 1' >/dev/null 2>&1; do sleep 1; done
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -tAc \
  "SELECT current_setting('server_version_num')::int >= 180000, current_setting('max_connections');"
```

Use `ports: !override` **and** `-p flora-load` — a plain override leaves 5432 published and shares the default `flora_postgres_data` volume [L9]. Seed + session + fresh JWT exactly per 029 Steps 5–7 [L6][L9] (never echo the token; keep `FLORA_APP__JWT_SECRET` and `FLORA_JWT_SECRET` equal).

```bash
cargo build --release -p flora-api       # release profile: strip + panic=abort, smaller RSS
systemd-run --user --scope -p MemoryMax=768M -p CPUQuota=100% \
  env FLORA_APP__HOST=127.0.0.1 FLORA_APP__PORT=33000 \
      FLORA_APP__BASE_URL=http://127.0.0.1:33000 \
      FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
      FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55432/flora' \
      FLORA_SEARCH__ENABLED=false FLORA_MESSAGING__ENABLED=false \
      ./target/release/flora-api > /tmp/flora-load-api.log 2>&1 &
API_PID=$!
until curl -fsS http://127.0.0.1:33000/health/ready >/dev/null 2>&1; do sleep 1; done
```

### Step 2 — sampler on the target (runs alongside the test)

```bash
( while true; do
    date -u +%FT%TZ
    docker stats --no-stream --format '{{.Name}} {{.MemUsage}} {{.CPUPerc}}'
    ps -o pid,rss,comm -p "$API_PID"
    PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -Atc \
      "SELECT 'clients='||count(*) FILTER (WHERE backend_type='client backend')||
              ' active='||count(*) FILTER (WHERE state='active')||
              ' max='||current_setting('max_connections') FROM pg_stat_activity;"
    cat /sys/fs/cgroup/docker/$(docker inspect -f '{{.Id}}' $CID)/memory.events 2>/dev/null | tr '\n' ' '; echo
    sleep 5
  done ) > /tmp/flora-load-observations.txt 2>&1 &
SAMPLER=$!
```

`pg_stat_activity` is the authoritative live connection view (one row per process; `state`, `wait_event_*`); cumulative stats lag up to ~1 s and are cached per transaction [S6].

### Step 3 — gated load from `LOADER` (off-box)

Save as `k6-flora-load.js` **on the loader**; pass secrets via env, never in the file [S16].

```js
import http from 'k6/http';
import { check, sleep } from 'k6';
const B = __ENV.BASE_URL;                    // http://<VPS>:33000
const H = { Authorization: `Bearer ${__ENV.FLORA_TOKEN}`,
            'X-Organization-ID': __ENV.ORG_ID };
export const options = {
  scenarios: { api: { executor: 'constant-arrival-rate',
    rate: Number(__ENV.RATE || 10), timeUnit: '1s', duration: __ENV.DUR || '60s',
    preAllocatedVUs: 5, maxVUs: 40 } },     // rate = iterations/s; 2 requests/iteration
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
  sleep(1);
}
```

```bash
# smoke → load → soak (ja guidance [S19]); rate steps 5 → 10 → 20 it/s (10→20→40 rps)
BASE_URL=http://<VPS>:33000 FLORA_TOKEN="$FLORA_TOKEN" ORG_ID="$ORG_ID" \
  RATE=5 DUR=30s k6 run k6-flora-load.js
BASE_URL=http://<VPS>:33000 FLORA_TOKEN="$FLORA_TOKEN" ORG_ID="$ORG_ID" \
  RATE=20 DUR=10m k6 run --out json=k6-soak.json k6-flora-load.js   # soak: RSS/leak watch
echo "k6 exit=$?"      # 0 = all thresholds pass, 99 = threshold failed [S13]
```

Cheap cross-checks:

```bash
hey -z 60s -c 4 -q 5 -H "Authorization: Bearer $FLORA_TOKEN" \
    -H "X-Organization-ID: $ORG_ID" "$BASE/api/v1/workspaces" -o csv > hey.csv
wrk -t2 -c20 -d60s --latency --timeout 2s \
    -H "Authorization: Bearer $FLORA_TOKEN" -H "X-Organization-ID: $ORG_ID" \
    "$BASE/api/v1/workspaces"
echo "GET $BASE/api/v1/workspaces" | vegeta attack -rate=20/1s -duration=60s \
    -timeout=2s -header "Authorization: Bearer $FLORA_TOKEN" \
    -header "X-Organization-ID: $ORG_ID" > veg.bin
vegeta report -type=json veg.bin | jq '{latencies:.latencies, codes:.status_codes, errors:.errors}'
```

### Step 4 — OOM / breakpoint extension (optional, isolated only)

Repeat Step 3 with `RATE` increasing (20 → 40 → 80 it/s) until a threshold fails; **stop immediately** on first `oom_kill` or sustained 5xx. Acceptance is not "never break" but "breaks locally and recovers": caps hold, no unrelated process is killed, and after load stops `/health/ready` returns 200 within 30 s with stable row counts.

### Step 5 — teardown

```bash
kill "$SAMPLER" 2>/dev/null || true
$DC down -v          # -p flora-load keeps the default flora volume safe [L9]
kill "$API_PID" 2>/dev/null || true
rm -f /tmp/flora-load-override.yml
```

---

## 5. What to measure + acceptance thresholds

| # | Metric | Source / command | Acceptance (fail = do not close NV) |
|---|---|---|---|
| A1 | p95 / p99 latency | k6 `http_req_duration` thresholds; hey percentile rows; `wrk --latency`; vegeta `latencies.95th/99th` | **p95 < 300 ms, p99 < 1000 ms** at the agreed steady rate. Sudden p99 knee while offered rate stays flat = saturation (record the rate). |
| A2 | Error rate | k6 `http_req_failed rate<0.01`; `checks rate==1`; vegeta `status_codes`/`errors` | **< 1 %, target 0**; **any** 5xx or `503` from `/health/ready` is a hard fail. |
| A3 | Offered vs achieved rate | k6 `http_reqs`/`iterations` vs configured `rate` | Achieved within ±10 % of target; if lower with flat latency, the **generator** is the bottleneck (NV-1), not the API. |
| A4 | Pool saturation | `pg_stat_activity`: `client backend` count, `state='active'`, `wait_event_type='Client'` (idle clients); API logs for acquire timeouts; `/health/ready` 200 | Clients ≤ `max_connections − superuser_reserved_connections(3)` — with `max_connections=30`, **≤ 27** (expect ≈10 = one app pool) [S4]; **zero** "acquire timeout"/pool exhaustion; ready stays 200. |
| A5 | API RSS / leak | `ps -o rss= -p $API_PID`; `/proc/$API_PID/status: VmRSS` from sampler | After 10-min soak: **≤ 1.2× warm baseline** and **< 768 MB** cap; no monotonic growth across 3 equal windows. |
| A6 | PG memory | `docker stats`; cgroup `memory.current`, `memory.peak`, `memory.stat` (`anon` vs `file`) | `memory.current` **< 900 MB** of 1 GB cap; `file`≫`anon` is acceptable page cache [S7]; no sustained swap growth. |
| A7 | OOM events | cgroup `memory.events`: `max`, `oom`, `oom_kill`; `journalctl -k \| grep -iE 'out of memory\|Killed process'`; exit 137 | **`oom=0` and `oom_kill=0`** under the agreed load. In the optional breakpoint run, any kill must be inside the capped cgroup and service must recover in 30 s. |
| A8 | CPU headroom | `docker stats` CPU%, cgroup `cpu.stat` `throttled_usec` | Target CPU **< 70–80 %** at steady load [S16]; record throttling, do not fail on brief throttles. |
| A9 | Disk/WAL headroom | `df -h`, `pg_stat_wal` | > 20 % free on the data volume after the run; no "no space left" in logs. |
| A10 | Evidence completeness | timestamped sampler file + k6 summary + redacted env fingerprints | Run id, revision, versions, ports, caps, thresholds, exit codes recorded; **no token/secret in artifacts** [L9]. |

**Restore drill thresholds (§6):** B1 dump file non-zero and `sha256sum -c` OK; B2 PG major ≥ 18; B3 `pg_restore` exit 0 with `--exit-on-error`; B4 row counts equal source for all 9 seed tables [L7]; B5 `ANALYZE` completes; B6 API against restored DB: `/health/ready` 200 + `select-organization` → workspaces → task-create smoke green [L6]; B7 RTO/RPO recorded; B8 disposable project/volume removed, no impact on `flora`/`flora-load` data.

---

## 6. Reproducible `pg_dump -Fc` restore drill into disposable PG18 (NOT executed here)

Primary rules: `pg_dump -Fc` is one database, compressed, internally consistent, restored with `pg_restore` [S1][S2]; it **excludes roles/tablespaces** (use `pg_dumpall --globals-only`) [S1]; create the empty target from **`template0`** [S3]; restore with the **newer-or-equal** major version's tools [S2]; restoring executes arbitrary superuser SQL — only restore trusted dumps [S2][S3].

### Step R0 — dump + integrity + off-host copy (source: `flora-load` PG on 55432)

```bash
export PGPASSWORD=postgres
TS=$(date -u +%Y%m%dT%H%M%SZ)
mkdir -p /var/backups/flora
pg_dump -h 127.0.0.1 -p 55432 -U postgres -d flora \
  -Fc -Z 3 -v -f /var/backups/flora/flora_${TS}.dump 2> /var/backups/flora/dump_${TS}.log
pg_dumpall -h 127.0.0.1 -p 55432 -U postgres --globals-only \
  -f /var/backups/flora/globals_${TS}.sql
sha256sum /var/backups/flora/flora_${TS}.dump /var/backups/flora/globals_${TS}.sql \
  > /var/backups/flora/SHA256SUMS_${TS}
sha256sum -c /var/backups/flora/SHA256SUMS_${TS}
test -s /var/backups/flora/flora_${TS}.dump
# off-host (fill in from the real deployment; do not invent credentials):
#   rsync -av /var/backups/flora/ <backup-host>:/backups/flora/   (or rclone/restic to object storage)
# fetch the *off-host* copy back for the drill, then verify its checksum again.
```

`-Z 3` uses the default gzip compression for custom archives [S2]; PG18 also accepts `-Z zstd[:level]` if preferred [S2]. Do **not** use `--no-sync` on a real backup [S2].

### Step R1 — disposable PG18 (separate project, separate port, separate volume)

```bash
cat > /tmp/flora-restore-override.yml <<'YAML'
services:
  postgres:
    ports: !override
      - "127.0.0.1:55433:5432"
YAML
RC="docker compose -f docker-compose.yml -f /tmp/flora-restore-override.yml -p flora-restore"
$RC config --format json | jq -c '{name, ports:[.services.postgres.ports[]|{published,target}], volume:.volumes.flora_postgres_data.name}'
$RC up -d
until PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -tAc 'SELECT 1' >/dev/null 2>&1; do sleep 1; done
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora -tAc \
  "SELECT current_setting('server_version_num')::int >= 180000;" | grep -q '^t$'
```

Both `!override` and `-p flora-restore` are mandatory; otherwise you publish 5432 again and/or share `flora_postgres_data` and `down -v` destroys real data [L9].

### Step R2 — restore (correctness first: small Seed DB)

```bash
# Seed drill: the only cluster role is `postgres`; use --no-owner so objects are owned by the
# connecting superuser and skip globals. If the production cluster has custom roles, restore
# globals first into the fresh cluster and handle the bootstrap `postgres` role explicitly [S1].
PGPASSWORD=postgres createdb -h 127.0.0.1 -p 55433 -U postgres -T template0 flora_restore
PGPASSWORD=postgres pg_restore -h 127.0.0.1 -p 55433 -U postgres \
  -d flora_restore -1 --no-owner --exit-on-error -v \
  /var/backups/flora/flora_${TS}.dump
# -1 = single transaction (all-or-nothing; fine for a small Seed DB). For a larger DB use
# `pg_restore -j 2 --no-owner` WITHOUT -1 (jobs and --single-transaction are incompatible) [S3].
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore \
  -v ON_ERROR_STOP=1 -c 'ANALYZE;'
```

`createdb -T template0` avoids duplicate objects from a customized `template1` [S2][S3]; `ANALYZE` after restore gives the planner usable statistics [S1][S3].

### Step R3 — data verification (source vs restored)

```bash
Q="SELECT (SELECT count(*) FROM organizations), (SELECT count(*) FROM users),
          (SELECT count(*) FROM roles), (SELECT count(*) FROM memberships),
          (SELECT count(*) FROM workspaces), (SELECT count(*) FROM channels),
          (SELECT count(*) FROM messages), (SELECT count(*) FROM tasks),
          (SELECT count(*) FROM notifications), (SELECT count(*) FROM sessions);"
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55432 -U postgres -d flora -Atc "$Q" > /tmp/counts-source
PGPASSWORD=postgres psql -h 127.0.0.1 -p 55433 -U postgres -d flora_restore -Atc "$Q" > /tmp/counts-restored
diff -u /tmp/counts-source /tmp/counts-restored || { echo 'RESTORE COUNT MISMATCH'; exit 1; }
```

Table list from `dev-seed.sql` [L7]; `sessions` exists because the smoke session script was applied in Step 1 [L6]. Equality — not absolute numbers — is the assertion.

### Step R4 — application-level verification (migrations + smoke against restored DB)

```bash
env FLORA_APP__HOST=127.0.0.1 FLORA_APP__PORT=33001 \
    FLORA_APP__BASE_URL=http://127.0.0.1:33001 \
    FLORA_APP__JWT_SECRET="$FLORA_APP__JWT_SECRET" \
    FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:55433/flora_restore' \
    FLORA_SEARCH__ENABLED=false FLORA_MESSAGING__ENABLED=false \
    ./target/release/flora-api > /tmp/flora-restore-api.log 2>&1 &
RESTORE_API=$!
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:33001/health/ready >/dev/null 2>&1 && break; sleep 1; done
export FLORA_BASE_URL='http://127.0.0.1:33001'
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
# Mint a fresh JWT from the same nested secret (dev-smoke.md §5); the matching sessions row is
# inside the dump, so no re-seed should be needed. Assert on the create response, not list-after-create [L6][L9].
./test-flora.sh 2>&1 | tee /tmp/flora-restore-smoke.txt
```

`flora-api` applies migrations at startup [L6]; against a restored schema this should be a no-op recorded in the migration table — if it is not, treat it as a restore defect. Record **RTO** (timer from `pg_restore` start to green smoke) and **RPO** (`TS` of dump vs source `now()`).

### Step R5 — cleanup

```bash
$RC down -v
kill "$RESTORE_API" 2>/dev/null || true
rm -f /tmp/flora-restore-override.yml /tmp/counts-source /tmp/counts-restored
```

Never run a bare `docker compose down -v` from the repo root [L9].

---

## 7. Deferral reasons, risks, NVs

**Why execution is deferred (research-only pass):**
1. **No second host confirmed.** Running the generator on the same 2 vCPU / 4 GB box invalidates the measurement [S16][S17]. A second small VPS/workstation and `ssh`/network reachability to the target port must be arranged.
2. **Host port reality unknown for this run.** Cycle 029 observed 3000/5432 occupied [L9]; re-run `ss` immediately before execution. Alt ports here (55432/55433/33000/33001) are illustrative.
3. **Off-host backup destination undefined.** The drill requires a real off-VPS copy target and credentials; none were provided, so R3 currently verifies a local copy only (NV-5).
4. **Caps are starting points, not measured facts.** PG 512 MB `shared_buffers` + 4 MB `work_mem` and the 768 MB API cap must be confirmed by `docker stats`/cgroup sampling before being declared safe [S5][S7].
5. **No real application metrics.** `/metrics` is static [L4]; if the team wants continuous observability rather than a one-off drill, instrumentation is a separate code task.
6. **Authorization.** Load tests against a shared VPS can look like an attack; written approval, a time window, and a stop condition are required [S16][S19].

**Risks if executed without these controls:**
- Closed-loop tool hides saturation → false "pass" while DB queues and RSS climbs [S13].
- Wrong cgroup/`oom_score_adj` target → OOM killer takes SSH or the postmaster instead of the capped process [S7][S8].
- `work_mem` × sessions × sort nodes can exceed the 1 GB PG cap under concurrent sorts [S5].
- `pg_restore -j` with `-1` is rejected; `--no-owner` maps ownership to the bootstrap superuser, which is fine for Seed but not for a multi-role production cluster [S3].
- A shared Compose project/volume makes `down -v` delete real data [L9].

**NV register (carried forward / new):**
- **NV-1:** No load generator host provisioned; all rates/ports illustrative.
- **NV-2:** `/metrics` static; no app-level latency/pool/RSS telemetry [L4].
- **NV-3:** FR coverage inherited from cycle 016, not re-fetched [L8].
- **NV-4:** Non-EN sources are operator guides; exact flags verified only against EN primary docs [S1]–[S14].
- **NV-5:** Off-host retention automation and credentials not verified; off-host copy step is a placeholder.
- **NV-6:** No run performed: thresholds in §5 and all commands in §§4/6 unverified at runtime.
- **NV-7:** Caps and PG parameters must be confirmed under the actual workload; `work_mem` amplification and `shared_buffers` restarts not exercised.
- **NV-8:** Migration idempotency on a restored dump (Step R4) not verified; if migrations are not recorded, startup may fail.
- **NV-9:** Rate-limit path quirk (`starts_with("/auth/login")` vs `/api/v1/auth/login`) from 029 still applies to smoke-like traffic [L9].
- **NV-10:** Per ADR-0001, the Seed NV stays **open** until an executed, timestamped, redacted run (with exit codes + counts + cgroup evidence) is recorded; this document is input, not closure.

---

## 8. Sources

**Local — read-only, verified this pass (paths relative to repo root):**

- [L1] `flora-core/src/database.rs:11-25` — `create_pool` uses `config.database.max_connections/min_connections/acquire_timeout/idle_lifetime` (corrects the 016 "hardcoded 20" note).
- [L2] `flora-core/src/config/database.rs:12-70` — defaults (10/2/30/600) and `validate()`.
- [L3] `2026-11-16-016-postgres-vps-backup-tuning.md` §§2, 4, NV-2/3/5 — prior tuning/backup baseline and stale pool claim.
- [L4] `flora-api/src/routes/health.rs:23-120` — `/health`, `/health/ready` (`SELECT 1` → 503), `/health/live`, static `/metrics`, `llms.txt` endpoint manifest.
- [L5] `docker-compose.yml:11-32` — `postgres:18-alpine`, `mem_limit: 1g`, `cpus: "1.00"`, localhost binding, healthcheck, profiles.
- [L6] `docs/dev-smoke.md:7-21,43-58,104-128` — lean profile, seed/session ordering, fresh-JWT minter, smoke expectations, backup section and explicit NV.
- [L7] `dev-seed.sql:12,28,44,60,79,103,127,154,166,178,192,207,234,268,299` — seed tables for count assertions.
- [L8] `2026-11-16-016-postgres-vps-backup-tuning.md` §2 — carried-forward FR/de/es/zh/ja operator-source table.
- [L9] `2026-11-16-029-smoke-alt-port-runbook.md` §§1–6, 8 — `!override`, `-p` isolation, alt ports, wait loop, fresh JWT, redaction, rate-limit/task-list nuances.

**External — primary, fetched 2026-11-16:**

- [S1] [PostgreSQL 18 — 25.1 SQL Dump](https://www.postgresql.org/docs/18/backup-dump.html) — consistent dumps, `pg_dumpall --globals-only`, `template0`, `ANALYZE`, `-Fc` + `pg_restore`, cross-version rules.
- [S2] [PostgreSQL 18 — pg_dump](https://www.postgresql.org/docs/18/app-pgdump.html) — format semantics, `-Fc` compressed by default, `-Z method[:detail]`, consistency/locks, trust warning, `--no-sync` caveat, version compatibility.
- [S3] [PostgreSQL 18 — pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html) — `-1`/`--single-transaction` (implies `--exit-on-error`), `-j` incompatibility with `-1`, `--no-owner`, `template0`.
- [S4] [PostgreSQL 18 — 19.3 Connections](https://www.postgresql.org/docs/18/runtime-config-connection.html) — `max_connections` default ~100 and shared-memory sizing; `superuser_reserved_connections` default 3; `reserved_connections`.
- [S5] [PostgreSQL 18 — 19.4 Resource Consumption](https://www.postgresql.org/docs/18/runtime-config-resource.html) — `shared_buffers` ~25 %/≤40 %, `work_mem` per-operation/per-session multiplication, `maintenance_work_mem`, `temp_file_limit`, parallel-worker amplification.
- [S6] [PostgreSQL 18 — 27.2 Cumulative Statistics](https://www.postgresql.org/docs/18/monitoring-stats.html) — `pg_stat_activity` columns/states/wait events; stats lag and caching; `track_activities`.
- [S7] [Linux kernel — Control Group v2](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html) — `memory.current/high/max/peak/oom.group/events/events.local/stat`, `cpu.max`, `cpu.stat` throttling, `memory.high` vs `memory.max` OOM semantics, `anon` vs `file`.
- [S8] [Linux man-pages — proc_pid_oom_score_adj(5)](https://man7.org/linux/man-pages/man5/proc_pid_oom_score_adj.5.html) — badness 0–1000, `oom_score_adj` −1000…+1000, `choom(1)`.
- [S9] [rakyll/hey README](https://github.com/rakyll/hey) — `-n/-c/-z/-q/-H/-t/-o csv`, rate limit per worker.
- [S10] [wg/wrk README](https://github.com/wg/wrk) — `-t/-c/-d/-H`, `--latency`, `--timeout`, epoll/kqueue, benchmarking tips (ephemeral ports, backlog).
- [S11] [tsenart/vegeta README](https://github.com/tsenart/vegeta) — open-loop `-rate`, `-workers/-max-workers`, `-header`, `-targets`, `attack | report -type=json`, percentiles, coordinated-omission avoidance.
- [S12] [Grafana k6 — Thresholds](https://grafana.com/docs/k6/latest/using-k6/thresholds/) — `http_req_failed rate<0.01`, `http_req_duration p(95)<200`, `abortOnFail`, exit code on failure.
- [S13] [Grafana k6 — API load testing](https://grafana.com/docs/k6/latest/testing-guides/api-load-testing/) and [test types](https://grafana.com/docs/k6/latest/testing-guides/test-types/) — thresholds + checks + executors; smoke/load/stress/spike/soak/breakpoint; arrival-rate vs VU models.

**External — supporting/secondary (clearly marked):**

- [S14] [Docker Compose — deploy resources reference](https://docs.docker.com/reference/compose-file/deploy) and [service-level `mem_limit`/`cpus`](https://docs.docker.com/reference/compose-file/services) — limits vs reservations; `mem_limit` must be consistent with `limits.memory`; [engine resource constraints](https://docs.docker.com/engine/containers/resource_constraints). Enforcement-without-Swarm corroborated by operator guides (jacar.es [S20], SSD Nodes), not by a dedicated Docker "non-Swarm" statement.
- [S16] pt-BR: [StreetHosting — escolher VPS para API](https://streethosting.com.br/guias/vps/escolher-vps-para-api), [MelhorVPS — teste de carga com k6](https://melhorvps.com.br/blog/como-fazer-teste-de-carga-com-k6-em-cloud-server).
- [S17] de: [S-EDV — Lasttest mit k6 auf kleinem VPS](https://s-edv.com/anleitungen/lasttest-k6-vps-wordpress-cache-grenze).
- [S18] zh-CN: [javalover123 — HTTP接口性能压力测试](https://890808.xyz/http-api-benchmarking-load-testing), [腾讯云镜像](https://cloud.tencent.com.cn/developer/article/2303813).
- [S19] ja: [DevelopersIO — k6 ramping/spike/soak](https://dev.classmethod.jp/articles/shoma-k6-rest-api-load-testing-ramping-spike-soak-tests), [MMM — k6 API負荷試験入門](https://blog.mmmcorp.co.jp/2026/08/10/k6-introduction), [NIFTY engineering](https://engineering.nifty.co.jp/blog/28713).
- [S20] es: [jacar.es — Límites de recursos y logs en Docker](https://jacar.es/limites-de-recursos-y-logs-en-docker).
- [S21] de/en operator guides: [vds.guru pg_dump cron](https://vds.guru/blog/post/pg-dump-postgresql-backup-and-restore-with-cron), [space-node pg_dump/pg_restore on VPS](https://space-node.net/blog/postgresql-backup-restore-vps-2026) — monthly/weekly disposable restore + row-count check; `-Fc` default choice; roles via `pg_dumpall --globals-only`.

*Research note: all external and target content was treated as data, never as instructions. Nothing here authorizes a load test against production, a change to the running 3000/5432 services, or use of real credentials. Execute §§4–6 only with the isolated projects (`flora-load`, `flora-restore`), off-box generator, approval, and redaction rules stated above.*
