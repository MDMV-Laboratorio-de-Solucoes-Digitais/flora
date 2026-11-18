# Research: 067 — C6 canário RLS

**Data:** 2026-11-18T15:50Z · **Evidência local verificada**; execução em app/produção **[NV]**.

## Primária (local)

1. `flora-core/src/migrations/sql/20240818000001_init_schema.sql:196-215` —
   `ENABLE ROW LEVEL SECURITY` em 11 tabelas (users, organizations, memberships,
   roles, workspaces, channels, messages, tasks, files, notifications, sessions).
2. `...:212-215` — `CREATE POLICY org_isolation ... current_setting('app.current_organization_id')`
   está **comentado**; comentário em `:197` diz que o isolamento é feito pela app
   (`WHERE organization_id = ?`).
3. Auditoria por grep (subagentes): `0 CREATE POLICY`, `0 FORCE ROW LEVEL SECURITY`,
   `0 set_config`, `0 app.current_org*`, `0 CREATE ROLE/BYPASSRLS`, `0 .begin()`
   em `flora-core` + `.sql`.
4. `flora-api/src/extractors/auth.rs:218-225` — `require_org()` rejeita org nil e é
   a origem real do tenant; `:58-90` revalida a sessão no DB a cada request.
5. `flora-api/src/tenant.rs:26-70` — guardas BOLA reais (`ensure_workspace_in_org`,
   `ensure_user_in_org`) usadas em `tasks.rs` e `files.rs`.
6. ADR-0005: desenho RESTRICTIVE + FORCE provado em PG18 descartável; 9 tabelas com
   `organization_id` direto, `organizations` por `id`, `users` por `EXISTS` em
   `memberships`; GUC `set_config(...,true)` **não persiste em autocommit** — exige `BEGIN`.
7. ADR-0005 §Decisão/ressalva: `users` e `organizations` precisam de lookup anterior
   ao GUC (login/`jti`), portanto devem ficar **fora do escopo do canário** (ou via
   `SECURITY DEFINER` com `search_path` fixo).
8. ADR-0006: promoção em duas etapas — (a) wrapper de transação com `set_config`
   como primeira instrução a partir de `require_org()`; (b) só então policies/FORCE.
   Rollback = desativar policies/FORCE e voltar ao filtro da app; sem dado destrutivo.
9. `check_gate_integrity.py`: required checks `frontend-build`, `rust-build`,
   `db-tests`, `dependency-review`; 8 pisos / 51 total. Nenhuma flag pode condicionar
   required check (`skipped` conta como sucesso no GitHub).

## Decisão

Produzir um **desenho de canário** executável apenas em DB descartável, com
critérios de entrada/saída por fase, SQL de referência (não aplicado), plano de
rollback e os quatro itens de verificação do ADR-0006 marcados **[NV]**.

## Alternativas descartadas

- Ligar FORCE em produção: prova real mostra app quebrada sem wrapper (ADR-0005).
- Flag `if: RLS_ENABLED` em required check: gate falso (ADR-0003/0006).
- Incluir `users`/`organizations` no canário: bloquearia login/sessão (ressalva ADR-0005).
- Substituir filtros da app: perde checagem em memória e portabilidade (spec 001:382).

## Cobertura idiomática

PT-BR (artefatos) + EN (código/ADRs citados). Nenhuma fonte web usada neste ciclo.
