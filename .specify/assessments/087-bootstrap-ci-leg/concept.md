# Concept: Contratos de bootstrap no job obrigatório de banco

- **Slug**: 087-bootstrap-ci-leg
- **Created**: 2026-11-25
- **Recommended option**: Lega bootstrap no job de banco com piso no guarda

## Options

### Option A — Lega bootstrap no job de banco com piso no guarda
- **Sketch**: Os contratos de bootstrap que exigem banco configurado passam a ser exercitados no job obrigatório, e o guarda de integridade passa a proteger a contagem dessa lega ao lado da cobertura de RBAC, já conquistada. Assim, provisionamento atômico, rejeição de segunda chamada e cenário concorrente deixam de poder regredir sem sinal vermelho — sem criar um check novo.
- **Appetite**: small — duas linhas de configuração de CI sobre o helper e o mapa existentes; piso preservado de RBAC.
- **Trade-offs**: Ganha cobertura obrigatória sobre o começo da vida da organização, com diff curto e reversível. Sacrifica a liberdade de mudar a lega sem passar pelo guarda; risco baixo, mas o determinismo do cenário concorrente no Postgres do CI ainda não foi medido.
- **Rabbit holes**: Criar required check separado ou gate próprio; alterar os contratos de teste; mexer no piso `rbac` ou em `validate/`, `release/` e `STATE.md`.

### Option B — Execução manual fora do CI
- **Sketch**: Continuar rodando os contratos de bootstrap apenas quando alguém configura o banco localmente.
- **Appetite**: small (nenhuma mudança de arquivo).
- **Trade-offs**: Zero mudança e zero risco de gate, mas repete o padrão do ciclo 086: a descoberta de regressões depende de inspeção humana.
- **Rabbit holes**: Transformar a execução manual em checklist.

### Option C — Gate separado só para bootstrap
- **Sketch**: Criar uma verificação independente para a cobertura de bootstrap, com required check próprio.
- **Appetite**: medium — novo mecanismo e alteração na proteção de branch.
- **Trade-offs**: Isola o sinal, mas fragmenta a verificação e foge do desenho de reforçar o required check existente sob ADR-0003.
- **Rabbit holes**: Novos contextos de branch; sincronia com os pisos anteriores; ADRs.

## Recommendation

**Option A**. Os goals do problem.md pedem que regressões de provisionamento, rejeição de segunda chamada e concorrência sejam detectáveis no job obrigatório, que a contagem seja protegida contra drift e que a cobertura de RBAC seja preservada como piso anterior. A lega com piso no guarda atende a esses goals e às métricas de sucesso (contagem observável, queda gera falha, `"rbac": 3` mantido), sem alterar a lógica de bootstrap nem criar check adicional (non-goals).

## Out of Scope (for the recommended option)

- Alterar a lógica de provisionamento atômico, idempotência ou concorrência do bootstrap.
- Criar required check ou promover a cobertura a gate separado.
- Modificar contratos de teste, código de aplicação, ADRs, constituição, `validate/`, `release/` ou `STATE.md`.
- Afirmar determinismo do cenário concorrente no CI sem medição — estado remoto **[NV]**.

## Assumptions to Validate

- O piso a proteger corresponde ao número real de contratos sob o filtro no checkout corrente: o artefato registra 4, mas o problem.md aponta piso 13 e o checkout 11 pisos/87; reconciliar sem reescrever o histórico.
- O filtro escolhido casa a regex existente do guarda e cobre exatamente os contratos desejados; os limites dessa regex não estão documentados fora do script.
- O piso `rbac` anterior permanece intacto no mesmo mapa.
- Novos contratos de bootstrap que aparecerem exigem revisão do piso — ainda não há dono definido para essa decisão.
