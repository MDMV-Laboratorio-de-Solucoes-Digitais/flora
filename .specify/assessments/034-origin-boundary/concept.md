# Concept: Fronteira de origem do WebSocket com prova

- **Slug**: 034-origin-boundary
- **Created**: 2026-11-25
- **Recommended option**: Origem verificada com prova isolada

## Options

### Option A — Origem verificada com prova isolada
- **Sketch**: O canal WebSocket passa a examinar a origem da conexão antes de qualquer autenticação ou acesso a dados: origem ausente segue aceita, `null` e origens não listadas são recusadas. O frontend resolve endpoints a partir de configuração de runtime, sem copiar endereços no código. A regra ganha testes puros e determinísticos, uma prova ao vivo em instância isolada com banco próprio e uma nota honesta no ADR dizendo o que foi evidenciado e o que segue fora de alcance. O critério só é considerado fechado quando também houver lane de regressão em navegador.
- **Appetite**: medium (incerto: depende de janela isolada para a prova ao vivo)
- **Trade-offs**: Fecha uma não-verificação antiga com evidência rastreável e sem dependências novas, mas amplia a superfície em frontend, API e testes e exige instância separada. Origem é fronteira de navegador, não autenticação; literais divergentes ou proxy que remove o cabeçalho invalidam a garantia, e a configuração de runtime é lida apenas na inicialização.
- **Rabbit holes**: usar o binário desatualizado em `:3000` como prova; tocar no processo alheio ou no trabalho paralelo em `/metrics`; tentar provar Tauri em todos os sistemas operacionais; presumir que proxy preserva `Origin`; tratar o vazamento por `?token=` como resolvido; abrir Documentos FASE 7, limpeza de affordances, `cargo-vet`, tag/release ou load/restore neste corte.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Apenas estabelecer a regra de origem no WebSocket com testes determinísticos puros e recusa imediata. A lacuna técnica imediata desaparece, mas o critério de reentrada continua aberto: faltam URLs de runtime, prova ao vivo, lane de navegador e nota na ADR.
- **Appetite**: small
- **Trade-offs**: É o corte mais barato e verificável sem infraestrutura extra; resolve a checagem, mas não produz a evidência que encerra a governança nem resolve a dependência de literais no frontend.
- **Rabbit holes**: declarar o critério fechado com base apenas em testes puros; deixar a configuração de runtime indefinidamente para depois.

### Option C — Não fazer: empurrar a fronteira para o proxy e aceitar o NV
- **Sketch**: Não mexer no produto: confiar que um proxy reverso ou gateway externo fará o filtro de origem e manter o critério registrado como não-verificado.
- **Appetite**: uncertain (sem evidência de que o proxy encaminhe `Origin`)
- **Trade-offs**: Zero mudança local, mas nenhuma evidência versionada, nenhum teste determinístico e uma dependência de comportamento de infraestrutura que hoje não está confirmada; não encerra a reentrada.
- **Rabbit holes**: regras de gateway divergentes dos literais da aplicação, ausência versus `null`, lock-in de fornecedor e afirmações de conformidade mais amplas do que o provado.

## Recommendation

Recomendo a **Option A**: as métricas do problema pedem predicado puro cobrindo ausência, `null` e origem não listada; testes oneshot sem banco; provas ao vivo em instância isolada na porta 3100 com banco próprio, incluindo preflight, `select-organization` com Bearer e sondas de origem; e nota na ADR sem reivindicar mais do que o provado. A Option B cobre só a primeira métrica; a Option C não produz evidência.

## Out of Scope (for the recommended option)

- Não afirmar o critério #2 como atendido antes de todas as verificações determinísticas, prova ao vivo, lane de regressão em navegador e nota na ADR.
- Não implementar Documentos FASE 7, não limpar affordances do frontend, não promover `cargo-vet`, não criar tag/release e não executar load/restore.
- Não tocar no processo em `:3000` nem no trabalho de `/metrics` de outro agente; não usar o binário desatualizado como evidência.
- Não afirmar que Tauri funciona em todos os sistemas operacionais, que proxy reverso preserva `Origin` ou que o vazamento por `?token=` foi resolvido.

## Assumptions to Validate

- Que a allowlist existente é a fonte compartilhada para a regra de origem e que a correspondência é exata de bytes.
- Que origem ausente deve ser permitida e `null`/não listadas recusadas antes de autenticação e banco.
- Que a porta 3100 e um banco próprio estão disponíveis para a prova isolada.
- Quem executa e valida a lane de regressão em navegador necessária para considerar o critério fechado.
- Quais literais de origem são necessários para Tauri em cada sistema operacional [NV].
- Se o proxy reverso da instalação encaminha `Origin` ao backend [NV].
- Como tratar o vazamento de token por `?token=` [NV] e o que acontece com a configuração de runtime lida apenas no startup.
