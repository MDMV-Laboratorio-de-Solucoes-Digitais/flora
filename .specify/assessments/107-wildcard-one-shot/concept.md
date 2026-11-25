# Concept: Wildcard de recuperação com papel e consumo único

- **Slug**: 107-wildcard-one-shot
- **Created**: 2026-11-25
- **Recommended option**: Seletor restrito a papel administrativo com consumo único persistente e expiração datada

## Options

### Option A — Seletor restrito a papel administrativo com consumo único persistente e expiração datada
- **Sketch**: o seletor temporário `*` deixa de escolher qualquer e-mail e passa a escolher apenas contas com papel administrativo built-in fora de organização de sistema; depois do primeiro uso, ele se desliga de vez, inclusive após restart, e qualquer nova tentativa recebe a mesma resposta genérico de credencial inválida. A janela de escalada vira um único consumo verificável, com a data de expiração e a remoção das variáveis registradas como condição de desligamento seguro. Para o operador, nada muda na experiência de login além do fato de que a recuperação só funciona uma vez.
- **Appetite**: medium — envolve estado persistente, uma única seleção compartilhada entre startup e login, e ADR de expiração; o veredito histórico `go` e a evidência local de que os papéis e a trava já existem apoiam o tamanho, mas restart/replica em staging e produção são **[NV]**.
- **Trade-offs**: ganha redução da janela de escalada, sobrevivência a restart e falha fechada sem enumeração de e-mail/papel em log ou resposta, mantendo os pisos do gate existentes. Sacrifica a simplicidade de uma flag em memória e introduz uma mudança aditiva de estado; a proteção não cobre restauração de backup anterior ao consumo, então a expiração documentada e a remoção das variáveis continuam sendo parte indispensable da segurança. A evidência de concorrência e do 401 genérico é local, não remota **[NV]**.
- **Rabbit holes**: reintroduzir a flag em memória "porque é mais simples"; criar required check ou novos contratos `#[ignore]` para observar o comportamento; alterar formato de sessão, rota de login ou credenciais reais; perseguir cobertura de staging/produção e restart multiréplica sem acesso observado; reescrever citações históricas de pisos (`auth 23` → `auth 25`) ou tratar a expiração do ADR como desligamento automático sem remoção das variáveis.

### Option B — Desligamento em memória após o primeiro login (a menor coisa que poderia funcionar)
- **Sketch**: marcar o consumo apenas no estado do processo, sem persistência: a primeira sessão desativa a flag em memória e os logins seguintes falham. É a mudança de menor superfície e não exige estado aditivo.
- **Appetite**: small — uma condição no processo em execução, sem mudança de esquema.
- **Trade-offs**: resolve o caso de um único processo enquanto ele permanece de pé, mas a configuração é imutável no estado da aplicação e o marcador não sobrevive a restart: o wildcard voltaria a valer, justamente o risco que o problema pede eliminar. Também não cobre múltiplas réplicas. Foi explicitamente rejeitada na decisão histórica; útil apenas como contraste, não como entrega.
- **Rabbit holes**: reiniciar o processo para "limpar" o consumo; duplicar a checagem em startup e login sem fonte comum; confundir teste local com comportamento em produção; usar a simplicidade da opção para adiar a expiração documentada.

### Option C — Desligar o wildcard imediatamente e recuperar por processo manual ("não fazer / fora de banda")
- **Sketch**: remover as variáveis temporárias e encerrar o opt-in sem substituí-lo; quando precisar de acesso de recuperação, seguir um procedimento humano fora da aplicação. Não há seletor, nem marcador, nem janela de escalada dentro do sistema.
- **Appetite**: small a medium — a remoção em si é barata, mas o procedimento de recuperação e sua autorização não estão descritos nos artefatos.
- **Trade-offs**: elimina o risco na origem sem mudança de código, mas não há evidência de que um processo manual de recuperação exista ou seja aceitável; o item autorizado do INBOX (“D11”) ficaria sem a correção determinística esperada, e o próximo trabalho poderia permanecer bloqueado. A expiração do ADR já aponta para a remoção das variáveis, mas como condição final — não como substituto imediato do mecanismo.
- **Rabbit holes**: inventar procedimento de recuperação sem autorização; interromper operação que dependa do opt-in em ambiente não observado **[NV]**; tratar a remoção das variáveis como prova de que nenhum marcador ou backup pode reativar o acesso; abandonar o item do INBOX sem registro.

## Recommendation

Recomendo a **Opção A**. É a que atende às métricas do problema: no máximo uma sessão emitida sob tentativas concorrentes, zero logins com o seletor após o consumo com 401 genérico, apenas `Admin`/`Owner` built-in em organização não-sistema selecionáveis, zero enumeração em log/resposta e pisos do gate inalterados (`bootstrap 13`, `auth 25` no guarda atual). Ela resolve a limitação da opção em memória — restart — e preserva a disciplina de não criar required check nem novos `#[ignore]`. A decisão histórica `go` é registro do que foi autorizado, não reavaliação aqui; staging, produção, credenciais e restart multiréplica permanecem **[NV]** e devem ser validados antes de reivindicar o comportamento nesses ambientes.

## Out of Scope (for the recommended option)

- Mudança de gate, required check, `REQUIRED`/`ALLOWED_JOB_IF` ou pisos; nenhum novo teste `#[ignore]`.
- Formato de sessão, rota de login, credenciais reais e outros ciclos do INBOX.
- Deploy, promoção, produção e qualquer afirmação sobre staging ou múltiplas réplicas sem observação direta **[NV]**.
- Reescrita de registro append-only ou de citações históricas de pisos (`auth 23` citado no artefato antigo).
- Promessa de que a expiração do ADR, por si, desliga o mecanismo sem a remoção das variáveis; nenhuma credencial real versionada, copiada ou exposta.
- Alteração do `shape.md` existente: este concept o complementa em nível de conceito, sem reescrevê-lo.

## Assumptions to Validate

- O marcador persistente sobrevive a restart e a seleção compartilhada cobre tanto startup quanto login sem duplicação; cobertura linha a linha não foi reverificada nesta leitura.
- Tentativas concorrentes produzem no máximo uma sessão, inclusive sob latência de replicação em múltiplas réplicas — não observado **[NV]**.
- Papéis `Admin` e `Owner` built-in em organização não-sistema são a condição correta; o verificador relaxou `is_builtin` para `name IN ('Admin','Owner')` para preservar contratos sem inflar pisos, divergência registrada no analyze e no ADR 0007.
- Consumo ocorre antes da emissão da sessão, na mesma transação, com rollback e 401 genérico (`invalid_credentials`); pergunta do intake permanece sem marcação de respondida.
- ADR 0007 fixa expiração em `2026-11-29T23:59:59Z` e quatro gates de reentrada/rotação; duração de 7 dias ainda é pergunta aberta no intake.
- Restauração de backup anterior ao consumo pode reativar o marcador: a remoção das variáveis na data limite é condição de desligamento seguro; execução operacional da remoção não está registrada localmente **[NV]**.
- Ausência de enumeração e do 401 genérico foi evidenciada em Postgres local e contratos, não em produção **[NV]**.
