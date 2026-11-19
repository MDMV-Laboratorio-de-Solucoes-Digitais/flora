# Define: E2E dedicado de criação de tarefa

- **Slug**: 075-task-create-e2e · **Definido**: 2026-11-19T10:13Z

## Problema

O único fluxo E2E que cria tarefas é um smoke ponta a ponta que não verifica pré-condições da UI nem a renderização por coluna. Uma regressão no guard do botão ou no mapeamento de status passaria despercebida nos checks atuais.

## Afetados / metas / não-metas

Afetados: maintainers e agentes que dependem da UI de Tasks (Fase 3 da constituição). Metas: cobrir guard, descrição e coluna Todo com teste determinístico e isolado. Não-metas: mudar código de produção, esquema, API, CI, ADRs, constituição; fechar issues sem autorização; duplicar o smoke existente.

## Métricas

- **SC-001** teste descoberto por `playwright test --list`.
- **SC-002** validação TS/ESLint/Prettier do arquivo sem erros.
- **SC-003** diff restrito a 1 arquivo de teste + artefatos do ciclo.
- **SC-004** gates do alvo permanecem PASS; nenhuma mutação de DB compartilhado.

## Baseline

Sem o teste dedicado, essas asserções só existem implicitamente no smoke e não são executadas como caso independente.
