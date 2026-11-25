# Concept: Espelho Forgejo previsível e advisory

- **Slug**: 089-forgejo-mirror-sync
- **Created**: 2026-11-25
- **Recommended option**: Publicação one-way, event-driven e advisory com guarda de ancestralidade

## Options

### Option A — Publicação one-way, event-driven e advisory com guarda de ancestralidade
- **Sketch**: a cada publicação das refs acordadas (`dev`, `main`, tags `v*`), o espelho institucional recebe o histórico local sem force, sem deleção de refs e sem reescrita. Antes de publicar uma branch, o mecanismo verifica que o que já está no espelho é ancestral do que será publicado; se divergir, se a credencial faltar ou se a publicação for recusada, a operação falha fechado e ninguém precisa adivinhar se "deu certo". Para o mantenedor, o espelho deixa de ser uma incógnita e passa a ser uma infraestrutura informativa, com a mesma autoridade de merge do CI inalterada.
- **Appetite**: small — um workflow de publicação, sem dependências, migrações ou mudança de gate; incerteza: estado ao vivo do remoto, do job e da proteção de branch é **[NV]**, e o custo de coordenação entre refs não foi medido.
- **Trade-offs**: ganha previsibilidade e auditabilidade sem deslocar a autoridade do gate ADR-0003; mantém a separação de papéis (GitHub = CI autoritativo, Forgejo = espelho). Sacrifica a conveniência de "sincronizar tudo" — apenas as refs acordadas são publicadas — e aceita parar em divergência em vez de reconciliar automaticamente, exigindo decisão humana quando o espelho se afastar. A publicação passa a depender de secrets corretamente provisionados e de conexão com o host institucional, ambos sem verificação ao vivo nos artefatos.
- **Rabbit holes**: promover o job a required check ou alterar proteção de branch "para garantir" a publicação; tentar reconciliação reversa (espelho → local) ou `--force`/`--mirror`; ampliar o gatilho para todas as branches/tags; perseguir coordenação perfeita entre pushes simultâneos em refs diferentes antes de entregar o fluxo básico; escrever credenciais em logs, argumentos ou artefatos.

### Option B — Publicação manual sob demanda (a menor coisa que poderia funcionar)
- **Sketch**: o operador publica no espelho quando precisar, conferindo antes que o histórico local é sucessor do histórico publicado. Nenhum automação nova, nenhum job, nenhuma credencial em CI; a responsabilidade pela checagem fica com quem executa.
- **Appetite**: small — apenas um procedimento operacional, sem código.
- **Trade-offs**: custo imediato quase zero e risco mínimo de reescrita se o procedimento for seguido; mas a atualização do espelho volta a depender de lembrança humana, o histórico pode ficar defasado por longos períodos, e não há trilha de execução consistente. Não atende à métrica de publicação previsível e auditável, embora possa servir como ponte temporária.
- **Rabbit holes**: formalizar o procedimento como mais um artefato de governança obrigatório; usar o push manual como desculpa para flexibilizar a proteção de branch; repetir a conferência de ancestralidade de forma improvisada em cada publicação sem registro.

### Option C — Espelhamento completo gerenciado pelo Forgejo ("comprar" em vez de construir)
- **Sketch**: em vez de publicar a partir do CI, o próprio Forgejo buscaria o repositório de origem e manteria uma cópia integral, inclusive branches e tags fora da lista acordada. O mantenedor não teria um job para operar, mas perderia o controle fino sobre quais refs são publicadas e sobre o momento da publicação.
- **Appetite**: medium — depende de configuração/disponibilidade do serviço externo; nenhuma evidência nos artefatos de que essa capacidade tenha sido avaliada ou autorizada, e o host do espelho não foi consultado **[NV]**.
- **Trade-offs**: remove a lógica de publicação do repositório e pode reduzir manutenção, mas um espelho integral pode trazer refs que a governança decidiu manter fora de escopo e pode reescrever o histórico publicado conforme a origem muda — exatamente o risco que o problema pede evitar. Também desloca a auditoria para fora do repositório e cria dependência de configuração externa sem evidência local.
- **Rabbit holes**: negociação de permissões e configuração no serviço externo; tentativa de replicar o guarda de ancestralidade dentro do mecanismo de espelho; inclusão de todas as refs "por conveniência"; confundir espelhamento com deploy ou com gate de merge.

## Recommendation

Recomendo a **Opção A**. É a que melhor satisfaz as métricas registradas: push de branch com ref remota ancestral chega ao espelho sem force; divergência, push rejeitado ou credencial ausente falham fechado sem publicação; o job permanece fora dos required checks e sem alteração de proteção; e o diff fica confinado ao workflow do espelho e aos artefatos do ciclo. A opção respeita o caráter one-way e a ADR-0003 sem exigir decisão externa sobre o serviço Forgejo. O estado ao vivo continua **[NV]** e deve ser confirmado antes de tratar a publicação como efetiva.

## Out of Scope (for the recommended option)

- Required checks, `REQUIRED`/`ALLOWED_JOB_IF`, proteção de branch e workflows de CI existentes; nenhuma promoção do job de espelho a gate.
- Reconciliação reversa, force-push, `--mirror`, deleção de refs ou qualquer reescrita do histórico publicado.
- Refs fora de `dev`, `main` e tags `v*`; branches/tags remotas ausentes tratadas por automação.
- Processo automático de reconciliação humana em caso de divergência — falha fechado, decisão posterior.
- `validate/`, `release/`, `.specify/memory/STATE.md`, ADRs, constituição, código de aplicação, PRs, merge, tags, deploy e produção.
- Afirmação de estado ao vivo do remoto, do job ou da proteção de branch **[NV]**; credenciais em logs ou artefatos.

## Assumptions to Validate

- O espelho é one-way e o histórico local é a fonte da verdade; a premissa `origin/dev` = `f12a328c` é snapshot de 2026-11-20 e o tip atual do espelho é **[NV]**.
- `FORGEJO_USER` e `FORGEJO_TOKEN` estão efetivamente provisionados como secrets e há responsável por rotacioná-los/validá-los **[NV]**.
- A proteção de `dev`/`main` permanece compatível com a ADR-0003; estado ao vivo **[NV]**.
- Serialização por ref é suficiente para publicações simultâneas; não há evidência de coordenação entre refs diferentes.
- O caminho de tags precisa (ou não) do mesmo guarda de ancestralidade das branches — questão em aberto nos artefatos.
- O host institucional não foi consultado (não allowlisted); acesso e comportamento do remoto **[NV]**.
