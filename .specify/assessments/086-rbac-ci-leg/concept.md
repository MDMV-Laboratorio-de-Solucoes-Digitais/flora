# Concept: Contratos RBAC no job obrigatório de banco

- **Slug**: 086-rbac-ci-leg
- **Created**: 2026-11-25
- **Recommended option**: Lega RBAC no job de banco com piso no guarda

## Options

### Option A — Lega RBAC no job de banco com piso no guarda
- **Sketch**: Os contratos de banco do RBAC que hoje só rodam com banco configurado passam a fazer parte do job obrigatório, e a quantidade deles passa a ser protegida pelo mesmo guarda de integridade que já vigia os demais pisos. Se a cobertura for removida ou a contagem cair, o gate falha em vez de passar em silêncio. Nenhum novo check é criado — o required check existente é reforçado.
- **Appetite**: small — duas linhas de configuração de CI, reaproveitando o helper e o mapa existentes; evidência local forte no decision.md.
- **Trade-offs**: Ganha sinal de regressão para autorização por papel/organização e imutabilidade de papéis embutidos, com diff pequeno e reversível. Sacrifica a flexibilidade de alterar a lega sem passar pelo guarda; risco baixo, desde que o piso combine com o número real de contratos.
- **Rabbit holes**: Criar required check separado ou promover a cobertura a gate próprio (muda proteção de branch); modificar os contratos de teste; mexer em `validate/`, `release/` ou `STATE.md`.

### Option B — Execução manual fora do CI
- **Sketch**: Os contratos continuam sendo rodados pontualmente por quem quiser, sem entrar no job obrigatório.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Nenhuma mudança de CI e zero risco de gate, mas mantém o *confidence gap*: a descoberta de regressões segue dependendo de inspeção humana.
- **Rabbit holes**: Transformar a execução manual em checklist recorrente.

### Option C — Gate separado só para RBAC
- **Sketch**: Uma verificação independente é criada exclusivamente para a cobertura de RBAC, com seu próprio required check.
- **Appetite**: medium — novo mecanismo e mudança na proteção de branch.
- **Trade-offs**: Isola o sinal, mas fragmenta a verificação e contraria o fit estratégico de manter o ADR-0003 íntegro.
- **Rabbit holes**: Novos contextos de branch; coordenação com os pisos existentes; ADRs.

## Recommendation

**Option A**. Os goals do problem.md pedem que regressões nos contratos de RBAC sejam observáveis no job obrigatório e que a contagem não possa cair sem falha determinística — exatamente o que a lega mais o piso no guarda entregam. Também atende ao goal de coerência entre artefatos e checkout sem reescrever registro histórico. As métricas de sucesso (contagem observável e não-zero; queda gera falha) são satisfeitas sem criar check novo e sem alterar a lógica de autorização (non-goals).

## Out of Scope (for the recommended option)

- Alterar a lógica de autorização por papel/organização ou a imutabilidade de papéis embutidos.
- Criar required check ou promover a cobertura a gate separado.
- Modificar os contratos de teste, a aplicação, ADRs, constituição, `validate/`, `release/` ou `STATE.md`.
- Afirmar estado remoto de CI, PR ou merge — permanece **[NV]**.

## Assumptions to Validate

- O piso a proteger corresponde ao número real de contratos `#[ignore]` do prefixo no checkout corrente: o artefato registra 3, mas o problem.md aponta piso 5 e o checkout 11 pisos/87; reconciliar sem reescrever o histórico.
- Postgres descartável e `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão disponíveis no job `db-tests`.
- O filtro da lega é exaustivo para os testes de RBAC que devem entrar; quem revisa o piso quando surgir um novo contrato ainda é uma pergunta em aberto.
- A evidência local registrada (3 passed, gate PASS) precisa de revalidação antes de qualquer merge.
