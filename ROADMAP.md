# ROADMAP — Flora

**Atualizado em:** 2026-11-15  
**Fonte de verdade operacional:** código, `Cargo.toml`, workflows em `.github/workflows/`, `.specify/memory/constitution.md`, `specs/001-003`, `.specify/assessments/003-004`.  
**Nota:** `README.md` contém visão/roadmap histórico; este arquivo consolida o estado real observado + prioridades atuais. Itens marcados como “verificado” exigem evidência executada (ver `OBJETIVO.md`).

## 1) Visão e horizonte

- **Produto:** suíte corporativa/colaborativa self-hostable (AGPL-3.0), Rust + PostgreSQL, frontend canônico SvelteKit/Svelte 5, evolução para Tauri.
- **Crescimento por implantação:** **Seed** (mínimo, VPS única) → **Sprout** → **Grove** → **Forest** (distribuído/HA).
- **Princípio 80/20:** entregar fatias verticais utilizáveis e verificáveis do núcleo Seed antes de ampliar escopo, integrações pesadas ou paridade com stacks complexas.

## 2) Onde estamos (2026-11-15)

| Trilha | Estado | Evidência resumida |
|---|---|---|
| Governança/SpecKit | **Em andamento (maduro em artefatos)** | Constituição v1.2.0; specs 001-003; assessments 003 (go) e 004 (go, este ciclo); bug flow não utilizado |
| Baseline Rust (local) | **Parcialmente verificado** | `cargo check -p flora-api -p flora-core`, `cargo fmt --all -- --check`, `cargo test --workspace --no-run`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54) OK em 2026-11-15; baseline 003 registrou clippy/test workspace verdes no ambiente de baseline |
| Baseline frontend canônico (`apps/frontend`) | **Parcialmente verificado** | Baseline 003: install/check/build OK; lint pós-formatação e testes unitários browser/Playwright inconclusivos por ambiente (ENOSPC). CI definida em `.github/workflows/ci.yml` |
| API/runtime (`flora-api` + `flora-core`) | **Implementação parcial e heterogênea** | Bootstrap, rotas e serviços/repositórios existem; auth/OIDC (PKCE) presente; integrações (WS fanout, indexação busca, eventos→notificações, binário de arquivos, RBAC/consistência auth) incompletas ou desalinhadas em pontos críticos |
| Reprodutibilidade local do Seed | **Gap** | Sem `docker-compose` de dependências no checkout; `dev-seed.sql` existe; setup ponta a ponta depende de serviços externos manuais |
| Integração/contrato com serviços (Postgres/Valkey/Meilisearch/OIDC) | **Não verificado ponta a ponta neste ambiente** | Requer stack de dependências e, para auth real, provedor OIDC configurado/mockado |

## 3) Marcos concluídos / parcialmente concluídos

- [x/~] Estrutura de workspace Rust e crates de domínio/serviço definidas (`flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-notifications`, `flora-tests`).
- [x/~] Especificações 001 (Seed backend) e 002 (rewrite Svelte 5) produzidas com tarefas amplamente marcadas (drift vs verificação real reconhecido).
- [x] Ciclo 003 (assessment + decisão go): baseline registrado, correções de formatação, alinhamentos CI/docs canônicos em partes e implementação de PKCE (T116) com evidência parcial de testes.
- [~] Frontend canônico identificado (`apps/frontend`); `apps/web` ainda presente como não-canônico.
- [x] **Ciclo 004 (este ciclo, 80/20):** consolidação de `OBJETIVO.md`/`ROADMAP.md`, pesquisa documentada e assessment/decisão para baseline local reproduzível do Seed (ver seção 4).

## 4) Próximos passos priorizados (80/20)

### P0 — Agora (ciclo 004 → execução seguinte)
1. **Baseline local reproduzível (Seed dev):** fornecer composição de dependências (Postgres, Valkey/Redis, Meilisearch; OIDC mock/Zitadel como perfil opcional/documentado), `.env`/exemplos e passos determinísticos para `migrate + run + seed` locais.
   - *Resultado esperado:* “subir stack + rodar `flora-api` + aplicar seed” documentado e reproduzível.
2. **Smoke path verificável:** health/readiness com checagens reais (ou plano explícito), script/checklist de smoke manual para auth stub/dev ou fluxo mínimo viável.
3. **Evidência:** registrar comandos executados e limitações (CI local/ambiente) nos artefatos do ciclo.

### P1 — Desbloqueio do núcleo utilizável (fatias verticais)
4. **Consistência de auth/contexto:** unificar validação JWT/sessão/org nas rotas críticas (ou definir contrato claro de headers apenas para dev) e fechar lacunas PKCE (teste de sucesso com mock de token/JWKS, casos de estado).
5. **Fluxo comunicação mínimo end-to-end:** persistir e entregar mensagens em tempo real de forma verificável (assinaturas/fanout WS ou alternativa documentada) para um canal/org.
6. **Busca e notificações integradas ao domínio:** ganchos de indexação em mutações principais e criação/entrega mínima de notificações para eventos core.
7. **Arquivos:** alinhar contrato de upload/download (metadados vs binário/multipart) e contabilidade de quota coerente com requisitos.

### P2 — Qualidade, produto e operação
8. **Testes de integração/contrato executáveis em ambiente provisionado** (DB/serviços) e estabilização de suíte frontend (lint/test browser) em CI.
9. **Hardening Seed:** observabilidade básica, limites/rate limit efetivos, políticas de retenção, backups/restore documentados.
10. **Redução de drift documental:** reconciliar README/tasks/specs com estado real; deprecar/migrar claramente `apps/web` se permanecer fora do caminho canônico.

### P3 — Expansão (pós-Seed estável)
11. Sprout/Grove: PGMQ/RustFS e componentes opcionais conforme necessidade real.
12. Forest: replicação, workers distribuídos, múltiplos nós, HA/observabilidade avançada.
13. Roadmap funcional futuro (fases): documentos, e-mail, integrações, feature flags de IA, Tauri — somente após núcleo Seed confiável.

## 5) Backlog de não-objetivos (manter fora até decisão explícita)

- Paridade total com Macro; integrações SaaS amplas; CRDT-everywhere; chamadas A/V, CRM, calendário, e-mail completo, MCP/agents como escopo Seed.
- Dependências de stack pesada para Seed (Kafka/OpenSearch/FusionAuth/DO/agent-harness como requisito).
- REA/Craft-Anything como componentes core/dependências do produto.
- Engenharia reversa não autorizada/proprietária ou reuso incompatível com AGPL.
- Deploy/infra de produção, merges/releases ou mudanças com risco/autorizações fora do ciclo sem aprovação explícita.

## 6) Critérios para avançar de fase

- **Seed “utilizável”:** stack local reproduzível + auth e fluxo núcleo (org/workspace + canal/mensagem ou tarefa) exercitáveis com evidência; verificações determinísticas rastreadas.
- **Sprout/Grove:** somente após Seed estável e necessidade comprovada de mensageria/armazenamento/busca/identidade ampliados.
- **Forest:** somente após requisitos de escala/resiliência e operação validados.
