# C6 — Desenho de canário RLS com rollback (sem habilitação em produção)

**Data:** 2026-11-18 · **Ciclo:** 067 · **Tipo:** desenho operacional, sem mudança de runtime
**Baseline:** `github/dev@9d5f6656f` · **Autoridade:** SE (STATE 13:04Z / 14:38Z / 14:50Z)

> **Conclusão primeiro.** O canário é **viável como desenho** e **desejável como
> próxima barreira de defesa**, mas **não pode ser ligado hoje**: a aplicação não
> tem transação por request, logo o GUC `app.current_organization_id` não persiste
> e o fail-closed zeraria as queries (prova registrada no ADR-0005). Este documento
> descreve (1) o estado verificado, (2) a pré-condição de código do ADR-0006,
> (3) quatro fases de canário em DB descartável, (4) o SQL de referência,
> (5) a verificação exigida e (6) o rollback. **Nada aqui foi aplicado.** A promoção
> para qualquer ambiente não-descartável exige autorização explícita do SE
> (ADR-0006 §3). Toda execução futura é **[NV]** até evidência colada.

---

## 1. Escopo, autoridade e fronteiras

- **Autorizado:** desenho, documentação e artefatos SDD.
- **Não autorizado / fora deste ciclo:** policies ou `FORCE` em runtime, wrapper de
  transação em código, required checks, proteção de branch, credenciais, deploy,
  tag/release, produção. Deploy e troca de credenciais continuam decisão explícita
  do SE (ADR-0006 §3; matriz de autorização 036).
- **Código/workflows/migrações/ADRs/constituição:** nenhuma alteração.
- **Factory:** lida como fonte (HEAD `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`),
  nunca modificada; nenhum artefato dela transplantado como autoridade.

## 2. Estado verificado (evidência local)

| Fato | Evidência |
|---|---|
| 11 tabelas com `ENABLE ROW LEVEL SECURITY` | `flora-core/src/migrations/sql/20240818000001_init_schema.sql:196-215` |
| **0** `CREATE POLICY` ativo; exemplo comentado em `:212-215` | auditoria por grep + ADR-0005 §Contexto |
| **0** `FORCE ROW LEVEL SECURITY`, **0** `set_config`, **0** `CREATE ROLE/BYPASSRLS` | auditoria por grep em `flora-core/src`, `flora-api/src` e `*.sql` |
| **0** `app.current_org*` **ativos** (1 ocorrência comentada em `init_schema.sql:215`) | auditoria por grep; ADR-0005:10 registra "appears only in a comment" |
| **0** `.begin()` | auditoria por grep em `flora-api/src` |
| Isolamento real = filtro da app, tenant da sessão verificada | `flora-api/src/extractors/auth.rs:218-225` (`require_org()`), `:58-90` (revalidação DB) |
| Guardas BOLA adicionais | `flora-api/src/tenant.rs:26-70`, usadas em `tasks.rs`/`files.rs` |
| Gate autoritativo | `scripts/ci/check_gate_integrity.py` — 4 required checks, 8 pisos / 51 total; `--remote` confirma `dev`/`main` |
| Prova em PG18 descartável | ADR-0005 — **executada fora deste checkout**; **[NV]** localmente |

**Interpretação:** "RLS enable" é configuração latente. Sem policy, o comportamento
padrão do PostgreSQL em tabelas com RLS habilitado e sem policy é **negar** — mas a
app conecta como owner/owner-like, que bypassa RLS por padrão. Portanto hoje RLS
não protege nem quebra nada; é inerte. O filtro da aplicação continua sendo a
fronteira de segurança primária.

## 3. Modelo de ameaça e fronteira de confiança

- **O que RLS protegeria aqui:** erros de omissão de `WHERE organization_id` em
  queries futuras, acesso direto ao DB por ferramenta com a role da app, e
  escalonamento cross-tenant por bug em middleware.
- **O que RLS NÃO protege:** SQL injection (parametrização é pré-requisito),
  chamador que controla o GUC (ele é confiança da app), admin com `BYPASSRLS`,
  dono da tabela, nem a decisão de autorização em si.
- **GUC é derivado de `require_org()`** (sessão verificada); nunca de header
  `X-Organization-ID` (drift já eliminado no ciclo 043). `set_config(...,true)` é
  local à transação; `SET`/`set_config(...,false)` são proibidos porque vazam no pool.

## 4. Pré-condição (ADR-0006 etapa a) — descrita, NÃO implementada

Antes de qualquer policy em runtime, a aplicação precisa de um wrapper que:

1. abra `BEGIN` explícito no início do request autenticado;
2. execute `SELECT set_config('app.current_organization_id', $org::text, true)`
   como **primeira instrução** antes de qualquer query tenant-scoped;
3. reafirme o GUC antes do `COMMIT` (defesa contra reuso de conexão no pool);
4. jamais deixe o GUC definido fora da transação (`is_local = true`);
5. trate `users`/`organizations` fora do escopo RLS (login e lookup por `jti`
   ocorrem antes do GUC) ou use `SECURITY DEFINER` com `search_path` fixo.

**Estado:** não implementado. `grep` não encontra `.begin()` em `flora-api`.
**[NV]** qualquer alegação de que handlers já chamam o wrapper.

## 5. Fases do canário (somente DB descartável)

| Fase | Objetivo | Critério de entrada | Critério de saída |
|---|---|---|---|
| **0 — Pré-condição** | wrapper de transação implementado e testado | SE autoriza código (fora deste ciclo) | `set_config(...,true)` chamado por todos os handlers tenant-scoped; suíte `app_only` verde com piso 51 |
| **1 — DB descartável** | provar que as policies filtram | Fase 0 concluída; snapshot do DB | 9/9 tabelas do escopo operacional ENABLE+FORCE; cross-org = 0 linhas; role da app sem `BYPASSRLS`. A prova histórica 11/11 do ADR-0005 é **[NV]** neste checkout |
| **2 — Canário controlado** | observar latência e plano de consulta | Fase 1 verde; carga sintética | p95 de queries dentro do SLO; nenhuma query sem GUC retornando linha; rollback ensaiado com sucesso |
| **3 — Mutation test** | provar que a policy é o que filtra | Fase 2 verde | remover policy ⇒ teste cross-org **falha** (prova de que o teste não é vácuo); restaure e repita |
| **4 — Promoção** | aplicar em ambiente real | **autorização explícita do SE** + ADR de drift se mecanismo mudar | **[NV]** — fora deste ciclo |

**Regra de parada:** qualquer fase com query retornando linha cross-tenant, ou
qualquer required check `skipped`, interrompe o canário e dispara o rollback (§8).

## 6. SQL de referência (desenho — NÃO aplicado)

```sql
-- Papéis (fora do runtime atual): app sem BYPASSRLS; owner separado para DDL/backup.
-- CREATE ROLE flora_app LOGIN;              -- sem BYPASSRLS
-- CREATE ROLE flora_owner LOGIN;            -- dono do schema; backup usa row_security=off

-- Para as 9 tabelas com organization_id direto:
-- channels, files, memberships, messages, notifications,
-- roles, sessions, tasks, workspaces
CREATE POLICY flora_tenant_isolation ON <tabela>
    AS RESTRICTIVE FOR ALL TO flora_app
    USING (organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::uuid)
    WITH CHECK (organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::uuid);
ALTER TABLE <tabela> FORCE ROW LEVEL SECURITY;

-- FORA do canário operacional (Fases 1-3): referência do ADR-0005 para a Fase 4.
-- Incluir organizations exigiria SECURITY DEFINER com search_path fixo + SE.
-- CREATE POLICY flora_tenant_isolation ON organizations
--     AS RESTRICTIVE FOR ALL TO flora_app
--     USING (id = NULLIF(current_setting('app.current_organization_id', true), '')::uuid)
--     WITH CHECK (id = NULLIF(current_setting('app.current_organization_id', true), '')::uuid);

-- users: sem coluna direta; forma provada no DB descartável (FORA do canário, ver §4, item 5)
-- USING (EXISTS (SELECT 1 FROM memberships m
--          WHERE m.user_id = users.id
--            AND m.organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::uuid))
```

**Fail-closed:** GUC ausente/vazio ⇒ `NULLIF(...,'')::uuid` ⇒ `NULL` ⇒ nenhuma
linha. Para uma string não-vazia e inválida, o cast `::uuid` levanta
`invalid input syntax for type uuid` — ainda é fail-closed (nega por erro), mas
**não** "zero linhas"; a distinção é herdada do ADR-0005:41-42 e registrada aqui.
Não há cláusula `OR true`, nem fallback permissivo, nem `BYPASSRLS` na role da
app. `WITH CHECK` impede que uma linha seja escrita em outra org.

## 7. Verificação exigida (ADR-0006 §Verification)

| Item | O que provar | Estado |
|---|---|---|
| (a) wrapper com `set_config` realmente usado pelos handlers | grep + teste de integração | **[NV]** — não implementado |
| (b) suítes `app_only` e `enforced` verdes com piso 51 | contagem dos legs `db-tests` | **[NV]** — suítes ainda não criadas |
| (c) rollup sem `skipped` em required check | checks do PR ao vivo | Observado ao vivo nesta sessão: PR #285 com 6 checks `pass` nos required (sem `skipped`); reverificação pelo verificador isolado e suítes futuras **[NV]** |
| (d) mutation test cross-org falhando sem a policy | remover policy ⇒ teste falha | **[NV]** — Fase 3 |

Complemento: backup/restore com `row_security=off` e role de backup dedicada —
**[NV]** até round-trip executado.

## 8. Rollback (sem dado destrutivo)

1. **Parar** a carga do canário e registrar o motivo (query cross-org, latência, etc.).
2. `ALTER TABLE <tabela> NO FORCE ROW LEVEL SECURITY;` nas tabelas do canário.
3. `DROP POLICY flora_tenant_isolation ON <tabela>;` (ou desativar equivalente).
4. **Não remover** os filtros `WHERE organization_id = ?` da app — eles nunca saíram.
5. Se criados, remover grants de `flora_app`/`flora_owner`; manter o wrapper de
   transação (inofensivo sem policies) ou revertê-lo em commit separado.
6. **Verificar:** executar as mesmas queries com GUC válido e sem GUC; confirmar
   contagens idênticas ao baseline pré-canário; anexar saídas a `validate.md`.

Nenhuma `DELETE`, `UPDATE` ou `TRUNCATE` faz parte do rollback. Se um snapshot foi
tirado na Fase 1, ele é mantido como evidência, não restaurado cegamente.

## 9. Riscos e mitigações

| Risco | Mitigação |
|---|---|
| GUC vaza no pool (`is_local=false`) | só `set_config(...,true)`; reassert antes do commit |
| Autenticação bloqueada pelo fail-closed | `users`/`organizations` fora do canário operacional (Fases 1–3, §4, item 5); inclusão na Fase 4 exige `SECURITY DEFINER` + SE |
| RLS substitui (e enfraquece) a app | filtros permanecem; RLS é defesa em profundidade |
| Flag condiciona required check | proibido por ADR-0003/0006; ambas as suítes rodam sempre |
| `BYPASSRLS` acidental | role da app sem o atributo; owner separado para DDL/backup |
| Backup incompleto | role de backup dedicada + `row_security=off` (round-trip **[NV]**) |

## 10. Gate de decisão para promoção

1. Existe necessidade que o filtro da app não resolve (omissão de `WHERE` já
   observada, acesso direto ao DB, requisito de auditoria)? Se **não** → manter
   app-only e arquivar o desenho.
2. Fases 0–3 verdes com evidência colada? Se **não** → repetir/rollback.
3. SE autorizou explicitamente deploy, credenciais e `(b)` suítes required? Se
   **não** → **parar**. Este documento não é autorização de promoção.

## 11. Idiomas, fontes e NV

- **Idiomas:** PT-BR (artefatos) + EN (código, ADRs e nomes de checks).
- **Fontes locais primárias:** ADR-0005, ADR-0006, ADR-0003, migração init,
  `extractors/auth.rs`, `tenant.rs`, `check_gate_integrity.py`, auditoria por grep.
- **[NV]:** prova em PG18 fora deste checkout; execução de Fases 0–3 em app;
  itens (a)–(d); CI após o merge; produção; backup/restore; semântica do runner
  GitHub e estado Forgejo.
- Nenhuma credencial foi copiada. Conteúdo de web/issues/skills foi tratado como
  dado não-confiável; nenhuma instrução incorporada foi seguida.
