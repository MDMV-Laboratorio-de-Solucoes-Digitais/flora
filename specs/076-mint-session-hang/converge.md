# Converge: 076 — hang do setup E2E do minter de sessão

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T19:12Z — entrada 1

- Decisão 80/20 (E=5.00) registrada no ciclo: `go (scoped)`, opção A — mover a
  pré-compilação de `flora-mint-session` para o `globalSetup` do Playwright, sem
  tocar código de produção, workflows, `seed.setup.ts`, STATE ou DB compartilhado.
  ADR-0003 drift e leg RBAC dependem de maintainer **[NV]**.
- Evidência consolidada dos scouts: runtime do binário pronto 0.00s (direto) e
  0.39s com target quente; `cargo build -p flora-core --bin flora-mint-session
  --features dev-mint` leva 30.18s com cache (target totalmente frio **[NV]**);
  `test.setTimeout(180_000)` em `seed.setup.ts:14` é a janela superior preservada;
  o CI já pré-compila em `.github/workflows/rust.yml:192-195`, e o job
  `e2e-browser` é `continue-on-error: true` em `rust.yml:150`.
- `analyze` PASS na primeira execução, sem correção de fonte:
  `artifacts=11, errors=[], fr=5, sc=4, tasks=5, go=true` (exit 0); FR-001..005 e
  SC-001..004 presentes em `plan.md`/`tasks.md`, T001..T005 únicos e sequenciais,
  `go (scoped)` e `[NV]` presentes.
- Implementação restrita a 2 paths:
  - `apps/frontend/playwright.config.ts` — uma linha, `globalSetup: './playwright/global-setup.ts',`.
  - `apps/frontend/playwright/global-setup.ts` (novo) — `execFileSync('cargo',
    ['build', '-q', '-p', 'flora-core', '--bin', 'flora-mint-session',
    '--features', 'dev-mint'], { cwd: ROOT, stdio: 'inherit' })` com a raiz
    resolvida a partir da localização do arquivo, como em `seed.setup.ts:6`.
- Verificação determinística executada: `playwright test --list` listou 4 testes
  (o `global-setup.ts` carregado pela chave `globalSetup`, não coletado como
  teste); `bun run check` 0 erros; 42/42 testes unitários PASS; gate
  `check_gate_integrity.py` PASS; `GLOBAL_SETUP_OK` executado via bun confirmou o
  caminho do `globalSetup`; `apps/frontend/tests/e2e/seed.setup.ts` byte-a-byte
  idêntico (linha 14 e bloco 36-53 preservados).
- Verificador isolado (1ª passagem): **PASS** funcional — pré-build fora da
  janela de 180s, stdio `inherit` para falha explícita, diff restrito, nenhuma
  mutação de DB.
- **Lint RED (pendente de confirmação)**: `bun run lint` falhou em
  `global-setup.ts:7:37` com `@typescript-eslint/no-unused-vars` no parâmetro
  `_c` (import de tipo `FullConfig` também não usado). A correção — remover o
  parâmetro e o import, mantendo `export default function globalSetup(): void` e
  o corpo idêntico — foi despachada em paralelo; o resultado do lint após a
  correção ainda não foi verificado nesta entrada e deve ser confirmado por
  apêndice. Formatters/gates não foram reexecutados por este registro.
- Dívidas e **[NV]**: duração exata com target totalmente frio não medida;
  status de CI ao vivo, merge, proteção de branch, Forgejo e produção não
  verificados; leg RBAC depende de aprovação de maintainer. DB compartilhado não
  inspecionado — garantia de não-mutação por construção.

## 2026-11-19T19:14:06Z — confirmação do lint (append-only)

- Correção aplicada pelo agente `FixLint076` em `apps/frontend/playwright/global-setup.ts`: removidos o parâmetro `_c` e o import de `FullConfig`; assinatura final `globalSetup(): void`, corpo inalterado.
- Reexecução determinística pelo orchestrator: `bunx prettier --check playwright.config.ts playwright/global-setup.ts` → `All matched files use Prettier code style!` exit 0; `bun run lint` → exit 0; `npx playwright test --list` → `Total: 4 tests in 3 files` exit 0; `python3 scripts/ci/check_gate_integrity.py` → PASS.
- O RED registrado acima está, portanto, fechado. Nenhuma linha anterior foi reescrita.
