# Concept: Inventário de bugs com status verificado

- **Slug**: 113-bug-status
- **Created**: 2026-11-25
- **Recommended option**: Fechamento documental com evidência local

## Options

### Option A — Fechamento documental com evidência local
- **Sketch**: Os registros de bugs deixam de apresentar como pendentes correções que já foram entregues e verificadas localmente. Cada um ganha uma nota datada que liga o bug à evidência que o sustenta, ao status resultante e ao que ainda não foi observado. Nada do que foi escrito antes é apagado: o histórico continua íntegro e a priorização do próximo ciclo passa a usar um inventário mais honesto.
- **Appetite**: small (dias); incerteza: fechar documentalmente não prova comportamento em produção, portanto o resultado carrega limites explícitos.
- **Trade-offs**: Desinfla a dívida aparente e torna rastreável por que cada bug foi considerado resolvido, sem mexer em código ou governança. O risco é tratar evidência local como prova remota; exige disciplina para manter o histórico append-only e para não transformar a tabela de status em reescrita disfarçada.
- **Rabbit holes**: Tentar confirmar produção com credenciais não autorizadas; reconciliar números históricos alterando notas antigas; promover PRs ainda na fila; perseguir revisão manual de critérios WCAG dentro deste ciclo; automatizar verificação de staging.

### Option B — Manter o inventário como está
- **Sketch**: Nenhum registro muda: os três bugs continuam listados como propostos ou planejados, e quem prioriza segue decidindo com base nesse quadro.
- **Appetite**: small (dias).
- **Trade-offs**: Custo zero e nenhum risco de fechamento indevido, mas mantém dívida stale desviando o próximo 80/20 para trabalho redundante e sem ligação entre status e código já mesclado.
- **Rabbit holes**: Usar contagem de bugs abertos como métrica de dívida real; tratar os registros antigos como fonte definitiva de prioridade.

### Option C — Fechar só após verificação remota
- **Sketch**: O fechamento esperaria observação em ambiente real — produção ou staging — com acesso e autorização adequados, incluindo a confirmação de comportamento com provedor de identidade externo.
- **Appetite**: large (meses); incerteza alta: depende de ambiente, credenciais e autorização SE 023 para deploy, todos fora do mandato registrado.
- **Trade-offs**: Reduziria o risco de fechar algo que ainda falha fora do ambiente local, mas troca um trabalho documental curto por dependência de infraestrutura e autorização, com custo e prazo imprevisíveis.
- **Rabbit holes**: Preparar staging efêmero; integrar com IdP externo; obter autorização de deploy em `main`; reabrir decisões de required checks no processo.

## Recommendation

Recomendo a **Option A**. O próprio critério 80/20 do ciclo registrou 375/625 para o fechamento documental, contra 144/625 para automação de staging efêmero, e a evidência local é determinística: dois testes para D15, rota e teste de fallback para OIDC, PR mesclado e log axe final para WCAG-111. O alvo é 3/3 bugs rastreados a evidência, nenhuma regra/gate/check alterado e **[NV]** explícito para produção, IdP externo e staging. Vale registrar a ressalva de que "axe sem achados" não significa conformidade AA plena: a cobertura média da automação é cerca de 57%, com resultados incertos exigindo revisão humana.

## Out of Scope (for the recommended option)

- Mudança de código, deploy, credenciais e required checks.
- Afirmar resolução em produção, staging real ou IdP externo sem observação.
- Reescrever notas append-only ou reconstruir o histórico dos artefatos de bug.
- Promover PRs ainda na fila SWE neste ciclo.
- Automação de staging efêmero e auditoria WCAG completa.
- Tratar números históricos do guarda como estado corrente sem distinção.

## Assumptions to Validate

- Os testes e registros citados como evidência local correspondem ao código efetivamente mesclado.
- O PR citado para o bug WCAG foi de fato mesclado na branch de integração.
- Notas datadas acrescentadas aos artefatos preservam o histórico append-only.
- O limite **[NV]** para produção é aceito como parte do fechamento documental.
- A distinção entre contagem histórica do guarda e estado atual será mantida sem reescrever notas antigas.
- A revisão manual de critérios WCAG não automatizáveis será tratada em decisão separada.
