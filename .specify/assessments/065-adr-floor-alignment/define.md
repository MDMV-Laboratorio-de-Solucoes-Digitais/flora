# Define: 065 — ADR floor alignment

## Problema

ADRs de governança citam cifras de pisos obsoletas (43) enquanto o gate
determinístico exige 51 — divergência entre documentação e autoridade.

## Requisitos funcionais

- **FR-001**: `docs/adr/0003:19` DEVE refletir `13/4/8/8/5/6/4/2 — total 51`.
- **FR-002**: `docs/adr/0006:10,52` DEVEM refletir `piso 51`.
- **FR-003**: nenhum código/gate/constituição/C5/C6 DEVE mudar.
- **FR-004**: autorização SE e divergência 50→51 DEVEM ficar registradas.

## Critérios de sucesso

- **SC-001**: `grep -n 43 docs/adr/0003* 0006*` → 0 nas citações de piso.
- **SC-002**: cifra impressa pelo guardião = 51.
- **SC-003**: gates locais verdes.
- **SC-004**: diff = 2 ADRs + artefatos.

## Não-objetivos

- Não reescrever histórico de `STATE.md`/converge.
- Não alterar o gate para 50 (o valor real é 51).

## Correção pós-verificação (2026-11-18T13:12Z)

O detalhamento correto é `13/4/8/8/5/6/4/3 — total 51` (workspace passou de 2 para 3 no ciclo 064). A citação anterior `/4/2` somava 50 e contradizia o total; corrigida no ADR-0003:19.
