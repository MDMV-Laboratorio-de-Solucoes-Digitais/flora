# Problem Definition: Listagem persistida de arquivos por workspace

- **Slug**: 115-files-list-persistence
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | define.md (equivalente local) | shape.md | decision.md

## Problem Statement

O frontend de `/files` não tinha uma fonte de verdade para listar arquivos do workspace selecionado: após upload, não havia recarga determinística, ordenação estável nem estado de carregamento/vazio, e o backend não expunha um endpoint de listagem com escopo explícito. Isso impede o fluxo de arquivos do Seed de ser verificável de ponta a ponta (`define.md:5-11`; `shape.md:5-16`).

## Affected Users & Stakeholders

- **Users**: membros da organização que precisam ver os arquivos do workspace atual, inclusive após troca rápida entre workspaces (`define.md:9`).
- **Stakeholders**: maintainer do gate e da tenancy, que precisa de isolamento cross-org e lockstep de pisos sem alterar required checks (`define.md:7`; `shape.md:15`).

## Goals

- Expor uma listagem filtrada por `workspace_id` com validação de sessão/membership e resposta sem `storage_path` (`define.md:5-7`).
- Manter o estado do frontend consistente em carga inicial, troca de workspace e dedup (`define.md:9`).
- Cobrir o fluxo com contrato DB e E2E auditado por WCAG (`define.md:7-11`).

## Non-Goals

- Alterar migrations/schema, `POST/GET` de download existente ou required checks (`define.md:11`).
- Permitir listagem sem `workspace_id` por retrocompatibilidade — `/files` já exige seleção na UI (`shape.md:9`).

## Success Metrics

- `GET /api/v1/files?workspace_id=...` retorna apenas arquivos do workspace/org da sessão (baseline: endpoint ausente — `define.md:5-7`).
- Contrato DB `files: 6 → 7` no gate (baseline: 6 — `define.md:7`).
- `/files` mostra `files.empty` e passa na auditoria WCAG no smoke (baseline: não verificado; `[NV]`).

## Cost of Inaction

O fluxo de arquivos continua dependente de estado local não verificável, com risco de cross-org e de race condition na troca de workspace; a lacuna #2 do Seed permanece aberta (`define.md:5-11`; `shape.md:13-16`).

## Open Questions

- [NEEDS CLARIFICATION: o DTO `FileAttachmentResponse` já contém todos os campos que o frontend precisa sem vazar `storage_path`?] (`shape.md:5-7`)
- [NEEDS CLARIFICATION: o piso `files: 7` e o total `78` permanecem corretos após outros contratos adicionados desde o ciclo 115?] (`define.md:7`)
