# Validation: 079 — instalação pinada do CLI `shadcn-svelte`

**Data**: 2026-11-19T21:18Z · **Branch**: `feature/079-install-shadcn-svelte` · **Baseline**: `ea5cd2069`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff --name-only ea5cd2069` | `apps/frontend/bun.lock` · `apps/frontend/package.json` — exatamente 2 paths de código (restante: artefatos do ciclo 079) |
| `git diff --stat ea5cd2069` | `apps/frontend/bun.lock \| 9 +++++++++` · `apps/frontend/package.json \| 1 +` — `2 files changed, 10 insertions(+)` |
| `git diff ea5cd2069 -- apps/frontend/package.json` | 1 linha adicionada em `devDependencies`: `+ "shadcn-svelte": "1.8.1",` (exata, sem `^`/`~`), entre `prettier-plugin-tailwindcss` e `svelte` |
| `git diff ea5cd2069 -- apps/frontend/bun.lock` | `+9` linhas: 1 na lista do workspace (`"shadcn-svelte": "1.8.1"`) + entradas `cn@0.3.3`, `commander@14.0.3`, `node-fetch-native@1.6.7` e `shadcn-svelte@1.8.1` (com `postcss@^8.5.10` já resolvido no lockfile) |
| `bun add -D -E shadcn-svelte@1.8.1` (em `apps/frontend`) | `exit 0` — instalação executada; `latest = 1.8.1` no registry em 2026-11-19 |
| `bun pm ls shadcn-svelte` (em `apps/frontend`) | `shadcn-svelte@1.8.1` no workspace do frontend |
| `bun run shadcn-svelte --help` (em `apps/frontend`) | `exit 0` — bin local (sem `bun x`); subcomandos `init`, `add`, `apply`, `registry build` listados; nenhum executado |
| `bun install --frozen-lockfile --dry-run` (em `apps/frontend`) | `exit 0` — sem diff de lockfile |
| `bun run prettier --check` / formatação | `exit 0` — `package.json` com tabs, sem trailing commas e aspas duplas (`prettier.config.js:5-8`) |
| `bun run check` (em `apps/frontend`) | `exit 0` — sem regressão de tipos |
| `bun run lint` (em `apps/frontend`) | `exit 0` — inclui `prettier --check .` |
| `bun run test:unit` | 42/42 |
| `bun run build` | `exit 0` |
| `python3 scripts/ci/check_gate_integrity.py` (raiz) | `PASS` — `exit 0` |
| `specs/079-install-shadcn-svelte/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}` · `exit=0` |
| Verificador isolado `ShadcnVerifier` | **PASS** — `components.json`, `layout.css`, os 51 componentes e `utils.ts` byte-idênticos ao base `ea5cd2069`; nenhum `init`/`add`/`apply` executado |

### Saídas coladas

Auditoria de escopo (`ea5cd2069`):

```
$ git diff --name-only ea5cd2069
apps/frontend/bun.lock
apps/frontend/package.json

$ git diff --stat ea5cd2069
 apps/frontend/bun.lock     | 9 +++++++++
 apps/frontend/package.json | 1 +
 2 files changed, 10 insertions(+)
```

Declaração exata em `apps/frontend/package.json`:

```
@@ -36,6 +36,7 @@
 		"prettier": "^3.8.3",
 		"prettier-plugin-svelte": "^4.1.0",
 		"prettier-plugin-tailwindcss": "^0.8.0",
+		"shadcn-svelte": "1.8.1",
 		"svelte": "^5.56.1",
 		"svelte-check": "^4.6.0",
 		"tailwind-variants": "^3.3.1",
```

Resolução em `apps/frontend/bun.lock` (trechos):

```
+        "shadcn-svelte": "1.8.1",                      # lista do workspace
+    "cn": ["cn@0.3.3", ...],
+    "commander": ["commander@14.0.3", ...],
+    "node-fetch-native": ["node-fetch-native@1.6.7", ...],
+    "shadcn-svelte": ["shadcn-svelte@1.8.1", "", { "dependencies": { "cn": "^0.3.0", "commander": "^14.0.0", "node-fetch-native": "^1.6.4", "postcss": "^8.5.10" }, "peerDependencies": { "svelte": "^5.0.0" }, "bin": { "shadcn-svelte": "./dist/index.mjs" } }, ...],
```

Analisador (`specs/079-install-shadcn-svelte/analyze.md`):

```
{"artifacts": 11, "errors": [], "fr": 5, "go": true, "sc": 4, "tasks": 5}
exit=0
```

## Fronteira

Somente `apps/frontend/package.json` (1 linha em `devDependencies`) e `apps/frontend/bun.lock` (entrada do pacote + transitivas) mudaram no código; todo o restante são artefatos do ciclo 079. `components.json`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/`, `src/lib/utils.ts`, `prettier.config.js`, `vite.config.ts`, gate/scripts/workflows (inclui `check_gate_integrity.py`), `.github/workflows/`, supply-chain, ADRs, constituição, `.specify/memory/STATE.md`, DB/rede (fora do registry), `.tokens/` e segredos permanecem byte-idênticos e intocados. `node_modules/` permanece ignorado e local. `init`, `add` e `apply` não foram executados — sobrescreveriam o CSS global e `components.json` e poderiam reinstalar os 51 componentes (docs oficiais, 2026-11-19). Nenhum path de código além dos 2 declarados foi tocado.

## [NV]

CLI não executado além de `--help` (`init`/`add`/`apply` deliberadamente não executados por serem destrutivos); CI ao vivo, Forgejo, aprovações, merge, produção e leg RBAC/maintainer. Upgrade futuro de versão, dist-tag `next` (`1.0.0-next.19`) e adoção de caret como nas demais devDependencies também ficam fora do ciclo — a pinagem `-E` é decisão registrada em `concept.md`. Nenhuma inferência além da evidência local, do registry e das fontes oficiais.
