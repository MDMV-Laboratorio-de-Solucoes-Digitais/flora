# Intake: Flora 80/20 Cycle 004 — Seed Runnable Baseline (2026-11-15)

## Captured request
Executar o próximo ciclo 80/20 para a Flora de forma autônoma (conforme workflow aplicável), com pesquisa ampla/atualizada, considerando `ROADMAP.md`/`OBJETIVO.md` (inexistentes no início), documentação/specs/assessments do projeto e estado real do repositório. Produzir consolidação (criar/atualizar objetivo, roadmap e artefatos relevantes) e avançar o ciclo de maior alavancagem com menor esforço.

## Restated idea
Tornar o **Seed v0.1 verificável e reproduzível localmente** como próximo incremento de maior alavancagem: consolidar objetivo/roadmap baseados em evidência, registrar pesquisa e decisão, e definir/executar o recorte 80/20 que destrava validação ponta a ponta (dependências locais, setup e critérios de smoke) sem ampliar escopo de produto nem assumir garantias não verificadas.

## Source and type
- **Source:** solicitação direta do usuário na sessão; evidências locais do repositório.
- **Type:** ciclo de assessment + planejamento/execução de baseline (governança + enablement de verificação), pré-requisito para fatias verticais de implementação subsequentes.

## Context (evidência local)
- Workspace Rust (`flora-api` entrypoint real; `flora-core` domínio/infra; crates de feature; `apps/frontend` canônico, `apps/web` não canônico). Sem `docker-compose` de dependências no checkout; `dev-seed.sql` presente.
- Specs 001-003 e assessment 003 existem; drift entre tasks/checklists/README e verificação real é reconhecido. PKCE tratado no ciclo 003 com lacunas de cobertura/evidência pontuais.
- Verificação local 2026-11-15: `cargo check -p flora-api -p flora-core`, `cargo fmt --all -- --check`, `cargo test --workspace --no-run`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54) **ok**; integração/runtime com Postgres/Valkey/Meilisearch/OIDC não verificados neste ambiente. Baseline 003 registrou clippy/test workspace verdes em seu ambiente, com frontend tests/lint parcialmente inconclusivos por limitações.
- Gaps relevantes: fanout realtime, integração indexação/notificação, contratos de arquivos, consistência auth/RBAC, quota/contabilidade e health/readiness efetivos.

## First-glance unknowns / constraints
- Execução remota de CI e disponibilidade de Docker/serviços/credenciais OIDC neste ambiente não garantidas (NV onde não executado).
- Escolha entre perfis dev (apenas infra core vs incluir mock OIDC/Zitadel) impacta esforço; decisão deve manter baixo risco e sem segredos.
- Mudanças que exigem autorização explícita (deploy/infra produção, merges/releases, segredos, serviços pagos/proprietários, RE não autorizada, reuso incompatível com AGPL) ficam fora.

## Desired outcome
- `OBJETIVO.md` e `ROADMAP.md` criados/atualizados com estado real + pesquisa.
- Assessment 004 com intake/research/decision (go) para baseline local reproduzível do Seed.
- Próximos passos concretos e verificáveis definidos para implementação subsequente.
