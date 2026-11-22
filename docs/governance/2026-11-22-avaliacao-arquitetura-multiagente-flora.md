# Avaliação crítica da arquitetura multiagente MDMV aplicada ao Flora

**Data**: 2026-11-22 · **Assessment**: `avaliacao-arquitetura-multiagente-flora`
**Fonte do plano**: `MDMV-Autonoma/docs/Arquitetura Multiagente Autônoma da MDMV.pdf`
**Factory (somente leitura)**: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`
**Gate do alvo**: `python3 scripts/ci/check_gate_integrity.py` — PASS (10 legs / 77 pisos), inalterado

## Resultado

Assessment documental concluído, **nenhuma mudança de código, gate, ADR, constituição, migration, credencial ou deploy**. Intake seguiu o aparato `speckit.assess.intake` da factory (slug, seções e `[NEEDS CLARIFICATION]`), lido sem modificar a factory.

Foram avaliadas **152 propostas** em nove eixos entregues (1.112 linhas de relatórios em `docs/research/mdmv-multiagent/`), com veredito **GO** e registro em `.specify/assessments/avaliacao-arquitetura-multiagente-flora/decision.md`.

## Síntese das decisões

- **Adotar agora (sem serviço novo)**: PKCE+sessão DB, RBAC, `sqlx` sem ORM, Postgres FTS, `tracing`/`/metrics`, `ConsentLedger` como extensão de modelo, `.tokens/` 0o600, validação semântica de invariantes, `proptest`, Briefings de Compreensão, revisão humana HITL, indicador de soberania, WCAG 2.2 AA como alvo com revisão humana.
- **Adaptar com forma própria do Flora**: `WorkflowEnginePort` (motor Absurd upstream, não SDK), `cedar-policy` para máquinas, CaMeL como defesa em profundidade, `LlmGatewayPort`, `rmcp` read-only, grafo `@links`, holdout com curadoria humana, `cargo-mutants` informativo, `ConsentLedger`, ISO 42001 como inventário documental.
- **Diferir com porta de reentrada**: A2A, MCP Tasks/Apps, Temporal/DBOS, `pg_cron`, gateway GoModel/Bifrost, cache semântico, CRM fiscal, NFS-e/DANFSe, Footprint de observabilidade pesado, Tauri.
- **Evitar/Rejeitar**: `absurd-sdk` 4 estrelas sem crates.io, Restate (BSL), `axum-jwt-oidc`, TensorZero arquivado, CaMeL como garantia única, permissão por menção do Macro, posts X com URL/AI bots, SMTP próprio, Meilisearch no Seed, `cnpj` em `organizations`, premissa 4 OCPU/24 GB, datas invertidas do EU AI Act/IBS-CBS.

## Correções ao plano que afetam decisões futuras

SQLx sem checagem offline; hardware Always Free 2 OCPU/12 GB por tenancy; MCP 2026-07-28; Signed Agent Card é MAY; RFC 10070 atualiza `aud` do client assertion; cotas/custos de YouTube/Hashnode/GSC e tipificação da Lei 15.325/2026 divergentes; oxlint/Biome sem plugin Svelte estável.

## [NV]

Eixos **auditoria/imutabilidade (RLS runtime, hash-chain, backup)** e **alternativas de orquestração/premissas fracas** não foram entregues por rate-limit na busca externa; o assessment não finge cobertura integral. Staging real, CI/deploy após este assessment e implementação das decisões permanecem **[NV]**. Nenhuma credencial foi copiada; instruções encontradas em conteúdo web foram tratadas como dados e registradas.

### Atualização — 2026-11-22T11:15Z (append-only)

Os dois eixos antes pendentes foram entregues: `auditoria-imutabilidade-flora.md` (AUD-01…17) e `alternativas-premissas-flora.md` (O1–O7/P1–P6/J1–J6). Assessment passa a **11 eixos / 188 decisões**. RLS no Flora é `ENABLE` sem policies e a app conecta como `postgres` — ADOPT no desenho, DEFER no runtime (ADR-0005/0006); nenhuma mudança de gate ou produção foi feita. ADR-0003 e `check_gate_integrity.py` permanecem a fronteira de autoridade; verificadores de cadeia, se criados, nascem **advisory**.
