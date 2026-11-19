# Implementation Plan: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/078-shadcn-config-paths/{intake,research,define,concept,decision}.md` (go scoped, opção A).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`.

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001 |
| T002 | FR-002 | SC-002 |
| T003 | FR-003 | SC-003 |
| T004 | FR-004 | SC-003, SC-004 |

## Passos

1. **T001** Remover exatamente a linha 5 de `apps/frontend/components.json`: `"config": "tailwind.config.ts"`, deixando o objeto `tailwind` sem a chave deprecada. Validar com `json.load` que `config` está ausente e que `baseColor`, `cssVariables` e `aliases` permanecem inalterados. *(FR-001, SC-001)*
2. **T002** Trocar, em `apps/frontend/components.json:6`, `"css": "src/app.css"` por `"css": "src/routes/layout.css"`. Confirmar que o arquivo existe, contém `@import 'tailwindcss'` e casa com `tailwindStylesheet` em `prettier.config.js:11`; `+layout.svelte`, `layout.css` e `vite.config.ts` não são tocados. *(FR-002, SC-002)*
3. **T003** Preservar byte-idênticos `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases` (`components.json:1-4,7-16`), com tabs, sem trailing commas e aspas duplas em JSON. Validar com `bunx prettier --check components.json` (em `apps/frontend`) e `json.load`. *(FR-003, SC-003)*
4. **T004** Auditar o escopo do diff (base `de141fc53`), confirmar que nenhuma dependência, segredo, `.tokens/`, gate/script/workflow, DB ou rede foi tocado, executar os gates determinísticos abaixo, registrar os `[NV]` em `spec.md`/`clarify.md` e confirmar que a reversão do commit restaura o arquivo original. *(FR-004, SC-003, SC-004)*

## Gates determinísticos

```
# 1. estrutura do JSON via json.load (stdlib, sem rede) — a partir da raiz
python3 - <<'PY'
import json
p = 'apps/frontend/components.json'
with open(p, encoding='utf-8') as f:
    c = json.load(f)
tw = c['tailwind']
assert 'config' not in tw, 'tailwind.config ainda presente'
assert tw['css'] == 'src/routes/layout.css', tw['css']
assert tw['baseColor'] == 'zinc' and tw['cssVariables'] is True, tw
assert c['$schema'] == 'https://shadcn-svelte.com/schema.json'
assert c['style'] == 'default'
assert c['aliases'] == {
    'components': '$lib/components', 'utils': '$lib/utils',
    'ui': '$lib/components/ui', 'hooks': '$lib/hooks',
}, c['aliases']
print('OK json.load:', tw)
PY

# 2. css existe, importa Tailwind e casa com prettier.config.js:11
test -f apps/frontend/src/routes/layout.css
grep -q "^@import 'tailwindcss';" apps/frontend/src/routes/layout.css
test "$(grep -c "src/routes/layout.css" apps/frontend/prettier.config.js)" -eq 1

# 3. formatação do Prettier e gates do frontend (em apps/frontend)
cd apps/frontend
bunx prettier --check components.json
bun run check
bun run lint
bun run test:unit
cd -

# 4. integridade de gate (raiz, sem DB/rede)
python3 scripts/ci/check_gate_integrity.py

# 5. auditoria de escopo do diff (base de141fc53)
git diff --name-only de141fc53..HEAD
```

A auditoria de escopo espera somente `apps/frontend/components.json` e os artefatos de `specs/078-shadcn-config-paths/` (mais os artefatos de assessment já registrados em `.specify/assessments/078-shadcn-config-paths/`); qualquer outro path invalida SC-004. O diff do `components.json` deve conter exatamente 1 linha removida (`config`) e 1 valor trocado (`css`).

## Rollback

Reverter o commit que introduziu T001/T002, reintroduzindo `"config": "tailwind.config.ts"` na linha 5 e `"css": "src/app.css"` na linha 6 de `apps/frontend/components.json`, restaurando o arquivo original byte-a-byte. Nenhum `style`, `baseColor`, `cssVariables`, `$schema`, `aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/script, workflow, ADR, constituição, STATE, DB, rede ou segredo é modificado. Os artefatos do ciclo 078 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.
