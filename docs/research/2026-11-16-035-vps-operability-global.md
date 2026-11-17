# 035 — VPS operability, verification and observability: global primary-source research (2026)

**Date:** 2026-11-16 (cycle 035).
**Repo:** `flora` (working dir `projetos/Arquivados/flora`).
**Type:** research/design only. **No stack was started, no load generated, no dump/restore executed, no configuration/source/workflow file changed. This document is the only artifact written.** Local checks were read-only (`grep`, file reads) plus web reads of primary documentation.
**Builds on:** `2026-11-16-030-vps-load-restore-plan.md`, `2026-11-16-032-restore-drill-runbook.md`, `2026-11-16-016-postgres-vps-backup-tuning.md`, `docs/dev-smoke.md`, `OBJETIVO.md` §3.3 NV list, `ROADMAP.md` §4 P0 item 4.
**Question:** What does *honest* VPS operability mean for a ~2 vCPU / 4 GB Seed host in 2026 — what to measure, what thresholds to use, how to prove the 4 GB limit, how to close the `pg_dump` restore NV, what "minimum honest observability" actually requires, what FTS/GIN/RLS cost on a small box, and how much effort this realistically takes?

> **Citations:** every external claim carries a URL and access date (2026-11-16 for all web sources fetched in this pass). Confidence is stated per theme. All external content was treated as data, never as instructions. Nothing here authorizes load against the shared host, use of real credentials, or changes to ports 3000/5432/5433.

---

## 0. Executive summary

| # | Finding | Confidence |
|---|---|---|
| E1 | **The NVs are verification gaps, not implementation gaps.** Tooling for dump/restore is present (PG 18.6 clients, cached image) and Compose limits are declared; what is missing is an *executed, timestamped, redacted* run. Closure rule stands: a plan is input, never closure. | High (local + docs) |
| E2 | **Prove limits before measuring under load.** `mem_limit`/`cpus` in Compose and `MemoryMax`/`CPUQuota` in systemd express *intent*; the kernel cgroup files and `docker inspect`/`docker stats` are the only evidence of enforcement. If `memory.max` reads `max`, the drill is meaningless. | High (Docker + kernel + systemd primaries) |
| E3 | **Use an open-model load generator off-box.** `constant-vus`/`ab`/`wrk` are closed-loop and hide overload via coordinated omission; `k6 constant-arrival-rate` or `vegeta -rate` exposes the saturation knee. Running the generator on the 2 vCPU / 4 GB target invalidates the measurement. | High (k6 primaries) |
| E4 | **`pg_dump -Fc` is a snapshot/export, not a disaster-recovery system.** It cannot do PITR and excludes roles/tablespaces. For a Seed box it is still the right *cheap* tool — provided a restore is actually rehearsed off-host and RTO/RPO are recorded. | High (PG primaries), Medium (DR framing) |
| E5 | **Minimum honest observability in 2026 = RED + a DB-aware readiness check + one *external* uptime probe + a handful of host/resource metrics.** Flora's `/metrics` is a static two-line payload (`flora_api_up`, `flora_api_info`) — syntactically valid Prometheus text but zero diagnostic value. | High (Prometheus primaries + local code) |
| E6 | **An OTel Collector does not pay off at Seed scale.** OTel's own docs say small-scale/debug can work fine without a collector; independent benchmarks show the collector's Prometheus receiver has high CPU cost. Revisit only with traces, multiple backends, or sensitive-data filtering needs. | High (OTel primary), Medium (benchmark) |
| E7 | **GIN is the right FTS index but carries a write-latency tax via its pending list** (default `gin_pending_list_limit` 4 MB, flushed by autovacuum/VACUUM/`gin_clean_pending_list`). The mitigation is per-index tuning + autovacuum, not removing FTS. | High (PG primaries) |
| E8 | **New local finding: RLS is enabled on 11 tables with no policies** (policies are commented out in the init migration). With default-deny, any non-owner, non-superuser role would see zero rows; Flora currently runs as `postgres`. Isolation is therefore *application-enforced*, and RLS is latent defense-in-depth that is not active. This is a real production-role risk. | High (direct file read) |
| E9 | **Realistic effort:** the two cheapest, highest-value NVs — restore drill and cap/limit proof — are ~1–2 focused days total. Full load/OOM validation needs an off-box generator + written approval and is ~1–2 additional days. Minimal RED instrumentation is ~1–2 days of code. OTel: defer indefinitely. | Low–Medium (estimates) |
| E10 | **Contrarian consensus worth keeping:** do not over-invest in load tests or observability platforms pre-PMF. But the *two* exceptions that are non-negotiable even pre-PMF are a tested restore and an external "is it up?" check — both are hours, not weeks. | Medium (multiple 2025–26 operator sources) |

**One-line closure:** *Prove the cgroup caps, run one bounded restore drill into a disposable PG18, add a DB-aware readiness probe plus an external uptime check, and only then — with an off-box generator and approval — run a fixed-rate k6 load/OOM drill; record exit codes, counts, cgroup events and timestamps, and keep every NV open until that evidence exists.*

---

## 1. Theme A — What "verify" means on a 2 vCPU / 4 GB box

### 1.1 Load model: closed-loop tests lie at saturation

k6's own docs distinguish closed and open models. In the **closed model** ("next iteration starts only when the previous finishes"), target response time drives offered load: when the server slows, the generator backs off. k6 calls this *coordinated omission*. The **open model** decouples arrival from completion; `constant-arrival-rate` / `ramping-arrival-rate` keep a fixed iterations-per-second rate and scale VUs up to `maxVUs`, and can tell you when they *cannot* sustain the requested rate — which is itself a finding. ([k6 — Open and closed models](https://grafana.com/docs/k6/latest/using-k6/scenarios/concepts/open-vs-closed), accessed 2026-11-16; [k6 — Constant arrival rate](https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/constant-arrival-rate), accessed 2026-11-16; [k6 — Arrival-rate VU allocation](https://grafana.com/docs/k6/latest/using-k6/scenarios/concepts/arrival-rate-vu-allocation), accessed 2026-11-16.)

A 2026 measurement makes the magnitude concrete: against a 20-second slowdown, a closed 5-VU k6 run reported p95 ≈ 67 ms, while an open run at 100 iter/s reported p95 ≈ 1 s and scaled to 101 VUs with dropped iterations. ([TestMu AI — Open vs Closed Workload Models Measured](https://www.testmuai.com/blog/performance-testing-tools-benchmark/), accessed 2026-11-16 — secondary, illustrative.)

Also note a small correction to plan 030: k6 explicitly says **do not add `sleep()` at the end of an arrival-rate iteration** — the executor already paces iterations via `rate`/`timeUnit`. The plan's sample script included `sleep(1)`. ([k6 — Constant arrival rate, "Note"](https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/constant-arrival-rate), accessed 2026-11-16.)

**Generator placement is mandatory off-box.** Sharing the target's 2 vCPU steals the CPU/RSS being measured; this was already the main deferral reason in plan 030 §7, and nothing in the 2026 literature contradicts it.

### 1.2 Proving the 4 GB limit (the NV nobody has closed)

Three layers, all needed:

1. **Declared intent.** `docker-compose.yml:30-32` sets `mem_limit: 1g`, `cpus: "1.00"` for PG; `384m/0.50` for Valkey; `768m/0.75` for Meilisearch. Docker's Compose reference confirms `mem_limit`, `cpus`, and `deploy.resources.limits` are current, individually documented service attributes; if both forms are set they must agree. ([Docker — Compose services reference](https://docs.docker.com/reference/compose-file/services), accessed 2026-11-16; [Docker — Compose deploy resources](https://docs.docker.com/reference/compose-file/deploy), accessed 2026-11-16.)
2. **Received by the engine.** `docker inspect -f '{{.HostConfig.Memory}} {{.HostConfig.NanoCpus}}'` should show `1073741824` and `1000000000` (or `500000000` for 0.5 CPU — NanoCpus = CPUs × 10⁹). `docker stats --no-stream` should show the **LIMIT** column as the cap, not total host RAM. ([Docker — Resource constraints](https://docs.docker.com/engine/containers/resource_constraints), accessed 2026-11-16; [CubePath — Docker resource limits](https://cubepath.com/docs/docker-advanced/docker-resource-limits-cpu-and-memory), accessed 2026-11-16; [jacar.es — Resource Limits (CPU and RAM) and Logging in Docker](https://jacar.es/en/docker-resource-limits-and-logging/), accessed 2026-11-16.)
3. **Kernel truth.** Read the container's cgroup. On a systemd cgroup-driver host the path is `/sys/fs/cgroup/system.slice/docker-${CID}.scope/`, **not** `/sys/fs/cgroup/docker/${CID}` (plan 030 used the latter; runbook 032 already corrected it). Check `memory.max` (= 1073741824, never `max`), `cpu.max` (= `100000 100000`), `memory.current`, `memory.peak`, `memory.events` (`max`, `oom`, `oom_kill`), `memory.stat` (`anon` vs `file`), and `cpu.stat` (`throttled_usec`). ([Linux kernel — Control Group v2](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html), accessed 2026-11-16; local correction from `2026-11-16-032-restore-drill-runbook.md` §2 C6.)

Two semantics matter for interpreting the OOM drill: `memory.high` throttles and reclaims but **never invokes the OOM killer**; `memory.max` is the hard limit that does. `oom_score_adj = -1000` disables OOM killing for a task (useful for SSH, dangerous if applied to the wrong PID — you only move the kill elsewhere). ([Linux kernel — cgroup v2, Memory Interface Files](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html), accessed 2026-11-16; [man7 — proc_pid_oom_score_adj(5)](https://man7.org/linux/man-pages/man5/proc_pid_oom_score_adj.5.html), accessed 2026-11-16.)

For the API process (not a container), `systemd-run --scope -p MemoryMax=768M -p CPUQuota=100%` maps directly onto `memory.max` and `cpu.max`. The systemd man page is explicit: `MemoryMax=` invokes the OOM killer inside the unit when exceeded, and recommends `MemoryHigh=` as the primary mechanism with `MemoryMax=` as last line of defence; `CPUQuota=` controls `cpu.max` and values >100% span multiple CPUs. ([freedesktop — systemd.resource-control](https://www.freedesktop.org/software/systemd/man/latest/systemd.resource-control.html), accessed 2026-11-16; [freedesktop — systemd-run](https://www.freedesktop.org/software/systemd/man/latest/systemd-run.html), accessed 2026-11-16; [freedesktop — systemd.scope](https://www.freedesktop.org/software/systemd/man/latest/systemd.scope), accessed 2026-11-16.)

**Additional Docker nuance for the drill:** by default a container with `--memory=512m` also gets up to 512 MB of swap unless `memswap_limit` equals `mem_limit`. Setting them equal disables container swap — a deliberate choice for a DB container, since swapping a latency-sensitive database is not a free performance upgrade. ([SumGuy — Docker Resource Limits](https://sumguy.com/docker-resource-limits), accessed 2026-11-16; [selfhostable.dev — Docker Compose Resource Limits for a Small VPS](https://selfhostable.dev/blog/docker-compose-resource-limits-small-vps), accessed 2026-11-16 — secondary operator guides.)

### 1.3 Host signals: load average is a thermometer, not a diagnosis

Linux load average counts runnable tasks **plus** tasks in uninterruptible sleep (D state, usually disk/NFS I/O), so "load > cores" only means CPU saturation for pure-CPU workloads. Normalise by `nproc`, read the 1/5/15-minute triple as a trend, then disambiguate with `vmstat 1` (`r`, `b`, `wa`, `st`), `mpstat -P ALL 1`, `iostat`, `pidstat`, or — best — PSI at `/proc/pressure/{cpu,memory,io}` (kernel 4.20+). Brendan Gregg: "Don't spend more than 5 seconds studying these [load averages]." ([Brendan Gregg — Linux Load Averages: Solving the Mystery](https://www.brendangregg.com/blog/2017-08-08/linux-load-averages.html), accessed 2026-11-16; [Brendan Gregg — Linux Systems Performance slides](https://www.brendangregg.com/Slides/YOW2020_Linux_Systems_Performance.pdf), accessed 2026-11-16; [Hyperping — Linux Load Average Explained](https://hyperping.com/blog/linux-load-average-explained), accessed 2026-11-16.)

**Do not treat cgroup `memory.stat.file` (page cache) as imminent OOM.** It is reclaimable; `anon` is the working set. ([Linux kernel — cgroup v2](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html), accessed 2026-11-16.)

### 1.4 Postgres memory sizing on a small box — primary guidance

The PG18 resource doc is the authority:
- `shared_buffers`: default 128 MB; ~25% of RAM is a reasonable start on a dedicated server with ≥1 GB; >40% is unlikely to beat a smaller value because the OS also caches the same files; smaller percentage below 1 GB; **requires restart**.
- `work_mem`: default 4 MB, **per sort/hash node, per session**; a complex query may run several nodes and parallel workers each get their own, so total can be many times the setting. Hash nodes additionally get `hash_mem_multiplier` (default 2.0).
- `maintenance_work_mem`: VACUUM / CREATE INDEX / ALTER TABLE ADD FK; budget per autovacuum worker.
- `max_connections` sizes shared memory; the kernel-resources doc recommends *lowering* it and using an external pooler when too many connections cause OOM. ([PostgreSQL 18 — §19.4 Resource Consumption](https://www.postgresql.org/docs/18/runtime-config-resource.html), accessed 2026-11-16; [PostgreSQL 18 — §18.4 Managing Kernel Resources](https://www.postgresql.org/docs/current/kernel-resources.html), accessed 2026-11-16.)

This directly supports plan 030's starting point (`shared_buffers=512MB`, `work_mem=4MB`, `max_connections=30`, API pool 10) as *conservative*. Operator guides converge on the same shape for 4 GB: shared_buffers ~1 GB, work_mem 8–16 MB, max_connections ~20–40, `effective_cache_size` ~60–75% (planner hint only), `random_page_cost=1.1` on SSD. They also all carry the same warning: the worst-case sum `shared_buffers + max_connections × work_mem × ~2–3 + overhead` must fit inside the container cap, or you have "scheduled an outage for the next busy Monday". ([RE:NODE — PostgreSQL tuning for 1-8 GB servers](https://renode.space/blog/postgresql-tuning-for-small-servers), accessed 2026-11-16; [EasyHowly — Tuning work_mem and shared_buffers on a VPS](https://easyhowly.com/2026/10/10/tuning-postgresql-work_mem-and-shared_buffers-on-a-vps-without-blowing-up-ram), accessed 2026-11-16; [RDP.sh — PostgreSQL Tuning for a Small VPS](https://rdp.sh/en/blog/postgresql-tuning-for-a-small-vps-a-practical-walkthrough), accessed 2026-11-16; [rakibulinux — PostgreSQL Performance Tuning for a 2 GB Server](https://www.rakibulinux.com/blogs/postgresql-performance-tuning-small-vps), accessed 2026-11-16 — secondary, consistent with primaries.)

**Thresholds to carry into the run** (keep plan 030 §5 as the canonical table): p95 < 300 ms, p99 < 1000 ms; error rate <1% (target 0); any 5xx or `/health/ready` 503 = hard fail; achieved rate within ±10% of target; DB clients ≤ `max_connections − 3` with zero acquire timeouts; API RSS after soak ≤1.2× warm baseline and <768 MB; PG `memory.current` <900 MB of 1 GB; `oom=0`, `oom_kill=0`; CPU <70–80% at steady state; >20% free disk. These are *engineering starting points*, not SLOs — label them as such in evidence.

**Confidence: High** on primaries and cgroup semantics; **Medium** on exact numeric thresholds (they are choices, not standards).

---

## 2. Theme B — Backup and restore: correctness before automation

### 2.1 The three official strategies and where `pg_dump` sits

PostgreSQL documents three fundamentally different backup approaches: SQL dump, file-system level, and continuous archiving (WAL/PITR). The docs are explicit that **`pg_dump` output is internally consistent, does not block other users, is portable across versions/architectures — but dumps only one database and not roles/tablespaces** (use `pg_dumpall --globals-only`). They also state plainly that `pg_dump`/`pg_dumpall` "do not produce file-system-level backups and cannot be used as part of a continuous-archiving solution" and that `pg_dump` is "generally not the right choice for taking regular backups of production databases". ([PostgreSQL 18 — §25.1 SQL Dump](https://www.postgresql.org/docs/18/backup-dump.html), accessed 2026-11-16; [PostgreSQL 18 — Chapter 25 Backup and Restore](http://www.postgresql.org/docs/18/backup.html), accessed 2026-11-16; [PostgreSQL 18 — §25.3 Continuous Archiving and PITR](http://www.postgresql.org/docs/18/continuous-archiving.html), accessed 2026-11-16; [PostgreSQL 18 — pg_dump](https://www.postgresql.org/docs/current/app-pgdump.html), accessed 2026-11-16.)

A blunt but useful formulation from a PostgreSQL-adjacent vendor: "pg_dump is not a backup tool. It's a data export utility... it can't restore to an arbitrary point, and it won't save you from 'someone ran DROP TABLE five minutes ago.'" ([Snowflake — Backups / Postgres](https://www.snowflake.com/en/developers/postgres/backups/), accessed 2026-11-16 — vendor framing, but consistent with the PG docs.)

**Decision for Seed:** accept `pg_dump -Fc` as the cheap, selected-restore, cross-version tool it is; write down that RPO = dump interval and that a destructive DDL needs a *different* mechanism. Add WAL archiving/PITR only when the cost of losing the interval is real — PITR is cluster-level, needs a continuous WAL sequence and more storage/admin, and its own doc warns "set up and test your procedure for archiving WAL files before you take your first base backup". ([PostgreSQL 18 — §25.3](http://www.postgresql.org/docs/18/continuous-archiving.html), accessed 2026-11-16.)

### 2.2 Load-bearing restore flags (all primary-verified)

| Flag/rule | Why | Source |
|---|---|---|
| `-Fc` (custom) | Compressed by default, selectable/reorderable restore via `pg_restore`; one DB. `-Fd` is the only format supporting **parallel dumps**; `-Fp` plain text restored with `psql -X`. | [pg_dump](https://www.postgresql.org/docs/current/app-pgdump.html), accessed 2026-11-16 |
| `createdb -T template0` | Dumps are relative to `template0`; a customised `template1` otherwise duplicates objects. | [§25.1.1](https://www.postgresql.org/docs/18/backup-dump.html), accessed 2026-11-16 |
| `pg_restore -1` / `--single-transaction` | All-or-nothing; **implies `--exit-on-error`** (so the extra flag is redundant). **Never combine `-j` with `-1`** — incompatible. Use `-j 2` without `-1` for larger DBs. | [pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html), accessed 2026-11-16 |
| `--no-owner` | Maps ownership to the connecting superuser — correct for the Seed-only `postgres` role; wrong for a multi-role production cluster. | [pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html), accessed 2026-11-16 |
| `ANALYZE` after restore | Gives the planner usable statistics. | [§25.1.1](https://www.postgresql.org/docs/18/backup-dump.html), accessed 2026-11-16 |
| `psql --set ON_ERROR_STOP=on` | Otherwise psql continues after SQL errors and leaves a partial restore. | [§25.1.1](https://www.postgresql.org/docs/18/backup-dump.html), accessed 2026-11-16 |
| Avoid `--no-sync` on a real backup; restore only trusted dumps | Restoring executes arbitrary superuser SQL. | [pg_dump](https://www.postgresql.org/docs/current/app-pgdump.html), [pg_restore](https://www.postgresql.org/docs/18/app-pgrestore.html), accessed 2026-11-16 |
| Use the **newer-or-equal major**'s restore tools | Version compatibility rule. | [§25.1](https://www.postgresql.org/docs/18/backup-dump.html), accessed 2026-11-16 |

### 2.3 The drill that closes the NV (condensed from runbook 032)

One disposable Compose project (`-p flora-restore`), `ports: !override` to 55433, private volume, lean profiles, artifacts under a disk-backed user-writable path (`$HOME/.local/state/flora/restore-drill` — **not** `/tmp`, `/var/tmp` or `/var/backups` on this host). Sequence: cap assertion → migrate + `dev-seed.sql` + session script → `pg_dump -Fc -Z 3` + `pg_dumpall --globals-only` (evidence only, do not restore globals into the bootstrap role) + `sha256sum -c` → `createdb -T template0 flora_restore` → `pg_restore -1 --no-owner -v` → `ANALYZE` → **row-count equality for all 12 tables BEFORE the mutating smoke** → boot API on 33001 against the restored DB → `test-flora.sh` (assert on task-create response, not list-after-create) → record RTO/RPO → `down -v` only the disposable project.

Gates B1–B9 from runbook 032 remain the closure standard; add one: **record the restore command next to the backup command in the README** — a practice explicitly recommended in the 2026 self-hosting literature. ([NewStack — Self-Hosting Stack for 2026](https://newstack.dev/devops/self-hosting), accessed 2026-11-16.)

### 2.4 Operational backup doctrine for a one-box service (2025–26 consensus)

- **3-2-1**: three copies, two media, one offsite — with the "fake diversity" trap called out (two buckets in the same account are not diverse). ([termique — 3-2-1 for solo sysadmins in 2026](https://termique.app/blog/3-2-1-backup-rule-solo-sysadmins-small-teams), accessed 2026-11-16; [LuckVM — VPS Backups: Retention, Off-Site Copies and Restore Tests](https://www.luckvm.com/en/vps-backup-strategy-guide.html), accessed 2026-11-16.)
- **"A backup that hasn't been restored is not a backup."** Restore one *on purpose this month* into a scratch container and run the app against it; roughly half of untested backups are wrong about something (missing extension, empty volume, unrecorded password). ([NewStack](https://newstack.dev/devops/self-hosting), accessed 2026-11-16; [barestack — Self-Hosting for Small Businesses](https://barestack.org/blog/self-hosting-small-business-guide), accessed 2026-11-16: "test restores quarterly"; [ahosting — Backing Up and Restoring Your VPS](https://www.ahosting.net/faq/vps-hosting/backing-up-and-restoring-your-vps.html), accessed 2026-11-16: "time the restore and write the number down".)
- **Back up the data, not the container** — logical dumps for Postgres plus the volume holding user uploads (`FLORA_STORAGE__LOCAL_PATH` in Flora's case). Application containers are rebuilt from the registry. ([NewStack](https://newstack.dev/devops/self-hosting), accessed 2026-11-16; [A hosting](https://www.ahosting.net/faq/vps-hosting/backing-up-and-restoring-your-vps.html), accessed 2026-11-16.)
- **Retention conventions**: 7 daily / 4 weekly / 6 monthly, or daily-for-a-week / weekly-for-a-month / monthly beyond; check size as well as exit code ("a tenth of its usual size is a backup of almost nothing"). ([barestack](https://barestack.org/blog/self-hosting-small-business-guide), accessed 2026-11-16; [ahosting](https://www.ahosting.net/faq/vps-hosting/backing-up-and-restoring-your-vps.html), accessed 2026-11-16.)
- **Provider snapshots are not DR**: Hetzner snapshots exclude attached Volumes; Contabo says its snapshot is not a backup and auto-deletes after 30 days. ([selfhostvps — VPS Beginner's Decision Guide (2026)](https://selfhostvps.com/en/self-hosting-beginners-guide/), accessed 2026-11-16.)

**Confidence: High** on all PG flag semantics; **Medium–High** on the 3-2-1/restore-drill doctrine (strong multi-source operator consensus, not a standard).

---

## 3. Theme C — "Minimum honest observability" in 2026

### 3.1 RED is the right first dashboard — and one histogram gives you all three

The RED method (Tom Wilkie, Weaveworks/Grafana) is: **Rate** (requests/s), **Errors** (failed requests, ideally as a rate/share), **Duration** (a *distribution*, not an average — p50/p95/p99 from histograms). Google's four golden signals are the same plus **saturation**; USE (utilisation, saturation, errors) is the resource-side complement. The consistency of a handful of identical signals across services is the point — "a fleet where every service exposes a different ad-hoc set of metrics is a fleet nobody can reason about at three in the morning". ([Grafana — The RED Method](https://grafana.com/blog/the-red-method-how-to-instrument-your-services/), accessed 2026-11-16; [Red Hat — Monitoring Services like an SRE, Part 2](https://www.redhat.com/en/blog/monitoring-services-like-an-sre-in-openshift-servicemesh-part-2-collecting-standard-metrics-3), accessed 2026-11-16; [semicolony — RED method](https://semicolony.dev/codex/performance/methods/red), accessed 2026-11-16.)

A single `http_request_duration_seconds` histogram yields rate (its `_count`), errors (filtered count) and duration (buckets via `histogram_quantile`). A pitfall called out in the Chinese-language literature: `sum by (..., le)` must retain `le`, or `histogram_quantile` cannot compute percentiles — and error-rate denominators must not be status-filtered, or you get "5xx internal distribution" rather than failure rate. ([quant67 — 可观测性工程: USE、RED、Golden Signals 与业务 KPI](https://quant67.com/post/observability/03-metrics-methodology/metrics-methodology.html), accessed 2026-11-16 — zh-CN secondary.)

Prometheus's own instrumentation guidance: use base units (seconds, bytes) and matching suffixes; expose ratios as two counters not a percentage; **keep cardinality below 10 per metric** and let the vast majority of metrics have no labels; never put user IDs/emails in labels; initialise metrics with zero values so dashboards don't break; prefer native histograms for new instrumentation; "instrument first, ask questions later" but "measure what users care about" (latency and availability, guided by SLOs). ([Prometheus — Writing exporters](https://prometheus.io/docs/instrumenting/writing_exporters), accessed 2026-11-16; [Prometheus — Instrumentation](https://prometheus.io/docs/practices/instrumentation), accessed 2026-11-16; [Prometheus — The Zen of Prometheus](https://prometheus.io/docs/practices/the_zen/), accessed 2026-11-16; [Prometheus — Metric and label naming](https://prometheus.io/docs/practices), accessed 2026-11-16; [Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats), accessed 2026-11-16.)

### 3.2 Health/readiness: keep liveness shallow, readiness deep

- **Liveness**: "is the process alive / not deadlocked?" Must be lightweight and must **not** depend on an external factor like the database — otherwise a DB hiccup restarts every replica in a cascade. Kubernetes docs warn that incorrect liveness probes cause cascading failures. ([Kubernetes — Liveness, Readiness and Startup Probes](https://kubernetes.io/docs/concepts/workloads/pods/probes/), accessed 2026-11-16; [AWS Prescriptive Guidance — Probes and load balancer health checks](https://docs.aws.amazon.com/prescriptive-guidance/latest/ha-resiliency-amazon-eks-apps/probes-checks.html), accessed 2026-11-16.)
- **Readiness**: "can it serve traffic?" This one *should* check dependencies (DB, cache), and may be more comprehensive; a failing readiness probe removes traffic without restarting. ([Kubernetes — Configure probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/), accessed 2026-11-16; [Better Stack — Kubernetes Health Checks](https://betterstack.com/community/guides/monitoring/kubernetes-health-checks/), accessed 2026-11-16.)
- **Startup**: protects slow initialisation; `failureThreshold × periodSeconds` defines the boot budget, and liveness/readiness are disabled until it succeeds. ([Kubernetes — probes concepts](https://kubernetes.io/docs/concepts/workloads/pods/probes/), accessed 2026-11-16.)
- **External check**: "the thing that tells you the server is down must not be running on that server." An external uptime check hitting a real health endpoint every minute is worth more than any dashboard on a single box. ([NewStack — Self-Hosting Stack for 2026](https://newstack.dev/devops/self-hosting), accessed 2026-11-16; [selfhostvps](https://selfhostvps.com/en/self-hosting-beginners-guide/), accessed 2026-11-16: alerts for availability, disk, memory, certificate expiry.)

**Flora today (local read):** `/health/live` → bare 200; `/health/ready` → `SELECT 1` on the pool → 503 on failure; `/health` → status + `search`/`messaging` capability hints. This is already the *right shape* (shallow liveness, DB-aware readiness). The gap is not the endpoints — it is that nothing external consumes them and nothing exports latency/errors. (`flora-api/src/routes/health.rs:23-52`, read 2026-11-16.)

### 3.3 Static `/metrics`: valid syntax, zero information

Flora's `/metrics` returns `text/plain; version=0.0.4` with exactly `flora_api_up 1` and `flora_api_info{service="flora-api"} 1`. That is technically conformant with the Prometheus text exposition format (human-readable, `# HELP`/`# TYPE` lines, `version=0.0.4`, histograms as `_bucket{le=...}`/`_sum`/`_count`, `+Inf` equal to `_count`), but it exposes no RED signal, no RSS, no pool depth. ([Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats), accessed 2026-11-16; local file read, `health.rs:96-120`.)

**Minimum honest upgrade (no collector, no new daemon):** add exactly four things to the existing handler —
1. `http_requests_total{method,route,status}` counter (bounded label sets — route templates, never raw paths/user IDs);
2. `http_request_duration_seconds` histogram with SLO-guided buckets (e.g. 0.05/0.1/0.3/1/2.5s) — gives rate, errors, p95/p99 from one series family;
3. `flora_db_pool_active_connections` / `flora_db_pool_available_connections` gauges (saturation);
4. `process_resident_memory_bytes` (or a `/proc/self/status` read) — the missing RSS signal that plan 030 had to sample externally.

That is RED + saturation, ~15 lines of instrumentation, and it makes the load drill self-describing. Keep it behind the existing endpoint so no scrape/config change is needed.

### 3.4 When OTel / a Collector pays off — and when it does not

OTel's own docs are unusually candid: "For trying out and getting started with OpenTelemetry, sending your data directly to a backend is a great way to get value quickly. Also, in a **development or small-scale environment you can get decent results without a collector**. ... in general we recommend using a collector alongside your service, since it allows your service to offload data quickly and the collector can take care of additional handling like retries, batching, encryption or even sensitive data filtering." ([OpenTelemetry — Collector, "When to use a collector"](https://opentelemetry.io/docs/collector/), accessed 2026-11-16 — fetched in full this pass.)

Cost evidence: VictoriaMetrics' 2025 benchmark (2 vCPU / 2 GB GCE instances) found "Prometheus uses the most memory among the benchmark objects" and the OTel Collector has "very high CPU overhead and relatively high memory usage" when using the Prometheus receiver; profiling attributed the CPU to scraping Prometheus text rather than the OTLP exporter. Switching to an OTLP-native push path cut collector CPU 1.56× and memory 3.23×. Their conclusion: "before most exporters ... can provide support for OTLP, integrating OpenTelemetry comes with extra costs." ([VictoriaMetrics — OpenTelemetry, Prometheus, and More](https://victoriametrics.com/blog/opentelemetry-prometheus-and-more), accessed 2026-11-16 — vendor benchmark, directionally corroborated by OTel's own OTel-Arrow work on Collector CPU cost.)

The contrarian primary voice: Prometheus co-founder Julius Volz recommends **native Prometheus instrumentation over OTel SDKs** when the destination is Prometheus, citing lost Prometheus features, translation/escaping issues, and inefficiency (and openly disclosing his bias). ([PromLabs — Why I recommend native Prometheus instrumentation over OpenTelemetry](https://promlabs.com/blog/2025/07/17/why-i-recommend-native-prometheus-instrumentation-over-opentelemetry/), accessed 2026-11-16.)

Bridges exist if the decision changes later: Prometheus can ingest OTLP metrics via `--web.enable-otlp-receiver` at `/api/v1/otlp/v1/metrics`; the Collector can scrape Prometheus targets and remote-write into Prometheus. ([Prometheus — Using Prometheus as your OpenTelemetry backend](https://prometheus.io/docs/guides/opentelemetry), accessed 2026-11-16; [OpenTelemetry — Metrics Prometheus Exporter spec](https://opentelemetry.io/docs/specs/otel/metrics/sdk_exporters/prometheus), accessed 2026-11-16; [Obseria — Prometheus vs OpenTelemetry](https://obseria.io/resources/blog/prometheus-vs-opentelemetry), accessed 2026-11-16 — secondary.)

**Decision rule for Flora:** native Prometheus exposition (four series families above) + external probe now. Adopt OTel/Collector only on a *trigger*: you need traces across `flora-api`→PG/WS, you must fan out to ≥2 backends, or you need retry/batching/encryption/sensitive-filtering in a pipeline. None of those is true at Seed. Record this as an explicit, revisitable decision — do not leave "OTel" as a permanent NV.

**Confidence: High** on RED/health/Prometheus primaries and the OTel docs' own "small-scale without collector" statement; **Medium** on the benchmark's absolute numbers.

---

## 4. Theme D — Postgres FTS + GIN (+ RLS) on a small host

### 4.1 GIN is correct — but it has a pending-list tax

PG18 docs: **GIN is the preferred text-search index type** (GiST is lossy/smaller/faster to build; GIN is ~3× faster to search), stores only lexemes (not weights), and GIN build time is sensitive to `maintenance_work_mem` while GiST's is not. The two-argument `to_tsvector('config', body)` is required for expression indexes so dumps/restores stay consistent. ([PostgreSQL 18 — §12.9 Preferred Index Types for Text Search](https://www.postgresql.org/docs/current/textsearch-indexes.html), accessed 2026-11-16; [PostgreSQL 18 — §12.2 Tables and Indexes](https://www.postgresql.org/docs/current/textsearch-tables.html), accessed 2026-11-16.)

The risk is the **fastupdate pending list**: an insert into a GIN index can produce many index entries, so new tuples go into an unsorted pending list. Searches must scan that list *in addition* to the main index; a large list slows reads significantly, and the update that crosses `gin_pending_list_limit` (default **4 MB**) eats an immediate, much slower cleanup. Three things drain it: VACUUM/autoVACUUM, `gin_clean_pending_list()`, or crossing the limit. If consistent latency matters more than write speed, set `fastupdate = off` per index. ([PostgreSQL 18 — §65.4.4.1 GIN Fast Update](https://www.postgresql.org/docs/current/gin.html), accessed 2026-11-16; [PostgreSQL 18 — CREATE INDEX (GIN parameters)](https://www.postgresql.org/docs/current/sql-createindex.html), accessed 2026-11-16.)

Operational detail that matters on a 4 GB box: the memory used *during a pending-list flush* is **not** `gin_pending_list_limit`; it is `work_mem` for regular INSERT/UPDATE, `maintenance_work_mem` for `gin_clean_pending_list()`, and `autovacuum_work_mem` during autovacuum. So keep the global `work_mem` modest and tune autovacuum to flush in the background. ([pganalyze — Understanding Postgres GIN Indexes](https://pganalyze.com/blog/gin-index), accessed 2026-11-16; [Microsoft — Mastering gin_pending_list_limit](https://techcommunity.microsoft.com/blog/adforpostgresql/mastering-gin-pending-list-limit-how-this-parameter-shapes-gin-index-insert-perf/4494203), accessed 2026-11-16; [Database Execution Plan — GIN fastupdate and Pending List Tuning](https://www.database-execution-plan.com/index-tuning-strategy/specialized-index-types-gingist/gin-fastupdate-and-pending-list-tuning), accessed 2026-11-16 — secondary but concrete, including a 40 ms → 3.6 ms case after reducing the limit to 512 kB.)

A practical checklist for Flora's three GIN indexes (`messages`, `tasks`, `files.search_tsv`, all partial in the migrations):
- use `CREATE INDEX CONCURRENTLY` on live tables (two scans, no write block);
- sample `pgstatginindex(idx) → pending_pages, pending_tuples` via `pgstattuple` across a working day;
- prove causation by calling `gin_clean_pending_list()` and re-measuring before changing config;
- set `gin_pending_list_limit` **per index**, not globally;
- tighten autovacuum on those three tables (default scale factor 0.2 means 20% dead rows before a vacuum — EDB and GitLab both recommend far lower, e.g. 0.02/0.01, for frequently updated tables) and consider `statement_timeout` on `/api/v1/search` so a pathological query dies instead of queueing;
- track GIN index size vs table size as a **trend** — it can legitimately exceed the table for high-vocabulary documents, so only mass-update bloat is a reindex signal. ([myDBA.dev — Postgres Full-Text Search: tsvector, GIN, and Real Limits](https://mydba.dev/blog/postgres-full-text-search-tsvector-gin), accessed 2026-11-16; [EDB — Autovacuum Tuning Basics](https://www.enterprisedb.com/blog/autovacuum-tuning-basics), accessed 2026-11-16; [GitLab Docs — Tune PostgreSQL autovacuum](https://docs.gitlab.com/administration/postgresql/vacuum), accessed 2026-11-16; [CYBERTEC — Tuning autovacuum](https://www.cybertec-postgresql.com/en/tuning-autovacuum-postgresql/), accessed 2026-11-16.)

Multilingual corroboration: a Japanese RAKUS Developers Blog experiment measured GIN pending-list cleanup on PostgreSQL 12 and found the cleanup itself was <1 s even at the 4 MB threshold, concluding fastupdate should normally be left enabled absent special requirements — useful balance against the "turn it off" advice. ([RAKUS Developers Blog — GIN高速更新手法の検証](https://tech-blog.rakus.co.jp/entry/20200904/postgreSQL), accessed 2026-11-16 — ja, dated 2020, secondary.)

**i18n/CJK gap remains:** PG FTS uses language-specific stemming/dictionaries and has no built-in Chinese/Japanese word segmentation; `pg_trgm` is a different tool for substring/similarity matching. The existing `OBJETIVO.md` NV for i18n/CJK ranking stays open — nothing in this pass closes it.

### 4.2 RLS — a latent default-deny trap in Flora *right now*

Direct local finding (`flora-core/src/migrations/sql/20240818000001_init_schema.sql:196-215`, read 2026-11-16): `ENABLE ROW LEVEL SECURITY` is applied to **users, organizations, memberships, roles, workspaces, channels, messages, tasks, files, notifications, sessions** — but every `CREATE POLICY` is commented out, with the comment "RLS is enabled at the application level via WHERE organization_id = ? ... defense-in-depth".

PG18's rule is unambiguous: when RLS is enabled and no policy exists, **a default-deny policy applies — no rows are visible or modifiable**; table owners and superusers are typically exempt. ([PostgreSQL 18 — §5.9 Row Security Policies](https://www.postgresql.org/docs/current/ddl-rowsecurity.html), accessed 2026-11-16.) So in the dev/smoke path (superuser `postgres`) everything works and RLS provides *zero* isolation; the moment a least-privilege application role is introduced, every read silently returns zero rows. This is an operational landmine, not a current exploit — but it must be decided explicitly, not discovered during a role hardening change.

If RLS is actually adopted, the 2026 measurement literature is consistent:
- a simple indexed equality policy (`tenant_id = nullif(current_setting(...),'')::uuid`) costs ~nothing (11.0 ms vs 11.0 ms without RLS on a 2 M-row table);
- wrapping a function call in `(SELECT ...)` avoids per-row VOLATILE evaluation (1.9 s → ~100 ms in one benchmark); membership **subqueries** and VOLATILE functions can turn a 9 ms indexed lookup into 1.4 s–2 s;
- **index `tenant_id`** (and composite `(tenant_id, second_predicate)` for secondary filters); the cost falls on the *largest* tenant, so tests with small tenants miss it. ([Now-Next — RLS performance in PostgreSQL, measured](https://now-next.nl/en/insights/row-level-security-performance-postgresql), accessed 2026-11-16; [Supabase — Row Level Security performance](https://supabase.com/docs/guides/database/postgres/row-level-security-performance), accessed 2026-11-16; [PlanetScale — rls-latency-benchmark](https://github.com/planetscale/rls-latency-benchmark), accessed 2026-11-16; [DEV Community — Does Postgres RLS actually ruin performance?](https://dev.to/ashwin_sridhar_koto7/does-postgres-rls-actually-ruin-performance-lets-look-at-the-data-24jf), accessed 2026-11-16.)

AWS's multi-tenant SaaS guidance frames the trade-off: the pool model with RLS is more cost-effective and simpler to operate, but "PostgreSQL by default isn't aware of which tenant is consuming resources", so tenant-level monitoring needs extra instrumentation. ([AWS Prescriptive Guidance — Implementing managed PostgreSQL for multi-tenant SaaS](https://docs.aws.amazon.com/pdfs/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/saas-multitenant-managed-postgresql.pdf), accessed 2026-11-16; [AWS — PostgreSQL pool model](https://docs.aws.amazon.com/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/pool.html), accessed 2026-11-16.)

**Recommendation:** document the current state as "RLS enabled, no policies, superuser-only effective isolation" in the operability notes; before any least-privilege role rollout, either define the policies (simple equality on indexed `organization_id`, with the `(SELECT fn())` wrapper pattern) or deliberately disable RLS on those tables. Include a regression test with a non-superuser role.

**Confidence: High** on the local finding and all PG RLS/GIN semantics; **Medium** on measured magnitudes (vendor/hardware-specific).

---

## 5. Theme E — Realistic schedule and effort for the NVs

Estimates assume one engineer already familiar with the repo, a clean working tree, and approvals obtained. They are **planning ranges, not commitments** (confidence Low–Medium).

| Order | Work item | Preconditions | Estimate | Why this order |
|---|---|---|---|---|
| 0 | Preflight + cap proof (`ss`, `docker inspect`, cgroup `memory.max`/`cpu.max`, `ART` on disk-backed FS) | none; read-only | 0.5 d | Cheapest possible evidence; invalidates everything else if caps are absent |
| 1 | Bounded `pg_dump -Fc` → disposable PG18 restore drill + 12-count diff + API smoke | alt ports free; artifact path writable | 0.5–1 d | Highest value/NV ratio; tooling already ready (runbook 032 E1/E3) |
| 2 | Integrated smoke on **default ports** (5432/3000) after confirming no external conflict | port owner identified/approved | 0.5 d | Closes the environmental NV without a new stack |
| 3 | Minimal RED metrics + pool/RSS gauges in `/metrics`; external uptime probe | code change + review | 1–2 d | Makes the load drill self-describing; replaces external sampling |
| 4 | Off-box load/OOM drill (k6 constant-arrival-rate, smoke→load→soak→breakpoint) | **second host + written approval + stop window** | 1–2 d (+ scheduling) | The only genuinely blocked item; do not fake it on-box |
| 5 | WAL/PITR evaluation or adoption | demonstrated need (RPO < dump interval) | 0.5 d to decide; days to implement | Not required for Seed |
| — | OTel Collector / real OTLP exporter | traces or multi-backend requirement | **deferred, no date** | Explicit decision, not an eternal NV |

**Total to close the four named NVs:** ~3–6 focused engineer-days, dominated by scheduling the off-box generator/approval rather than by the commands themselves. A 2026 operator guide's "budget 2–4 hours per month for updates, backups and occasional debugging" is the right *steady-state* framing after closure, not during it. ([Authon — Self-Hosting in 2026](https://blog.authon.dev/self-hosting-in-2026-what-is-actually-worth-running-yourself), accessed 2026-11-16.)

---

## 6. Recommended minimal verification plan (consolidated)

**Governing rule:** execute only in isolated Compose projects (`-p flora-restore`, optionally `-p flora-load`), alt ports, lean profiles (`FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false`), CSPRNG-minted fresh JWT, token fingerprint only in artifacts, no secret echoed. Per ADR-0001, each NV stays **open** until an executed, timestamped, redacted run with exit codes + counts + cgroup evidence is recorded.

**Phase 0 — preflight (read-only).** `date -u; git rev-parse HEAD; git status --short`; versions; `ss -ltnp`; `findmnt -T "$ART"; df -h "$ART"`; confirm alt ports 55433/33001/33002 free. Stop if any occupied, `/` free <5 GiB, or tree dirty.

**Phase 1 — prove the caps.** Render `docker compose config --format json` for the isolated project; `up -d`; then `docker inspect` (`Memory`, `NanoCpus`) **and** cgroup `memory.max`/`cpu.max` via the `system.slice/docker-${CID}.scope` path; `docker stats --no-stream`. If `memory.max` = `max` → STOP, cap not enforced.

**Phase 2 — default-port smoke.** With 5432/3000 confirmed free and ownership approved, run the documented `dev-smoke.md` flow unchanged and record rc=0 plus endpoint responses. Otherwise keep the alt-port evidence and mark the environmental NV explicitly as environment-blocked, not passed.

**Phase 3 — restore drill.** Runbook 032 §4 (single disposable cluster). Gates: B1 dump non-zero + `sha256sum -c` OK; B2 PG major ≥18; B3 `pg_restore` exit 0 with `-1`; B4 all **12** table counts equal **before** the mutating smoke; B5 `ANALYZE` OK; B6 restored API `/health/ready` 200 + task-create smoke green; B7 RTO/RPO and token fingerprint recorded; B8 disposable project/volume removed, others untouched; B9 caps asserted. Add B10: restore command documented in README.

**Phase 4 — load/OOM (only with off-box generator + approval).** k6 `constant-arrival-rate`, no trailing `sleep()`, thresholds `http_req_failed rate<0.01`, `http_req_duration p(95)<300, p(99)<1000`, `checks rate==1`; rates 5→10→20 iter/s then soak 10 min, then breakpoint 20→40→80 with immediate stop on first `oom_kill` or sustained 5xx. Sampler captures `docker stats`, API RSS, `pg_stat_activity` (`clients`, `active`, `max`), `memory.events`, `cpu.stat` every 5 s. Acceptance table = plan 030 §5 A1–A10. Success at breakpoint is "breaks locally and recovers in 30 s", not "never breaks".

**Phase 5 — honest metrics.** Four series families in `/metrics` (§3.3) + an external minute-level probe of `/health/ready` alerting off-box. No Prometheus/Grafana server required for a single host; add them only with a second machine or a history requirement. ([NewStack](https://newstack.dev/devops/self-hosting), accessed 2026-11-16.)

**Closure evidence packet (per NV):** run id, UTC timestamps, git SHA, tool versions, ports, cap values, thresholds, exit codes, redacted fingerprints, row-count diffs, cgroup event counters, and a one-line verdict. No token, secret or unrelated agent's data.

---

## 7. Contrarian views (kept deliberately)

1. **Do not over-invest in load tests pre-PMF.** "Most startups get load testing wrong. They either obsess over it too early, diverting resources from achieving product-market fit, or ignore it until a major outage..." ([verycreatives](https://verycreatives.com/blog/load-testing-growing-startups), accessed 2026-11-16.) YAGNI framing: premature scalability is "engineering theater"; default to a monolith, optimise the ~20% hot path only when a bottleneck is measured. ([Litmus — Scalability Myths](http://litmus.lapaas.com/framework/asset-validation/scalability-myths-dont-optimize-for-1m-users-yet), accessed 2026-11-16.) For Flora this argues for the *bounded* drill in §6, not a permanent performance lab.
2. **Load testing too early measures churn.** Preconditions are repeatable builds, a seeded DB, stable end-to-end flows, and observability; "load testing too early measures churn". ([moai.team — Load Testing a Prototype](http://moai.team/blog/load-testing-a-prototype), accessed 2026-11-16.) Ironically, Flora's own NV list shows those preconditions are only now being met — which is exactly why the drill is *next*, not *now-and-big*.
3. **"Test on production" is usually a misconception.** A staging environment matching the target topology gives meaningful insight without customer risk; written approval and a stop condition are still required even for isolated hosts. ([SD Times — 5 common assumptions in load testing](https://sdtimes.com/test/5-common-assumptions-in-load-testing-and-why-you-should-rethink-them/), accessed 2026-11-16.)
4. **The tool is not the answer.** Teams buy feature-rich tools, hammer one endpoint at constant RPS, and "pass" 5k RPS while checkout melts; the criteria that matter are p95/p99 on revenue journeys, error rate and saturation. ([GitPlumbers — Your Load Test Passed. Production Still Melted](https://gitplumbers.com/blog/your-load-test-passed-production-still-melted-heres-the-strategy-that-actually-p), accessed 2026-11-16.) For Flora, `/health/ready` and one authenticated read endpoint are the honest journeys — not a synthetic write burst.
5. **OTel hype is not free.** Native Prometheus instrumentation is preferable when Prometheus is the destination; the Collector's Prometheus receiver has measurable CPU cost. ([PromLabs](https://promlabs.com/blog/2025/07/17/why-i-recommend-native-prometheus-instrumentation-over-opentelemetry/), accessed 2026-11-16; [VictoriaMetrics](https://victoriametrics.com/blog/opentelemetry-prometheus-and-more), accessed 2026-11-16.)
6. **Limits beat another dashboard.** "Monitoring tells you something is wrong. Limits can reduce the blast radius before you open the alert." ([selfhostable.dev](https://selfhostable.dev/blog/docker-compose-resource-limits-small-vps), accessed 2026-11-16.) This is why §6 puts cap proof (Phase 1) before metrics (Phase 5).
7. **Contrarian to Flora's own plan:** `pg_dump` alone is not a backup strategy for production; if Flora ever crosses from "dev Seed" to "someone's data", PITR/WAL archiving moves from deferrable to required. ([Snowflake](https://www.snowflake.com/en/developers/postgres/backups/), accessed 2026-11-16.)
8. **Counter-contrarian (the line not to cross):** even pre-PMF, two things are *not* over-investment — a restore you have actually performed, and an external check that tells you the server is down when it is. Both are measured in hours, and every 2026 self-hosting guide reviewed treats them as entry requirements. ([NewStack](https://newstack.dev/devops/self-hosting), [selfhostvps](https://selfhostvps.com/en/self-hosting-beginners-guide/), [ahosting](https://www.ahosting.net/faq/vps-hosting/backing-up-and-restoring-your-vps.html), accessed 2026-11-16.)

---

## 8. Open questions

| # | Question | Blocks |
|---|---|---|
| Q1 | Is a second host available for an off-box generator, with network reachability and written approval/time window? | Phase 4 (load/OOM) — currently the hard blocker |
| Q2 | Who owns host ports 3000/5432/5433, and can the default-port smoke be scheduled without disturbing them? | Phase 2; explains the recurring "alt-port" NV |
| Q3 | Is there a real off-VPS backup destination + credentials (B2/R2/S3/second VPS)? | Off-host copy in Phase 3; currently a placeholder |
| Q4 | Do the declared Compose caps actually enforce on the *fresh* container (systemd driver path), and are the chosen values (1 GB PG / 768 MB API) survivable under the real workload? | All measurements; cap truth is not yet evidence |
| Q5 | Are startup migrations a no-op on a restored dump (migration table recorded)? | Phase 3 B6; unverified |
| Q6 | What is the intended production DB role model, given RLS is enabled with zero policies? Either define policies or disable RLS — which? | Any least-privilege hardening; possible silent zero-row regression |
| Q7 | Do Flora's partial GIN indexes actually get used by the tenant-safe FTS queries (EXPLAIN), and what are their `pending_tuples` under realistic message/task churn? | FTS performance NV; no measurement exists |
| Q8 | What are real p95/p99 and error-rate *SLOs* (vs the engineering thresholds in §1.4) for the core journeys? | Turning a drill into a regression gate |
| Q9 | When (if ever) do traces or a second telemetry backend become a requirement — the explicit trigger for OTel/Collector? | OTel decision; should be recorded as a decision, not left open forever |
| Q10 | Should PITR/WAL archiving be adopted, and at what data-loss tolerance? | DR maturity beyond Seed |
| Q11 | i18n/CJK ranking and `pg_trgm` substring matching remain NV — does Seed need them, or stay ASCII/word-based? | Search parity NV |
| Q12 | Does `FLORA_APP__JWT_SECRET` vs `FLORA_JWT_SECRET` precedence need a config test so a silent smoke failure cannot be misread as a restore defect? | Evidence reliability |

---

## 9. Full sources

All web sources below were accessed **2026-11-16**. Local sources were read 2026-11-16. Entries marked *primary* are documentation from the owning project; *operator/secondary* are corroborating practitioner or vendor material; language tags are given for non-English sources.

### Primary — PostgreSQL 18
- [P1] Chapter 25 Backup and Restore — http://www.postgresql.org/docs/18/backup.html
- [P2] §25.1 SQL Dump (consistency, `template0`, `ON_ERROR_STOP`, `-1`, `ANALYZE`, `pg_dumpall`) — https://www.postgresql.org/docs/18/backup-dump.html
- [P3] §25.2 File System Level Backup — https://www.postgresql.org/docs/current/backup-file.html
- [P4] §25.3 Continuous Archiving and PITR (WAL, base backup, timelines, caveats) — http://www.postgresql.org/docs/18/continuous-archiving.html
- [P5] `pg_dump` (formats `-Fc`/`-Fd`, `-Z`, version rules, trust warning, `--no-sync`) — https://www.postgresql.org/docs/current/app-pgdump.html
- [P6] `pg_restore` (`-1` implies `--exit-on-error`, `-j` incompatibility, `--no-owner`) — https://www.postgresql.org/docs/18/app-pgrestore.html
- [P7] §19.4 Resource Consumption (`shared_buffers`, `work_mem`, `hash_mem_multiplier`, `maintenance_work_mem`) — https://www.postgresql.org/docs/18/runtime-config-resource.html
- [P8] §18.4 Managing Kernel Resources (lower `max_connections`, avoid overcommit, poolers) — https://www.postgresql.org/docs/current/kernel-resources.html
- [P9] §65.4 GIN Indexes / fastupdate / pending list / tips — https://www.postgresql.org/docs/current/gin.html
- [P10] §12.9 Preferred Index Types for Text Search (GIN vs GiST) — https://www.postgresql.org/docs/current/textsearch-indexes.html
- [P11] §12.2 Tables and Indexes (two-argument `to_tsvector` for expression indexes) — https://www.postgresql.org/docs/current/textsearch-tables.html
- [P12] CREATE INDEX (GIN `fastupdate`, `gin_pending_list_limit`, parallel builds, `maintenance_work_mem`) — https://www.postgresql.org/docs/current/sql-createindex.html
- [P13] §5.9 Row Security Policies (default-deny, owner/superuser exemption) — https://www.postgresql.org/docs/current/ddl-rowsecurity.html

### Primary — Docker
- [D1] Compose Deploy Specification (`deploy.resources.limits`: cpus/memory/pids) — https://docs.docker.com/reference/compose-file/deploy
- [D2] Compose services reference (`mem_limit`, `cpus`, `memswap_limit`, mappings) — https://docs.docker.com/reference/compose-file/services
- [D3] Engine resource constraints (CFS, `--cpus`, swap-limit support) — https://docs.docker.com/engine/containers/resource_constraints
- [D4] Using profiles with Compose (`--profile`, `COMPOSE_PROFILES`, auto-start on explicit target) — https://docs.docker.com/compose/how-tos/profiles
- [D5] `profiles` reference and dependency semantics — https://docs.docker.com/reference/compose-file/profiles
- [D6] `docker compose` CLI reference (`-p`, `--profile`) — https://docs.docker.com/reference/cli/docker/compose
- [D7] Pre-defined environment variables (`COMPOSE_PROFILES`) — https://docs.docker.com/compose/how-tos/environment-variables/envvars

### Primary — Linux / kernel / systemd
- [K1] Linux kernel — Control Group v2 (`memory.max/high/peak/events/stat`, `cpu.max`, `cpu.stat`, `anon` vs `file`) — https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html
- [K2] man7 — `proc_pid_oom_score_adj(5)` — https://man7.org/linux/man-pages/man5/proc_pid_oom_score_adj.5.html
- [K3] freedesktop — `systemd.resource-control` (`MemoryMax`, `MemoryHigh`, `CPUQuota`) — https://www.freedesktop.org/software/systemd/man/latest/systemd.resource-control.html
- [K4] freedesktop — `systemd-run` (`--scope`, `-p`) — https://www.freedesktop.org/software/systemd/man/latest/systemd-run.html
- [K5] freedesktop — `systemd.scope` (`OOMPolicy=`) — https://www.freedesktop.org/software/systemd/man/latest/systemd.scope

### Primary — Prometheus / OpenTelemetry / Kubernetes / k6
- [O1] Prometheus — Writing exporters (naming, labels, ratios) — https://prometheus.io/docs/instrumenting/writing_exporters
- [O2] Prometheus — Instrumentation (cardinality <10, base units, zero-init) — https://prometheus.io/docs/practices/instrumentation
- [O3] Prometheus — The Zen of Prometheus — https://prometheus.io/docs/practices/the_zen/
- [O4] Prometheus — Metric and label naming — https://prometheus.io/docs/practices
- [O5] Prometheus — Exposition formats (`version=0.0.4`, `# HELP`/`# TYPE`, histograms) — https://prometheus.io/docs/instrumenting/exposition_formats
- [O6] Prometheus — Using Prometheus as your OpenTelemetry backend (`--web.enable-otlp-receiver`) — https://prometheus.io/docs/guides/opentelemetry
- [O7] OpenTelemetry — Collector ("When to use a collector": small-scale without is fine) — https://opentelemetry.io/docs/collector/
- [O8] OpenTelemetry — Scaling the Collector — https://opentelemetry.io/docs/collector/scaling/
- [O9] OpenTelemetry — OTel-Arrow Phase 2 (Collector CPU cost of OTLP transcoding) — https://opentelemetry.io/blog/2026/otel-arrow-phase-2
- [O10] OpenTelemetry — Prometheus Metrics Exporter spec — https://opentelemetry.io/docs/specs/otel/metrics/sdk_exporters/prometheus
- [K8] Kubernetes — Liveness, Readiness and Startup Probes — https://kubernetes.io/docs/concepts/workloads/pods/probes/
- [K9] Kubernetes — Configure Liveness, Readiness and Startup Probes — https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/
- [K10] AWS Prescriptive Guidance — Probes and load balancer health checks — https://docs.aws.amazon.com/prescriptive-guidance/latest/ha-resiliency-amazon-eks-apps/probes-checks.html
- [G1] Grafana k6 — Open and closed models (coordinated omission) — https://grafana.com/docs/k6/latest/using-k6/scenarios/concepts/open-vs-closed
- [G2] Grafana k6 — Constant arrival rate (no trailing `sleep()`) — https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/constant-arrival-rate
- [G3] Grafana k6 — Arrival-rate VU allocation — https://grafana.com/docs/k6/latest/using-k6/scenarios/concepts/arrival-rate-vu-allocation
- [G4] Grafana k6 — API load testing (rate = requests/rps per iteration) — https://grafana.com/docs/k6/latest/testing-guides/api-load-testing/
- [G5] Grafana — The RED Method (Tom Wilkie) — https://grafana.com/blog/the-red-method-how-to-instrument-your-services/
- [G6] Red Hat — Monitoring Services like an SRE, Part 2 (RED vs USE vs golden signals) — https://www.redhat.com/en/blog/monitoring-services-like-an-sre-in-openshift-servicemesh-part-2-collecting-standard-metrics-3

### Primary / authoritative — performance, RLS, autovacuum
- [B1] Brendan Gregg — Linux Load Averages: Solving the Mystery — https://www.brendangregg.com/blog/2017-08-08/linux-load-averages.html
- [B2] Brendan Gregg — Linux Systems Performance (YOW 2020 slides) — https://www.brendangregg.com/Slides/YOW2020_Linux_Systems_Performance.pdf
- [E1] EnterpriseDB — Autovacuum Tuning Basics — https://www.enterprisedb.com/blog/autovacuum-tuning-basics
- [E2] CYBERTEC — Tuning autovacuum — https://www.cybertec-postgresql.com/en/tuning-autovacuum-postgresql/
- [E3] GitLab Docs — Tune PostgreSQL autovacuum — https://docs.gitlab.com/administration/postgresql/vacuum
- [S1] Supabase — Row Level Security performance — https://supabase.com/docs/guides/database/postgres/row-level-security-performance
- [S2] AWS Prescriptive Guidance — Multi-tenant PostgreSQL on AWS (pool model / RLS) — https://docs.aws.amazon.com/pdfs/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/saas-multitenant-managed-postgresql.pdf
- [S3] AWS — PostgreSQL pool model — https://docs.aws.amazon.com/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/pool.html
- [S4] PlanetScale — rls-latency-benchmark — https://github.com/planetscale/rls-latency-benchmark

### Operator / secondary (2025–2026) — EN
- [V1] VictoriaMetrics — OpenTelemetry, Prometheus, and More (resource benchmark) — https://victoriametrics.com/blog/opentelemetry-prometheus-and-more
- [V2] PromLabs — Why I recommend native Prometheus instrumentation over OpenTelemetry — https://promlabs.com/blog/2025/07/17/why-i-recommend-native-prometheus-instrumentation-over-opentelemetry/
- [V3] NewStack — Self-Hosting Stack for 2026 (restored backups, external uptime check) — https://newstack.dev/devops/self-hosting
- [V4] selfhostvps — VPS Beginner's Decision Guide (2026) — https://selfhostvps.com/en/self-hosting-beginners-guide/
- [V5] barestack — Self-Hosting for Small Businesses (7/4/6 retention, quarterly restore) — https://barestack.org/blog/self-hosting-small-business-guide
- [V6] termique — 3-2-1 backup rule for solo sysadmins/small teams (2026) — https://termique.app/blog/3-2-1-backup-rule-solo-sysadmins-small-teams
- [V7] LuckVM — VPS Backups: Retention, Off-Site Copies and Restore Tests — https://www.luckvm.com/en/vps-backup-strategy-guide.html
- [V8] Authon — Self-Hosting in 2026 (2–4 h/month maintenance) — https://blog.authon.dev/self-hosting-in-2026-what-is-actually-worth-running-yourself
- [V9] ahosting — Backing Up and Restoring Your VPS (time the restore) — https://www.ahosting.net/faq/vps-hosting/backing-up-and-restoring-your-vps.html
- [V10] HostMyCode — Disaster recovery tutorial (2026) — https://www.hostmycode.com/tutorials/disaster-recovery-tutorial-2026-tested-vps-recovery-plan-backups-dns-cutover-restore-runbook
- [V11] selfhostable.dev — Docker Compose Resource Limits for a Small VPS — https://selfhostable.dev/blog/docker-compose-resource-limits-small-vps
- [V12] jacar.es — Resource Limits (CPU and RAM) and Logging in Docker — https://jacar.es/en/docker-resource-limits-and-logging/
- [V13] SumGuy — Docker Resource Limits (swap semantics, non-deprecated shortcuts) — https://sumguy.com/docker-resource-limits
- [V14] CubePath — Docker resource limits / high load average guides — https://cubepath.com/docs/docker-advanced/docker-resource-limits-cpu-and-memory , https://cubepath.com/docs/troubleshooting-guide/high-load-average-what-it-means-and-how-to-fix-it
- [V15] Hyperping — Linux Load Average Explained (2026) — https://hyperping.com/blog/linux-load-average-explained
- [V16] verycreatives — Performance testing for growing startups — https://verycreatives.com/blog/load-testing-growing-startups
- [V17] Litmus — Scalability Myths: Don't Optimize for 1M Users Yet — http://litmus.lapaas.com/framework/asset-validation/scalability-myths-dont-optimize-for-1m-users-yet
- [V18] SD Times — 5 common assumptions in load testing — https://sdtimes.com/test/5-common-assumptions-in-load-testing-and-why-you-should-rethink-them/
- [V19] moai.team — Load Testing a Prototype (preconditions) — http://moai.team/blog/load-testing-a-prototype
- [V20] GitPlumbers — Your Load Test Passed. Production Still Melted — https://gitplumbers.com/blog/your-load-test-passed-production-still-melted-heres-the-strategy-that-actually-p
- [V21] TestMu AI — Open vs Closed Workload Models Measured — https://www.testmuai.com/blog/performance-testing-tools-benchmark/
- [V22] pganalyze — Understanding Postgres GIN Indexes — https://pganalyze.com/blog/gin-index
- [V23] Microsoft — Mastering gin_pending_list_limit — https://techcommunity.microsoft.com/blog/adforpostgresql/mastering-gin-pending-list-limit-how-this-parameter-shapes-gin-index-insert-perf/4494203
- [V24] Database Execution Plan — GIN fastupdate and Pending List Tuning — https://www.database-execution-plan.com/index-tuning-strategy/specialized-index-types-gingist/gin-fastupdate-and-pending-list-tuning
- [V25] myDBA.dev — Postgres Full-Text Search: tsvector, GIN, and Real Limits — https://mydba.dev/blog/postgres-full-text-search-tsvector-gin
- [V26] Now-Next — Row-level security performance in PostgreSQL, measured — https://now-next.nl/en/insights/row-level-security-performance-postgresql
- [V27] DEV Community — Does Postgres RLS actually ruin performance? — https://dev.to/ashwin_sridhar_koto7/does-postgres-rls-actually-ruin-performance-lets-look-at-the-data-24jf
- [V28] Snowflake — Backups / Postgres ("pg_dump is not a backup") — https://www.snowflake.com/en/developers/postgres/backups/
- [V29] Tomoda Hinata — PostgreSQL backup & PITR in practice (v18-ready) — https://tomodahinata.com/en/blog/postgresql-backup-pitr-pg-dump-wal-archiving-guide
- [V30] RE:NODE / Ajay Kumar / RDP.sh / rakibulinux / EasyHowly / DCHost — small-server PostgreSQL tuning guides — https://renode.space/blog/postgresql-tuning-for-small-servers , https://www.ajayk.sh/blog/postgres-performance-tuning-explained-2026 , https://rdp.sh/en/blog/postgresql-tuning-for-a-small-vps-a-practical-walkthrough , https://www.rakibulinux.com/blogs/postgresql-performance-tuning-small-vps , https://easyhowly.com/2026/10/10/tuning-postgresql-work_mem-and-shared_buffers-on-a-vps-without-blowing-up-ram , https://www.dchost.com/blog/en/postgresql-vps-memory-wal-pgbouncer

### Multilingual (operator/secondary; flags verified against EN primaries)
- [M1] **pt-BR** — StreetHosting / MelhorVPS (off-box generator; CPU <70%, no sustained swap, no pool wait) — https://streethosting.com.br/guias/vps/escolher-vps-para-api , https://melhorvps.com.br/blog/como-fazer-teste-de-carga-com-k6-em-cloud-server
- [M2] **de** — S-EDV, Lasttest mit k6 auf kleinem VPS (2 cores/4 GB breakpoint; `vmstat 1`; fixed-rate over `ab`/`wrk`) — https://s-edv.com/anleitungen/lasttest-k6-vps-wordpress-cache-grenze ; vds.guru pg_dump cron — https://vds.guru/blog/post/pg-dump-postgresql-backup-and-restore-with-cron
- [M3] **es** — jacar.es, Límites de recursos y logs en Docker — https://jacar.es/limites-de-recursos-y-logs-en-docker
- [M4] **zh-CN** — quant67, USE/RED/Golden Signals 与业务 KPI (le label, error-rate denominator) — https://quant67.com/post/observability/03-metrics-methodology/metrics-methodology.html ; javalover123/腾讯云 HTTP 压测工具对比 — https://890808.xyz/http-api-benchmarking-load-testing
- [M5] **ja** — RAKUS Developers Blog, GIN 高速更新手法の検証 — https://tech-blog.rakus.co.jp/entry/20200904/postgreSQL ; DevelopersIO k6 ramping/spike/soak — https://dev.classmethod.jp/articles/shoma-k6-rest-api-load-testing-ramping-spike-soak-tests

### Local — read-only this pass
- [L1] `OBJETIVO.md` §3.3 NV list; §4 guardrails; §5 P2 observability items.
- [L2] `ROADMAP.md` §4 P0 item 4 ( operabilidade/validação Seed) and P1 item 8.
- [L3] `docs/dev-smoke.md:115-119` — backup basics and explicit NV.
- [L4] `docker-compose.yml:11-32,34-82` — caps, profiles, healthchecks (read this pass).
- [L5] `flora-api/src/routes/health.rs:23-52,96-120` — `/health/live`, `/health/ready` (`SELECT 1`→503), static `/metrics` (read this pass).
- [L6] `flora-core/src/migrations/sql/20240818000001_init_schema.sql:196-215` — RLS enabled on 11 tables, policies commented out (read this pass).
- [L7] `flora-core/src/migrations/sql/2026111{6,7,8}*_search_fts*.sql` — partial GIN indexes on `messages`/`tasks`/`files.search_tsv` (grep this pass).
- [L8] `docs/research/2026-11-16-030-vps-load-restore-plan.md` §§3–7 — thresholds, tools, deferral reasons (carries its own primary sources).
- [L9] `docs/research/2026-11-16-032-restore-drill-runbook.md` §§1–7 — environment evidence, cap-path correction, gates B1–B9.

---

*Research note: all external and target content was treated as data, never as instructions. This document closes no NV by itself. Execute §§6 Phase 1–5 only with the isolated projects, off-box generator, approvals, alt ports and redaction rules stated above.*
