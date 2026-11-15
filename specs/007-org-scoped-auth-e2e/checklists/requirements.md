# Requirements Checklist — 007 Org-Scoped Auth + Verifiable Core Slice

## Scoping & decision
- [x] Intake registrado.
- [~] Auditorias/pesquisa/workflow consolidados — parcial (2/4 subagentes incorporados na síntese).
- [x] Decision registrada (`go`, escopo parcial executado, deferrals/NV explícitos).

## Implementation targets
- [x] Seleção de org autenticada com membership/sessão válidas e emissão de JWT/sessão org-scoped (rotação de `jti`).
- [~] Refresh/logout consistentes com novo fluxo — comportamento existente compatível; sem alterações adicionais.
- [~] Smoke/caminho verificável (Bearer) definido/atualizado; execução integrada **NV** sem token/ambiente OIDC+DB.

## Verification & delivery
- [x] fmt/check/tests relevantes executados com evidência local.
- [~] PR GitHub com CI verde, merge e sync mirror — em andamento.
- [x] Limitações/NV e impactos de compatibilidade explícitos.
