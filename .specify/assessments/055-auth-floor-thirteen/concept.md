# Concept: Piso do leg auth sem folga

- **Slug**: 055-auth-floor-thirteen
- **Created**: 2026-11-25
- **Recommended option**: Exigir os treze testes nos dois locais

## Options

### Option A — Exigir os treze testes nos dois locais
- **Sketch**: Ajustar o piso exigido para o leg de autenticação nos dois pontos que o definem e atualizar o texto de consolidação para refletir o novo total, mantendo tudo o mais como está. A verificação inclui uma mutação temporária para confirmar que o piso mais alto é realmente exigido.
- **Appetite**: small (incerteza: baixa — três edições literais; execução no runner ao vivo [NV]).
- **Trade-offs**: Fecha a folga de um teste no check obrigatório de merge com risco baixo, já que os treze testes passaram com banco real no ciclo anterior. Em troca, qualquer teste que deixe de ser descoberto passa a deixar o gate vermelho — que é exatamente o efeito desejado.
- **Rabbit holes**: Elevar o piso além da contagem real, criando folga fictícia; ajustar o filtro de descoberta quando ele já cobre os treze; editar testes; atualizar o ADR no mesmo movimento, o que exige aprovação de maintainer.

### Option B — Só atualizar o guardião (menor coisa que poderia funcionar)
- **Sketch**: Elevar o piso apenas no verificador de integridade, sem tocar na definição do leg executado.
- **Appetite**: small (incerteza: baixa).
- **Trade-offs**: O guardião passa a detectar divergência entre o número exigido e o configurado, mas o leg continua aceitando doze testes; a folga no check real permanece e a checagem pode ficar inconsistente.
- **Rabbit holes**: Deixar guardião e workflow dessincronizados; tratar o guardião como substituto do gate de execução.

### Option C — Não fazer / manter o piso atual
- **Sketch**: Manter o piso de doze e aceitar a folga de um teste como risco conhecido.
- **Appetite**: none (sem custo de implementação; o custo é a possibilidade de um teste desaparecer sem a CI ficar vermelha).
- **Trade-offs**: Zero mudança em arquivos de CI, mantém a fragilidade apontada no problema: um teste pode sumir ou ser ignorado sem tornar o gate vermelho.
- **Rabbit holes**: Compensar com mais testes em outros legs; transformar a folga em "margem de segurança" sem evidência.

## Recommendation

Recomendo a **Option A**. Ela atende diretamente à meta de `define.md` — exigir os treze testes que o filtro já descobre e refletir o total de cinquenta — e é um fortalecimento de gate, não uma mudança de comportamento. A recomendação depende da confirmação por mutação de que o piso é exigido na prática, conforme o critério de sucesso registrado.

## Out of Scope (for the recommended option)

- ADR-0003, constituição e documentação de governança com o texto antigo.
- Outros pisos, checks, nomes de job ou testes.
- Proteção de branch, C5/C6, deploy, tag ou release.
- Editar testes Rust ou criar um novo check.
- Qualquer piso acima da contagem real descoberta.

## Assumptions to Validate

- A listagem de testes retorna treze entradas e o guardião aprova com treze sobre treze.
- A mutação temporária (piso treze sem treze testes reais) faz o leg de autenticação falhar.
- Os testes do workspace e os gates frontend permanecem verdes.
- O diff de código fica limitado a três linhas em dois arquivos.
- Execução do leg no runner e CI ao vivo: [NV] — observar no PR antes do merge.
