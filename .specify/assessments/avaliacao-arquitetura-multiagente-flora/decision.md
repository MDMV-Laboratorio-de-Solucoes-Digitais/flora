# Decision — avaliação da arquitetura multiagente aplicada ao Flora

**Data**: 2026-11-22 · **Slug**: `avaliacao-arquitetura-multiagente-flora`
**Veredito: GO (assessment concluído; nenhuma implementação autorizada por este artefato)**

## Decisão

O Flora deve permanecer como **plano de controle humano-agente**, com Seed determinístico e PostgreSQL-canônico. Das 152 propostas avaliadas, as que entram agora são majoritariamente restrições documentais e contratos de porta; nenhuma exige serviço obrigatório novo. Serviços pesados (Temporal, Absurd como deployed engine, A2A, MCP Tasks, DTU, gateway LLM externo, ERP fiscal) ficam **DEFER** com porta de reentrada explícita. Dependências sem manutenção, licenças incompatíveis e premissas falsas são **REJECT/AVOID**.

**Autorizações não concedidas**: nenhuma mudança de gate, ADR, constituição, credencial, deploy, migration ou contrato de tenant neste assessment. Implementações exigem ciclos SDD separados.

## Registro consolidado (152 itens)

Justificativa, evidência (título+URL+data) e `[NV]` estão nos relatórios indicados em `research.md`.

### Runtime durável (`runtime-duravel-flora.md`)
R1 ADOPT · R2 ADAPT · R3 ADOPT · R4 ADOPT (com correção de premissa) · R5 ADAPT/DEFER no Seed · R6 ADAPT · R7 **REJECT** (`absurd-sdk` como dependência) · R8 DEFER (Temporal) · R9 DEFER (DBOS) · R10 **AVOID** (Restate/BSL) · R11 REJECT/DEFER/AVOID por item · R12 ADAPT (HITL) · R13 ADAPT (números direcionais) · R14 DEFER (`pg_cron`).

### Segurança (`seguranca-agentes-flora.md`)
S1 DEFER (service accounts/Zitadel) · S2 **ADAPT** (RFC 10070: `aud` = issuer) · S3 **AVOID** (`axum-jwt-oidc`) · S4 ADAPT (`cedar-policy`) · S5 DEFER (OPA) · S6 DEFER (Casbin) · S7 DEFER (regorus) · S8 ADAPT (CaMeL) · S9 **REJECT** (CaMeL como garantia única) · S10 ADAPT (llguidance via porta) · S11 ADOPT (validação semântica) · S12 DEFER (alternativas; GBNF ADAPT se self-host) · S13 ADOPT (`.tokens/` 0o600) · S14 ADAPT (SecureFileCreator/PathValidator) · S15 ADOPT (PKCE+sessão DB) · S16 ADOPT/ADAPT para máquinas (RBAC) · S17 ADOPT no design / DEFER runtime (RLS) · S18 ADAPT (HITL `await_event`) · S19 ADOPT (segredos fora do LLM).

### Interoperabilidade (`interoperabilidade-controle-flora.md`)
I-01 ADAPT (MCP read-only) · I-02 ADAPT (`rmcp` pinned) · I-03 DEFER (MCP Tasks) · I-04 AVOID no Seed / DEFER Grove (MCP Apps) · I-05 DEFER (A2A) · I-06 DEFER (`a2a-rs`) · I-07 ADAPT como verificação / AVOID publicar card · I-08 DEFER (Registry) / REJECT (v0.3, ACP) · I-09 ADOPT/ADAPT (delegação durável) · I-10 ADOPT (memória opaca) · I-11 ADAPT (cartão HITL) · I-12 ADAPT (grafo `@links`) · I-13 **AVOID** (permissão por menção) · I-14 ADAPT (busca/notificação do grafo) · I-15 DEFER (AG-UI) · I-16 DEFER (A2UI) · I-17 REJECT (ACP duplicata) · I-18 AVOID (outros SDKs MCP Rust).

### Infraestrutura (`infra-observabilidade-flora.md`)
INF-01 **REJECT** (premissa 4 OCPU/24 GB) · INF-02 ADAPT (2 OCPU/12 GB por tenancy) · INF-03 ADOPT (sem SLA) · INF-04 ADAPT (recuperação por ociosidade) · INF-05 ADOPT (`iptables -I` anti-SSRF) · INF-06 ADAPT (sandbox Grove) · INF-07 ADAPT (Tailscale com ACL explícita) · INF-08 ADAPT (Coolify) · INF-09 DEFER (Runner local) · INF-10 ADOPT (Actions multi-arquitetura) · INF-11 ADAPT (RustFS via trait) · INF-12 DEFER (MinIO) · INF-13 DEFER (SeaweedFS) · INF-14 **REJECT** no Seed / DEFER Grove (Meilisearch) · INF-15 ADOPT (Postgres FTS) · INF-16 ADAPT (OpenObserve) · INF-17 DEFER (VictoriaMetrics) · INF-18 ADAPT (Uptime Kuma) · INF-19 DEFER/AVOID (SigNoz/Prom,SaaS) · INF-20 ADOPT (retenção/quotas) · INF-21 ADOPT (observabilidade Seed) · INF-22 ADOPT (sem Tauri no Seed) · INF-23 ADOPT (Zitadel ~512 MB) · INF-24 DEFER (números de RAM do plano).

### Fábrica (`dark-factory-pipeline-flora.md`)
F1 ADAPT (Nível 4.5) · F2 DEFER (grafo DOT) · F3 DEFER (linter DOT) · F4 ADAPT (holdout; curadoria ADOPT, LLM-judge DEFER) · F5 DEFER (DTU; stubs primeiro) · F6 ADAPT (`cargo-mutants` informativo) · F7 **ADOPT** (`proptest`) · F8 ADAPT (manter ESLint+svelte-check) / DEFER troca · F9 ADAPT (WCAG 2.2 AA) · F10 **ADOPT** (Briefings de Compreensão) · F11 DEFER (model stylesheet) · F12 ADAPT (medir fração determinística).

### Gateway LLM (`gateway-llm-finops-flora.md`)
G1 ADOPT contrato / DEFER implementação (`LlmGatewayPort`) · G2 DEFER/ADAPT Grove (GoModel) · G3 DEFER (Bifrost) · G4 **REJECT** (TensorZero arquivado) · G5 ADAPT direcional (benchmarks) · G6 ADAPT (cascata) · G7 ADAPT (prompt caching) · G8 ADAPT (orçamento persistido) · G9 ADAPT (chaves lógicas) · G10 ADAPT (decodificação restrita via porta) · G11 ADOPT (indicador de soberania) · G12 DEFER (cache semântico) · G13 ADAPT (proxy Rust in-process) · G14 AVOID/DEFER (LiteLLM/Portkey, roteadores aprendizes).

### Canais/marketing (`canais-marketing-flora.md`)
C1 DEFER (X texto) · C2 REJECT (X URL/bots) · C3 DEFER (LinkedIn) · C4 ADAPT (Bluesky) · C5 ADAPT (Dev.to `canonical_url`) · C6 DEFER (Hashnode Pro) · C7 DEFER (Threads) · C8 DEFER (YouTube) · C9 ADAPT (GSC read-only) · C10 DEFER (BigQuery) · C11 ADOPT (Listmonk+relay) · C12 REJECT (SMTP próprio) · C13 **ADOPT** (`ConsentLedger`) · C14 ADAPT (LGPD) · C15 ADAPT (CONAR) · C16 REJECT (Lei 15.325 descrita no PDF) · C17 ADOPT (CDC/CC) · C18 ADOPT (privacidade radical) · C19 ADAPT (analytics cookieless) · C20 DEFER (CRM nativo) · C21 ADAPT (CPQ→Spec) · C22 DEFER (ledger fiscal) · C23 DEFER (WhatsApp) · C24 DEFER (TimescaleDB).

### Compliance fiscal/jurídica (`compliance-fiscal-flora.md`)
C1 ADOPT (CNPJ alfanumérico como regra documental) · C2 REJECT/DEFER (`cnpj` no tenant) · C3 DEFER no Flora (NFS-e) · C4 AVOID (DANFSe no Flora) · C5 ADAPT/DEFER (datas IBS/CBS) · C6 DEFER/ADOPT fora do Flora (LTDA) · C7 ADOPT/DEFER (DPO) · C8 ADOPT documento / AVOID tabela (ROPA) · C9 ADAPT/DEFER (ISO 42001) · C10 ADAPT/DEFER (EU AI Act Art. 50) · C11 **ADOPT** (revisão humana) · C12 DEFER/REJECT (PL 2338 como vigente) · C13 ADOPT serviço dedicado / AVOID tabelas fiscais no Flora.

### Frontend/acessibilidade (`frontend-acessibilidade-flora.md`)
FR-01 ADAPT · FR-02 ADOPT (runes) · FR-03 ADOPT (shadcn+Tailwind) · FR-04 ADAPT (pinagem) · FR-05 DEFER/REJECT (troca por oxlint) · FR-06 ADAPT (WCAG 2.2) · FR-07 ADOPT (revisão humana) · FR-08 ADOPT (WCAG 2.2, não 2.1) · FR-09 ADAPT (ISO 24495-1) · FR-10 DEFER (-2/-3) · FR-11 DEFER (DTCG) · FR-12 ADOPT (branding) · FR-13 ADOPT como restrição (sem Tauri no Seed) · FR-14 ADAPT (indicador de soberania).

## [NV] e limites

- Eixos **auditoria/imutabilidade** e **alternativas/premissas fracas** não entregues por rate-limit de busca web; ver `research.md`.
- Nenhuma decisão foi implementada; nenhum gate/required check/ADR/contrato foi alterado.
- Pesquisa web tratada como dado não-confiável; instruções encontradas foram registradas e ignoradas; nenhuma credencial copiada.

### Registro suplementar — 2026-11-22T11:15Z (append-only)

**Auditoria/imutabilidade (`auditoria-imutabilidade-flora.md`)**: AUD-01 REJECT produto/ADAPT padrão (`AuditLedgerPort`) · AUD-02 ADAPT append-only (trigger + dono NOLOGIN, `REVOKE` insuficiente) · AUD-03 ADAPT SHA-256 com `pgcrypto` e chave fora do DB · AUD-04 ADOPT âncora/ADAPT WORM · AUD-05 ADAPT commit assinado como 2ª âncora · AUD-06 ADOPT desenho/DEFER runtime (RLS) · AUD-07 ADAPT PITR · AUD-08 ADOPT contrato/DEFER execução (restore) · AUD-09 ADAPT retenção fiscal · AUD-10 ADOPT papéis PG · AUD-11 ADAPT `pgAudit` · AUD-12 AVOID immudb no Seed · AUD-13 DEFER notarização · AUD-14 ADAPT verificação diária · AUD-15 ADOPT criptografia/chaves · AUD-16 ADAPT logs/volume · AUD-17 ADOPT gate atual/ADAPT checks novos **advisory**.

**Alternativas/premissas (`alternativas-premissas-flora.md`)**: O1–O7 AVOID/DEFER orquestradores; Goose ADAPT opcional; nenhum substitui `check_gate_integrity.py` · P1–P6 REJECT/DEFER premissas financeiras (99,77%, US$ 68,69, 85–98%) · J1–J6 REJECT premissas jurídicas de fonte única e correção das contradições (4 OCPU/24 GB, HITL vs deploy autônomo).

Os dois eixos saem da condição **[NV]**; detalhes não verificados seguem marcados nos relatórios.
