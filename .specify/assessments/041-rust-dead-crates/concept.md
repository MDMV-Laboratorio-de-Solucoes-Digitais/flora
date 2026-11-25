# Concept: Grafo Rust mais enxuto

- **Slug**: 041-rust-dead-crates
- **Created**: 2026-11-25
- **Recommended option**: Remoção pura auditada em snapshot congelado

## Options

### Option A — Remoção pura auditada em snapshot congelado
- **Sketch**: Enxugar o grafo e a configuração do workspace retirando dependências e membros sem uso verificado, com uma checagem antes e depois sobre um snapshot congelado e com as isenções ainda necessárias preservadas. Nada muda no comportamento do código; a premissa de "sem call sites" é tratada como algo a ser revalidado, não como verdade permanente.
- **Appetite**: medium (incerteza: a contagem atual do lockfile diverge do número histórico, e não há medição de esforço).
- **Trade-offs**: Reduz checkout e grafo sem mudança de comportamento e sem alterar gates. Em troca, exige auditoria sobre snapshot congelado para evitar falso positivo por concorrência, e a premissa "sem uso" pode deixar de valer após reintroduções deliberadas como a do ciclo 096.
- **Rabbit holes**: Generalizar para todo crate "aparentemente morto" quando `flora-search` foi explicitamente preservada; perseguir ganho de memória/CPU não medido; arrastar para dentro do escopo o wiring do `RbacService` ou RLS/least-privilege.

### Option B — Só o inequívoco no recorte atual (menor coisa que poderia funcionar)
- **Sketch**: Remover apenas aquilo que hoje não tem call site nem isenção ativa, preservando o que voltou a ser usado. Entrega parte do ganho com a menor exposição a reintroduções.
- **Appetite**: small (incerteza: quais itens continuam inequívocos após a reintrodução registrada).
- **Trade-offs**: Menor risco de remover algo que voltou a ser usado e compatível com a reintrodução do ciclo 096; porém o delta de grafo/configuração é menor que o registrado no ciclo original.
- **Rabbit holes**: Tratar a contagem histórica do lockfile como se fosse o estado atual; confundir "grep vazio em um recorte" com prova permanente de ausência de uso.

### Option C — Não fazer / manter o grafo
- **Sketch**: Não remover dependências nem members; manter o checkout e o grafo atuais e registrar a incerteza sobre call sites indiretos como pendência.
- **Appetite**: none (sem custo de implementação; o custo é o peso morto que permanece).
- **Trade-offs**: Zero risco de remover algo usado, mas mantém o checkout mais pesado em um cenário de VPS barata e não resolve a meta de enxugamento verificável.
- **Rabbit holes**: Transformar a pendência em uma auditoria permanente de supply chain fora deste ciclo.

## Recommendation

Recomendo a **Option A**, alinhada ao recorte aprovado em `decision.md`. Ela atende às metas de `problem.md` — reduzir grafo e configuração por remoção pura, manter gates idênticos ao baseline e preservar o que ainda tem call site — desde que a verificação rode sobre snapshot congelado e a contagem do lockfile seja refeita no estado atual, sem reaproveitar o número histórico.

## Out of Scope (for the recommended option)

- Mudança de comportamento de código, backend ou migration.
- ADR, constituição, deploy, tag, release, secrets e required checks/proteções.
- Qualquer afirmação de performance além de tamanho de grafo/configuração.
- Wiring do `RbacService`, RLS/role least-privilege e decisões de autoridade do SE.
- Generalizar a remoção para crates como `flora-search`, preservados por decisão registrada.

## Assumptions to Validate

- Como confirmar call sites indiretos via `build.rs`, feature-gate ou reexport, e como repetir a checagem após reintroduções como a do ciclo 096.
- Qual efeito mensurável em memória/CPU se espera além da redução do grafo.
- Quem autoriza o papel least-privilege e as políticas de RLS (hoje [NV]).
- O que falta para o wiring do `RbacService` entrar em escopo, incluindo assign/revoke e proteção de último Owner.
- Se a contagem de testes permanece idêntica após a regeneração do `Cargo.lock` no estado atual.
- Se as isenções removidas do cargo-vet afetam `dotenvy` ou `jsonwebtoken/rust_crypto`, que devem ser mantidos.
