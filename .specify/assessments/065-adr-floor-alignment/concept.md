# Concept: Alinhar as ADRs ao piso real do gate

- **Slug**: 065-adr-floor-alignment
- **Created**: 2026-11-25
- **Recommended option**: Atualizar as citações ao valor vigente do gate

## Options

### Option A — Atualizar as citações ao valor vigente do gate
- **Sketch**: As duas ADRs de governança deixam de citar a cifra antiga e passam a descrever o detalhamento e o total que o guardião realmente exige hoje. A autorização que permitiu o patch e a divergência entre o número citado pelo SE e o número corrente ficam registradas como nota datada, sem reescrever a história.
- **Appetite**: small — três citações literais em dois documentos.
- **Trade-offs**: Ganha coerência entre documentação e autoridade determinística, sem tocar em código. Sacrifica a cifra que constava da autorização original (50), que é substituída pelo valor real (51) com a divergência explicada. Risco nulo: é documentação.
- **Rabbit holes**: Mudar o gate para 50 para "bater" com a autorização; reescrever notas históricas de `STATE.md`; tocar a constituição para acomodar a mudança.

### Option B — Ajustar o gate ao número autorizado
- **Sketch**: Em vez de corrigir as ADRs, o guardião passa a exigir o total que o SE citou na autorização.
- **Appetite**: small.
- **Trade-offs**: Evita editar documentos de governança, mas contradiz o mapa de pisos real e quebra o determinismo que as ADRs descrevem — troca um número errado por outro.
- **Rabbit holes**: Alterar o mapa de pisos; revisar outros checks que dependem do total; propagar a divergência para mais artefatos.

### Option C — Não fazer
- **Sketch**: Manter "43" nas ADRs e aceitar a divergência com o gate como dívida conhecida.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Nenhum risco de edição, mas a documentação continua contradizendo a autoridade e o leitor não sabe qual número vale.
- **Rabbit holes**: — (nenhum; a incerteza permanece).

## Recommendation

**Option A**. O define.md fixa como sucesso a ausência da cifra antiga nas citações (SC-001), a igualdade com o número impresso pelo guardião (SC-002) e o diff restrito a dois ADRs (SC-004). A correção pós-verificação do próprio define.md registra o detalhamento correto (`13/4/8/8/5/6/4/3 — total 51`), após o workspace passar de 2 para 3 no ciclo 064; alinhar a esse valor evita a contradição interna que a versão `/4/2` geraria. A autorização SE e a divergência 50→51 permanecem registradas (FR-004).

## Out of Scope (for the recommended option)

- Alterar código, gate, constituição, C5/C6 ou workflows.
- Reescrever o histórico de `STATE.md` ou de converge.
- Mudar o total do guardião para 50.

## Assumptions to Validate

- O total corrente do guardião é 51 com detalhamento `13/4/8/8/5/6/4/3`; reconfirmar no checkout antes de datar as edições.
- A autorização do SE cobre o patch documental mesmo tendo citado o valor anterior (50) — a divergência já está registrada em `STATE.md`.
- Estado remoto/CI permanece **[NV]** nesta avaliação documental.
