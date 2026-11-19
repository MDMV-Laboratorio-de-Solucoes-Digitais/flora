# Requirements Quality Checklist — 082

**Feature**: [spec.md](../spec.md) · **Slug**: 082-remove-css-variables-key
**Created**: 2026-11-19T22:36Z

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T22:36Z, contra `define.md`/`decision.md` (acessados 2026-11-19T22:36Z). A execução dos gates determinísticos, a validação online do arquivo editado e os consumidores externos de `components.json` permanecem `[NV]`.

## Completude

- [x] FR-001 especifica a remoção exata da linha `apps/frontend/components.json:7` (`"cssVariables": true`) do objeto `tailwind`. A chave não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `tailwind.additionalProperties = false`, `required = ["css","baseColor"]`) e é descartada em silêncio pelo parser Zod `$strip` do CLI instalado `shadcn-svelte@1.8.1` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`); zero consumidores locais.
- [x] FR-002 especifica o ajuste da linha `apps/frontend/components.json:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula que separava `baseColor` da chave agora excluída), preservando o valor `zinc`. Resultado: `tailwind` com exatamente `css` e `baseColor`, JSON válido e sem trailing comma (`prettier.config.js:7`). A remoção isolada de `:7` deixaria trailing comma e JSON inválido — portanto SC-001 só é satisfeito após FR-001 **e** FR-002.
- [x] FR-003 exige byte-identicidade de `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), do valor de `tailwind.baseColor` (`:6`) e de todo o bloco `aliases` (`:9-14`); proíbe remover `style` (declarado no schema, `schema-Bq2uExa4.d.mts:409`, apenas deprecado em v4), alterar `baseColor`, `$schema`, `aliases` ou `tailwind.css`, e exige manter a formatação do Prettier local: tabs (`prettier.config.js:5`), aspas duplas obrigatórias em JSON (`singleQuote` em `:6` vale para JS/TS) e nenhuma trailing comma (`:7`).
- [x] FR-004 limita o diff rastreado a `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada) e aos 5 artefatos `.specify/assessments/082-remove-css-variables-key/`; nenhum outro arquivo de código, config, doc, gate, CI, lockfile ou `node_modules/` é modificado; base `8f24519fe` preservada em todo o resto. Nenhuma dependência nova, segredo ou mutação de DB/rede; documentos do próprio ciclo em `specs/082-remove-css-variables-key/` são artefatos, não código.

## Clareza e consistência

- [x] SC-001 é verificável: `python3 -c` com `json.load` em `apps/frontend/components.json` retorna `set(obj["tailwind"]) == {"css", "baseColor"}` → `True`, e `cssVariables` está ausente em todos os níveis do objeto carregado; o `json.load` só passa após T001 (remove `:7`) e T002 (remove a vírgula em `:6`). `[NV]` depende da edição.
- [x] SC-002 é verificável: `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` não retorna `apps/frontend/components.json` nem qualquer arquivo de código/script/CI; apenas os registros históricos já existentes em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*` permanecem; `$schema`, `style`, `baseColor`, `css` e `aliases` seguem byte-idênticos (FR-003).
- [x] SC-003 é verificável: em `apps/frontend`, `bunx prettier --check components.json` termina com exit 0 (tabs, aspas duplas, sem trailing comma — `prettier.config.js:5-7`); `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` (`package.json:8,11,13,15`; `test:unit` é `vitest`, `package.json:15`, que exige `--run` para execução única) seguem sem regressão; `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. Nenhum desses comandos lê `cssVariables` em runtime. `[NV]` não executados nesta avaliação.
- [x] SC-004 é verificável: `git diff --name-only` lista somente `apps/frontend/components.json` e os 5 artefatos 082; zero mutação de gate, CI, scripts, workflows, lockfiles, `node_modules/`, ADRs, specs numeradas, DB, rede ou segredos; zero PRs abertos; base `8f24519fe` preservada.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos (R4/R7); a rastreabilidade por T001..T004 em `plan.md` e `tasks.md` usa cada ID nominalmente, com SC-001 em T001 e T002 (o `json.load` só passa após remover a chave e a vírgula).
- [x] Evidência determinística (linha colada em `apps/frontend/components.json:4-8`; schema oficial com URL e data de acesso; `$strip` no CLI 1.8.1 em `schema-Bq2uExa4.d.mts:407-414`/`index-Ch0UJ9Cm.d.mts:13-16`; varredura de consumidores zero; `prettier.config.js:5-7`) distinguida de `[NV]` para os gates pós-edição, a validação online e os consumidores externos.

## Edge cases

- [x] E se `"cssVariables": true` já estiver ausente? FR-001 é idempotente: nada a remover; nesse caso a vírgula em `:6` provavelmente também já foi ajustada e SC-001 está satisfeito.
- [x] E se apenas a linha `:7` for removida, sem ajustar a vírgula em `:6`? O JSON fica inválido (trailing comma) e SC-001 falha; por isso T001 e T002 são inseparáveis para a verificação com `json.load`.
- [x] E se `style` for removido "por ser deprecado em v4"? FR-003 proíbe: `style` está declarado no schema (`schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecado; SC-002 detecta a mudança por comparação byte-idêntica.
- [x] E se um operador ou consumidor externo esperar `cssVariables`? O CLI já descarta a chave via `$strip`; nenhum leitor oficial depende dela (`schema-Bq2uExa4.d.mts:410-414`). Consumidores em repos fora do checkout são `[NV]`, sem quebra de leitura prevista.
- [x] E se a formatação for alterada (espaços em vez de tabs, aspas simples, trailing comma)? FR-003 exige o estilo do Prettier local e SC-003 falha em `bunx prettier --check components.json` (`prettier.config.js:5-7`).
- [x] E se houver risco de DB/rede/segredos/`.tokens/`? Nenhum comando de DB é executado; a fronteira de paths permanece explícita em FR-004/SC-004; base `8f24519fe` preservada.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T22:31Z) antes da escrita dos artefatos, com E=(2×5)/(1×1)=5,0 e opção A (remover `:7` e ajustar a vírgula em `:6`, somente em `apps/frontend/components.json`) eleita.
- [x] R3 zero perguntas carry-forward registradas em `clarify.md` (divergência `feature.json` 081→082 documentada via `--dry-run`, sem bloqueio); os `[NV]` não são perguntas bloqueantes.
- [x] R4 e R7 respeitados na rastreabilidade: FR/SC individuais em `spec.md`, `plan.md` e `tasks.md`, sem intervalos; T001..T004 rastreáveis, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/078-*`/`.specify/assessments/078-*`, `specs/082-*` (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.
