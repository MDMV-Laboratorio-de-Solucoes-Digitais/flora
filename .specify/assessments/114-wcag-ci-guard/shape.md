# Assess — shape: 114

**Data**: 2026-11-22

## Abordagem

1. **Pure helper sem acoplamento a Playwright (`a11yAudit.ts`)**: recebe apenas um shape estrutural mínimo compatível com `AxeBuilder.analyze().violations` (`id`, `impact?`, `nodes[].target`). Isso permite testar formatação, fallback `'indefinido'` e status `passed` via Vitest `server` sem subir navegador nem importar `@axe-core/playwright` no código de `src/lib`.
2. **Correções cirúrgicas em `/files`**:
   - `FileUpload.svelte`: manter `<input type="file">` funcional e acessível via `aria-label={t['file.selectInput']}`; ajustar cor de erro para `text-red-700 dark:text-red-400`.
   - `FilePreview.svelte`: marcar `<svg>` decorativo com `aria-hidden="true"` e atualizar o link de download para `text-blue-700 underline hover:text-blue-800 dark:text-blue-400`.
3. **Hardening em `seed-flow.e2e.ts`**: `auditAccessibility` continua imprimindo `summary` via `process.stdout.write` e registrando `wcag-axe` annotation, e passa a exigir `expect(findings, summary).toEqual([])`. Passo 5 navega para `/files` após a criação de tarefa, valida título `h1` e `<select>` habilitado, e audita `/files`.

## Riscos e mitigação

- **Risco**: `e2e-browser` detectar outra violação real em `/files` no Chromium headless. **Mitigação**: `e2e-browser` roda no PR antes do merge em `dev`; qualquer achado real é corrigido em código antes do merge, jamais por supressão de regra.
- **Risco**: tema escuro não exercitado pelo Playwright smoke. **Mitigação**: contrastes `dark:` calculados e registrados; execução em tema escuro documentada como `[NV]` e oportunidade de automação futura.
