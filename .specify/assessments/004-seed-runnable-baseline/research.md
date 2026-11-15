# Research: 004 — Seed Runnable Baseline

**Date:** 2026-11-15  
**Confidence:** média-alta para direção; média para detalhes dependentes de versões/ambiente (validar contra lockfiles/config locais ao implementar).

## 1) Estado interno (evidência do repositório)

### Produto e governança
- Suíte colaborativa self-hostable em Rust (AGPL), modular monolith; Seed v0.1 visa org/workspace, OIDC/RBAC, canais/mensagens/threads, tarefas, arquivos, busca, notificações (`README.md`, `.specify/memory/constitution.md`, `specs/001-*`).
- Constituição v1.2.0; specs 001-003 presentes; assessment 003 (go) concluiu baseline parcial e PKCE; bug flow não utilizado. Drift documentado entre checkmarks/README e verificação real.

### Implementação e verificação
- Runtime: `flora-api` (config → pool Postgres → migrações → `AppState` → Axum). `flora-core` provê config/DB/migrations/modelos/repos/serviços/storage.
- Verificado localmente (2026-11-15): `cargo check -p flora-api -p flora-core`, `cargo fmt --all -- --check`, `cargo test --workspace --no-run`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54) ok. Integração com serviços externos não exercitada localmente.
- Gaps críticos: ausência de compose dev; integrações incompletas (WS fanout, indexação/notificações orientadas a eventos, contrato binário de arquivos, consistência auth/RBAC, health checks dependentes de serviços, quota total); cobertura PKCE de sucesso/mock e verificação JWKS limitadas.

## 2) Pesquisa externa (síntese de fontes primárias/oficiais)

### Seed utilizável em suites OSS self-hosted
- Padrão recorrente (Mattermost, Zulip, Rocket.Chat, Nextcloud, Element/Matrix): pilha mínima operável (DB/proxy/TLS/storage/DNS-email conforme caso) + auth/admin + confiabilidade do fluxo central + notificações + onboarding mínimo + **backup/restore testado** + caminho de upgrade. Amplitude de features vem depois de estabilidade do piloto.

### 80/20 para stack Rust/Axum + Postgres + Valkey + Meilisearch + OIDC
- **Dev/seed infra:** não expor serviços internos; imagens pinadas, restart/log rotation, volumes, healthchecks e `depends_on` por saúde; migrações controladas antes de tráfego; graceful shutdown/readiness real.
- **OIDC:** Authorization Code + PKCE (S256), biblioteca confiável, validação completa (state/nonce/redirect/issuer/audience/JWKS conforme fluxo), sessão server-side, cuidado com proxy/cookies/clock.
- **Realtime:** Pub/Sub sem durabilidade (at-most-once) — persistir e suportar retomada/idempotência quando perda não é aceitável.
- **CI/verificação:** fmt/clippy/test + cache; smoke/integration com serviços e banco limpo para migrações; scripts/checklists de verificação de deploy local.

### Framework de decisão
- Priorizar fatia vertical/walking skeleton quando não há jornada núcleo verificável ponta a ponta; se falta operabilidade/reprodutibilidade, endurecer baseline do slice; escolher uma ação concluível com evidência observável (Shape Up, RICE/impact-effort, SRE readiness).

## 3) Opções consideradas

| Opção | Descrição | Prós | Contras | Esforço | Impacto |
|---|---|---|---|---|---|
| A | Continuar implementando features isoladas (WS/search/arquivos/auth) sem baseline local reproduzível | Avança código pontual | Difícil verificar ponta a ponta; alto risco de regressão/drift | Médio/alto | Médio (incerto) |
| B | **Baseline local reproduzível do Seed** (compose dev + config/docs/smoke) + consolidação objetivo/roadmap | Destrava verificação e onboarding; base para fatias verticais; baixo risco | Não entrega feature de usuário final por si só | **Baixo/médio** | **Alto** |
| C | Provisionar ambiente completo de produção/CI remota e automação ampla | Máxima fidelidade | Requer autorizações/infra/segredos; fora do escopo autônomo | Alto | Alto (adiado) |

## 4) Recomendação
Adotar **Opção B** como ciclo 80/20 atual: consolidar direção/evidência (OBJETIVO/ROADMAP/pesquisa) e definir/executar baseline local reproduzível (infra dev + documentação + critérios de smoke), seguida de fatias verticais focadas em auth/contexto, mensagem/tarefa e integrações núcleo com evidência.

**Fontes detalhadas:** `docs/research/2026-11-15-flora-seed-80-20-research.md` e referências primárias ali citadas.
