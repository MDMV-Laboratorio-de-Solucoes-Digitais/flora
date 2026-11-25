# Concept: Validar o scan OSV em execução real

- **Slug**: 063-validate-osv-run
- **Created**: 2026-11-25
- **Recommended option**: Disparar o gatilho já declarado e registrar o resultado

## Options

### Option A — Disparar o gatilho já declarado e registrar o resultado
- **Sketch**: Usar o próprio gatilho manual que o workflow já expõe para provar que o pin funciona, acompanhar a execução até o fim e colar no registro de estado o identificador, o status, o desfecho e o timestamp. Se falhar, a falha é registrada como ela é — sem esconder e sem consertar o workflow no mesmo movimento.
- **Appetite**: small — uma ação remota já autorizada pelo próprio workflow e um apêndice datado.
- **Trade-offs**: Ganha evidência real e fecha o `[NV]` aberto no ciclo 052. Sacrifica a quietude do repositório: há um run e um upload de relatório, que são comportamento normal do workflow. Risco controlado: nenhuma credencial é manuseada e nenhuma alteração de código entra no meio.
- **Rabbit holes**: Múltiplos disparos (ruído em security-events); editar o workflow sem evidência de defeito; disparar outros workflows ou mexer em branches/PRs remotos.

### Option B — Esperar a execução agendada
- **Sketch**: Não disparar nada; deixar o cron semanal gerar a primeira execução e observá-la quando ela acontecer.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Nenhum efeito remoto imediato, mas o `[NV]` fica aberto por dias e a validação do pin continua pendente.
- **Rabbit holes**: — (nenhum; apenas se espera).

### Option C — Não fazer / manter validação estática
- **Sketch**: Contente-se com a checagem estática do pin feita em 052 e siga sem execução real.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Nenhum efeito remoto imediato, mas o `[NV]` fica aberto por dias e a validação do pin continua pendente.
- **Rabbit holes**: — (nenhum; a incerteza permanece).

## Recommendation

**Option A**. O `define.md` coloca como metas fechar a lacuna de evidência do pin e registrar o resultado determinístico do run, preservando o sinal advisory de supply-chain sem alterar comportamento. O gatilho `workflow_dispatch` é parte do próprio workflow (decision.md), então não é necessária autorização adicional. É a opção que produz a evidência faltante, registra o desfecho inclusive se falhar e mantém o diff sem código, conforme as métricas de sucesso.

## Out of Scope (for the recommended option)

- Alterar o workflow, o pin ou qualquer configuração se a execução falhar (ciclo separado).
- Disparar outros workflows ou tocar branches/PRs remotos.
- Modificar código, ADRs, constituição ou gates.

## Assumptions to Validate

- O gatilho `workflow_dispatch` continua disponível e o run é observável até conclusão; o resultado antes de obtido é **[NV]**.
- O upload de SARIF é comportamento esperado do workflow, não efeito colateral indevido.
- O registro em `STATE.md` é append-only e não reescreve notas anteriores.
