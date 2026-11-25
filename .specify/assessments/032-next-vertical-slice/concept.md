# Concept: Allowlist de origem efetiva

- **Slug**: 032-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Origem efetiva com verificação determinística

## Options

### Option A — Origem efetiva com verificação determinística
- **Sketch**: A configuração de origens deixa de ser decorativa e passa a determinar quais origens são aceitas: uma lista vazia, um curinga ou um valor incompreensível falha rápido em vez de abrir a porta. Requisições de origens permitidas e não permitidas passam a ter comportamentos distintos e verificáveis, inclusive no pré-voo. A documentação acompanha o que foi implementado e o que continua fora de alcance, e a fronteira de autorização é respeitada: pesquisa e rascunho sim, promoção de required checks não.
- **Appetite**: medium
- **Trade-offs**: Fecha uma lacuna de segurança real em instalação self-hosted multi-tenant com verificação determinística e sem dependências novas; em troca, muda o comportamento de requisições cross-origin e pode exigir que o operador defina origens de produção que hoje não estão listadas. Não protege WebSocket nem transforma bearer em requisição credenciada.
- **Rabbit holes**: afirmar que o critério #2 de reentrada da ADR-0002 foi atendido; promover `cargo-vet` a required ou editar proteção de branch; criar tag/release; executar load/restore; introduzir OTLP; reabrir tenancy, RBAC/RLS ou Documentos FASE 7.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Tornar a configuração efetiva com falha rápida e um único teste de negação de origem; sem matriz de pré-voo e sem reconciliação documental. A exposição arbitrária já cai, mas quem integra pode não perceber uma origem legítima sendo recusada.
- **Appetite**: small
- **Trade-offs**: É o menor passo com efeito de segurança imediato e pouca superfície; sacrifica a verificação positiva e a clareza documental, deixando a operação sem critério para distinguir erro de configuração de comportamento esperado.
- **Rabbit holes**: curar a lista de origens até que os testes passem; tratar o teste de negação como prova de que a allowlist está correta em produção.

### Option C — Não fazer: empurrar a fronteira para o proxy
- **Sketch**: Não implementar a allowlist na aplicação: delegar o filtro de origens a um proxy reverso ou gateway externo e manter a configuração interna como documento.
- **Appetite**: uncertain (depende de infraestrutura não verificada)
- **Trade-offs**: Dispensa mudança no produto, mas desloca a garantia para fora do repositório e não é reproduzível em checkout limpo; exige que o proxy preserve o cabeçalho de origem, hoje sem confirmação, e não corrige a documentação.
- **Rabbit holes**: regras por IP em vez de origem, proxy que remove o cabeçalho, falsa sensação de que WebSocket também está protegido, dependência de fornecedor.

## Recommendation

Recomendo a **Option A**: as métricas pedem testes determinísticos de pré-voo permitido e de origem não permitida sem ACAO, rejeição de allowlist vazia/curinga/ininterpretável, drift residual fechado em ROADMAP/OBJETIVO/referências de ADR e nenhuma mudança de proteção sem autorização. A Option B deixa a verificação positiva sem rede de segurança; a Option C não tem evidência de que o proxy preserva a origem.

## Out of Scope (for the recommended option)

- Não afirmar que o critério #2 de reentrada da ADR-0002 foi atendido: faltam URLs de runtime, prova ao vivo não-browser e verificação de `Origin` em WebSocket.
- Não promover `cargo-vet` a required; não editar proteção de branch, CODEOWNERS ou rulesets sem autorização.
- Não criar tag/release, não executar load/restore e não introduzir OTLP.
- Não atribuir a CORS a proteção de WebSocket nem tratar bearer token como requisição credenciada.
- Não reabrir decisões de tenancy, RBAC/RLS ou Documentos FASE 7.

## Assumptions to Validate

- Quais URLs de runtime do frontend compõem a allowlist fora dos valores de desenvolvimento/preview registrados.
- Como verificar `Origin` em WebSocket, hoje fora da proteção de CORS.
- Quem autoriza a promoção de checks e a alteração de proteção de branch; estado remoto [NV].
- Qual é a origem autorizada em produção, dado que loopback é apenas sinalizado como aviso.
- Que as 13 isenções informacionais e a contagem de 455 permanecem inalteradas.
- Que bearers não contam como credenciais e que a correspondência de origem é exata, sem normalização implícita.
