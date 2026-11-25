# Concept: Registro canônico dos escalonamentos

- **Slug**: 062-escalation-register
- **Created**: 2026-11-25
- **Recommended option**: Tabela append-only no registro de estado

## Options

### Option A — Tabela append-only no registro de estado
- **Sketch**: Os itens que ficaram bloqueados por governança passam a ter um lugar único e datado: para cada um, a evidência que o bloqueia, o que exige autorização, a ação necessária e o status atual. Ao lado, uma nota com o estado do scan OSV verificado ao vivo — incluindo o fato de ainda não haver execuções e de o pin continuar sem validação prática. Nada bloqueado é executado; o ciclo só organiza o que já se repetia em dezenas de notas.
- **Appetite**: small — um apêndice em arquivo existente.
- **Trade-offs**: Ganha registro canônico, menos reanálise a cada sessão e menor risco de alguém iniciar trabalho não autorizado. Sacrifica a atualização automática: o registro é uma fotografia datada. Risco nulo: nenhum item é disparado.
- **Rabbit holes**: Executar ADR/N3/C5/C6 "aproveitando"; disparar `workflow_dispatch` do OSV; forjar artefatos para dívidas de fase; criar outro documento e fragmentar mais o registro.

### Option B — Documento separado de escalonamentos
- **Sketch**: Um arquivo novo concentra os itens bloqueados, deixando o registro de estado como estava.
- **Appetite**: small.
- **Trade-offs**: Tema bem delimitado, mas cria uma segunda fonte que precisa ser sincronizada — exatamente o tipo de duplicação que o problema denuncia.
- **Rabbit holes**: Manter as duas listas coerentes; decidir qual é a autoritativa.

### Option C — Não fazer
- **Sketch**: Continuar reavaliando os mesmos quatro itens em cada sessão, espalhados pelas notas existentes.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Sem custo de escrita, mas gasta orçamento de análise toda vez e mantém o risco de um ciclo futuro agir sem autorização.
- **Rabbit holes**: — (nenhum; o padrão se repete).

## Recommendation

**Option A**. O problema é de registro, não de execução: FR-001 pede a tabela com os quatro itens, FR-002 o estado OSV verificado ao vivo e FR-003 proíbe executar qualquer item bloqueado. O research.md já traz os valores do `gh api` (workflow ativo, 0 runs, cron semanal, pin ainda `[NV]`), o que permite satisfazer SC-002 sem efeito remoto. Apêndice em arquivo existente preserva o histórico e mantém gates verdes (SC-003/SC-004).

## Out of Scope (for the recommended option)

- Alterar ADRs, migração N3, C5/C6, constituição ou CI.
- Disparar o workflow OSV ou qualquer ação remota.
- Reescrever notas históricas de `STATE.md`.

## Assumptions to Validate

- Os quatro itens (ADR-0003/0006, N3, C5, C6) continuam sendo os únicos bloqueados; confirmar antes de datar a tabela.
- Os metadados públicos do workflow obtidos por `gh api` são suficientes; nenhuma credencial é copiada.
- A aprovação de maintainer/SE/dono e a futura execução do OSV permanecem **[NV]**.
