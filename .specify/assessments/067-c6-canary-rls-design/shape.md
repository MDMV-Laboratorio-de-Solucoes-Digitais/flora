# Shape: 067 — C6 canário RLS

## Forma escolhida

Um documento de desenho (`docs/research/2026-11-18-067-c6-canary-rls-design.md`)
com: (1) estado verificado; (2) modelo de ameaça e fronteira de confiança
(GUC é confiança da app, RLS é defesa em profundidade); (3) pré-condição ADR-0006(a)
— wrapper de transação — descrita mas **não implementada**; (4) quatro fases de
canário em DB descartável com critérios de entrada/saída; (5) SQL de referência
para 9 tabelas + `organizations` + ressalva de `users`; (6) matriz de verificação
com os itens (a)–(d) do ADR-0006 **[NV]**; (7) rollback em 6 passos sem dado
destrutivo; (8) registro explícito do que exige autorização do SE.

Acompanham: refresh append-only de `docs/PROGRESSO.md` e `docs/research/index.md`
com contagens reais e notas datadas de "estado verificado".

## Alternativas descartadas

- ADR novo: ADR-0005/0006 já cobrem o desenho e o rollout; duplicaria autoridade.
- Implementar o wrapper de transação agora: fora do escopo "documento, sem produção"
  e exigiria testes/DB; ficaria como pré-condição explícita da Fase 0.
- Migrations com policies comentadas: já existem; repeti-las não agrega evidência.
- Incluir `users`/`organizations` no canário: quebra login (ressalva ADR-0005).

## Risco e reversão

Risco operacional **nulo** neste ciclo: nenhum arquivo de runtime é alterado.
Risco do desenho (futuro) está documentado: GUC stale em pool, ausência de
`BEGIN`, bloqueio de autenticação, `BYPASSRLS`, backup/restore. Reversão do
trabalho deste ciclo = `git revert` dos commits do PR.

## Verificação

Gates locais + CI ao vivo + verificador isolado lendo ADR-0005/0006 e o documento.
Execução em app/produção permanece **[NV]**.
