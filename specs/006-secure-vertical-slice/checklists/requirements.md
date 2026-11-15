# Requirements Checklist — 006 Secure Vertical Slice

## Scoping & decision
- [x] Intake e objetivo do slice registrados.
- [x] Auditorias (auth/tenancy, frontend↔backend) e pesquisa de boas práticas consolidadas.
- [x] Decision finalizada (`go`, escopo parcial executado, deferrals explícitos).

## Implementation targets
- [x] Identidade/tenant derivados de auth JWT validada nas rotas do escopo crítico (workspaces, channels/messages, tasks, org, files, search, notifications, websocket).
- [~] Fluxo núcleo e2e definido — endurecimento backend aplicado; e2e completo (org selection/token scoping + contrato frontend/smoke) **pendente/NV**.
- [~] Contrato frontend↔backend alinhado — **não** neste incremento (desalinhamento significativo documentado).

## Verification & evidence
- [x] Verificações determinísticas relevantes executadas e registradas (`fmt --check`, `check`, `test -p flora-core --lib`, `test -p flora-tests --lib`).
- [NV] Smoke e2e integrado executado — NV (ambiente/serviços/OIDC; token org-scoped ausente no fluxo).
- [x] Limitações/impactos explícitos (quebra de clientes header-based como `test-flora.sh`/frontend atual; gaps de contrato).

## Boundaries
- [x] Sem deploy/produção, segredos, serviços pagos ou expansão para P2.
