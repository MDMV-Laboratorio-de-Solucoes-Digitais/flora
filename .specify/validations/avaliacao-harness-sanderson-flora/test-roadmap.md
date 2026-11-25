# Roteiro de testes — assessment harness Sanderson (flora)

**Slug**: `avaliacao-harness-sanderson-flora` · **Data**: 2026-11-24 · **Superfície**: CLI/artefatos versionados (sem UI de produto alterada)

## Escopo e IDs

Validação documental e de governança: intake → research → define → shape → decision; 34 propostas P01–P34; nenhuma mudança de código de produto, gate, CI, migration ou credencial. Não há FR/T de spec nova; a âncora é o assessment, não um ciclo de implementação.

## Pré-condições

- Worktree isolada `assessment/avaliacao-selfevolving-tools-flora` a partir de `github/dev@0ef79ec60`.
- Paths `.specify`, `.specify/assessments` e o diretório do slug são diretórios reais, não symlinks, e resolvem dentro da raiz da worktree.
- Trabalho alheio nos checkouts/worktrees compartilhados permanece intocado.

## Passos

1. **Segurança de caminho** — resolver `.specify`, `.specify/assessments` e `.specify/assessments/avaliacao-harness-sanderson-flora` com `Path.resolve()`, afirmar `is_dir()` e ausência de symlink. Resultado esperado: três linhas `is_symlink=False`, `inside=True`. Automatizável: sim (Python stdlib). Captura: terminal.
2. **Artefatos obrigatórios** — testar existência e não-vazio de `intake.md`, `research.md`, `define.md`, `shape.md`, `decision.md`. Resultado esperado: 5 arquivos, 0 vazios. Automatizável: sim. Captura: terminal.
3. **Estrutura do intake** — afirmar presença de `# Idea Intake:`, `## Idea (as captured)`, `## Restated`, `## Origin & Context`, `## First-Glance Unknowns`, `**Slug**:` e `[NEEDS CLARIFICATION:`. Resultado esperado: `intake-structure: OK`. Automatizável: sim. Captura: terminal.
4. **Matriz completa** — contar linhas `^| P` em `decision.md` e validar vereditos contra `{ADAPT,ADOPT,DEFER,AVOID,REJECT}`; afirmar 34 linhas e nenhum rótulo inválido. Resultado esperado: `decision-matrix: OK (34)`. Automatizável: sim. Captura: terminal.
5. **Veredito e rastreabilidade** — afirmar `Veredito: go` em `decision.md`, presença de seção de fontes com URLs `https://` e datas, e declaração de cobertura idiomática. Resultado esperado: três afirmativas verdadeiras. Automatizável: sim. Captura: terminal.
6. **Correções e injeções** — afirmar que `research.md` registra divergências factuais do vídeo, injeções detectadas em fontes externas e que nenhuma foi executada. Resultado esperado: seções presentes. Automatizável: sim (verificação textual). Captura: terminal.
7. **Gate determinístico do ALVO** — `python3 scripts/ci/check_gate_integrity.py` na worktree. Resultado esperado: `gate-integrity: PASS`, exit 0. Automatizável: sim. Captura: terminal (saída completa em `evidence.md`).
8. **Higiene do diff** — `git diff --check` e `git status --short` na worktree; confirmar apenas paths do assessment e notas append-only autorizadas. Resultado esperado: `git diff --check` sem saída; status sem arquivos de runtime/credencial. Automatizável: sim. Captura: terminal.
9. **Preservação alheia** — `git status --short` no checkout compartilhado e `git worktree list` confirmam que os paths sujos e worktrees de outras sessões continuam presentes e intocados. Resultado esperado: nenhuma mudança introduzida por este ciclo fora da worktree própria. Automatizável: sim (comparação com baseline do Bloco 0.b). Captura: terminal.
10. **Escalonamento** — confirmar que nenhum PR `dev→main` (#410) foi mesclado, nenhum deploy/auto-merge/force-push executado e que o relatório final lista `[NV]` remotos. Resultado esperado: `gh pr view 410 --json state` = `OPEN` e registro no `evidence.md`. Automatizável: sim (leitura). Captura: terminal.

## Plano de capturas

Saídas de terminal dos passos 1–10 coladas em `evidence.md` com data/hora; sem imagens (superfície CLI). Dados sensíveis (tokens) não são impressos.

## Rollback

Artefatos são novos ou append-only. Reversão = remover os paths criados por esta sessão na worktree e restaurar notas append-only ao conteúdo anterior via commit reverso explícito; nenhum gate/CI/workflow é modificado. A worktree pode ser removida sem afetar `dev`/`main`.
