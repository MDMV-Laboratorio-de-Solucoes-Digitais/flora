# Analysis: `121-settings-theme-selector`

- **Consistência com `app.html`**: O script inline de `apps/frontend/src/app.html` lê `localStorage.getItem('flora.theme') === 'dark'` e adiciona `.dark` no `<html>`. O store usa exatamente `THEME_STORAGE_KEY = 'flora.theme'` e escreve `'dark'` ou `'light'`, eliminando divergência entre carregamento inicial e alternância em runtime.
- **SSR & ESLint (`unicorn`)**: `vite.config.ts` executa testes de unidade no projeto `server` (`environment: 'node'`). Toda leitura de `globalThis.localStorage` e `globalThis.document?.documentElement` deve usar `globalThis`, checagens defensivas e `try/catch`, permitindo testabilidade pura com doubles de `storage` e `root`.
- **Contraste `--destructive`**: Em tema claro (`--background: 210 40% 98%`, `--card: 0 0% 100%`), `--destructive: 0 84.2% 60.2%` tem contraste `3.59:1` (< `4.5:1`). Reduzir luminosidade para `40%` (`0 84.2% 40%`, hex `#BD1414`) garante contraste > `6.2:1` em `--background`/`--card` e > `5.2:1` em `bg-destructive/10`.
- **Preservação de pisos DB**: Nenhuma alteração em Rust, SQL ou `flora-tests`; os `85` pisos permanecem inalterados.
