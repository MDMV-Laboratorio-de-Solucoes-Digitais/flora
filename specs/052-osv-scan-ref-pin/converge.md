# Converge: 052 — OSV scan ref pin

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-17T23:44Z — entrada 1

- Decisão R1 `go (scoped)`; assess (intake/research/define/shape/decision), spec,
  clarify, plan, checklist, tasks e analyze criados. `analyze` apontou conflito
  de IDs SC em `define.md`; corrigido na fonte por apêndice e reexecutado → PASS.
- Implementação: `.github/workflows/osv-scan.yml:14` passa de `@v2` (ref inexistente)
  para `@v2.7.1` (verificado por `git ls-remote` + docs oficiais). Diff de código:
  1 arquivo, 1 linha, 1 inserção/1 remoção. Inputs inalterados.
- `.gitignore`: ignore de runtime `.specify/queue/` (ledger + worktrees), com
  diff explícito registrado; rótulo do comentário corrigido.
- Gates (2026-11-17T23:42–23:43Z, na worktree da feature):
  `check_gate_integrity.py` PASS; fmt exit 0; clippy exit 0; deny exit 0
  (bans/licenses/sources ok); build exit 0; test exit 0 (34+15+21+35 passed,
  0 failed, 50 ignored no total do workspace); frontend check 0 erros/0 warnings,
  lint ok, unit 42/42, build exit 0.
- Gateway previsto: PR para `dev/` com CI verde observada ao vivo antes do merge.
- Nenhum PR/worktree/stash alheio tocado; factory lida em
  `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`, nada materializado no alvo.
