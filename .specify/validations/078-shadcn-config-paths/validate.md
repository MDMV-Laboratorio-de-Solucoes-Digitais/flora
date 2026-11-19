# Validation: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

**Data**: 2026-11-19T20:38Z · **Branch**: `feature/078-shadcn-config-paths` · **Baseline**: `de141fc53`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff -- apps/frontend/components.json` | 1 linha removida (`"config": "tailwind.config.ts",`) + 1 valor trocado (`src/app.css` → `src/routes/layout.css`); nenhum outro arquivo de código |
| `git diff --name-only de141fc53..HEAD` | `apps/frontend/components.json` — exatamente 1 arquivo de código (restante: artefatos do ciclo 078) |
| `python3 -c` com `json.load` em `components.json` | `{'css': 'src/routes/layout.css', 'baseColor': 'zinc', 'cssVariables': True}` — chave `config` ausente |
| schema oficial `https://shadcn-svelte.com/schema.json` (acessado 2026-11-19) | `tailwind.required = ["css","baseColor"]`, `additionalProperties=false`; `config` (deprecado no v4) removido — objeto válido |
| `ls apps/frontend/tailwind.config.*` / `ls apps/frontend/src/app.css` | ambos vazios (`No such file or directory`) — os caminhos antigos não existem |
| `apps/frontend/src/routes/layout.css:1` e `prettier.config.js:11` | `@import 'tailwindcss';` · `tailwindStylesheet: './src/routes/layout.css'` — mesmo caminho |
| varredura de consumidores (`tailwind.config` / `src/app.css`) | zero correspondências no código; apenas menções nos artefatos do ciclo 078 |
| `bunx prettier --check apps/frontend/components.json` | `exit 0` |
| `bun run check` | 0 erros |
| `bun run lint` | `exit 0` |
| `bun run test:unit -- --run --project server` | 42/42 |
| `python3 scripts/ci/check_gate_integrity.py` | `PASS` — `exit 0` |
| `specs/078-shadcn-config-paths/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}` · `exit=0` |
| Verificador isolado `ComponentsVerifier` | **PASS** — diff exato, schema oficial satisfeito (required css+baseColor, config removido), CSS real carregado, zero consumidores locais; ressalva pré-existente: `style`/`cssVariables` fora de escopo e [NV] |

### Saídas coladas

Diff observado contra a base `de141fc53` (1 arquivo, 1 remoção + 1 troca):

```
diff --git a/apps/frontend/components.json b/apps/frontend/components.json
index 00da7d737..8afff4cdc 100644
--- a/apps/frontend/components.json
+++ b/apps/frontend/components.json
@@ -2,8 +2,7 @@
 	"$schema": "https://shadcn-svelte.com/schema.json",
 	"style": "default",
 	"tailwind": {
-		"config": "tailwind.config.ts",
-		"css": "src/app.css",
+		"css": "src/routes/layout.css",
 		"baseColor": "zinc",
 		"cssVariables": true
 	},
```

`json.load` (stdlib) sobre `apps/frontend/components.json`:

```
{'css': 'src/routes/layout.css', 'baseColor': 'zinc', 'cssVariables': True}
```

Analisador (`specs/078-shadcn-config-paths/analyze.md`):

```
{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
exit=0
```

## Fronteira

Somente `apps/frontend/components.json` (linhas 5-6: 1 linha removida + 1 valor trocado) mudou no código; todo o restante são artefatos do ciclo 078. `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` permanecem byte-idênticos, com tabs, sem trailing commas e aspas duplas. `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows (inclui `check_gate_integrity.py`), supply-chain, ADRs, constituição, `.specify/memory/STATE.md`, DB/rede, `.tokens/` e segredos permanecem intocados. `style` deprecado no Tailwind v4 e `cssVariables` não declarado no schema ficam fora de escopo (preservados por escolha de escopo mínimo, não são regressão deste ciclo).

## [NV]

Validação pelo CLI externo `shadcn-svelte` não instalado (exigiria rede); validação de schema em runtime não executada (cobertura por schema/docs oficiais e checagens locais determinísticas); extensões de arquivo não varridas na busca de consumidores; `style` deprecado e `cssVariables` fora do schema, preservados por escopo mínimo; CI ao vivo, Forgejo, aprovações, merge, produção e leg RBAC/maintainer. Nenhuma inferência além da evidência local.
