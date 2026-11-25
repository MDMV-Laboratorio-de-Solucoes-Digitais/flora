# Concept: Sincronização do lockfile npm derivado

- **Slug**: 092-npm-lock-sync
- **Created**: 2026-11-25
- **Recommended option**: Regeneração determinística e restrita ao lockfile npm

## Options

### Option A — Regeneração determinística e restrita ao lockfile npm
- **Sketch**: o artefato npm deixa de estar congelado e volta a descrever o mesmo grafo que o manifesto e o lockfile canônico da CI já descrevem — incluindo `shadcn-svelte`. Para quem lê o arquivo, a mudança é invisível além do desaparecimento da divergência conhecida: nenhum script é executado, nenhum gerenciador é trocado e o pipeline continua ancorado no lock canônico. A operação é repetível, com o mesmo resultado a cada execução.
- **Appetite**: small — um comando de regeneração de lockfile, sem instalação, sem mudança de manifesto e sem toque em CI; incerteza: idempotência e ausência de novos diffs são registro local, não reexecução independente nesta avaliação, e host do registry não foi consultado **[NV]**.
- **Trade-offs**: ganha conformidade entre o artefato derivado e as fontes, com diff confinado a um arquivo e reversão trivial pelo controle de versões. Sacrifica a ideia de que a regeneração resolve alertas: a queda observada entre inventário do Dependabot e contagem do `npm audit` vem de fontes distintas e não é remediação de finding individual. Também não cria política que impeça novo drift futuro, e não há consumidor identificado do `package-lock.json`, então parte do valor é documental.
- **Rabbit holes**: trocar `bun.lock` pelo lock npm na CI "para uniformizar"; editar `package.json` para acomodar a dependência faltante; executar lifecycle scripts ou instalar `node_modules`; tentar remediar os 25 alertas dentro deste ciclo; alargar o diff a source, workflows, `validate/`, `release/` ou `STATE`; perseguir acesso ao registry sem autorização.

### Option B — Ajuste manual pontual da entrada faltante (a menor coisa que poderia funcionar)
- **Sketch**: acrescentar ao `package-lock.json` apenas as linhas da dependência ausente e de suas transitivas, sem regenerar o arquivo. É a intervenção de menor diff imediato e não depende de nenhuma ferramenta externa.
- **Appetite**: small — edição local de um arquivo, sem comandos de instalação.
- **Trade-offs**: resolve a ausência visível de `shadcn-svelte` com custo mínimo e nenhum efeito em lifecycle scripts, mas não garante que o grafo completo esteja coerente com o manifesto e com o lock canônico, nem que uma segunda execução produza o mesmo resultado. Deixa a manutenção manual como prática permanente e não fornece a evidência de idempotência exigida pelo problema.
- **Rabbit holes**: copiar transitivas de outro lock sem conferência; tratar a presença da entrada raiz como prova de conformidade; repetir o ajuste manual a cada nova dependência; transformar o procedimento em check de CI.

### Option C — Eliminar o artefato derivado ou migrar a CI para npm ("comprar" uma mudança de processo)
- **Sketch**: em vez de sincronizar o `package-lock.json`, decidir que ele não deve existir (ficando apenas `bun.lock`) ou que a CI passa a usar npm, eliminando a dualidade de uma vez. Para o mantenedor, haveria uma fonte única de lockfile em vez de um artefato a manter.
- **Appetite**: medium — muda política de lockfile, ferramental de CI e eventuais consumidores; nenhuma evidência nos artefatos de que a migração seja desejada ou autorizada.
- **Trade-offs**: remove a causa estrutural do drift e pode simplificar o longo prazo, mas troca um reparo reversível de um arquivo por uma mudança de infraestrutura com risco de quebrar a CI e os consumidores desconhecidos do lock npm. Contraria a política registrada de `bun.lock` como canônico e amplia o diff muito além do problema medido; a contagem de advisories e a promoção da PR ficariam **[NV]**.
- **Rabbit holes**: reescrita de workflows e scripts de CI; remoção de arquivos usados por terceiros não identificados; exigência de instalação real com lifecycle scripts; promoção de checagem de advisories a required check; recontagem de security alerts sem fonte comparável.

## Recommendation

Recomendo a **Opção A**. Ela atende às métricas registradas: `package-lock.json` passa a refletir `package.json` e `bun.lock` com `shadcn-svelte 1.8.1` na raiz e na entrada transitiva, a alteração de código fica confinada ao lockfile npm, a regeneração é idempotente no registro local, e a CI permanece ancorada em `bun.lock`. É reversível por restauro via controle de versões e não altera processo, gerenciador nem infraestrutura. O benefício é conformidade do artefato, não remediação de advisories — essa distinção fica explícita nas premissas.

## Out of Scope (for the recommended option)

- `package.json`, `bun.lock`, source, workflows, pipeline de CI e qualquer troca de gerenciador.
- Execução de lifecycle scripts, instalação de `node_modules` e uso de registry fora do comando contratual de lockfile-only **[NV]**.
- Remediação individual dos 25 alertas do Dependabot; promoção de checagem de advisories a required check.
- `validate/`, `release/`, `.specify/memory/STATE.md`, PR, merge, tag, deploy e produção.
- Afirmação de contagem viva de alertas pós-triagem ou de execução ao vivo do GitHub Actions **[NV]**; a queda 25 → 21 não é comparação estrita entre fontes.
- Definição de política anti-drift futura — lacuna registrada como pergunta aberta.

## Assumptions to Validate

- `bun.lock` é a fonte de verdade da CI e `package-lock.json` é artefato derivado; nenhuma troca de gerenciador foi autorizada.
- O comando de regeneração é determinístico e idempotente no ambiente contratual; a reexecução não foi feita nesta avaliação e o host do registry não é allowlisted **[NV]**.
- Consumidores do `package-lock.json` existem ou importam; nenhum foi identificado nos artefatos, então o valor pode ser sobretudo documental.
- A ausência de execução de lifecycle scripts é aceitável para a conformidade buscada; integridade de transitivas e comportamento de instalação real não foram observados.
- Contagem de alertas do Dependabot (25, 7 high) e contagem do `npm audit` (21) vêm de fontes distintas; nenhum finding foi triado ou corrigido neste ciclo.
- Estado de CI/PR/merge/produção do ciclo permanece **[NV]**.
