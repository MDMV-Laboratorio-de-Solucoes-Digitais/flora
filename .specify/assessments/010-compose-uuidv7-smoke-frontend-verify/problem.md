# Problem: 010

- **Documented dev/smoke path quebrado em fresh install:** `docker-compose.yml` usava `postgres:16-alpine`, mas a migração inicial exige `uuidv7()` (nativo no PostgreSQL 18), causando falha de migração/startup.
- **Evidência de smoke integrada incompleta no caminho documentado:** ciclo 009 comprovou fluxo em stack alternativa PG18; faltava reconciliar compose/docs e evidenciar caminho compatível.
- **Ambiente local com restrição:** porta host `5432` ocupada externamente impede bind literal padrão (NV ambiental), exigindo caminho equivalente documentado/parametrizado.
- **Verificação frontend:** configuração de testes server não cobria explicitamente `tests/**` no projeto server conforme esperado para execução determinística alinhada; checks precisavam ser executados/registrados para o slice atual.
