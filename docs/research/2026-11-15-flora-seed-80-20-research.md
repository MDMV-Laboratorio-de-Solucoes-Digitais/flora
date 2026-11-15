# Pesquisa consolidada — Flora Seed 80/20 (2026-11-15)

**Objetivo:** subsidiar `OBJETIVO.md`, `ROADMAP.md` e o próximo ciclo 80/20 com informações atualizadas e de alta confiabilidade.  
**Método:** pesquisa web via fontes primárias/oficiais e guias técnicos reputados (consultas em múltiplos idiomas quando útil; priorização de documentação oficial). Síntese realizada por subagentes de pesquisa e consolidada neste repositório.  
**Data de acesso:** 2026-11-15.

## 1) Plataformas colaborativas self-hosted/OSS: o que define um “Seed” utilizável

**Exemplos analisados:** Mattermost, Zulip, Rocket.Chat, Nextcloud Hub/Talk, Element/Matrix.

**Achados-chave (consenso de fontes oficiais):**
- Um seed viável é uma **pilha mínima com forma de produção**: runtime suportado, banco configurado, proxy reverso/TLS, storage persistente, DNS/e-mail/notificações conforme necessário e **caminho de backup/restore** (não apenas código de aplicação).
- **Identidade/admin** (bootstrap, política de acesso, SSO/LDAP/OIDC quando aplicável, MFA/desprovisionamento) é alavanca inicial de adoção e segurança.
- **Confiabilidade do fluxo central** (mensagens/histórico/busca básica, notificações, arquivos) supera amplitude de funcionalidades instáveis; chamadas/tarefas extras só se essenciais ao piloto e com dependências de infra atendidas.
- **Onboarding e operação** (estrutura inicial, canal de suporte/champions, observabilidade mínima, upgrades/patches) determinam retenção do piloto.

**Implicação para Flora:** priorizar “deploy/dev + auth utilizável + fluxo núcleo confiável + notificações + backup/restore documentado” antes de HA, federação, ecossistema amplo ou features avançadas.

**Fontes (seleção):** Mattermost Deployment/Rollout/Auth docs; Zulip requirements/install/self-hosting; Rocket.Chat system requirements/deploy/auth/releases (2026); Nextcloud admin/Talk docs e release blogs (2025–2026); Element/Synapse/ESS docs e materiais Matrix (2025–2026).

## 2) Stack Flora (Axum/SQLx/Postgres/Valkey/Meilisearch/OIDC): 80/20 técnico para estágio Seed

**Princípios de alta confiabilidade (fatos/práticas bem estabelecidas):**
- **Single-VPS/dev baseline:** um único ponto público (proxy 80/443); **não expor** Postgres/Valkey/Meilisearch; imagens pinadas, `restart` e rotação de logs; volumes persistentes e **teste de restore**.
- **Orquestração/saúde:** `depends_on` com `service_healthy` quando aplicável; endpoints de liveness/readiness com checagens baratas e reais; graceful shutdown (Axum/Tokio) para evitar perda de requisições.
- **Migrações:** versionadas e aplicadas de forma controlada antes de servir tráfego; evitar concorrência não coordenada; validar em banco limpo em CI quando possível (SQLx).
- **OIDC (Zitadel):** Authorization Code + **PKCE (S256)**, biblioteca confiável (descoberta/JWKS/validações), `state`/`nonce` e redirect URIs exatos; sessão server-side (cookies HTTP-only/secure) após troca de tokens; atenção a proxy/HTTPS/clock skew.
- **Realtime/notificações:** Pub/Sub (Redis/Valkey) é at-most-once e não durável — para não perder eventos, persistir estado/eventos (DB/outbox/Streams) e suportar retomada/idempotência; para notificações simples, SSE + registros persistentes costuma ser o 80/20.
- **CI/Verificação:** `fmt`/`clippy`/`test` com cache; smoke/integration com serviços (Postgres e demais conforme núcleo); verificação de deploy local (health, login, escrita/leitura, busca) e runbooks.

**Armadilhas comuns:** portas internas expostas, assumir “container up” = pronto, migrações concorrentes, logs sem limite, backups nunca restaurados, falhas de callback/cookie atrás de proxy, perda de mensagens por depender só de Pub/Sub.

**Fontes primárias (seleção):** Docker Compose/docs; Axum/SQLx/Tokio docs; PostgreSQL backup docs; Redis/Valkey docs (security, pubsub, streams, persistence); Meilisearch docs (security, backup, docker); Zitadel OIDC docs + OWASP OAuth2; Rust Cargo/CI docs. Guias suplementares recentes (2026) usados como prática contextual.

## 3) Framework de decisão 80/20 (próximo passo)

1. **Classificar incerteza dominante:** viabilidade/integração → spike/POC time-boxed; escopo difuso → assess/shape/spec; falta de fluxo ponta a ponta → fatia vertical/walking skeleton; falta de operabilidade/demo → hardening/production-readiness do slice; defeito bloqueador no caminho crítico → bug-fix direcionado.
2. **Pontuar candidatos:** valor de usuário ponta a ponta, redução de risco/integração, verificabilidade, deploy/demoabilidade (self-host), esforço/confiança, apetite e capacidade de desbloquear próximos passos.
3. **Regras práticas:** sem fluxo núcleo end-to-end → priorizar 1 jornada completa mínima; se local funciona mas não é reproduzível/operável → priorizar baseline de execução + readiness; escolher **uma** ação concluível que gere evidência observável.

**Fontes:** Basecamp Shape Up; materiais sobre walking skeleton/thin vertical slice/tracer bullet; RICE/impact-effort (Atlassian/NN/g); SVPG discovery vs delivery; Google SRE (PRR/launch/checklist).

## 4) Decisão aplicada à Flora (2026-11-15)

- **Contexto real:** código e specs avançados em partes, porém sem baseline local reproduzível de dependências, com integrações núcleo incompletas e verificação ponta a ponta dependente de serviços não provisionados no ambiente.
- **80/20 escolhido (ciclo 004):** **Baseline local reproduzível do Seed** (composição dev de Postgres/Valkey/Meilisearch + configuração/instruções/seed e critérios de smoke/verificação) e consolidação documental (`OBJETIVO.md`, `ROADMAP.md`, assessment 004). Isso maximiza destravamento de validação e ciclos seguintes com menor risco de retrabalho.
- **Próximas fatias (após 004):** consistência auth/contexto + PKCE gaps de teste; fluxo mínimo realtime/mensagem ou tarefa ponta a ponta verificável; integração de indexação/notificação; alinhamento de contratos de arquivos.

## 5) Limitações desta pesquisa

- Recomendações de operação/stack devem ser validadas contra versões fixadas no projeto e ambiente alvo.
- Resultados de CI remota e execuções dependentes de infraestrutura externa não foram revalidados integralmente nesta sessão; claims de “verde” devem permanecer atreladas a evidência executada.
