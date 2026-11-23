# Assess — intake: 114 — blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Data**: 2026-11-22 · **Origem**: avaliação contínua pós-ciclos 111/112 + auditoria do código em `github/dev@b32f7cafe`

## Problema

1. Em `apps/frontend/tests/e2e/seed-flow.e2e.ts:10-20`, `auditAccessibility()` executa `AxeBuilder` com tags WCAG 2.0/2.1/2.2 A+AA, mas permanece puramente informativa (só registra anotação e escreve no log) mesmo após o ciclo 112 ter zerado todas as violações em `/tasks` e `/channels`. Regressões futuras de acessibilidade nessas rotas passariam silenciosamente no `e2e-browser`.
2. A rota `/files` (ciclo 105) não é visitada nem auditada em `seed-flow.e2e.ts`, e possui defeitos determinísticos de acessibilidade WCAG 2.2 AA:
   - `apps/frontend/src/lib/components/files/FileUpload.svelte:60`: `<input type="file">` sem `<label>` nem `aria-label` (regra `label`, critical).
   - `apps/frontend/src/lib/components/files/FileUpload.svelte:58`: erro `text-red-500` sobre fundo claro `#f8fafc` com contraste abaixo de 4,5:1 (regra `color-contrast`, serious).
   - `apps/frontend/src/lib/components/files/FilePreview.svelte:14-24`: `<svg>` decorativo sem `aria-hidden="true"` e link de download com `text-blue-500` sem sublinhado estático e com contraste insuficiente em tema claro (regra `color-contrast`, serious).

## Pedido

Transformar a auditoria WCAG 2.2 AA do smoke E2E em asserção determinística que falha em qualquer violação (sem desabilitar regras axe), extrair lógica pura testável unitariamente em `a11yAudit.ts`, corrigir os defeitos de `/files` e estender `seed-flow.e2e.ts` para auditar `/files` junto de `/tasks` e `/channels`.

## Fora de escopo

Alteração de required checks ou `check_gate_integrity.py`, backend/Rust/migrations, endpoint `GET /api/v1/files` (candidato 115), tema escuro no E2E (permanece `[NV]`), deploy em produção.
