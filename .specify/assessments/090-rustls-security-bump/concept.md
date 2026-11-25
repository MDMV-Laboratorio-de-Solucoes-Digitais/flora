# Concept: Convergência de rustls no grafo resolvido e na auditoria

- **Slug**: 090-rustls-security-bump
- **Created**: 2026-11-25
- **Recommended option**: Bump patch convergido no lockfile e na isenção

## Options

### Option A — Bump patch convergido no lockfile e na isenção
- **Sketch**: o mantenedor vê os quatro alertas de `rustls` desaparecerem do grafo que realmente alimenta os builds, com a exceção de auditoria apontando para a mesma versão corrigida. A faixa de compatibilidade declarada continua a mesma, nenhuma API muda e a verificação permanece sendo o gate e os testes já existentes — não aparece um novo check de segurança no caminho de merge.
- **Appetite**: small — uma versão no grafo resolvido e uma na isenção, já com registro local de verificação; incerteza: a equivalência "versão corrige os quatro advisories" veio do plano local, não de consulta independente ao banco de advisories **[NV]**.
- **Trade-offs**: ganha fechamento dos quatro alertas com um único patch compatível e elimina a divergência entre o que é executado e o que foi auditado. Sacrifica a pureza literal de "mudou só `rustls`": a resolução pode normalizar arestas transitivas, como registrado para `getrandom` em `tempfile`. Também não resolve, por si, a dúvida sobre o critério `safe-to-deploy` da isenção nem o estado ao vivo de CI/produção; se uma checagem externa ainda acusar achados, a decisão de seguimento permanece humana.
- **Rabbit holes**: reescrever a faixa declarada no manifesto ou migrar de major "aproveitando" o bump; promover `cargo deny advisories`/`dependency-review` a required check ou relaxar o gate; reauditar todas as isenções(); perseguir números de linha defasados nos registros históricos (`Cargo.lock:2748` → `:2787`); transformar a correção em PR, merge, tag ou deploy dentro deste ciclo.

### Option B — Manter a versão travada e documentar a aceitação dos advisories (a menor coisa que poderia funcionar)
- **Sketch**: não alterar nada no grafo; registrar localmente que os quatro alertas são conhecidos e por que a versão atual permanece. É a intervenção de menor custo imediato, sem risco de resolução de dependências.
- **Appetite**: small — apenas registro documental, nenhuma mudança de dependência.
- **Trade-offs**: evita qualquer efeito transitivo e preserva o diff zero, mas deixa a exposição no grafo usado pelos builds e mantém a auditoria enganosa (isenção em `0.23.46`, grafo em `0.23.43`). Não atende às métricas de versão corretiva nem de convergência e pode fazer uma checagem de dependências futura bloquear o merge por achado já conhecido; nesse cenário, a tentação de relaxar o gate cresceria — justamente o que a política local rejeita.
- **Rabbit holes**: reescrever a faixa declarada no manifesto ou migrar de major "aproveitando" o bump; promover `cargo deny advisories`/`dependency-review` a required check ou relaxar o gate; reauditar todas as isenções; perseguir números de linha defasados nos registros históricos (`Cargo.lock:2748` → `:2787`); transformar a correção em PR, merge, tag ou deploy dentro deste ciclo.

### Option C — Correção ampla de dependências ou mudança de política de verificação ("comprar" uma solução maior)
- **Sketch**: em vez de um patch pontual, atualizar a faixa declarada/versões principais de TLS e revisar a política de supply-chain de uma vez, incluindo a possibilidade de automatizar a checagem de advisories no CI. O mantenedor passaria a ter um processo amplo de atualização, com mais superfície de decisão.
- **Appetite**: medium a large — sai do escopo de duas linhas e envolve manifesto, código, workflows e política; sem evidência de que seja necessário para os quatro alertas nomeados.
- **Trade-offs**: pode reduzir o risco de recorrência e tornar a postura de segurança mais explícita, mas multiplica o risco de mudança de API, de efeitos no grafo e de conflito com a ADR-0003 se a checagem virar required check. Custo e benefício não foram avaliados nos artefatos para este ciclo.
- **Rabbit holes**: migração de `rustls`/`tokio-rustls`, reescrita de código de aplicação, revisão das 424 isenções do `cargo vet`, criação de job de advisories, mudança de proteção de branch e deploy de produção.

## Recommendation

Recomendo a **Opção A**. Ela atende diretamente às métricas do problema: `rustls` na versão corretiva sem ocorrência residual da afetada, isenção alinhada ao grafo, gate `check_gate_integrity.py` PASS e `cargo test --workspace` com zero falhas, com `dependency-review` limpo no registro local. É a correção de menor superfície compatível com a disciplina de diff cirúrgico e com a decisão de não promover checagem de advisories a required check. As ressalvas — fonte da equivalência entre versão e advisories, efeitos transitivos e estado ao vivo **[NV]** — ficam como premissas a validar, não como garantias.

## Out of Scope (for the recommended option)

- `Cargo.toml`, código de aplicação, `flora-tests/`, `.github/workflows/` e qualquer mudança de API.
- Novos required checks, promoção de `cargo deny advisories` ou `dependency-review` a gate, relaxamento do gate autoritativo ADR-0003.
- ADRs, constituição, `validate/`, `release/` e `.specify/memory/STATE.md`; PR, merge, tags, deploy e produção neste ciclo.
- Afirmação independente de que `0.23.46` corrige cada advisory sem fonte verificada; host do banco de advisories não consultado **[NV]**.
- Afirmação de estado de CI do PR, aprovações de maintainer, proteção de branch ou ambiente de produção sem **[NV]**.
- Reauditação completa das isenções cargo-vet ou reavaliação geral do critério `safe-to-deploy` além da entrada afetada.

## Assumptions to Validate

- `0.23.46` cobre efetivamente os quatro advisories canônicos (`0285`, `0346`, `0348`, `0349`); a evidência local é o plano de job advisory, sem consulta independente ao banco **[NV]**.
- A faixa `0.23` declarada no manifesto continua satisfazendo todas as dependências que usam `rustls`; nenhuma resolução alternativa foi observada no checkout.
- O critério `safe-to-deploy` da isenção continua adequado para a versão corrigida; `cargo vet` reportou `424 exempted`, o que não prova a reavaliação individual.
- Efeitos transitivos da resolução (ex.: aresta de `getrandom` em `tempfile`) são aceitáveis e não alteram comportamento; verificação autoritativa é o gate e os testes.
- Contagens de gate (10/58 no handoff; 11/87 no checkout) e `dependency-review` limpo são registros de 2026-11-20, não medições atuais; estado ao vivo **[NV]**.
