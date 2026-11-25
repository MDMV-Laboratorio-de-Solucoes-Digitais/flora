# Concept: Invariantes de domínio sob entradas arbitrárias

- **Slug**: 108-proptest-invariants
- **Created**: 2026-11-25
- **Recommended option**: Invariantes puros sob entradas geradas

## Options

### Option A — Invariantes puros sob entradas geradas
- **Sketch**: Os invariantes de permissão, paginação e seletor de recuperação deixam de ser conferidos apenas por exemplos escritos à mão. Quem revisa uma mudança nesse domínio passa a contar com contraexemplos gerados de forma reproduzível, sem depender de banco e sem alterar o comportamento do sistema. O valor está em tornar visível uma string inesperada que conceda permissão ou um offset extremo que rompa o limite, antes da revisão humana.
- **Appetite**: small (dias); incerteza: o regime de burn-in contínuo ainda não foi decidido, e a evidência registrada cobre apenas a execução pontual com 500 casos.
- **Trade-offs**: Ganha sinal sobre não-concessão e limites onde exemplos fixos eram cegos, aproveitando dependência já declarada. Sacrifica cobertura de integração e de domínios que exigem banco; exige que a própria evidência respeite os lints restritivos do workspace. Há risco de instabilidade do gerador e de falsa sensação de robustez se a contagem de casos for confundida com sensibilidade real.
- **Rabbit holes**: Tentar cobrir todos os invariantes de tenant, sessão e RBAC de uma vez; disputar com as regras de lint ao escrever asserções; promover o burn-in a critério de gate; reabrir a duplicidade de bibliotecas de propriedades sem necessidade.

### Option B — Manter exemplos fixos e confiar no gate
- **Sketch**: Nada muda na rotina: a evidência continua sendo os exemplos já escritos e a contagem de testes verificada pelo guarda do gate. A revisão segue confiando na leitura dos casos existentes.
- **Appetite**: small (dias).
- **Trade-offs**: Custo zero e nenhum risco de instabilidade, mas preserva o ponto cego original: uma entrada arbitrária pode conceder permissão indevida ou produzir limite incorreto sem que exemplo algum perceba. O gate mede quantidade, não sensibilidade.
- **Rabbit holes**: Usar contagem de testes como proxy de correção; tratar a dependência já instalada como se já fornecesse cobertura.

### Option C — Propriedades amplas em todo o núcleo
- **Sketch**: A geração de casos passa a percorrer um conjunto maior de regras de domínio — permissões, tenant, sessão e paginação — com entradas mais ricas, buscando uma rede de segurança ampla em vez de alguns invariantes escolhidos.
- **Appetite**: large (meses); incerteza alta: não há evidência local de custo ou estabilidade para essa amplitude.
- **Trade-offs**: Maior abrangência potencial, porém sai do conjunto puro e pode exigir dados de banco, fixtures e coordenação entre áreas; o risco de instabilidade e de tempo de execução cresce junto com o escopo.
- **Rabbit holes**: Modelar o domínio inteiro apenas para torná-lo gerável; alterar regras de produção para facilitar o teste; transformar a iniciativa em projeto de infraestrutura de testes.

## Recommendation

Recomendo a **Option A**. Ela atende diretamente aos objetivos registrados: quatro propriedades em três módulos, zero contraexemplos em 500 casos para round-trip/não-concessão, limites e equivalência do seletor sob entradas geradas, sem tocar gate, required checks, dependências ou produção. O resultado é evidência local e reproduzível para quem revisa autorização, não uma garantia externa. A incerteza permanece sobre burn-in contínuo, política de atualização da dependência e a biblioteca duplicada — pontos a validar, não pré-condições para o conceito.

## Out of Scope (for the recommended option)

- Banco de dados, migrations, gate, pisos e required checks.
- Dependências novas, contrato de código e qualquer mudança de comportamento em produção.
- Substituir revisão humana ou contratos DB pela cobertura de propriedades.
- Citar o diagnóstico "zero propriedades geradas" como estado corrente.
- Cobertura ampla de tenant/sessão e promoção do burn-in a checagem obrigatória.

## Assumptions to Validate

- Os invariantes escolhidos são puros e não exigem banco para serem verificados.
- É possível escrever as asserções sem violar os lints em modo `deny` do workspace.
- A execução com 500 casos é reproduzível e suficiente como evidência do ciclo.
- A política de burn-in (pontual ou contínua) será decidida separadamente, sem virar gate automaticamente.
- A dependência duplicada permanece por compatibilidade até decisão explícita.
