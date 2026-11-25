# Problem Definition: Busca utilizável no Seed e fechamento de deriva documental

- **Slug**: 033-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O campo de busca no cabeçalho do frontend é decorativo: existe como affordance, mas não consulta o backend nem apresenta resultado ao usuário (intake.md; research.md:3,5). A auditoria de frontend do próprio ciclo aponta esse falso afforance como o maior ganho visível ao usuário por esforço, com o backend já verificado; contudo não há teste determinístico que proteja o contrato da busca, e os campos `type`/`source` do backend são strings abertas (research.md:3). A lane de componentes do frontend não é gate de merge — o CI executa apenas o projeto `server` do Vitest, e o Playwright roda somente `**/*.e2e.ts` de forma informativa —, de modo que uma mudança de busca poderia passar sem sinal determinístico (research.md:3). Por fim, a auditoria pós-032 registra drift documental residual (intervalos em OBJETIVO/ROADMAP, anotação histórica "442", banners de snapshots de 032 e prosa de validação) (research.md:8). O problema é a distância entre uma interface que promete busca e uma capacidade que não entrega resultado verificável.

## Affected Users & Stakeholders

- **Users**: membro de uma organização no Seed que procura tarefas ou mensagens pelo cabeçalho e encontra um campo sem efeito.
- **Stakeholders**: mantenedores e responsáveis por produto — precisam de capacidade de busca com valor visível e coberta por teste determinístico; revisores de documentação — precisam que o histórico de ciclos seja descrito com precisão.

## Goals

- Fazer a busca do cabeçalho corresponder a uma consulta real ao backend, com estados de resultado, carregamento e erro honestos para quem a utiliza.
- Proteger o contrato da busca com verificação determinística executada no gate existente do CI.
- Manter a documentação de governança alinhada aos ciclos já concluídos.
- Deixar explícito o que a busca não garante — relevância multilíngue, navegação comprovada em navegador e conformidade com ADR-0002 #2.

## Non-Goals

- Não afirmar que o critério #2 de reentrada de ADR-0002 foi atendido; não implementar Documentos FASE 7.
- Não promover cargo-vet a required, não criar tag/release e não executar load/restore.
- Não transformar testes de componente ou e2e informativo em gates de merge.
- Não reivindicar relevância CJK/i18n nem prova de navegação de resultado em navegador real.

## Success Metrics

- Teste unitário do cliente de busca executa no projeto `server` do Vitest e permanece verde (baseline: nenhum teste; campo decorativo).
- Resultados tipados retornados pelo backend aparecem na interface em estado normal, vazio e de erro (baseline: nenhum resultado exibido).
- Drift documental residual corrigido em OBJETIVO/ROADMAP, anotação "442" e banners de snapshots de 032 (baseline: pendente).
- Checks required continuam verdes; resultado de CI remoto e Forgejo [NV].

## Cost of Inaction

Enquanto o campo for decorativo, o usuário perde tempo com uma affordance que não responde e o projeto perde a chance de entregar valor visível com backend já pronto; sem teste no gate, regressões no contrato de busca não seriam detectadas. O custo é confiança do usuário e sinal de CI mais fraco; nenhuma métrica de uso foi medida e anexada a este ciclo [NV].

## Open Questions

- [NEEDS CLARIFICATION: qual relevância e suporte a i18n/CJK se espera, dado que `type` e `source` são strings abertas no backend?]
- [NEEDS CLARIFICATION: quais campos do resultado devem ser exibidos e como o frontend deve tratar valores de `type`/`source` desconhecidos?]
- [NEEDS CLARIFICATION: é aceitável entregar a busca sem prova em navegador real da navegação para o resultado?]
- [NEEDS CLARIFICATION: quem decide o momento de retomar Documentos FASE 7 e a auditoria de affordances falsas restantes?]
