# Assess — research: 114

**Data**: 2026-11-22 · **Cobertura idiomática**: pt-BR (código/dicionário/governança) + en (WCAG/axe)

## Evidência local (`github/dev@b32f7cafe`)

- `apps/frontend/tests/e2e/seed-flow.e2e.ts:8-20`: `WCAG_TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa']`; `auditAccessibility(page, route)` chama `process.stdout.write` e `test.info().annotations.push`, sem asserção `expect(...)`.
- `apps/frontend/tests/e2e/seed-flow.e2e.ts:21-79`: `select-org` consome o `jti` de uso único uma única vez; as navegações seguintes (`/channels`, `/tasks`) reutilizam a sessão já rotacionada, logo adicionar `await page.goto('/files')` ao final do mesmo teste não consome um novo `jti`.
- Log do job `e2e-browser` no PR #371 (run `45052173583`, job `137299501951`, head `715120c10`):
  - `WCAG 2.2 AA em /tasks: sem achados`
  - `WCAG 2.2 AA em /channels: sem achados`
  - `4 passed (38.7s)`
- `apps/frontend/src/routes/(app)/files/+page.svelte:74-99`: possui `<h1 class="text-2xl font-bold">{t['files.title']}</h1>`, `<Label for="files-workspace">` associado a `<select id="files-workspace">`, `<FileUpload>`, `<h2>` e `<Button>`.
- `apps/frontend/src/lib/components/files/FileUpload.svelte:57-60`: div de erro usa `text-red-500` (`#ef4444`, ~3,62:1 sobre `#f8fafc`) e `<input type="file" class="text-sm" onchange={handleUpload} />` sem `aria-label` nem `<label>`.
- `apps/frontend/src/lib/components/files/FilePreview.svelte:14-24`: `<svg class="h-4 w-4" ...>` sem `aria-hidden="true"`, link `<a>` usa `text-blue-500 hover:underline` (`#3b82f6`, ~3,54:1 sobre `#ffffff` / `#f8fafc`).

## Cálculo de contraste WCAG 2.2 AA (1.4.3 ≥ 4,5:1)

Sobre fundo claro `#f8fafc` (`bg-background` `210 40% 98%`) e escuro `#0f172a` (`222.2 84% 4.9%`):
- `text-red-700` (`#b91c1c`) sobre `#f8fafc`: **6,18:1** · `dark:text-red-400` (`#f87171`) sobre `#0f172a`: **6,45:1**.
- `text-blue-700` (`#1d4ed8`) sobre `#f8fafc`: **6,41:1** · `dark:text-blue-400` (`#60a5fa`) sobre `#0f172a`: **7,02:1**; `underline` garante distinção não dependente apenas de cor (WCAG 1.4.1).

## Fontes primárias

- W3C WCAG 2.2 (SC 1.4.1, 1.4.3, 4.1.2) — https://www.w3.org/TR/WCAG22/ (W3C, 2023-10-05; revalidação de critérios aplicados desde o ciclo 112).
- axe-core rule descriptions (`label`, `color-contrast`, `svg-img-alt`) — https://github.com/dequelabs/axe-core/blob/develop/doc/rule-descriptions.md (acesso em 2026-11-22).

## [NV]

Varredura em tema escuro, staging real e produção permanecem `[NV]` (o fixture E2E executa no tema claro padrão).
