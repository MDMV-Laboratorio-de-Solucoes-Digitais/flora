# Concept: Integridade do Seed fechada

- **Slug**: 029-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Fechamento da integridade

## Options

### Option A — Fechamento da integridade
- **Sketch**: O lint restrito passa a cobrir também os testes, com as poucas exceções necessárias declaradas de forma visível e sem alterar o corpo dos testes. O adiamento do desktop deixa de ser rumor e vira decisão formal com consequências registradas. Os documentos voltam a contar a mesma história, e o smoke integrado é finalmente tentado em infraestrutura isolada — com evidência de sucesso ou com o bloqueio explicitamente registrado.
- **Appetite**: medium
- **Trade-offs**: Fecha quatro NVs de uma vez com mudanças pequenas em gate, ADRs e documentação, sem mexer no comportamento da aplicação. Exceções visíveis podem virar válvula de escape se não forem vigiadas, e a tentativa de smokedepende de portas e projeto isolados.
- **Rabbit holes**: refatorar corpos de teste; reintroduzir números obsoletos na documentação; promover checks a required sem autorização; teimar no smoke nas portas ocupadas; resolver e2e de browser e SBOM neste corte.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Apenas estender o lint restrito aos alvos de teste com exceções registradas e adotar formalmente o ADR de diferimento do desktop. A CI passa a ser honesta sobre lint e o adiamento deixa de ser relitigado, mas o resto permanece.
- **Appetite**: small
- **Trade-offs**: É o corte mais barato e fecha dois NVs com risco mínimo; deixa a contradição documental e a ausência de evidência do smoke intactas, que são justamente o que corrói a confiança no marco.
- **Rabbit holes**: tratar o ADR adotado como se resolvesse o smoke; acumular exceções de lint sem registro justificado.

### Option C — Não fazer: manter o status quo e comprar auditoria
- **Sketch**: Não alterar gate nem ADRs: aceitar lint parcial e diferimento informal, contratando eventual auditoria externa para atestar o estado do Seed.
- **Appetite**: uncertain (sem evidência de custo ou de que satisfaz a auditoria interna)
- **Trade-offs**: Evita conflito com trabalho paralelo e custo imediato, mas exceções não documentadas continuam invisíveis, o adiamento volta a cada ciclo e nenhuma auditoria externa substitui a execução do smoke.
- **Rabbit holes**: depender de relatório externo para decisões internas, perder o histórico de autoridade dos ADRs.

## Recommendation

Recomendo a **Option A**: as métricas do problema pedem que o lint cubra `--all-targets` com as sete exceções registradas em nível de módulo e corpos inalterados, que o ADR de diferimento seja formalmente adotado, que as contradições do ciclo anterior desapareçam e que o smoke tenha evidência de execução ou NV explícito com bloqueio identificado. A Option B cobre só as duas primeiras; a Option C não produz evidência executável.

## Out of Scope (for the recommended option)

- E2E de browser com `retries: 0`; processo de release/tag e SBOM.
- Gate bloqueante do `cargo-vet` com CODEOWNERS; exporter OTLP; validação de carga/restauração em 2vCPU/4GB.
- Editar corpos de teste, reverter trabalho paralelo, acessar produção ou alterar comportamento da aplicação.
- Não transformar o smoke em gate de merge.

## Assumptions to Validate

- Que as sete exceções são apenas em código de teste e que o precedente de anotação em nível de módulo se aplica.
- Que o rascunho do ADR de diferimento é coerente com os demais documentos e com o código, e pode ser adotado como está.
- Se a adoção do ADR exige aprovação de stakeholders além da equipe de documentação.
- Quais portas isoladas e nome de projeto usar para o smoke, e se um bloqueio de portas deve ser registrado como NV.
- Qual das versões divergentes do README prevalece e quem valida a correção sem conflito com edições de outros agentes.
- Se `retries: 0` no e2e é decisão definitiva ou resposta temporária à rotação de `jti`.
