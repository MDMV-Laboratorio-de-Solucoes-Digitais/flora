# ADR-0006 — Rollout de RLS sem condicionar required checks

**Date:** 2026-11-17
**Status:** Accepted
**Scope:** Flora `flora` repository — CI / rollout

## Context

ADR-0003 define merge como decisão de máquina sob quatro required checks
(`frontend-build`, `rust-build`, `db-tests` com piso 51, `dependency-review`); GitHub
conta `skipped` como sucesso, então `if:`/`continue-on-error` em required check criaria
um gate verde que não testa nada. A flag de rollout não pode seguir esse caminho.
Além disso, a prova do ADR-0005 mostrou que **ligar enforcement hoje quebraria a app**
(não há transação por request).

## Decision

1. A promoção do enforcement RLS é **uma mudança de código em duas etapas**, não um flip
   de config: (a) introduzir wrapper de transação que faça `set_config(...,true)` como
   primeira instrução a partir de `require_org()`; (b) só então ativar as policies/FORCE
   no runtime. Até (a), o valor de enforcement permanece no modo atual (app-only) e
   as policies vivem apenas no DB descartável de prova.
2. Nenhuma flag de rollout poderá condicionar um required check — nem `if:`, nem
   `continue-on-error`, nem divisão que resulte em `skipped`. Se no futuro forem criadas
   suítes separadas (`app_only` / `enforced`), **ambas devem rodar sempre** e serem
   required, preservando ADR-0003.
3. Deploy em produção, troca de credenciais da role, mudança de required
   checks/protections/CODEOWNERS e tag/release continuam exigindo autorização explícita
   do SE (`docs/research/2026-11-16-036-authorization-matrix.md:28,40`). A exceção é o
   deploy automático de `main` decidido pelo SWE em 2026-11-25 e registrado na emenda do
   ADR-0003: ele é disparado por push/merge sob guard determinístico, sem autorização
   casuística; as demais mudanças listadas permanecem sob autorização do SE.
4. Reversão = desativar policies/FORCE no DB e voltar ao filtro da app; nenhuma ação
   destrutiva de dados.

## Considered options

| Option | Why rejected |
|---|---|
| `if: RLS_ENABLED` dentro de required check | `skipped` = sucesso (ADR-0003); gate falso. |
| Ligar FORCE em produção direto | Zero linhas; sem evidência prévia (prova ADR-0005). |
| Só config, sem ADR | ADR-0001 §4 exige ADR para drift de mecanismo. |

## Consequences

- Positive: enforcement novo só entra após prova executada; rollback trivial; nenhum
  required check enfraquecido.
- Negative: mais uma etapa de código antes da promoção; duas modalidades a manter
  até adoção; custo de um leg DB adicional se as suítes forem separadas no futuro.
- Drift: ADR-0001 §4 e ADR-0003 — flag nunca condiciona gate; leg novo é sempre executado.

## Verification

**NV** até execução: (a) wrapper de transação com `set_config` realmente usado pelos handlers;
(b) suítes `app_only` e `enforced` verdes com contagens mínimas (piso 51); (c) rollup
sem `skipped` em nenhum required check; (d) prova cross-org falhando quando a policy é
removida (mutation test). Nada marcado verde sem execução (ADR-0001).

## Relationship

Depende de ADR-0004 e ADR-0005. Não altera o regime de merge autônomo do ADR-0003.

## Nota 2026-11-25 — deploy automático e required checks

- A promoção rotineira para produção por push/merge em `main` não condiciona required check algum: os quatro workflows de deploy ficam fora do conjunto `frontend-build`/`rust-build`/`db-tests`/`dependency-review` e são guardados por `scripts/ci/check_deploy_workflows.py`.
- `if:` e `continue-on-error` nas etapas de deploy não entram nos jobs obrigatórios; a regra desta ADR contra gate vazio permanece intacta.
- **[NV]** execução real do deploy e estado de RLS em produção; nenhuma autorização de flip de RLS foi derivada desta nota.
