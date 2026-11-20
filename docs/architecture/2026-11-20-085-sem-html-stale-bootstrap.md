# Nota de arquitetura — 2026-11-20 (ciclo 085)

- Não há HTML stale em `docs/`: a busca por `*.html`/`*.htm` não retornou arquivos. Não existe, portanto, diagrama ou página HTML de API para regenerar neste ciclo.
- A nova rota pública `POST /api/v1/bootstrap` está documentada no contrato `specs/085-local-first-seed/contracts/bootstrap.md` (request, respostas 201/409/422/429/500, atomicidade, concorrência, auditoria e a exceção FR-012). Nenhuma duplicação de contrato foi criada em `docs/architecture/`.
- Evidência local do ciclo 085: gate de integridade PASS e E2E real `POST /api/v1/bootstrap` → `201`, segunda chamada → `409`, `GET /api/v1/tasks` com token → `200`. Contagens por `find`: `specs/` 75, assessments 72, validations 70, releases 70, bugs 4, research 137 + `index.md`.
- **[NV]** produção, CI futuro e Forgejo: estado local, não afirmação sobre execução remota.
