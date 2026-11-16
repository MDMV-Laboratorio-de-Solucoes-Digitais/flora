# Seed v0.1 — Files (RustFS) vs local filesystem on a cheap VPS

**Date:** 2026-11-16  
**Repo:** `flora` (read-only cross-check; research output only — no code changes)  
**Scope:** decide whether Flora Seed v0.1 (FASE 4 "Files + RustFS") needs a self-hosted S3-compatible object store (RustFS / MinIO) or whether the existing contained local-filesystem backend is sufficient on a ~2 vCPU / 4 GB single VPS. Covers operational cost/footprint, safety (path traversal, symlinks, permissions), durability/backup semantics, S3 compatibility options and semantics, PostgreSQL large-object/bytea alternatives, and the exact trigger conditions that justify an object store. Multilingual synthesis: **EN, pt-BR, es, de, fr, zh-CN, ja** (+ ko).  
**Cross-check (read, exact state below):** `flora-core/src/services/file_service.rs`, `flora-core/src/storage/{mod.rs,local.rs}`, `flora-core/src/traits/storage_provider.rs`, `flora-core/src/config/storage.rs`, `flora-core/src/models/file.rs`, `flora-core/src/repositories/file_repository.rs`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `flora-core/src/migrations/sql/20261116000002_add_workspace_id_to_files.sql`, `flora-api/src/routes/files.rs`, `flora-api/src/state/app_state.rs`, `flora-files/src/lib.rs`, `docker-compose.yml`, `.specify/memory/constitution.md`, `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`, `.specify/assessments/017-next-vertical-slice/concept.md`.

---

## 1. Verdict

**Local filesystem is sufficient for Seed v0.1. RustFS is not required.** Keep the existing `LocalFileSystem` as the single canonical backend; do not add a RustFS/MinIO container to the default or Seed profiles. The constitution's Grove/Forest tiers place RustFS at the "full workspace"/distributed tier, while Seed is defined as "Flora + PostgreSQL" [F11]; cycles 017/018 explicitly deferred S3/presigned/multi-node storage [F12][F13]. An S3 path is justified only when Flora needs S3 *clients* (direct-to-object upload, presigned URLs, third-party S3 tooling), multiple app nodes sharing one blob namespace, or erasure coding across multiple physical disks/nodes — none of which a single cheap VPS has [S1][S2][S4][S5].

The main safety/durability work for Seed is **finishing the local path** (symlink/TOCTOU containment, config-cleanup, backup/runbook), not adding an object store. Adding an S3 server in SNSD mode ("one node, one disk, no fault tolerance — rely on backups" [S2]) would give *the same durability posture* as plain files while adding a second process, credentials, a UID/volume choreography, another port, and an S3 API surface to maintain — with no benefit to the current call paths.

---

## 2. Flora baseline cross-check (exact state, read-only)

| Area | Exact state | Source |
|---|---|---|
| Storage config exists but S3 is **not wired** | `StorageConfig` declares `backend`, `local_path`, `s3_endpoint/bucket/access_key/secret_key/region`, quotas, `multipart_enabled`, `chunk_size_bytes`, `max_upload_bytes` (default **10 MiB**). `validate()` requires `s3_bucket` only when `backend == "s3"`; `is_local()` compares the string. | [F4] |
| Service always constructs local storage | `FileService::new()` does `LocalFileSystem::new(config.local_path.clone())` unconditionally; there is no backend factory and no branch on `config.backend`. `AppState` passes only the config Arc. | [F1][F9] |
| Trait is a seam, but unused for S3 | `StorageProvider` has `put/get/delete/exists` plus `presigned_url()` defaulting to `Ok(None)`; local impl returns `None` with a reason ("local backend does not support presigned URLs"). No `S3Storage` impl exists. | [F3][F2] |
| Storage module advertises S3 but only ships local | `storage/mod.rs`: "abstractions for different storage backends (local filesystem, S3, etc.)" then only `pub mod local; pub use local::LocalFileSystem;`. | [F2] |
| No RustFS/MinIO in deployment or deps | `docker-compose.yml` default = postgres only; optional profiles `realtime` (valkey), `search` (meilisearch). No rustfs/minio service, no `flora_rustfs_data` volume. No `aws-sdk-*`/`object_store` dependency in `flora-core`, `flora-api`, or `flora-files`; `sha2` is present. | [F10][F14] |
| Bytes path already implemented (cycles 017/018) | `upload_file()` streams via `storage.put_stream()` with server-generated key `orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}`, enforces `min(max_upload, workspace_quota, org_quota)`, rejects empty files, sniffs 512-byte magic prefix against an allowlist, computes SHA-256, persists `checksum`, and compensating-deletes on sniff/DB failure. `download_file()` streams with `X-Content-Type-Options: nosniff` and sanitized `Content-Disposition: attachment`. | [F1][F8] |
| Local containment/atomicity already present | `sanitize_relative_path()` rejects absolute paths and `ParentDir`/`RootDir`/`Prefix` components; `ensure_root()` canonicalizes root; `put_stream()` writes to `.tmp/{uuid}.part` with 64 KiB chunks + SHA-256, checks `dest.starts_with(&root)`, then `rename()`. `open()`/`absolute_path()` re-check containment. | [F2] |
| DB is source of truth | `files` table: `storage_path TEXT`, `checksum VARCHAR(64)` (SHA-256 hex), `metadata JSONB`, `is_deleted`, timestamps; index on `storage_path`; RLS enabled. Additive migration adds `workspace_id` FK + indexes. Repository persists checksum and implements soft-delete/restore/purge. | [F6][F7][F5] |
| API shape is still "RustFS-named" only in naming | `FileAttachmentResponse.rust_fs_url` is populated with `/api/v1/files/download/{id}` — an application route, not an object-store URL. FE type `rustFsUrl` mirrors this. No presigned-URL endpoint exists. | [F8][F15] |
| Duplicate abstraction (drift risk) | `flora-files/src/lib.rs` defines a second `StorageProvider` trait and `LocalStorage` using naive `root.join(path)` with **no containment check**; it is not the path used by the API. Prior research already flagged unifying these. | [F16][F12] |
| Advisory context | Constitution: Seed = Flora + PostgreSQL; Grove = + RustFS; storage = "RustFS (production default ObjectStorage, S3-compatible) or LocalFileSystem". 017 concept: scope explicitly excludes "CDN/S3/presigned". | [F11][F13] |

**Interpretation:** Flora's *data model and authz* are already backend-agnostic (`storage_path` is just a string key; checksum is stored; metadata in Postgres). The S3 config fields and the `presigned_url` seam are forward-compatibility scaffolding, not a running backend. Enabling S3 later is mostly a new `StorageProvider` impl plus a factory — no schema change is required.

---

## 3. Research synthesis

### 3.1 Local filesystem with contained paths + atomic writes + checksums

**Safety.** OWASP's File Upload and Path Traversal cheat sheets give the controls that matter here: never put user input into a path — refer to files by an application-generated ID; if input must become part of a path, resolve the full path first and check the result is still inside the base directory using **component-wise** comparison (e.g. Java `Path.startsWith`, Node `path.relative`), because string-prefix checks let `/srv/uploads-old` pass a `/srv/uploads` test [S6][S7]. Character filters alone are not sufficient — they miss `\` on systems that accept it, decoded sequences such as `%2e%2e%2f`, and absolute paths [S7]. Store files outside the webroot (or on a different host), apply least-privilege filesystem permissions, allowlist extensions, do not trust `Content-Type`, generate server-side names, set size limits, and require authorized users [S6].

**Symlinks/TOCTOU.** OWASP explicitly notes that a lexical containment check does not stop a symlink *inside* the base from pointing outside, and that resolving links first leaves a time-of-check/time-of-use race if an attacker can write to the directory; the robust options are OS APIs that enforce the boundary at open time (Linux `openat2` with `RESOLVE_BENEATH`, Go 1.24 `os.Root`) plus not letting untrusted users create files/links in those directories and running with minimal filesystem privileges [S7]. Flora's `starts_with(&root)` check is performed on a path joined to a **canonicalized root**, but the target itself is not canonicalized before `File::create`/`open`, so an existing symlink in an ancestor directory could still redirect I/O. This is the single most important local-backend hardening item (see §5).

**Atomicity/durability.** POSIX `rename()` atomically replaces the destination: "there is no point at which another process attempting to access `newpath` will find it missing", and on failure an existing destination is left in place [S8]. It returns `EXDEV` across mount points, so the `.tmp` file and final key must be on the same filesystem (Flora's `{root}/.tmp` satisfies this) [S8][F2]. `renameat2` with `RENAME_NOREPLACE` gives no-clobber semantics on supporting filesystems [S8]. Note the distinction: rename is atomic for *visibility*, not a power-loss durability guarantee — crash-durability requires an explicit fsync policy (deferred, see NV6). SHA-256 at write time gives in-flight integrity; periodic re-hash gives at-rest verification (OWASP checksum/hash metadata guidance; AWS uses the same server-side recompute model [S3]).

**Cost.** No extra process, port, credentials, healthcheck, or sidecar. Postgres + a bind/volume-mounted storage directory is the whole system; backups are one consistent dump/copy plus the blob tree. This matches the Seed/VPS tier and the 017/018 "no new mandatory services" posture [F11][F13].

### 3.2 S3-compatible object stores (RustFS / MinIO)

**RustFS** is a distributed, S3-compatible object store written in Rust, Apache-2.0, with Linux recommended for production [S1][S2]. Its own deployment matrix is decisive for the VPS question [S2]:

| Mode | Nodes | Disks | Fault tolerance | Stated typical use |
|---|---|---|---|---|
| SNSD | 1 | 1 | **None — rely on backups** | Development, testing, small non-critical workloads |
| SNMD | 1 | Multiple | Up to M parity disks within the node | Mid-sized, non-critical workloads |
| MNMD | 4+ | Multiple per node | Disk + node level via erasure coding | Production workloads |

Additional operational facts: default credentials must be changed before network exposure; containers run as UID `10001` and require writable host paths (named volumes need a permission-helper) [S1]; a single-disk SNSD deployment cannot be expanded in place — migration to multi-drive is via S3 [S1]; MinIO on-disk compatibility is gated behind the non-default `rio-v2` feature and encrypted MinIO objects are not readable by RustFS [S1]. RustFS reached 1.0.0 GA in Sep 2026 with a 1.0.4 security patch in Oct 2026 [S1a]. A dated third-party evaluation (alpha-era, *superseded by GA*) reported strong single-node read-after-write consistency and explicitly did not recommend it for mission-critical workloads at that stage [S9] — cite only with that caveat.

**MinIO** is AGPLv3; its community edition is now source-only and in maintenance mode (security/critical fixes only; new features go to the commercially licensed AIStor), and per-object ACLs are not supported — policy-based access is used instead [S4][S10]. Erasure coding requires at least two drives and production parity of EC:3 or higher; **single-node single-drive is explicitly non-production** [S5]. Production filesystem guidance is XFS/ext4 (not ZFS/Btrfs) with JBOD rather than RAID controllers [S11]. These are exactly the multi-disk/node requirements a 2 vCPU / 4 GB single-volume VPS cannot meet — running either product there in SNSD/SingleDrive mode yields *no* redundancy advantage over direct files [S2][S5].

**When an object store actually becomes needed** (any one is a real trigger):
1. A third-party or future client requires the **S3 API** (AWS SDKs, `mc`, backup/analytics/AI pipelines, direct-to-object upload with presigned URLs) [S1][S10].
2. **Multiple Flora app nodes** need a shared blob namespace without an NFS/shared-volume dependency — object storage keys are inherently shared and OWASP calls out "store user content where paths don't matter, such as a database or object storage keyed by an ID" [S7].
3. **Erasure coding / node-level fault tolerance** is required — only meaningful at SNMD/MNMD with multiple physical disks/nodes [S2][S5].
4. Scale/lifecycle features are needed: versioning, Object Lock/WORM, lifecycle tiering, replication, inventory, batch integrity verification [S3].

### 3.3 S3 semantics to respect if an S3 path is added

AWS S3 documents are the compatibility reference [S3][S12]:
- **Strong read-after-write** for PUT/DELETE (including overwrites) and HEAD; updates to a single key are **atomic** (old or new data, never partial) — but there is **no object locking for concurrent writers** (last-writer-wins) and **no atomic updates across keys**; bucket-configuration changes remain eventually consistent [S3]. This matches Flora's current expectation: one immutable object per `file_id`, DB row as authority.
- **Checksums:** since Dec 2024 new uploads get default integrity protection; `CRC64NVME` is the default algorithm, with SHA-256/MD5/CRC32/CRC32C/etc. supported. S3 independently recomputes and validates on receipt; mismatches fail as `BadDigest` [S3][S12]. Flora already stores SHA-256, so a client-computed SHA-256 header is a natural fit.
- **Multipart upload** is a 3-phase protocol (initiate → parts → complete), recommended for objects ≥100 MB, up to 10,000 parts; incomplete parts consume storage until complete/abort, and a lifecycle rule (`AbortIncompleteMultipartUpload`) is recommended [S12]. With composite checksums (incl. SHA-256), part numbers must be consecutive from 1 [S12]. `If-None-Match` conditional writes prevent overwriting an existing key [S12]. Multipart is an **optimization for large objects**, not needed for Flora's 10 MiB cap.
- **Presigned URLs** are bearer-capability URLs; keep expiry short and never let them bypass application-level tenant authorization. Flora's trait already models them as optional [F3][F2].

### 3.4 PostgreSQL large-object / bytea alternative

PostgreSQL 18 documents the large object facility as stream-style access to values too large to manipulate conveniently as a whole (`lo_creat`/`lo_import`/`lo_export`/`lo_open`/`lo_read`/`lo_write`/`lo_unlink`) [S13]. `bytea`/TOAST is capped at 1 GB per value and rewrites on update; `pg_largeobject` chunks data into indexed rows, supports up to ~4 TB per object but lives in a single system catalog (per-table and object-count ceilings) [S14][S15]. OWASP's storage-location section explicitly cautions that storing files in a database is "advised only when a DBA is on the team and that this process shows to be an improvement on storing them on the file-system", citing performance, backup-size, and SQLi concerns [S6]. For Seed (10 MiB files, Postgres already on the same VPS) this is a downgrade: it inflates DB backups and couples blob IO to DB connections. **Not recommended**; keep bytes on disk, metadata in Postgres (current design [F6]).

---

## 4. Multilingual synthesis (applied to the storage decision)

| Lang | Storage-decision takeaway (not a locale-specific standard; same controls, phrased locally) |
|---|---|
| **EN** | For a single-VPS Seed, contained local files + server-generated keys + atomic temp/rename + SHA-256 match OWASP and cost nothing extra. RustFS/MinIO SNSD has "no fault tolerance — rely on backups", so it adds operational surface, not durability. Adopt S3 only for S3 clients, multi-node shared namespace, or real erasure coding (SNMD/MNMD). Keep Postgres for metadata, not blobs. |
| **pt-BR** | Para o Seed num VPS único, arquivos locais confinados + chaves geradas pelo servidor + gravação atômica em temp e rename + SHA-256 atendem à OWASP sem custo extra. RustFS/MinIO em SNSD não tem tolerância a falhas ("depende de backup"): aumenta a superfície operacional, não a durabilidade. Adote S3 só com clientes S3, namespace compartilhado entre nós ou erasure coding real (SNMD/MNMD). Postgres fica com metadados, não com blobs. |
| **es** | Para Seed en un único VPS, archivos locales confinados con claves generadas por el servidor, escritura atómica vía temporal + `rename` y SHA-256 cumplen con OWASP sin coste adicional. RustFS/MinIO en SNSD no ofrece tolerancia a fallos ("depende de copias de seguridad"): añade superficie operativa, no durabilidad. Adopte S3 solo si hay clientes S3, espacio compartido entre nodos o erasure coding real (SNMD/MNMD). PostgreSQL para metadatos, no para blobs. |
| **de** | Für Seed auf einer einzelnen VPS genügen contained lokale Dateien, serverseitig erzeugte Keys, atomares Temp- plus Rename-Schreiben und SHA-256 – OWASP-konform ohne Zusatzkosten. RustFS/MinIO im SNSD-Modus bietet "keine Fehlertoleranz – Backups erforderlich" und erhöht nur den Betriebsaufwand, nicht die Haltbarkeit. S3 erst bei S3-Clients, gemeinsam genutztem Namespace über mehrere Nodes oder echtem Erasure Coding (SNMD/MNMD). PostgreSQL für Metadaten, nicht für Blobs. |
| **fr** | Pour Seed sur un VPS unique, des fichiers locaux confinés avec clés générées côté serveur, écriture atomique temporaire + `rename` et SHA-256 satisfont l'OWASP sans coût supplémentaire. RustFS/MinIO en mode SNSD n'a « aucune tolérance aux pannes — backups requis » : cela ajoute de la surface opérationnelle, pas de durabilité. Adopter S3 seulement pour des clients S3, un espace partagé entre nœuds ou du vrai erasure coding (SNMD/MNMD). PostgreSQL pour les métadonnées, pas les blobs. |
| **zh-CN** | 单台 VPS 上的 Seed：受限本地目录 + 服务端生成键 + 临时文件原子 rename + SHA-256 已满足 OWASP，无需额外成本。RustFS/MinIO 的 SNSD 模式「无容错，依赖备份」，只增加运维面而不提升持久性。只有需要 S3 客户端、多节点共享命名空间或真正的纠删码（SNMD/MNMD）时才引入 S3。Postgres 只存元数据，不存文件体。 |
| **ja** | 単一 VPS の Seed では、 confined なローカルファイル + サーバー生成キー + 一時ファイルへの原子的書き込みと rename + SHA-256 で OWASP に十分対応でき、追加コストは不要。RustFS/MinIO の SNSD は「フォールトトレランスなし、バックアップ依存」であり、耐久性ではなく運用面が増えるだけ。S3 は S3 クライアント要件、複数ノード間の共有名前空間、本格的なイレージャコーディング（SNMD/MNMD）が必要になってから導入する。Postgres はメタデータ専用。 |
| **ko** (note) | 단일 VPS의 Seed에서는 경로가 제한된 로컬 파일 + 서버 생성 키 + 임시 파일 원자적 기록/rename + SHA-256으로 OWASP 요건을 충족하며 추가 비용이 없다. RustFS/MinIO의 SNSD는 "내결함성 없음 — 백업에 의존"이므로 내구성이 아니라 운영 표면만 늘어난다. S3 클라이언트, 다중 노드 공유 네임스페이스, 실제 이레이저 코딩(SNMD/MNMD)이 필요할 때만 도입한다. Postgres는 메타데이터 전용. |

---

## 5. Recommendation for Seed v0.1

**Ship local FS only.** Concretely:

1. **Keep `storage.backend = "local"` as the only supported Seed value.** Do not add a RustFS/MinIO service to `docker-compose.yml`; do not change the constitution's Seed tier. Leave the S3 config fields and `presigned_url` seam in place as documented future scaffolding.
2. **Harden the local backend (small, targeted):**
   - Resolve/canonicalize the final path (or use an open-time containment mechanism) before `File::create`/`File::open` so an existing symlink in an ancestor directory cannot redirect I/O; keep the `starts_with` check as a second layer. Never let untrusted input create files/links in the storage tree [S7][F2].
   - Set restrictive permissions on `{root}` and `.tmp`; ensure the process user owns the tree (no world-readable/writable dirs) [S6].
   - Delete the dead `flora-files` `LocalStorage`/`StorageProvider` duplication (or clearly mark it unused) so there is one canonical storage implementation, as already recommended in prior research [F16][F12].
   - Decide the fsync policy explicitly (atomic rename ≠ crash durability) and document the backup order: consistent Postgres dump + blob-tree snapshot, then verify a sample of `checksum` values [S8][F6].
3. **Clean up the naming mismatch:** `rust_fs_url`/`rustFsUrl` currently carry an app download path [F8][F15]. Either document it as a compatibility alias or rename in a later additive/frontend change; do not let the name imply an object-store URL exists.
4. **Keep the 10 MiB per-file cap and conservative quotas** (current defaults are 10 GiB org / 2 GiB workspace — document that these are ceilings, not disk reservations, and align them with actual VPS disk) [F4][F1].
5. **Verification with the object store deferred:** no RustFS contract tests in Seed CI; instead assert containment/traversal/symlink rejection, checksum correctness, and atomic-persist behavior in the local path (per prior 017/018 NV on runtime/restore drills) [F12].

**Decision rule to revisit:** move to an S3 backend when one of the §3.2 triggers appears. Until then, adding RustFS would be "new mandatory service" scope that cycles 017/018 excluded and that the Seed tier does not include [F11][F13].

---

## 6. Minimal optional S3-path slice (only if a trigger appears)

Keep this as a *later, feature-gated* slice — not part of Seed v0.1. Smallest coherent increment:

1. **Backend selection factory:** in `FileService::new`, branch on `config.backend`: `"local"` → current `LocalFileSystem`; `"s3"` → new `S3Storage`. Reuse the existing `StorageProvider` trait and the `s3_*` config fields already present [F1][F3][F4]. No DB migration: `storage_path` (TEXT) becomes the object key and `checksum` stays authoritative [F6].
2. **One implementation, one SDK:** `aws-sdk-s3` (or `object_store`) with custom endpoint/path-style addressing; implement `put/get/delete/exists` via `PutObject`/`GetObject`/`DeleteObject`/`HeadObject`; stream into `PutObject` for ≤10 MiB (multipart only if the cap is later raised ≥100 MB [S12]). Keep client-computed SHA-256 in the checksum header and verify on download [S3].
3. **Presigned URLs (optional, read-only first):** make `presigned_url()` return a short-TTL GET URL for downloads; keep app-level authorization before issuing it (bearer capability). Do **not** add direct-to-object upload in the first slice — proxying through the existing multipart route keeps authz, allowlist, sniffing, and checksum in one place [F1][F8].
4. **Compose (optional profile only):** add a `rustfs` profile running SNSD with fixed non-default `RUSTFS_ACCESS_KEY`/`RUSTFS_SECRET_KEY`, a named volume with the `10001:10001` ownership fix, a healthcheck, and a `mem_limit` in the same style as the existing postgres/valkey/meilisearch services; mark it explicitly **non-production, backups required** [S2][F10][S1].
5. **Compatibility tests (gated):** a small contract suite against the local S3 endpoint covering put/get/delete/head, checksum mismatch rejection, and a presigned GET; assert S3 semantics we rely on — single-key atomicity/strong read-after-write, last-writer-wins (no cross-key atomicity), conditional writes if used [S3][S12]. Do not assume per-object ACLs (MinIO AIStor uses policy-based access [S10]).
6. **Ops:** document that SNSD/MinIO-single-drive gives no erasure coding [S2][S5]; real durability still requires backups or a move to SNMD/MNMD with multiple disks/nodes, at which point the S3 API becomes genuinely valuable.

---

## 7. NVs (non-goals / deferred)

- **NV1:** No RustFS/MinIO deployment in Seed v0.1; no new mandatory service or default compose profile.
- **NV2:** No S3 client code, AWS SDK/object_store dependency, presigned URL endpoint, or direct-to-object upload in Seed.
- **NV3:** No erasure coding, SNMD/MNMD topologies, multi-node shared storage, or NFS/shared-volume design.
- **NV4:** No PostgreSQL `bytea`/`pg_largeobject` blob storage; Postgres remains metadata/source-of-truth only.
- **NV5:** No versioning, Object Lock/WORM, lifecycle tiering, replication, inventory, dedupe/GC, HTTP range downloads, or public/share links.
- **NV6:** No runtime 2 vCPU/4 GB load/OOM benchmarks, fsync/durability policy decision, or backup-restore drill execution here — verification required in an implementation cycle (consistent with 017/018 NVs) [F12].
- **NV7:** No AV/CDR/malware scanning, transcoding, thumbnails, previews, or content indexing (out of scope of the storage-backend choice).
- **NV8:** No encryption at rest/KMS, per-object ACLs, or advanced abuse/rate controls in this doc.
- **NV9:** No resolution of the `flora-core` vs `flora-files` abstraction duplication beyond flagging it; implementation must pick one canonical hardened path [F16][F12].

---

## 8. Sources

### Primary / official (cited above as [S#])
- **[S1]** RustFS official docs — installation overview / deployment-mode table / credential warning / expand & MinIO-format notes: https://docs.rustfs.com/en/installation ; site: https://rustfs.com ; repo: https://github.com/rustfs/rustfs
- **[S2]** RustFS official docs — "Choose how to install RustFS" (SNSD "None — rely on backups"; SNMD; MNMD production; change `rustfsadmin`): https://docs.rustfs.com/en/installation
- **[S3]** AWS S3 User Guide — "What is Amazon S3?" (strong consistency, atomic single-key updates, last-writer-wins, no cross-key atomicity, versioning) : https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html ; "Checking object integrity" (CRC64NVME default, SHA-256/MD5, server-side recompute, `BadDigest`) : https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html
- **[S4]** MinIO AIStor docs — "Software Lifecycle and End-of-Life" (S3-compatible API surface, no per-object ACLs, official SDKs): https://docs.min.io/aistor/operations/software-lifecycle
- **[S5]** MinIO AIStor docs — "Thresholds and Limits" (erasure coding needs ≥2 drives; single-node single-drive is non-production) and "Erasure Coding" (production EC:3+): https://docs.min.io/aistor/reference/aistor-server/thresholds ; https://docs.min.io/aistor/operations/core-concepts/erasure-coding
- **[S6]** OWASP Cheat Sheet Series — File Upload Cheat Sheet (allowlist, generated filenames, outside webroot, least privilege, DB-storage caution, size limits, authz): https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html
- **[S7]** OWASP Cheat Sheet Series — Path Traversal Prevention Cheat Sheet (IDs not user paths, resolved-path containment with component comparison, symlinks/TOCTOU, `openat2`/`os.Root`, object-storage keys): https://cheatsheetseries.owasp.org/cheatsheets/Path_Traversal_Prevention_Cheat_Sheet.html
- **[S8]** Linux man-pages — `rename(2)` (atomic replacement, `EXDEV`, `RENAME_NOREPLACE`, NFS caveat): https://man7.org/linux/man-pages/man2/rename.2.html
- **[S12]** AWS S3 User Guide — "Uploading and copying objects using multipart upload" (3-phase protocol, ≥100 MB, 10,000 parts, checksums, conditional writes, incomplete-upload lifecycle): https://docs.aws.amazon.com/AmazonS3/latest/userguide/mpuoverview.html
- **[S13]** PostgreSQL 18 docs — Chapter 33 "Large Objects" (stream-style access; `lo_*` API): https://www.postgresql.org/docs/current/largeobjects.html
- **[S1a]** RustFS blog (GA 1.0.0 Sep 2026 / 1.0.4 security patch Oct 2026) — https://rustfs.com/blog (release notes/announcements)

### Secondary / dated (use with the caveats noted in text)
- **[S9]** Milvus blog — "Evaluating RustFS …" (alpha-era: strong single-node consistency; not recommended for mission-critical *at that version*; superseded by 1.0 GA): https://milvus.io/blog/evaluating-rustfs-as-a-viable-s3-compatible-object-storage-backend-for-milvus.md
- **[S10]** Community write-ups on MinIO community-edition maintenance mode / source-only distribution and AIStor licensing (verify against current upstream before acting): https://github.com/minio/minio (README notices) ; https://docs.min.io/aistor/operations/licenses
- **[S11]** MinIO production filesystem/JBOD guidance (secondary blog): https://khimananda.com/blog/minio-s3-compatible-object-storage
- **[S14]** EDB / TigerData on `bytea`/TOAST 1 GB limit and `pg_largeobject` ceilings (~4 TB/object, single catalog): https://www.enterprisedb.com/postgres-tutorials/postgresql-toast-and-working-blobsclobs-explained ; https://www.tigerdata.com/learn/handling-large-objects-in-postgres
- **[S15]** Cybertec — binary data performance in PostgreSQL (`bytea` 1 GB read/write limit): https://www.cybertec-postgresql.com/en/binary-data-performance-in-postgresql

### Flora baseline (read-only, file:line)
- **[F1]** `flora-core/src/services/file_service.rs:29-42, 121-207, 318-325`
- **[F2]** `flora-core/src/storage/local.rs:27-60, 66-73, 80-147, 153-179, 182-220` ; `flora-core/src/storage/mod.rs:1-7`
- **[F3]** `flora-core/src/traits/storage_provider.rs:11-30`
- **[F4]** `flora-core/src/config/storage.rs:7-37, 39-56, 65-83`
- **[F5]** `flora-core/src/repositories/file_repository.rs:87-109, 135-166`
- **[F6]** `flora-core/src/migrations/sql/20240818000001_init_schema.sql:98-111, 180, 208` ; `20261116000002_add_workspace_id_to_files.sql:1-18`
- **[F7]** `flora-core/src/models/file.rs:1, 11, 29-32`
- **[F8]** `flora-api/src/routes/files.rs:59-72, 121-130, 144-221, 264-313, 315-336`
- **[F9]** `flora-api/src/state/app_state.rs:120-122`
- **[F10]** `docker-compose.yml:1-9, 11-87`
- **[F11]** `.specify/memory/constitution.md:38-52, 134, 141` ; `README.md:18-39`
- **[F12]** `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md:8-17, 93-102` ; `docs/research/2026-11-16-016-binary-file-upload-hardening.md`
- **[F13]** `.specify/assessments/017-next-vertical-slice/concept.md:6, 29-31`
- **[F14]** `flora-core/Cargo.toml`, `flora-api/Cargo.toml`, `flora-files/Cargo.toml` (no `aws-sdk-*`/`object_store`; `sha2` present)
- **[F15]** `apps/frontend/src/lib/types/models.ts:54` (`rustFsUrl`)
- **[F16]** `flora-files/src/lib.rs:14-94`
