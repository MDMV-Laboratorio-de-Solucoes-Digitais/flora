# Assess — shape: 113

**Data**: 2026-11-22

## Abordagem

Append-only em cada artefato de bug, com tabela `bug → evidência → status → [NV]`. Nenhuma linha antiga é reescrita; quando o arquivo não existe (OIDC), ele é criado como registro de fechamento.

## Riscos

Risco de "fechar" um bug que ainda falha em produção → mitigado com [NV] explícito e evidência local determinística; nenhuma afirmação remota não observada.
