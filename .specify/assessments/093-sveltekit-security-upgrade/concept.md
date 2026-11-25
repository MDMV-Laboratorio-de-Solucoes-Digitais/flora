# Concept: Upgrade de segurança do frontend para SvelteKit 3

- **Slug**: 093-sveltekit-security-upgrade
- **Created**: 2026-11-25
- **Recommended option**: Salto de major com versões corrigidas e migração explícita de configuração

## Options

### Option A — Salto de major com versões corrigidas e migração explícita de configuração
- **Sketch**: o frontend passa a usar a versão do kit que cobre os quatro achados — incluindo o de severidade high — e o aplicativo continua compilando porque as duas quebras do SvelteKit 3 são tratadas como parte da mesma mudança: a configuração migrada para o caminho agora lido e o alias `$lib` preservado explicitamente. Para o mantenedor, o resultado é um `dependency-review` sem o bloqueio conhecido, com check, lint, testes e build mantidos, sem afrouxar o gate que apontou o problema.
- **Appetite**: medium — upgrade de major com duas quebras de configuração, dois lockfiles e verificação de check/lint/testes/build; evidência local de que os peers já atendem ao kit corrigido e de que as verificações passaram, mas o resultado remoto da PR é **[NV]**.
- **Trade-offs**: ganha a única versão que cobre o conjunto completo dos quatro advisories e destrava o merge sem relaxar o `dependency-review`. Sacrifica a simplicidade de um patch na linha 2.x: exige migrar configuração e conviver com a possibilidade de incompatibilidade no runtime de deploy (`adapter-node`) até ser observada. Green local não substitui a CI remota; uma migração incompleta trocaria o bloqueio por build que não compila.
- **Rabbit holes**: reescrever API de aplicação, runtime, schema ou contrato de produção "aproveitando" o major; mexer em workflows, required checks ou no próprio `dependency-review`; promover a PR, tag ou deploy dentro do ciclo; perder `$lib` ou a configuração de preprocess/adapter na migração; perseguir contagem viva de advisories sem reconsulta autorizada; reescrever o registro histórico de tarefas para conciliar snapshots divergentes.

### Option B — Patch na linha 2.x para os advisories que o oferecem (a menor coisa que poderia funcionar)
- **Sketch**: aplicar apenas `2.70.4`, a versão corretiva disponível na linha 2.x para dois dos advisories moderados, mantendo o restante da configuração intacta. É a mudança de menor superfície e não exige migração de major.
- **Appetite**: small — atualização de versão pontual nos arquivos do frontend, sem mudança de configuração.
- **Trade-offs**: evita as duas quebras do SvelteKit 3 e o risco no `adapter-node`, mas não cobre os outros dois advisories — inclusive o `GHSA-3r8g` (high), corrigido apenas em `3.1.1`. Portanto o `dependency-review` continuaria bloqueando a PR: não atende à métrica central de desbloquear o merge sem flexibilizar o gate. Útil apenas como passo intermediário se o objetivo fosse reduzir achados antes de uma migração posterior.
- **Rabbit holes**: tratar a redução parcial do número de advisories como sucesso; acrescentar exceções locais para os achados restantes; adiar a decisão de major indefinidamente; confundir inteligência de flags experimentais com não-exposição.

### Option C — Não fazer agora: aguardar patches 2.x para o conjunto completo
- **Sketch**: manter as versões atuais e esperar que a linha 2.x publique correções para os quatro advisories; a PR permanece bloqueada até lá e nenhuma configuração é migrada. É a opção "não fazer / esperar o upstream".
- **Appetite**: small — nenhuma mudança de código, apenas espera e acompanhamento.
- **Trade-offs**: custo imediato zero e risco de regressão nulo, mas mantém um advisory high na dependência do frontend e o merge de `dev` para `main` bloqueado. Não há evidência nos artefatos de que patches 2.x para os dois advisories restantes estejam disponíveis ou previstos, então o tempo de espera é incerto. Pressiona a tentação de relaxar o `dependency-review`, justamente o que as decisões 089/091/092 rejeitam.
- **Rabbit holes**: transformar a espera em exceção permanente no required check; recontar advisories em fontes não comparáveis para justificar o adiamento; promover a PR antes da verificação remota; perder a janela de correção quando o patch final aparecer.

## Recommendation

Recomendo a **Opção A**. Ela é a única que remove as versões afetadas (`2.70.3`, `5.5.7`, `5.56.10`) sem ocorrência residual nos locks e cobre os quatro advisories, inclusive o de severidade high que o patch 2.x não resolve; por isso é a que pode tirar o `dependency-review` do bloqueio sem relaxá-lo. Mantém `bun.lock` como canônico com `package-lock.json` sincronizado, preserva `$lib` e a configuração, e limita o diff ao frontend e aos artefatos do ciclo. As verificações locais verdes são registro, não substituto da CI remota: o resultado da PR e a contagem pós-upgrade seguem **[NV]** como premissa a validar antes de qualquer promoção.

## Out of Scope (for the recommended option)

- API de aplicação, runtime, schema, migration, contrato de produção, workflows e required checks — inclusive qualquer flexibilização do `dependency-review`.
- PR, merge, tag, deploy, credenciais e produção neste ciclo; recontagem autorizada de estado remoto fora do escopo documental.
- `validate/`, `release/` e `.specify/memory/STATE.md`; reescrita do registro histórico ou dos snapshots divergentes de tarefas.
- Afirmação de que a CI remota passou ou de que a contagem de advisories é zero sem **[NV]**.
- Mudança de gerenciador de lockfile ou abandono de `bun.lock` como fonte canônica da CI.

## Assumptions to Validate

- `@sveltejs/kit 3.1.1` cobre os quatro advisories; `2.70.4` cobre apenas dois, então o major é necessário para desbloquear o `dependency-review`.
- Peers já declarados (`vite ^8.0.16`, `typescript ^6.0.3`, `@sveltejs/vite-plugin-svelte ^7.1.2`, `svelte 5.57.1`, `adapter-node 6.0.5`) satisfazem o kit corrigido.
- A migração das duas quebras (configuração lida pelo plugin e `$lib` declarado explicitamente) preserva build, alias e configuração da aplicação; verificações locais registradas não foram reexecutadas nesta avaliação.
- Compatibilidade com o runtime `adapter-node` após o major, incluindo health de produção, não foi observada nos artefatos do ciclo **[NV]**.
- Versões afetadas residuais em outros locks além dos dois do frontend não foram confirmadas; consumidores de `svelte.config.js`/`$lib` fora de `apps/frontend` não foram identificados.
- Resultado remoto do `dependency-review` na PR #318 e contagem viva de advisories após o upgrade são **[NV]**; registros posteriores de promoção são fotografias datadas.
