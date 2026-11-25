# Concept: Gate de verificação executável

- **Slug**: 026-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Gate executável e documentos honestos

## Options

### Option A — Gate executável e documentos honestos
- **Sketch**: A afirmação "Seed v0.1 verificado" deixa de depender de interpretação: formatação, lint restrito e checagem de dependências passam a ser exigidos automaticamente antes de uma mudança entrar, e uma falha de licença deixa de travar a checagem. Os documentos que mandam no projeto passam a descrever o que realmente roda, e uma dependência que não existe deixa de constar como se existisse. A evidência do ciclo anterior é finalizada com resultado real.
- **Appetite**: medium
- **Trade-offs**: Transforma uma promessa em controle de merge, com custo de CI e necessidade de corrigir a regra de licenças; exige mexer em documentos de autoridade, que podem conflitar com edições alheias. Em troca, qualquer leitor passa a confiar no rótulo "verificado" sem auditoria manual.
- **Rabbit holes**: tentar auditar integralmente milhões de linhas de dependências; transformar advisories em gate impeditivo com rede RustSec; promover o cargo-vet a bloqueante neste corte; reescrever toda a governança em vez de corrigir o desvio; reativar o trabalho paralelo.

### Option B — Menor correção que poderia funcionar
- **Sketch**: Corrigir apenas a regra de licenças e acrescentar um único check de dependências na CI, deixando lint e documentação para depois. O operador passa a ver a checagem de licenças passar, mas a verificação continuaria parcialmente declarada em prosa.
- **Appetite**: small
- **Trade-offs**: É o menor passo que produz sinal automático e destrava a checagem local; sacrifica a promessa completa do gate e não resolve a dependência fantasma nem as contradições documentais.
- **Rabbit holes**: tratar o check isolado como "gate completo", acumulando exceções sem visibilidade; adiar sine die a finalização da evidência pendente.

### Option C — Não fazer: confiar em revisão externa
- **Sketch**: Não instituir gate local: manter a verificação como revisão humana e/ou contratar auditoria externa de dependências e governança.
- **Appetite**: large (incerto: sem evidência de custo ou periodicidade)
- **Trade-offs**: Evita mexer na CI e nos documentos imediatamente, mas não produz gate determinístico em checkout limpo, adiciona custo e dependência externa e contradiz a restrição de nenhum serviço obrigatório novo.
- **Rabbit holes**: transformar relatórios em fonte normativa; perder o histórico de decisões do repositório; depender de disponibilidade de terceiros para afirmar o marco.

## Recommendation

Recomendo a **Option A**: as métricas do problema são explícitas — `fmt`, clippy restrito e checagem de dependências executando na CI, licença sem erro, documentos sem contradição com o ADR-0001 e nenhuma dependência declarada apontando para versão inexistente — e só ela cobre todas ao mesmo tempo. A Option B deixa o rótulo "verificado" ainda dependente de interpretação; a Option C não é determinística em checkout limpo.

## Out of Scope (for the recommended option)

- Store, imports e isenções do `cargo-vet`; job de advisories com rede RustSec; SBOM.
- Aposentadoria do scaffolding PGMQ/Valkey; e2e de browser; Tauri; exporter OTLP completo.
- Validação de carga/restauração em 2vCPU/4GB e paridade com Meilisearch.
- Novos serviços obrigatórios, mudanças no frontend e reversão de trabalho paralelo.

## Assumptions to Validate

- Que a correção da regra de licenças é suficiente para a checagem sair de erro para sucesso (evidência local registrada).
- Que `bans` e `sources` já passam e não exigem ajuste adicional.
- Que a versão declarada de `opentelemetry-otlp` não existe no registro público [NV: disponibilidade remota não verificada localmente].
- Que a CI aceita os novos passos sem ultrapassar o orçamento atual e que a evidência do ciclo 025 tem resultado de PR real.
- Se a isenção `AGPL-3.0-or-later` deve ser global ou por crate, e se advisories ficam separados como sinal informativo.
- Qual versão da constituição passa a ser normativa e como as anteriores são referenciadas.
