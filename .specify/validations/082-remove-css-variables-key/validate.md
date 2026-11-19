# Validation: 082 — remoção da chave `cssVariables` de `components.json`

**Data**: 2026-11-19T22:54Z · **Branch**: `feature/082-remove-css-variables-key` · **Baseline**: `8f24519fe`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff --stat 8f24519fe -- apps/frontend/components.json` | `apps/frontend/components.json \| 3 +--` — `1 file changed, 1 insertion(+), 2 deletions(-)`; único path de código (restante: artefatos do ciclo 082) (FR-004, SC-004) |
| `git diff 8f24519fe -- apps/frontend/components.json` | 2 remoções + 1 adição no bloco `tailwind`: removidas `"baseColor": "zinc",` e `"cssVariables": true`; adicionada `"baseColor": "zinc"` (sem vírgula); `+1 −2`, sem reindentação (FR-001, FR-002, FR-003) |
| `python3 -c 'import json; o=json.load(open("apps/frontend/components.json")); print(set(o["tailwind"]) == {"css","baseColor"})'` | `True`; `cssVariables` ausente em todos os níveis do objeto carregado; `tailwind = {css, baseColor}` (SC-001) |
| `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` | zero correspondência em `apps/frontend/components.json`, código, scripts ou CI; apenas os registros históricos em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*` (SC-002) |
| `cd apps/frontend && bunx prettier --check components.json` | `exit 0` — tabs, aspas duplas em JSON, nenhuma trailing comma (`prettier.config.js:5-7`), sem reformatar (SC-003) |
| `cd apps/frontend && bun run check` | `exit 0` — sem regressão de tipos (SC-003) |
| `cd apps/frontend && bun run lint` | `exit 0` — inclui `prettier --check .` (SC-003) |
| `cd apps/frontend && bun run test:unit -- --run` | `42/42` — suíte unitária verde (SC-003) |
| `cd apps/frontend && bun run build` | `exit 0` (SC-003) |
| `python3 scripts/ci/check_gate_integrity.py` | gate do alvo **PASS**; `exit 0` — nenhum comando acima lê `cssVariables` em runtime (SC-003) |
| `git diff --name-only 8f24519fe` | somente `apps/frontend/components.json` + artefatos do ciclo 082 (5 de assessment + documentos); zero PRs abertos (SC-004) |
| `specs/082-remove-css-variables-key/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}` · `exit=0` |
| Verificador isolado `CssVerifier` | **PASS** sem findings — remoção exata de `:7` e ajuste da vírgula em `:6`, `tailwind` com exatamente `{css, baseColor}`, `style` preservado (declarado no schema, só deprecado em v4), `$schema`/`css`/`baseColor`/`aliases` byte-idênticos, diff restrito a 1 arquivo |

### Saídas coladas

Auditoria de escopo observada contra a base `8f24519fe`:

```
$ git diff --stat 8f24519fe -- apps/frontend/components.json
apps/frontend/components.json | 3 +--
 1 file changed, 1 insertion(+), 2 deletions(-)

$ git diff --name-only 8f24519fe
apps/frontend/components.json
(+ artefatos do ciclo 082 — .specify/assessments/082-remove-css-variables-key/ e specs/082-remove-css-variables-key/)
```

Edição exata em `apps/frontend/components.json` (bloco `tailwind`):

```
@@ -3,8 +3,7 @@
 	"style": "default",
 	"tailwind": {
 		"css": "src/routes/layout.css",
-		"baseColor": "zinc",
-		"cssVariables": true
+		"baseColor": "zinc"
 	},
 	"aliases": {
 		"components": "$lib/components",
  +1 -2
```

Objeto carregado (`json.load`, SC-001):

```
$ python3 -c 'import json; o=json.load(open("apps/frontend/components.json")); print(set(o["tailwind"]) == {"css","baseColor"})'
True
# cssVariables ausente em todos os níveis
```

Analisador (`specs/082-remove-css-variables-key/analyze.md`):

```
{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
exit=0
```

Decisão 80/20 (`.specify/assessments/082-remove-css-variables-key/decision.md`): `E=(2×5)/(1×1)=5,0` — `go (scoped)`, opção A.

Nota de proveniência: esta passagem de documentação não reexecutou gates, build, testes, formatters ou git — as saídas acima são a evidência consolidada do ciclo (implementação + verificador isolado `CssVerifier` **PASS** sem findings + orquestração). Nenhuma inferência além dela.

## Fronteira

Somente `apps/frontend/components.json` mudou no código: a linha `:7` (`"cssVariables": true`) foi removida e a vírgula em `:6` ajustada de `"baseColor": "zinc",` para `"baseColor": "zinc"` — `+1/−2` em 1 arquivo, sem reindentação. `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor `zinc` de `baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`) permanecem byte-idênticos ao base `8f24519fe`. **`style` não foi removido**: está declarado no schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19; `schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecado em v4; remover seria mudança de contrato fora do escopo. O schema declara `tailwind.properties = {css, baseColor, config}`, `required = ["css","baseColor"]` e `additionalProperties = false`, sem `cssVariables` em nenhum nível; o CLI `shadcn-svelte@1.8.1` descarta a chave via parser Zod `$strip` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`), sem erro de runtime. Todo o restante são artefatos do ciclo 082: `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, `specs/078-*`, worktree `state-closure-072`, specs alheias, DB/rede, segredos, `.tokens/` e lockfiles permanecem byte-idênticos e intocados.

## [NV]

Validação online do arquivo editado contra `https://shadcn-svelte.com/schema.json` (exigiria rede + edição; a decisão se sustenta no schema colado com URL e data de acesso e no parser `$strip` local); consumidores externos do `components.json` (repos fora do checkout podem esperar `cssVariables`; nenhum leitor oficial depende dela); CI ao vivo, Forgejo, PRs, aprovações, merge, tag, produção e fechamento de issues (zero PRs neste ciclo); leg RBAC/maintainer (ADRs, constituição, `FLOORS` e workflows dependem de aprovação, ciclo separado). Nenhuma inferência além da evidência local consolidada no ciclo.
