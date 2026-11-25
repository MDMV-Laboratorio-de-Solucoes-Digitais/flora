# Concept: Listagem de papéis realmente tenant-safe

- **Slug**: 042-real-rbac-list
- **Created**: 2026-11-25
- **Recommended option**: Listagem real derivada da sessão

## Options

### Option A — Listagem real derivada da sessão
- **Sketch**: Substituir a listagem de papéis que hoje devolve um placeholder por uma listagem da organização da própria sessão verificada, com uma regra de permissão simples e testável e um conjunto de campos acordado, sem vazar campos não combinados. A lista de uma organização sem papéis continua vazia, e as divergências de status HTTP registradas na auditoria são corrigidas.
- **Appetite**: medium (incerteza: o comportamento com banco real segue sem observação [NV]).
- **Trade-offs**: Entrega o primeiro comportamento RBAC verificável do Seed sem depender de header ou path e sem vazamento de campos. Em troca, cobre apenas a leitura: escrita e decisões de Owner ficam de fora, e o isolamento continua dependente do código da aplicação, já que RLS não está ativo.
- **Rabbit holes**: Implementar `create`/`get`/`assign`/`revoke` e a proteção de último Owner — decisão de dono adiada; adicionar teste de isolamento com banco real com helper module-private; ampliar o aceite de papéis sem evidência de que clientes esperam isso.

### Option B — Só a regra pura de autorização (menor coisa que poderia funcionar)
- **Sketch**: Isolar e testar a decisão de "pode listar ou não" sem alterar o handler placeholder. Entrega a parte de autorização com a menor mudança.
- **Appetite**: small (incerteza: baixa, desde que a regra já exista no recorte atual).
- **Trade-offs**: Cobre a decisão de gate isoladamente e com testes determinísticos, mas não entrega dados reais nem corrige os defeitos de status/payload apontados na auditoria.
- **Rabbit holes**: Confundir testes verdes com o payload real — foi exatamente o ponto cego que deixou todos os usuários semeados, inclusive Admin, receberem 403.

### Option C — Não fazer / manter o placeholder
- **Sketch**: Manter a listagem vazia e registrar a pendência; não tomar decisão de autorização neste ciclo.
- **Appetite**: none (sem custo de implementação; o custo é a capacidade bloqueada).
- **Trade-offs**: Zero risco de alterar autorização, mas o Seed segue sem nenhum comportamento RBAC verificável e os documentos continuam citando uma fonte de tenancy obsoleta.
- **Rabbit holes**: Aceitar `X-Organization-ID` como fonte de tenancy para "resolver" o problema sem sessão verificada.

## Recommendation

Recomendo a **Option A**. Ela responde diretamente às metas de `problem.md`: listar papéis reais da organização com tenancy vinda apenas da sessão, exigir associação e permissão por regra pura, evitar vazamento de campos e corrigir as divergências de status HTTP. A recomendação não inclui operações de escrita nem RLS, conforme o recorte aprovado, e depende de validar o payload real dos usuários semeados — não apenas os testes unitários.

## Out of Scope (for the recommended option)

- RBAC completo em todas as rotas; `PermissionService` no `AppState`.
- RLS/role least-privilege, constituição, ADR, required checks, deploy, tag e release.
- Semântica de `create`/`get/{id}`/`assign`/`revoke` e proteção de último Owner.
- Afirmar que mais de um endpoint é real neste recorte.
- Aceitar `X-Organization-ID` como fonte de tenancy.

## Assumptions to Validate

- Quem decide a semântica das operações de escrita e a proteção de último Owner, diferidas em c041-02.
- Se algum cliente depende de `organization_id` ou timestamps hoje omitidos do DTO.
- Como a checagem de associação se comporta em sessão expirada ou após troca de organização.
- Se o retorno vazio para organizações sem papéis é contrato estável para todos os clientes atuais.
- Se um teste de isolamento com banco real será adotado, dado o helper module-private e o comportamento live [NV].
- Quais papéis, além de `OrgRead`/`OrgAdmin` e `*`, a checagem deve aceitar.
