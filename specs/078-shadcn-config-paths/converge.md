# Converge: 078 — caminhos de configuração do `components.json` (shadcn-svelte)

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T20:38Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/078-shadcn-config-paths/decision.md`):
  `E=(2×5)/(1×1)=10,00` — candidato 078 eleito sobre as alternativas GIN-JSONB
  (2,00), OTLP (1,67) e Tauri/ADR (depende maintainer); leg RBAC bloqueada por
  maintainer **[NV]**. Veredito **go (scoped), opção A**: remover exatamente a
  linha `"config": "tailwind.config.ts"` de `tailwind` em
  `apps/frontend/components.json:5` e trocar `"css": "src/app.css"` por
  `"css": "src/routes/layout.css"` na linha 6, preservando byte-idênticos
  `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco
  `aliases`.
- **Evidência determinística consolidada (orchestrator)**, base `de141fc53`,
  branch `feature/078-shadcn-config-paths`, gate PASS, zero PRs:
  - `git diff` = 1 path: `apps/frontend/components.json` — semanticamente
    1 linha removida (`config`) e 1 valor trocado (`css` de `src/app.css` para
    `src/routes/layout.css`); estatística `1 insertion(+), 2 deletions(-)`.
  - `json.load` → `{'css':'src/routes/layout.css','baseColor':'zinc',
    'cssVariables':True}`; sem chave `config`; `$schema`, `style` e `aliases`
    intactos.
  - `bunx prettier --check components.json` (em `apps/frontend`) → exit 0.
  - `bun run check` → 0 erros; `bun run lint` → exit 0;
    `bun run test:unit -- --run --project server` → 42/42.
  - `python3 scripts/ci/check_gate_integrity.py` → PASS.
  - CSS real `src/routes/layout.css` existe, importa `@import 'tailwindcss';` e
    casa com `tailwindStylesheet` em `prettier.config.js:11`; varredura
    determinística encontrou zero consumidores locais de `config`/`css`.
  - Fontes oficiais: `https://shadcn-svelte.com/docs/components-json` e
    `https://shadcn-svelte.com/schema.json` (acesso 2026-11-19) — `css` e
    `baseColor` são obrigatórios, `config` marcado como deprecado no Tailwind v4.
- **Analyzer** (`analyze.md`, READ-ONLY): `artifacts=11, errors=[], fr=4, sc=4,
  tasks=4, go=true` (exit 0) na 1ª execução. FR-001..FR-004, SC-001..SC-004 e
  T001..T004 presentes em `plan.md`/`tasks.md`, únicos e sequenciais; `[NV]` e
  `go (scoped)` presentes; nenhuma correção de fonte necessária. Analyze não
  alterou arquivos de código.
- **Verificador isolado `ComponentsVerifier`**: **PASS** — diff exato (somente
  `components.json`, 1 linha removida + 1 valor trocado), schema oficial
  satisfeito (`css` + `baseColor` required; `config` removido), CSS real
  carregado e coerente com `prettier.config.js`, zero consumidores locais;
  reversão do diff restaura o arquivo original byte-a-byte. Ressalva
  pré-existente e fora de escopo **[NV]**: `style` deprecado no v4 e
  `cssVariables` não declarado no schema foram preservados por escolha de
  escopo mínimo, sem bloqueio.
- **Dívidas e [NV]**:
  - CLI externo `shadcn-svelte` não instalado; validação pelo CLI (`bunx`
    exigiria rede) não executada **[NV]**.
  - Validação de schema em runtime não executada neste ciclo; conformidade
    afirmada apenas contra o schema oficial estático acessado em 2026-11-19
    **[NV]**.
  - Status de CI ao vivo, merge, PRs, Forgejo e produção não verificados **[NV]**.
  - Leg RBAC depende de aprovação de maintainer **[NV]**.
  - Extensões não varridas pela varredura determinística; consumidores em
    outras extensões são **[NV]**.
- **Próxima portaria**: emitir validate (R4) e release (SemVer proposto
  `0.2.8 → 0.2.9`) em `.specify/validations/078-shadcn-config-paths/` e
  `.specify/releases/078-shadcn-config-paths/`, respectivamente; rollback
  restrito a `apps/frontend/components.json`. Nenhuma tag, deploy ou merge
  executados neste ciclo — PR a critério do maintainer.

converge written: 1 file; gates/formatters skipped
