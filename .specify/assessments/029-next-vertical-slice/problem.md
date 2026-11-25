# Problem Definition: Integridade do Seed ainda parcialmente não executada

- **Slug**: 029-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

A auditoria pós-028 registra CI, seis suítes DB-gated e artefatos como VERIFICADOS, mas aponta quatro pendências: o clippy estrito com `--all-targets` ainda é NV, a decisão de adiar o Tauri não está formalizada em ADR, o ciclo 028 introduziu novo desvio documental e o smoke ponta a ponta nunca foi executado (`intake.md:8`).

No lint, as únicas exceções são `expect_used` em código de teste — quatro em `health.rs` e três em `oidc.rs` — e já existe precedente de anotação em nível de módulo em `extractors/auth.rs` (`research.md:3`). Enquanto o gate da CI cobre apenas `--lib --bins`, os alvos de teste ficam fora da verificação estrita (`decision.md:9`).

A decisão de diferimento do Tauri existe como rascunho coerente com ADR-0001, a constituição, o README e o código, mas não foi adotada formalmente (`research.md:4`). A documentação voltou a divergir: README apresenta versões diferentes em linhas distintas, AGENTS e ADR-0001 contêm afirmações superadas, e os documentos de pesquisa do 028 não indicam que foram sucedidos (`research.md:8`). Por fim, o runbook do smoke já identificou os mecanismos necessários — variável de porta, override de `ports`, projeto `-p flora-smoke-alt` para segurança de volume, gate de prontidão antes do `jq` e segredo JWT aninhado —, mas as portas 3000/5432 estão ocupadas por outros processos (`research.md:5`). E2E de browser e SBOM dependem de pré-requisitos maiores: `retries: 0` por rotação de `jti`, e contrato de versão/tag, respectivamente (`research.md:6-7`).

## Affected Users & Stakeholders

- **Users**: desenvolvedores que confiam no lint da CI como sinal de integridade; operadores que leem ADRs para saber o que está fora do Seed; qualquer pessoa tentando executar o smoke. [ASSUMPTION: perfis inferidos de `intake.md:8` e `research.md:3-5`; sem entrevistas registradas.]
- **Stakeholders**: mantenedores da CI e do ADR-0001/ADR-0002, responsáveis por README/AGENTS e pelos snapshots de pesquisa, equipe de validação do Smoke.

## Goals

- Fazer com que o gate estrito cubra também os alvos de teste, sem alterar o corpo dos testes (`research.md:3`).
- Registrar formalmente o diferimento do Tauri e suas consequências no ADR-0001 (`research.md:4`).
- Eliminar as contradições documentais introduzidas no ciclo anterior (`research.md:8`).
- Obter evidência executável do smoke integrado em infraestrutura isolada, ou registrar explicitamente o bloqueio encontrado (`decision.md:12`).

## Non-Goals

- Fora do enquadramento: e2e de browser com `retries: 0`, processo de release/tag e SBOM, gate bloqueante do `cargo-vet` com CODEOWNERS, exporter OTLP e validação de carga/restauração em 2vCPU/4GB (`decision.md:16`).
- Também não compõem o problema editar corpos de teste, reverter trabalho paralelo, acessar produção ou alterar o comportamento da aplicação (`decision.md:19`; `intake.md:16-17`).

## Success Metrics

- O lint estrito da CI cobre `--all-targets` (baseline: apenas `--lib --bins` — `decision.md:9`).
- As sete exceções de teste estão registradas em nível de módulo e os corpos dos testes permanecem inalterados (baseline: quatro em `health.rs` e três em `oidc.rs`, sem anotação — `research.md:3`).
- O ADR-0002 está formalmente adotado e o ADR-0001 reflete suas consequências (baseline: rascunho não adotado — `research.md:4`).
- Não há afirmações contraditórias em README, AGENTS, ADR-0001 e snapshots do 028 (baseline: quatro focos de desvio — `research.md:8`).
- O smoke integrado tem evidência de execução com sucesso ou NV explícito com bloqueio identificado (baseline: não executado — `intake.md:8`; `research.md:5`).

## Cost of Inaction

Enquanto o lint estrito excluir os testes, exceções não documentadas podem se acumular sem visibilidade. O adiamento do Tauri não registrado será relitigado em cada ciclo, e documentos que se contradizem tornam impossível saber qual afirmação é normativa. A falta de execução do smoke deixa a última gate do ADR sem evidência, enfraquecendo a própria definição de "Seed verificado" e a confiança do marco v0.1.

## Open Questions

- [NEEDS CLARIFICATION: quais portas isoladas e nome de projeto devem ser usados para a tentativa de smoke? `research.md:5` registra o conflito em 3000/5432, sem definir alternativa.]
- [NEEDS CLARIFICATION: um bloqueio de portas deve ser registrado como NV mesmo com runbook disponível? `decision.md:12` admite as duas saídas, sem critério de escolha.]
- [NEEDS CLARIFICATION: a adoção do ADR-0002 exige aprovação de stakeholders além da equipe de documentação? `research.md:4` descreve o rascunho como coerente, sem indicar aprovadores.]
- [NEEDS CLARIFICATION: qual das duas versões de README deve prevalecer? `research.md:8` aponta a divergência, sem atribuir autoridade.]
- [NEEDS CLARIFICATION: `retries: 0` no e2e é uma decisão definitiva ou uma resposta temporária à rotação de `jti`? `research.md:6` a trata como obrigatória, sem horizonte de revisão.]
