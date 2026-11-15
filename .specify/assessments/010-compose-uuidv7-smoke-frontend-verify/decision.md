# Decision: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Date:** 2026-11-15  
**Verdict:** **go**

## Rationale
- A incompatibilidade `postgres:16-alpine` + `DEFAULT uuidv7()` (init migration) quebrava o caminho dev/smoke documentado para fresh installs; alinhar compose/docs para **PostgreSQL 18** (nativo `uuidv7`) é a correção 80/20 de menor risco e consistente com schema/CI (`db-tests` já em PG18), sem editar migrações `sqlx` aplicadas.
- Evidenciar o fluxo `select org → workspace → task` no caminho documentado/compatível e registrar NVs ambientais (porta 5432 ocupada, volumes PG16 existentes) aumenta reprodutibilidade para PMEs/VPS.
- Ajustes mínimos de verificação determinística frontend (config de testes/checks) e execução dos gates viáveis fortalecem o slice auth→org→workspace→task entregue no ciclo 009.

## Scope (010)
1. **Compose/docs `uuidv7`:** `docker-compose.yml` → `postgres:18-alpine`; `docs/dev-smoke.md` atualizado (requisito PG18, nota de volume PG16→18, conflito de portas, correção de nota de chave Meilisearch/consistência operacional).
2. **Smoke integrado:** executar e evidenciar `select org → workspace → task` no caminho compatível com a documentação (PG18, compose/stack equivalente). Se porta padrão 5432 estiver ocupada externamente, usar mapeamento/override isolado e registrar NV/desvio ambiental explícito.
3. **Frontend determinístico:** incluir testes `tests/**` no projeto Vitest `server` (alinhamento com CI/local), executar checks viáveis (`bun run check`, `lint`, `test:unit -- --run --project server`, `build`) e corrigir falhas mínimas/in-scope quando necessário; registrar NV se houver bloqueio.

## Non-goals
- Editar migração `sqlx` já aplicada; implementar polyfill/extensão `uuidv7` para PG16 no runtime (avaliado, maior risco/complexidade para fresh init); OIDC/JWKS e2e real; expansão ampla de produto/UI; CI remoto além do PR.

## NV / constraints
- Porta host `5432` pode estar ocupada por processo externo (não descartável): execução literal na porta padrão pode ser NV; evidência será fornecida com stack isolada/portas alternativas equivalentes quando necessário.
- Volumes locais previamente criados em PG16 exigem reset para uso com PG18 (ambiente descartável) — não automatizado.
- Sucesso de CI remoto do PR a confirmar após abertura.
