# Decision: 064 — N3 org-scoped roles

**Verdict:** go (scoped — autorização SE explícita em STATE 11:13Z)

## Decisão

Implementar org-scoping de resolução de papéis (camada 1), adicionar
`UNIQUE(organization_id, id)` e a FK composta com `ON DELETE RESTRICT`, com
pré-voo fail-closed. Auditoria local limpa; produção **[NV]** — sem deploy.

## Delimitadores

- Código em `flora-core`/`flora-api`, 2 migrations, testes e artefatos.
- Nenhum dado alterado, nenhum deploy, nenhuma limpeza, nenhum ADR/C5/C6.

## NV

Dados de produção e aprovação de deploy: **[NV]** — SE autorizou apenas o
código/migrations; deploy depende de auditoria em produção.
