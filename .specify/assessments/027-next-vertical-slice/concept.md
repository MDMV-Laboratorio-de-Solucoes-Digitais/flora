# Concept: Seed que anuncia só o que executa

- **Slug**: 027-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Fecho honesto do Seed

## Options

### Option A — Fecho honesto do Seed
- **Sketch**: O produto passa a anunciar apenas mecanismos que realmente entregam: o transporte de notificações que nunca é usado sai, enquanto a entrega real em banco e o perfil opcional de tempo real permanecem intactos. A API ganha correlação mínima por requisição sem coletor nem novo serviço. ADR, README, planejamento e especificações passam a falar do estado verificado, e não de decisões já superadas.
- **Appetite**: medium
- **Trade-offs**: Entrega coerência entre código e documento com baixo risco de comportamento, mas mexe em estrutura do workspace e no arquivo de travamento, e exige cuidado para não tocar nos caminhos que funcionam. A documentação pode entrar em conflito com edições paralelas.
- **Rabbit holes**: mexer no serviço central de notificações, na dedup em Postgres ou no perfil de tempo real; tentar entregar exporter OTLP completo; reescrever todas as especificações em vez de corrigir afirmações pontuais; reabrir decisões já tomadas.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Apenas reconciliar os documentos e anexar rastreamento por requisição, deixando o transporte não consumido no código. Para o leitor, a documentação melhora e a API ganha correlação; para o operador, a armadilha continua disponível.
- **Appetite**: small
- **Trade-offs**: É a opção mais barata e reversível, mas não fecha a métrica de que nenhuma referência ao transporte morto permanece, e mantém vivo o caminho que colidiria com a dedup já implementada.
- **Rabbit holes**: tratar "documentação corrigida" como equivalente a "código limpo"; adiar a remoção para sempre por causa de medo de mexer no lockfile.

### Option C — Não fazer: comprar plataforma externa
- **Sketch**: Manter o código como está e comprar/aderir a serviços externos de mensageria e observabilidade para dar suporte ao que a documentação anuncia.
- **Appetite**: large (incerto: sem evidência de custo ou contrato)
- **Trade-offs**: Tira o problema da mesa sem remover código morto, mas adiciona serviços obrigatórios, custo e dependência de rede, contrariando a restrição de VPS barato e nenhum serviço novo.
- **Rabbit holes**: migração de transporte, lock-in de fornecedor, reconciliação de documentos com topologias externas, exigência de coletores e credenciais.

## Recommendation

Recomendo a **Option A**: as métricas pedem que não reste referência ao transporte não executado, que o roteador da API tenha rastreamento por requisição, que os documentos não contenham afirmações superadas e que o arquivo de travamento mude apenas pelo que foi removido — só a remoção combinada à reconciliação documental satisfaz esse conjunto. A Option B deixa a armadilha latente; a Option C introduz serviços proibidos.

## Out of Scope (for the recommended option)

- Gate `cargo-vet` com store e imports; e2e de browser com JWT fresco; job de advisories OSV/Dependabot; SBOM de release.
- Exporter OTLP, Tauri e validação de carga/restauração em 2vCPU/4GB.
- Novos serviços obrigatórios, mudanças no frontend e reversão de trabalho paralelo.
- Não atingir o serviço central de notificações, a dedup em Postgres nem o perfil `realtime` com Valkey.

## Assumptions to Validate

- Que a crate de transporte realmente não tem consumidores em Rust e que seus campos de configuração nunca são lidos.
- Que o caminho XADD, se ativado, colidiria com o índice de dedup existente.
- Que rastreamento por requisição sem coletor entrega correlação útil para diagnóstico.
- Se o perfil `realtime` deve seguir documentado como parte do Seed ou explicitamente rotulado como opcional.
- Quem valida a redação final do ADR-0001 após a reconciliação.
- Qual convenção usar para tarefas de especificações que marcam como concluído trabalho nunca conectado.
- Que o diff do arquivo de travamento será limitado à remoção da crate.
