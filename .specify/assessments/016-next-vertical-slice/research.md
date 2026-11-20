# Research: 016 — B (binary files) vs C (Seed/VPS operability)

**Date:** 2026-11-16  
**Method:** Repo/code inspection + internal research cross-check + multilingual web research (EN, pt-BR, es, de, fr, zh-CN, ja; **ko, ru partially accessible via terminology/triangulation, NV for locale-primary depth**). Authoritative sources prioritized (Axum/tower/OWASP, Docker, PostgreSQL, Meilisearch, Valkey). No runtime benchmarks; no app source modified.

## 1) Multilingual evidence synthesis

### 1.1 (B) Safe binary upload/download on cheap VPS

| Lang | Query intent / terms | Key guidance (reliable consensus) |
|---|---|---|
| **EN** | “Axum multipart streaming upload to disk size limit DefaultBodyLimit RequestBodyLimitLayer”, “OWASP file upload path traversal MIME magic bytes checksum quota” | Axum `Multipart` consumes body and must be last extractor; `DefaultBodyLimit` (default 2 MB) is extractor-local, while `tower_http::limit::RequestBodyLimitLayer` is a global middleware cap (pre-reads by `Content-Length`, enforces on read for chunked). For large uploads on low-RAM hosts: disable/raise `DefaultBodyLimit` only with a paired hard limit, stream `field.chunk()` to temp file with byte-count cap + SHA-256, never buffer whole file (`field.bytes()` risky). OWASP: allowlist types/extensions, validate magic bytes (not `Content-Type`/extension alone), server-generated storage names/paths, canonicalized root containment (path traversal), authz on every download, safe headers (`Content-Disposition`, `X-Content-Type-Options: nosniff`), cleanup temp/partials, AV/CDR optional later. |
| **pt-BR** | “upload/download binário seguro Axum streaming limite tamanho MIME disco local VPS PostgreSQL metadados” | Upload/download seguro exige validação em camadas: limite no proxy/HTTP e por arquivo, allowlist de tipo/extensão, nome/caminho gerado no servidor, gravação em streaming para disco local, verificação por assinatura/magic bytes, metadados/integridade no Postgres, autorização em cada download e limpeza de temporários; não confiar em metadados do cliente. |
| **es** | “subida/descarga segura de archivos binarios Rust Axum prevención path traversal cuotas almacenamiento local VPS” | Subida/descarga segura: límites estrictos, lista blanca, nombres generados por el servidor, almacenamiento fuera de rutas ejecutables, validación por firma/magic bytes, control de acceso por descarga y prevención de *path traversal* y symlink escape; cuotas con contabilidad real de bytes usados. |
| **de** | “sicherer Datei-Upload/Download Rust Axum Größenlimit Dateityp lokaler Speicher VPS Best Practices” | Strikte Größenlimits (mehrschichtig), Allowlist, serverseitig erzeugte Pfade/Namen, Streaming auf lokale Festplatte, Magic-Bytes-Prüfung, sichere Auslieferung (Header/Autorisierung), atomare Ablage/Temp-Cleanup, Schutz vor Path Traversal und Missbrauch des Speicherplatzes (Quotas). |
| **fr** | “téléversement/téléchargement sécurisé fichiers binaires Axum limite taille type MIME stockage local VPS bonnes pratiques” | Téléversement/téléchargement sécurisé: limites de taille strictes, liste blanche de types, noms/chemins générés côté serveur, écriture en streaming sur disque, vérification signature/magic bytes, contrôle d’accès systématique, en-têtes sûrs et prévention du *path traversal*; quotas et nettoyage des fichiers partiels. |
| **zh-CN** | 文件上传下载安全 Axum 流式写盘 大小限制 MIME 白名单 路径遍历 本地存储 VPS | 对低内存 VPS 采用分块/流式写入临时文件并原子重命名，分层限流/限大小，服务端生成路径与文件名，白名单+魔数校验，下载鉴权与安全响应头，失败清理与配额核算（中文实践与 Axum 教程/社区经验同向）。 |
| **ja** | Axum ファイルアップロード ストリーミング サイズ制限 パストラバーサル MIME 検証 ローカル保存 VPS | 低メモリ環境では全体バッファリングを避けストリームで一時ファイルへ保存、`DefaultBodyLimit`/`RequestBodyLimitLayer` の多層制限、サーバ生成パス、MIME/マジックバイト許可リスト、ダウンロード認可・安全ヘッダ、失敗時クリーンアップが基本。 |
| **ko** | (접근성 제한/NV) “Axum 파일 업로드 스트리밍 크기 제한 경로 탐색 방지 로컬 디스크” | 용어/관행 삼각검증: 스트리밍 저장, 크기 제한, 서버 생성 경로, MIME/매직바이트 허용목록, 경로 탐색 방지, 다운로드 인가 — 국제 권고(OWASP/Axum)와 충돌 없음. |
| **ru** | (доступность ограничена/NV) “безопасная загрузка файлов Axum streaming лимит размера path traversal локальное хранилище” | Триангуляция терминов: потоковая запись на диск, многоуровневые лимиты, allowlist/MIME+magic bytes, серверные имена/пути, защита от path traversal, авторизация скачивания — соответствует OWASP/Axum best practices. |

**B complexity (repo-specific):** real bytes path missing end-to-end (multipart handlers, storage wiring, authz+workspace scoping, aggregate quotas, checksum, schema/workspace linkage, FE `workspace_id`/auth download behavior, e2e alignment). Security-sensitive; higher implementation/test surface.

### 1.2 (C) Seed/VPS operability hardening (2 vCPU / 4 GB)

| Lang | Query intent / terms | Key guidance (reliable consensus) |
|---|---|---|
| **EN** | “Docker Compose resource limits memory CPU profiles healthchecks small VPS 4GB”, “Meilisearch max-indexing-memory optional”, “Valkey maxmemory eviction”, “PostgreSQL tuning 4GB cheap VPS”, `pg_dump` backup | Compose: set per-service memory/CPU limits+reservations, healthchecks + `depends_on` conditions, profiles/overlays to separate `core` vs optional `realtime/search`, localhost binds, log rotation awareness; enforce/verify limits (`docker stats`, runtime-dependent for non-Swarm). Postgres on 4 GB: modest `shared_buffers` (e.g. ~256–512 MB starting), keep `work_mem` low, bounded `max_connections`, `effective_cache_size` as planner hint, maintenance modest; `pg_dump` logical backups off-host + periodic restore drills. Meilisearch: constrain `MEILI_MAX_INDEXING_MEMORY`/threads for small RAM but treat as hint (LMDB behavior) — disabling when unused is strongest debloat. Valkey: set `maxmemory`+explicit policy aligned to semantics (pubsub/cache vs correctness-sensitive), persistence only if needed. |
| **pt-BR** | perfis opcionais, degradação controlada, limites de memória/CPU, backup e restauração | Rodar perfil enxuto `api+postgres`, habilitar Valkey/Meilisearch por perfil/demanda, impor limites de recursos e healthchecks, validar com `docker stats`, backups `pg_dump` + armazenamento local e testes de restauração; configuração explícita e serviços opcionais realmente opcionais no boot. |
| **es** | perfiles opcionales, dependencias degradables, límites de recursos, copia de seguridad/restauración | Separar servicios obligatorios y opcionales, límites y healthchecks en Compose, degradación controlada cuando falta búsqueda/mensajería, ajuste conservador de Postgres para 4 GB y rutinas de respaldo/restauración probadas. |
| **de** | optionale Profile, degradierte Abhängigkeiten, Ressourcenbegrenzungen, Backup/Wiederherstellung | Optionale Komponenten per Profil aktivieren, harte Ressourcenbegrenzungen und Healthchecks setzen, Start ohne optionale Dienste ermöglichen (degradierter Modus), PostgreSQL für wenig RAM konservativ einstellen, regelmäßige Backups und Wiederherstellungstests. |
| **fr** | profils optionnels, dépendances dégradables, limites de ressources, sauvegarde/restauration | Profils Compose (cœur/temps réel/recherche), limites mémoire/CPU, healthchecks, démarrage tolérant aux services optionnels, réglages PostgreSQL prudents pour 4 Go, sauvegardes `pg_dump` + stockage local et exercices de restauration. |
| **zh-CN** | 小内存 VPS Docker Compose 资源限制 健康检查 可选服务 PostgreSQL 调优 备份恢复 | 4GB VPS 优先核心服务（API+Postgres），按需启用 Valkey/Meilisearch，配置内存/CPU 上限与健康检查，降低连接数与保守调优，定期逻辑备份并演练恢复。 |
| **ja** | 低スペックVPS Compose リソース制限 ヘルスチェック 任意サービス PostgreSQL チューニング バックアップ | 必須/任意サービスを分離しプロファイル化、メモリ/CPU 制限とヘルスチェック、任意依存の起動失敗を許容する設計、4GB 向け PostgreSQL 保守設定、`pg_dump` と復元訓練を重視。 |
| **ko** | (접근성 제한/NV) “저사양 VPS Docker Compose 리소스 제한 선택 서비스 PostgreSQL 튜닝 백업” | 핵심/옵션 서비스 분리, 리소스 제한·헬스체크, 옵션 의존성의 정상적 degraded 부팅, 4GB 보수 튜닝, 백업/복구 점검 — 문서화된 국제 관행과 일치. |
| **ru** | (доступность ограничена/NV) “VPS 4GB Docker Compose лимиты ресурсов опциональные сервисы PostgreSQL бэкап” | Профили core/realtime/search, лимиты и healthcheck, опциональные зависимости не должны блокировать старт, консервативная настройка PostgreSQL, регулярный `pg_dump` и проверка восстановления. |

**C complexity (repo-specific):** well-localized to config/state/compose/env/docs with clear existing audit/quick-win map; mostly wiring/gating/contract alignment and operational defaults; lower security novelty than B.

## 2) Authoritative sources consulted

- **Axum/tower (EN):** `axum::extract::Multipart` (default 2 MB, body/ordering), `axum::extract::DefaultBodyLimit` vs `tower_http::limit::RequestBodyLimitLayer` (local extractor vs global middleware semantics), streaming guidance (`field.chunk()`, avoid buffering). Fetched: docs.rs axum/axum-core, tower-http limit, axum tutorials/guides (Irurueta, rs4ts, axum-tutorial, Leapcell/Helium/Medium corroborating patterns).
- **OWASP/ASVS (EN, multilingual intent aligned):** File Upload Cheat Sheet, Unrestricted File Upload, ASVS v5 file handling (size validation, magic bytes/content checks, server-generated paths, path traversal/LFI/RFI/SSRF-safe file ops, safe download headers/Content-Disposition), filename/storage hardening.
- **Docker/Compose (EN):** Compose `deploy.resources` limits/reservations, `healthcheck`, `depends_on` conditions/profiles patterns; operational caveats on enforcement (non-Swarm/`--compatibility`) and need to verify with `docker stats`.
- **PostgreSQL (EN):** runtime resource config (`shared_buffers`, `work_mem`, `maintenance_work_mem`, `max_connections`), backup/restore (`pg_dump`/`pg_restore`), small-RAM tuning guidance.
- **Meilisearch (EN):** launch config, `MEILI_MAX_INDEXING_MEMORY`/`MEILI_MAX_INDEXING_THREADS` (budget/hint behavior, can exceed/LMDB considerations), indexing RAM/thread behavior on constrained hosts.
- **Valkey (EN):** memory optimization, `maxmemory`/eviction policies, persistence/AOF tradeoffs for small instances.
- **Repo evidence:** `flora-api/src/{main.rs,state/app_state.rs}`, `flora-core/src/{database.rs,config/*.rs}`, `docker-compose.yml`, `.env.example`, prior 014/015 research & audits (authoritative for current-state gaps).

## 3) Comparative assessment (80/20 for cycle 016)

| Criterion | **B — Binary files** | **C — Seed/VPS operability** |
|---|---|---|
| User value (P0) | High (real files missing) | High (reliable boot/run on target VPS, prerequisite for usable Seed) |
| VPS 2vCPU/4GB impact | Neutral/risk if naive buffering; needs careful streaming/limits | **Directly reduces risk** (caps, optional deps off, pool/right-sizing, clearer profiles) |
| Implementation risk | **Higher**: multipart/streaming, FS safety, checksums, quotas, schema/workspace linkage, FE contract/auth download, cleanup atomicity | **Lower/moderate**: config gating, pool wiring/init dedup, compose/env/docs; known quick-win map |
| Cross-cutting leverage | Narrower vertical feature | **Broad**: stabilizes dev/CI/manual smoke and future slices (including B) |
| Verifiability now | Requires new DB/storage behavior + FE alignment; more NVs | Deterministic config/unit/boot-path checks + compose validation; DB-gated existing patterns reusable |
| Debloat alignment | Adds necessary functionality (not debloat) | **Strong debloat/optionality** (Postgres-first core, optional search/messaging) |

**Research-backed recommendation:** prioritize **C** in 016 to make “cheap VPS Seed” operable and truthful, then execute **B** with safer footing. This matches `OBJETIVO.md`/`ROADMAP.md` emphasis on operability, optionality, and verifiable slices.

## 4) Limitations / NV (research)

- NV-R1: `websearch` results were partially constrained by provider quota in-session; synthesis relies on fetched authoritative docs (Axum/tower/OWASP/Docker/PostgreSQL/Meilisearch/Valkey) and internal repo research. No conflicting guidance identified.
- NV-R2: **ko, ru** — limited direct locale-primary retrieval; covered by terminology mapping and triangulation against EN/official sources (non-conflicting).
- NV-R3: No production/runtime measurements on target VPS hardware; numeric caps/pool defaults are conservative starting points requiring validation.
- NV-R4: Exact `deploy.resources` enforcement semantics depend on runtime/mode; must be verified in target environment.