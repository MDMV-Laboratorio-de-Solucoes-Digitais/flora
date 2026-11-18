# Research: 060 — Refresh verified counts

**Data:** 2026-11-18T03:49Z · **Evidência local determinística**; remoto **[NV]**.

## Primária (executada)

1. `find specs -type f | wc -l` → 224; `find specs -mindepth 1 -maxdepth 1 -type d | wc -l` → 50.
2. `.specify/assessments` → 47 dirs / 192 arquivos; validations 46/47;
   releases 46/46; bugs 4/12; docs/research 135 (sem `index.md`); docs/adr 6.
3. `docs/PROGRESSO.md:11` → nota anterior com 02:23Z e números menores
   (47/202, 44/175, 42/43, 42/42) — histórico, não erro de fato.
4. `index.md:15` defere explicitamente a contagem a `PROGRESSO` — reforça
   que a nota precisa refletir o estado atual.

## Decisão

Acrescentar uma nova seção datada em `docs/PROGRESSO.md` com as contagens
atuais e a âncora `262ac042f`, sem tocar na nota de 02:23Z. Nenhum código,
migração ou gate muda.

## Alternativas descartadas

- Reescrever a tabela antiga: quebraria append-only e histórico.
- Atualizar `docs/research/index.md`: contagem de research não mudou (135).
- Editar `index.md`: não contém números próprios; apenas deferência.
- N3: escalado por risco de migração/ADR-0006 — não executado.

## Cobertura idiomática

PT-BR (docs do alvo) + EN (caminhos/âncora). **[NV]** para estado remoto.
