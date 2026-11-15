# Concept: 010 (80/20)

- **Solução mínima segura:** alinhar `docker-compose.yml` e `docs/dev-smoke.md` para PostgreSQL 18+ (nativo `uuidv7`) e ajustar mount de volume para layout PG18, **sem alterar migrações `sqlx` aplicadas** (evita quebra de checksum/histórico). Corrigir bootstrap token documentado para o secret do `.env.example`.
- **Smoke evidenciado apesar de NV de porta:** usar stack compose PG18 isolada com override de portas host quando `5432` estiver ocupado, mantendo mesmo fluxo (migrate/seed/bootstrap/`test-flora.sh` select-org→workspace→task) e registrar desvio como NV ambiental.
- **Frontend determinístico:** incluir `tests/**/*` no projeto Vitest `server` e excluir `tests/e2e/**`; executar check/lint/test:unit/build e registrar resultados.
- **Entrega:** artefatos SDD (assessment/spec/plan/tasks/checklists/validation/release), branch/PR para `dev`, CI verde, merge e sync mirror.
