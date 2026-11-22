# Decision: 107 — wildcard one-shot

**Data**: 2026-11-22 · **Veredito: go**

## Decisão

Implementar D11 com papel administrativo built-in, marcador persistente one-shot e ADR de expiração. Rejeitadas as alternativas em memória, em `organizations.settings` e em `users`.

## Racional

Correção autorizada pelo SE, determinística e verificável com os gates existentes; nenhum required check novo; risco concentrado numa migration aditiva e num helper SQL compartilhado.

## [NV]

Staging/produção, credenciais, restart multiréplica. Nenhuma credencial real versionada.
