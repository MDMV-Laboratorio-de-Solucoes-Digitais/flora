# Concept: Deploy do frontend em origem própria

- **Slug**: 095-frontend-deploy
- **Created**: 2026-11-25
- **Recommended option**: Separação de origens web/API com publicação manual da imagem do frontend

## Options

### Option A — Separação de origens web/API com publicação manual da imagem do frontend
- **Sketch**: o FQDN atual passa a servir a aplicação (`/`, `/tasks`) e a API ganha uma origem identificável para `/health`, `/metrics` e `/llms.txt`. O frontend é publicado a partir do seu próprio Dockerfile, por decisão manual, sem trigger automático de push/PR/merge; o browser recebe a base da API correta em produção, e a allowlist de origens permitidas é explícita, sem curinga, coerente com a checagem de origem do websocket. Para o usuário final, o 404 nas rotas da aplicação desaparece sem que o gate de merge seja alterado.
- **Appetite**: medium — envolve imagem, publicação, DNS/FQDN e configuração de runtime/CORS, com verificação por smoke de rotas; evidência local forte da causa raiz, mas execução remota, DNS e respostas HTTP finais são **[NV]** nos artefatos do ciclo.
- **Trade-offs**: resolve a causa determinística (a imagem publicada continha apenas a API) em vez de maquiar o sintoma, e cria uma fronteira clara entre web e API que permite configuração coerente de base e CORS. Sacrifica a simplicidade de uma única origem e exige coordenação manual de DNS/deploy; consumidores não identificados das rotas de API no FQDN atual podem ser afetados. A configuração entregue ao browser e o preflight CORS foram registrados como verificados, mas não reconsultados **[NV]**.
- **Rabbit holes**: publicar a imagem da API para o FQDN web "aproveitando" o workflow existente; criar trigger automático de publicação ou ligar deploy a required check; usar curinga na allowlist CORS ou deixar o browser apontando para `localhost`; mudar API, schema, runtime ou credenciais para acomodar o deploy; perseguir medição remota em hosts não allowlisted; tratar o desvio entre publicação prevista no GHCR e build direto do Git no Coolify como se fosse o mesmo caminho.

### Option B — Servir web e API na mesma origem com uma única imagem (a menor coisa que poderia funcionar)
- **Sketch**: ajustar o que é publicado no FQDN atual para incluir os assets do frontend junto com a API, mantendo um único host e um único workflow manual. Para o usuário, `/` e `/tasks` passariam a responder sem mudança de DNS.
- **Appetite**: medium — ainda exige reconstrução da imagem e decisão de roteamento, mas evita nova origem e novo FQDN; não há evidência nos artefatos de que essa opção tenha sido avaliada como suficiente.
- **Trade-offs**: evita a coordenação de DNS e o risco para consumidores da API na origem atual, mas mantém rotas de aplicação e rotas de API misturadas — exatamente a condição que impede uma base de API e uma allowlist CORS coerentes. Pode trocar o 404 por colisões de rota ou por configuração de runtime ainda apontando para `localhost`; o gateway/fachada necessário para separar os caminhos não é descrito nos artefatos.
- **Rabbit holes**: reescrever o `Dockerfile` raiz para incluir Node e assets; inserir proxy reverso na imagem; duplicar configuração de CORS para duas aplicações no mesmo host; confundir health da API com health da web; promover a mudança a automação de deploy.

### Option C — Não publicar agora: manter o FQDN da API e documentar a limitação do frontend
- **Sketch**: aceitar que `flora.mdmv.qzz.io` continua servindo apenas a API e registrar o 404 em `/` e `/tasks` como limitação conhecida, aguardando uma decisão de infraestrutura externa (DNS/FQDN/Coolify) antes de agir. A publicação do frontend fica para um ciclo próprio.
- **Appetite**: small — nenhuma mudança de imagem, workflow ou config; apenas documentação.
- **Trade-offs**: custo zero e nenhum risco para consumidores não documentados das rotas de API, mas o usuário final continua sem aplicação no FQDN — não atende à métrica central do problema. Adiar também não resolve a base da API nem o CORS, e mantém a confusão entre origem web e origem da API como dívida operacional.
- **Rabbit holes**: transformar a limitação documentada em aceitação permanente; reclassificar o 404 como comportamento esperado; promover a API para "frontend" por conveniência; perder a janela de correção quando a infraestrutura estiver disponível.

## Recommendation

Recomendo a **Opção A**. Ela ataca a causa raiz verificável por inspeção — o workflow publicava a imagem construída pelo `Dockerfile` da raiz, que contém somente a API — e atende às métricas: `/` e `/tasks` em origem web, `/health`, `/metrics` e `/llms.txt` em origem da API, `config.js` apontando para a base correta, allowlist CORS exata sem `*`, e publicação manual sem trigger automático nem mudança de required checks. Mantém deploy fora do gate sob ADR-0003 e o diff restrito ao ciclo. A execução remota e as respostas HTTP finais são **[NV]** nos artefatos, bem como o desvio registrado entre publicação no GHCR e build direto do Git no Coolify; ambos ficam como premissas a validar antes de reivindicar sucesso operacional.

## Out of Scope (for the recommended option)

- Required checks, gates de CI, API de aplicação, runtime, schema, migration e credenciais.
- Trigger automático de publicação (push/PR/merge) e promoção de deploy a condição de merge; ADR-0003 permanece inalterada.
- Afirmação de respostas HTTP remotas, configuração de DNS/FQDN ou resultado do deploy sem **[NV]**; reconsulta de hosts não allowlisted.
- `validate/`, `release/` e `.specify/memory/STATE.md`; reescrita do registro histórico de deploy.
- Consumidores não identificados das rotas de API no FQDN atual — realocação assumida, não inventariada.

## Assumptions to Validate

- O FQDN atual servia a API porque a imagem publicada continha apenas `flora-api`; localmente `/` e `/tasks` respondiam 200 sob dev server, e o manifesto do build registrava as rotas.
- Existe uma origem web própria distinta da origem da API; configurações atuais de DNS/FQDN no Coolify não foram observadas **[NV]**.
- O `config.js` entregue ao browser e o preflight CORS com a origem exata incluída na allowlist foram registrados como verificados; estado remoto atual **[NV]**.
- A publicação manual continua sob decisão humana e fora do gate; autorização para disparar e promover o deploy não foi identificada nos artefatos.
- Estratégia de rollback não foi validada para o caminho efetivamente usado: o fechamento registrou build direto do Git no Coolify em vez da imagem prevista no GHCR, então reapontar para imagem anterior pode não se aplicar **[NV]**.
- Fluxo E2E autenticado e healthcheck do Coolify não foram observados; consumidores das rotas de API no FQDN original não foram identificados.
