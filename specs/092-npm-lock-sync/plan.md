# Implementation Plan: 092 — npm lock sync

**Branch**: `feature/092-npm-lock-sync` | **Date**: 2026-11-20 |
**Spec**: [spec.md](./spec.md)

**Input**: `.specify/assessments/092-npm-lock-sync/decision.md` (go)

## Summary

Regenerar `apps/frontend/package-lock.json` a partir do manifesto com o comando
contratual `npm install --package-lock-only --ignore-scripts`, preservando
`bun.lock` como lockfile canónico da CI. Uma única alteração de ficheiro,
reversível por restauro via controlo de versões.

## Technical Context

**Primary Dependencies**: npm (lockfile-only), `apps/frontend/package.json`,
`apps/frontend/bun.lock`.

**Target Path**: `apps/frontend/package-lock.json` (único ficheiro alterado).

**Testing**: verificação por diff e reexecução idempotente do comando; gates de
build/lint/formatação ficam com a integração principal.

**Constraints**: `--ignore-scripts`; sem tocar em `bun.lock`, `package.json`,
source, CI, `validate`, `release` ou `STATE`; sem credenciais.

## Steps

1. **Evidência**: confirmar drift (`shadcn-svelte` ausente no lockfile npm,
   presente no manifesto/`bun.lock`), 25 alertas e comando contratual.
2. **Execução**: em `apps/frontend`,
   `npm install --package-lock-only --ignore-scripts`.
3. **Verificação**: presença de `shadcn-svelte`, idempotência, diff restrito ao
   lockfile npm, nenhum script executado.
4. **Fecho**: rastreabilidade registada em decision/spec/plan/tasks.

## Constitution Check

Sem violações: nenhuma nova dependência declarada, nenhuma alteração de
configuração de CI, nenhum código produzido e nenhuma credencial envolvida.

## Project Structure

```text
.specify/assessments/092-npm-lock-sync/decision.md
specs/092-npm-lock-sync/
├── spec.md
├── plan.md
└── tasks.md
apps/frontend/
├── package.json          # intacto
├── bun.lock              # canónico CI, intacto
└── package-lock.json     # único alvo
```

## Complexity Tracking

Nenhuma.
