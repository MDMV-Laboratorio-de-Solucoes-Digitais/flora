# ADR-0005 — Multi-tenant RLS, fail-closed, derived from the verified session

**Date:** 2026-11-17
**Status:** Accepted (design) / Enforcement EVOLUTIVE — not enabled at runtime
**Scope:** Flora `flora` repository — tenancy / PostgreSQL

## Context

Eleven tables have `ENABLE ROW LEVEL SECURITY` with no policies
(`20240818000001_init_schema.sql:200-215`); the GUC appears only in a comment. The app
connects as superuser/owner (ADR-0004), so RLS is inert. Real isolation today is the
app-level `WHERE organization_id = ?`, with the org coming exclusively from the verified
session (`AuthExtractor::require_org()`, `flora-api/src/extractors/auth.rs:218-226`;
`x-workspace-id` is an unverified header, `:180-184`).

Prova executada em PG18 descartável (`/tmp/opencode/dark-factory-80-20/rls-10-executed-proof.md`):
- `users` **não tem `organization_id`** — tenancy é via `memberships(user_id, organization_id)`.
  9 tabelas têm a coluna direta; `organizations` usa `id`; `users` exige `EXISTS`.
- `set_config('app.current_organization_id', $org, true)` **não persiste entre statements
  em autocommit** — exige `BEGIN` explícito. A app não tem nenhuma transação hoje
  (116 usos de `db_pool`, zero `.begin()`).
- Revisor adversarial: GUC é confiança do chamador; **RLS é defesa em profundidade,
  não a fronteira de segurança** — o filtro da app continua primário (spec 001:382).

## Decision

1. **Design das policies** (provado no DB descartável):
   - 9 tabelas com `organization_id` direta (channels, files, memberships, messages,
     notifications, roles, sessions, tasks, workspaces):
     `CREATE POLICY flora_tenant_isolation ... AS RESTRICTIVE FOR ALL TO flora_app
     USING (organization_id = NULLIF(current_setting('app.current_organization_id',true),'')::uuid)
     WITH CHECK (mesma expressão)` + `FORCE ROW LEVEL SECURITY`.
   - `organizations`: `id = GUC`. `users`: `EXISTS (SELECT 1 FROM memberships m WHERE
     m.user_id = users.id AND m.organization_id = GUC)` — sem recursão. **Ressalva de
     rollout** (seed-13): o carregamento da sessão por `jti` e o login precisam de lookup
     anterior ao GUC; por isso, no desenho final de promoção, `users` e `organizations`
     devem ser tratados fora do escopo RLS (ou com `SECURITY DEFINER` de sessão com
     `search_path` fixo), evitando que a própria autenticação seja bloqueada pelo
     fail-closed. A prova em DB descartável validou a forma `EXISTS`; a decisão de
     exclusão no login fica registrada como requisito do wrapper de transação.
   - **Fail-closed**: GUC ausente/inválido → `NULL` → nenhuma linha; nunca fallback
     permissivo; sem role com `BYPASSRLS`.
2. **Contrato do GUC**: derivar **apenas** de `require_org()` (sessão verificada);
   `set_config(..., true)` como primeira instrução de uma transação explícita, reassertir
   antes do commit. `SET`/`set_config(...,false)` são proibidos (vazam no pool).
3. **Enforcement é EVOLUTIVO**: as policies existem e são testadas no DB descartável,
   mas **não são ativadas no runtime** enquanto não houver wrapper de transação por
   request. Ligar `FORCE` na app hoje zeraria as queries (fail-closed sem GUC ativo).
   A promoção é o ADR-0006 + uma mudança de código explícita, não um flip de config.
4. Filtros da app permanecem; `organization_id` é imutável por policy (WITH CHECK);
   `SECURITY DEFINER` desautorizado por padrão; backup usa role com `BYPASSRLS` dedicada.

## Considered options

| Option | Why rejected |
|---|---|
| `FORCE` sem policies | Default-deny → zero linhas; provado no DB descartável. |
| `current_setting(...,true)` com fallback "ver tudo" | Fail-open silencioso; viola OWASP Deny by Default. |
| GUC por header `X-Organization-ID` | Header não verificado; drift já eliminado no 043. |
| RLS substituir filtros da app | Perde checagem em memória e portabilidade (spec 001:382). |
| Ativar enforcement agora | Sem transação por request, GUC não persiste → app quebrada (prova real). |

## Consequences

- Positive: desenho validado com evidência executada; isolamento em camadas pronto para
  ser ligado quando o wrapper existir; risco de GUC stale documentado e mitigado.
- Negative: complexidade adicional (transação por request) antes da promoção; duas
  modalidades a manter até adoção; GUC continua confiança da app — SQL injection
  continua fora do escopo da policy (parametrização é pré-requisito).
- Drift: ADR-0001 §4 — capability "tenancy" passa a ter backend Postgres + app.

## Verification

Provado em DB descartável: 11/11 tabelas ENABLE+FORCE com policy RESTRICTIVE;
fail-closed sem GUC → 0 linhas; dono = `flora_owner`; app sem BYPASSRLS.
**NV** até execução na app: isolamento A/B via requests reais, `WITH CHECK` sob GUC
ativo em transação, round-trip backup/restore com `row_security=off`.

## Relationship

Depends on ADR-0004. Rollout governado por ADR-0006.
