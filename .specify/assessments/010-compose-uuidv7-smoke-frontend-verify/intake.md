# Intake: Flora 80/20 Cycle 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Date:** 2026-11-15  
**Status:** draft (research/audits in progress)

## Captured request
Executar o próximo ciclo 80/20 (após ciclo 009 mergeado em `dev@03de975e1`) seguindo o fluxo mdmv-dark-factory para este repositório, com pesquisa extensiva multi-idioma e orquestração de subagentes, avançando compatibilidade dev reproduzível e verificabilidade do fluxo núcleo.

## Candidate scope (from 009 handoff / OBJETIVO-ROADMAP)
1. **Reconciliar compose/migração `uuidv7`:** alinhar `docker-compose.yml` e/ou migração para Postgres suportado (pg18 nativo, extensão/função compatível para pg16, ou estratégia segura) e evidenciar smoke `docs/dev-smoke.md` pelo caminho documentado.
2. **Smoke integrado documentado:** executar e registrar evidência `select org → workspace → task` via compose/documented path (ou NV detalhado com causa mitigada).
3. **Frontend/API alinhamento mínimo e verificação determinística:** estabilizar slice auth→org→workspace→task conforme necessário e executar checks/lint/build/tests do `apps/frontend` quando viável (registrar evidência/NV).

## Constraints
- Mudanças em migrações existentes exigem cuidado (DBs que já aplicaram init migration); preferir solução backward/forward-safe e documentada.
- Ambiente pode ter conflitos de porta (5432 observado no ciclo 009) e limitações de toolchain/runtime; evidenciar com transparência.
- Manter segurança/tenancy e escopo 80/20 (sem OIDC/JWKS e2e real, sem paridade ampla).

## Subagents (in progress)
- dark-factory workflow/gates for cycle 010
- compose/migrations/uuidv7 audit + safe fix proposal
- frontend deterministic verification/setup audit
- multilingual research (uuidv7/PG, migration safety, SvelteKit testing, smoke/CI)
- docker/compose environment probe for documented smoke

## Desired outcome
- Assessment/spec 010 com decisão `go` e escopo acotado.
- Caminho dev/smoke documentado funcional e evidenciado (compatibilidade `uuidv7` resolvida).
- Verificações determinísticas relevantes verdes/evidenciadas (Rust + frontend conforme viável).
- PR GitHub com CI verde, merge em `dev`, sync `origin/dev`, manter apenas `dev`/`main`.

## Early finding: environment probe (`ses_e1e70c41cffe7ciYAWvFvne7bG`, completed)
- **No-go (as-is) for documented smoke now:** host `5432` occupied (not by flora compose), `.env` absent, and `docker-compose.yml` still `postgres:16-alpine` while migrations require `uuidv7()` (PG18-native; `postgres:18-alpine` image available locally). Ports 6379/7700/3000 free; no flora containers/volumes.
- Safe execution will require compatible Postgres in compose (or equivalent), isolated project/volumes, consistent `.env`/JWT secret, and avoidance of non-disposable DBs on 5432.
