# Shape: instalação pinada do CLI `shadcn-svelte`

- **Slug**: 079-install-shadcn-svelte · **Recomendado**: Opção A (devDependency exata, sem `init`) · **Data**: 2026-11-19T20:57Z

## Opção A — devDependency exata `1.8.1`, sem comandos do CLI (recomendada)

- **Sketch**: em `apps/frontend`, `bun add -D -E shadcn-svelte@1.8.1`. Appetite: trivial (1 linha no `package.json` + entradas no lockfile).
- **Trade-offs**: torna o bin local e reproduzível, com `latest = 1.8.1` e licença MIT confirmados no registry (2026-11-19); não sobrescreve CSS/`components.json` nem reinstala os 51 componentes. `-E` evita `^` e congela a versão — o restante do `package.json` usa caret, mas a pinagem do toolchain é intencional. `[NV]` validação pós-instalação depende de executar o comando.
- **Rabbit holes**: rodar `init` "para validar" (destrutivo); usar `@latest` no `bun add` (versão flutuante); editar `components.json`/`layout.css` para acomodar o CLI; rodar `bunx shadcn-svelte add --all` (reinstala componentes); editar CI/gate; instalar na raiz em vez de `apps/frontend`.

## Opção B — `bun add -D shadcn-svelte` (caret)

- **Sketch**: instalar com intervalo `^1.8.1`, seguindo o padrão caret das demais devDependencies.
- **Trade-offs**: mais consistente com o arquivo, mas perde a versão exata; um minor/patch futuro pode alterar a saída do CLI e invalidar componentes já commitados. Custo de reprodutibilidade > benefício de consistência. Rejeitada.

## Opção C — manter o CLI efêmero via `bun x shadcn-svelte@latest …`
- **Sketch**: não instalar; continuar invocando o CLI sob demanda.
- **Trade-offs**: zero diff, mas é exatamente o estado que o usuário pediu para mudar: depende de rede, usa `latest` (flutuante) e não fica rastreado no lockfile. Rejeitada.

## Fora de escopo

`components.json`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/`, `src/lib/utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/CI/gate (`scripts/ci/check_gate_integrity.py`), `.github/workflows/`, supply-chain, ADRs, constituição, STATE, specs/079-* (exceto os 5 artefatos), DB/rede fora do registry, segredos, `.tokens/`, fechamento de issues, leg RBAC, upgrade de versão e execução de `init`/`add`/`apply`.

## Premissas

O pedido do usuário é autorização explícita para instalar; `shadcn-svelte` é ferramenta de geração, não dependência de runtime ⇒ devDependency; `latest = 1.8.1` e MIT (npm/bun, 2026-11-19); `init` é para projeto novo e sobrescreve o CSS global, recria `components.json` e reinsta componentes, com opções `--reinstall`/`--no-reinstall` (docs oficiais, 2026-11-19); `components.json` é opcional e só exigido pelo CLI (docs, 2026-11-19). `[NV]` todos os comandos de verificação dependem da execução da instalação.
