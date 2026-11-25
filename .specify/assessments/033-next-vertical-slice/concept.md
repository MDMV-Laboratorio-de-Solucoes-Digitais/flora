# Concept: Busca do cabeçalho que responde

- **Slug**: 033-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Busca tipada com teste no gate

## Options

### Option A — Busca tipada com teste no gate
- **Sketch**: O campo do cabeçalho deixa de ser enfeite: ao receber um termo, consulta o backend, mostra resultados tipados e informa honestamente quando está carregando, quando não há resultado e quando houve erro. Um teste determinístico do cliente de busca passa a rodar no projeto do Vitest que a CI já exige, protegendo o contrato sem criar novo gate. A documentação de governança é alinhada aos ciclos concluídos.
- **Appetite**: small–medium (incerto: depende do formato real dos resultados do backend)
- **Trade-offs**: Entrega o maior ganho visível ao usuário com backend já verificado e sinal de CI imediato; em troca, expõe ao usuário campos `type`/`source` que são strings abertas e não promete relevância multilíngue nem navegação provada em navegador real. Testes de componente seguem fora do gate de merge.
- **Rabbit holes**: transformar isso em busca global com relevância CJK/i18n; promover componentes ou e2e informativo a required; reativar serviço externo de busca; promover `cargo-vet`, criar tag/release ou executar load/restore; implementar Documentos FASE 7; limpar todas as affordances falsas de uma vez; reivindicar o critério #2 da ADR-0002.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Ligar o campo a uma listagem simples de resultados, sem estados diferenciados e sem teste novo. O usuário finalmente vê algo acontecer, mas um erro silencioso ou uma resposta vazia ficam indistinguíveis.
- **Appetite**: small
- **Trade-offs**: Custo mínimo e valor visível imediato; não protege o contrato da busca, de modo que uma regressão passaria despercebida no gate atual, e deixa os estados de erro opacos.
- **Rabbit holes**: confundir "campo preenchido" com "busca funcionando"; adiar o teste para sempre por causa do escopo do CI.

### Option C — Não fazer: comprar busca externa
- **Sketch**: Não usar a busca em banco já verificada: comprar/aderir a serviço externo de indexação e relevância para alimentar o cabeçalho.
- **Appetite**: large (incerto: sem evidência de custo ou necessidade)
- **Trade-offs**: Traz relevância pronta, mas adiciona serviço obrigatório, custo e sincronização de dados de tenants, contrariando a restrição de nenhum serviço novo; resolve menos do que a busca já suportada localmente.
- **Rabbit holes**: reindexação, paridade com mecanismo anterior, latência de sincronização e dependência de fornecedor.

## Recommendation

Recomendo a **Option A**: as métricas pedem teste unitário do cliente executando no projeto `server` do Vitest, resultados tipados visíveis nos estados normal/vazio/erro, drift documental corrigido e required checks verdes. A Option B não protege o contrato; a Option C reintroduz um serviço que o Seed já substituiu por busca em banco verificada.

## Out of Scope (for the recommended option)

- Não afirmar que o critério #2 de reentrada da ADR-0002 foi atendido; não implementar Documentos FASE 7.
- Não promover `cargo-vet` a required, não criar tag/release e não executar load/restore.
- Não transformar testes de componente ou e2e informativo em gates de merge.
- Não reivindicar relevância CJK/i18n nem prova de navegação de resultado em navegador real.

## Assumptions to Validate

- Que o backend já verificado expõe resultados tipados suficientes para a interface, apesar de `type`/`source` serem strings abertas.
- Que a CI executa apenas o projeto `server` do Vitest e que o Playwright roda apenas `*.e2e.ts` de forma informativa.
- Que valores desconhecidos de `type`/`source` podem ser exibidos sem quebrar a interface.
- Que a anotação histórica "442" e os intervalos de ROADMAP/OBJETIVO são os únicos focos de drift residual.
- Resultado de CI remoto e Forgejo [NV].
- Quem decide quando retomar Documentos FASE 7 e a auditoria das demais affordances falsas; se é aceitável entregar sem prova de navegação em navegador.
