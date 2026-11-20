# Decision: 008 — Select-Organization Tests & Reproducible Dev Smoke

**Date:** 2026-11-15  
**Verdict:** **go** (escopo acotado, condicionado por NV de runtime/DB integrado)

## Rationale
- **Problem validity:** alta — `select-organization` era ponto crítico sem testes dedicados e sem caminho de smoke reproduzível sem OIDC.
- **Value vs effort:** alta — testes determinísticos CI-safe + bootstrap/dev smoke documentado elevam verificabilidade do Seed em VPS barata com baixo risco.
- **Risk:** baixo — mudanças limitadas a testes/artefatos/docs/scripts de suporte dev; sem alteração de comportamento de runtime da API.

## Selected scope
1. Adicionar e validar testes de contrato CI-safe para `select-organization` (auth ausente; `organization_id` nil rejeitado com contrato atual `422` de validação).
2. Entregar caminho reproduzível de dev smoke sem OIDC (docker compose + migrations via API + `dev-seed.sql` + bootstrap de sessão/JWT determinístico) para `select org → workspace → task`, com nuances conhecidas documentadas.
3. Consolidar artefatos SpecKit 008 e evidência de verificação; registrar NV explicitamente.

## Explicit non-goals / NV
- Testes DB-backed automatizados para sucesso/membership/sessão inválida em CI: **NV** sem Postgres em workflow; casos e pré-condições documentados.
- Execução integrada real do smoke (serviços rodando/API/DB) neste ambiente: **NV**.
- OIDC real/JWKS completo, alinhamento frontend (`apps/frontend`), deploy, serviços pagos: fora de escopo.

## Next 80/20
- Adicionar serviço Postgres no CI ou testes `#[ignore]`/env-gated com job dedicado para cobrir casos DB de `select-organization`.
- Alinhar fatia mínima frontend (callback `state`, `select-organization`, troca de token, workspace/task reais).
- Endurecer verificação de ID token OIDC (JWKS) e considerar authority check de sessão por `jti` em extrator/middleware.
