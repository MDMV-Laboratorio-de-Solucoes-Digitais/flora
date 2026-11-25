# Concept: Uma trilha de navegador determinística

- **Slug**: 030-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Um smoke de navegador + documentação honesta

## Options

### Option A — Um smoke de navegador + documentação honesta
- **Sketch**: O happy path de seleção de organização, canais/chat e tarefas passa a ter uma única execução em navegador que realmente roda e produz evidência interpretável, com sessão emitida de forma determinística em vez de segredo fixo. O job de navegador fica informativo: gera sinal sem virar condição de merge. Em paralelo, a prosa sobre lint e os banners históricos passam a descrever o que foi executado, e o artefato de evidência passa a identificar revisão, segredo e transcript.
- **Appetite**: medium (incerto: execução remota em CI não é garantida neste ambiente)
- **Trade-offs**: Fecha a última lane de verificação sem produto sem adicionar serviços, ao custo de uma superfície extra de emissão de sessão e de um job de CI; como a seleção de organização gira a sessão, o teste não pode ter repetições e fica sensível a mudanças de interface. Evidência informativa não é o mesmo que gate obrigatório.
- **Rabbit holes**: cobrir todos os fluxos do frontend de uma vez; espalhar marcadores de teste pela interface; promover o job a required; ressuscitar especificações órfãs; resolver release/SBOM, CORS ou load/restore neste corte.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Executar um único smoke de navegador com a sessão emitida por procedimento manual local e corrigir a prosa de lint. O happy path ganha uma verificação, mas ela depende de um operador preparar o ambiente.
- **Appetite**: small
- **Trade-offs**: Nenhum binário novo na aplicação e menos configuração de CI; em troca, a verificação não é reproduzível por qualquer pessoa e o artefato dificilmente terá revisão e segredo rastreados.
- **Rabbit holes**: confundir "executado uma vez na máquina de quem escreveu" com "evidência reproduzível"; deixar o artefato sem transcript versionado.

### Option C — Não fazer: comprar teste de navegador gerenciado
- **Sketch**: Terceirizar a lane de navegador para um serviço gerenciado de testes em nuvem, sem emissão local nem job próprio.
- **Appetite**: large (incerto: custo e política de dados não avaliados)
- **Trade-offs**: Remove configuração local e fornece relatórios prontos, mas introduz serviço externo, credenciais e envio de dados de tenant para fora da instalação, contrariando a restrição de nenhum serviço novo; não corrige a documentação.
- **Rabbit holes**: integração com provedor, retenção de artefatos, exposição de sessões e vazamento de segredos.

## Recommendation

Recomendo a **Option A**: as métricas pedem um smoke determinístico com `retries: 0` e `workers: 1` que permaneça verde em execuções repetidas, artefato com revisão fixada, impressão digital do segredo e transcript versionado, prosa corrigida em README/CI/OBJETIVO e nenhum required check regredido. A Option B não entrega evidência reproduzível; a Option C adiciona serviço externo proibido.

## Out of Scope (for the recommended option)

- Tag/release + SBOM; allowlist CORS; execução de load/restore; promoção de `cargo-vet` a required; OTLP.
- Não converter o smoke em gate de merge nem afirmar cobertura de todos os fluxos do frontend.
- Não atribuir a este ciclo a resolução do critério #2 de reentrada da ADR-0002 nem de Documentos FASE 7.
- Não criar novos serviços, não reverter trabalho paralelo e não acessar produção.

## Assumptions to Validate

- Que `retries: 0` e `workers: 1` são obrigatórios por causa da rotação de `jti` na seleção de organização.
- Que seletores semânticos estáveis bastam para um único smoke, sem marcadores dedicados na interface.
- O que conta como "evidência suficiente" para aprovar o slice quando o job não é required.
- Se o artefato de evidência fica versionado no repositório ou apenas referenciado por caminho externo.
- Quem valida que a correção da prosa de lint não conflita com edições de outros agentes.
- Que o resultado de CI remoto e Forgejo permanece [NV] neste ambiente.
