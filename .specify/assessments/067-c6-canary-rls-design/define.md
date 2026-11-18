# Define: 067 — C6 canário RLS

## Problema

O isolamento multi-tenant real depende apenas do filtro `WHERE organization_id = ?`
da aplicação. RLS está "ligado" em 11 tabelas, porém inerte: sem policies, sem GUC
por request, sem papéis de DB e sem transação. A promoção exige um desenho de
canário com rollback verificável antes de qualquer mudança em produção.

## Requisitos funcionais

- **FR-001**: O desenho DEVE registrar o estado verificado com `file:line` e
  distinguir "ENABLE RLS" de "enforcement ativo".
- **FR-002**: O desenho DEVE definir fases de canário com critérios explícitos de
  entrada/saída, restritas a DB descartável/não-produção.
- **FR-003**: O desenho DEVE especificar policies `RESTRICTIVE` + `FORCE` para as
  9 tabelas com `organization_id` direto, com expressão fail-closed
  (`NULLIF(current_setting(...,true),'')::uuid`), sem fallback permissivo.
- **FR-004**: O desenho DEVE registrar as ressalvas de `users`/`organizations` e
  mantê-las fora do canário; `BYPASSRLS` proibido na role da app.
- **FR-005**: O desenho DEVE conter rollback passo a passo, sem ação destrutiva de
  dados, com verificação de que as queries voltam a retornar linhas.
- **FR-006**: O desenho DEVE listar os itens (a)–(d) de verificação do ADR-0006 como
  **[NV]** até execução real.
- **FR-007**: O refresh de docs DEVE registrar contagens obtidas por `find`/scripts,
  preservar append-only e marcar **[NV]** toda afirmação remota não observada.

## Critérios de sucesso

- **SC-001**: `check_gate_integrity.py` PASS (8 pisos / 51) e CI verde antes do merge.
- **SC-002**: nenhuma alteração em código, workflows, migrações, ADRs ou constituição.
- **SC-003**: documento contém fases, SQL de referência, rollback, `[NV]` e cita ADR-0005/0006.
- **SC-004**: contagens de `PROGRESSO.md`/`research/index.md` conferem com `find`;
  histórico preservado.

## Não-objetivos

- Não habilitar RLS em produção, não criar policies/FORCE em runtime.
- Não alterar required checks, proteção de branch, credenciais, deploy ou tags.
- Não substituir os filtros da aplicação por RLS.
- Não forjar artefatos de fases passadas nem reescrever `converge.md` alheio.
