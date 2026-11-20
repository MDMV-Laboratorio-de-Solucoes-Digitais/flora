# Feature Specification: First Seed Journey local one-time bootstrap

**Feature Branch**: `feature/085-local-first-seed`

**Created**: 2026-11-20T01:20:00Z

**Status**: Draft

**Input**: Usuário escolheu "Implementar First Seed Journey local: endpoint one-time (só com banco vazio) cria org/admin/workspace e já autentica", com ciclo SDD completo.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Criar a primeira organização e entrar (Priority: P1)

Como administrador de uma instalação nova sem provedor de identidade externo, eu preencho o formulário "First Seed Journey" e, ao confirmar, entro diretamente no workspace já com a organização selecionada.

**Why this priority**: é o único caminho que transforma a tela inicial em um começo utilizável; sem ele o botão permanece decorativo.

**Independent Test**: em uma instalação sem dados, preencher nome da organização e e-mail do administrador, confirmar e verificar que a lista de tarefas do workspace é exibida com a organização recém-criada.

**Acceptance Scenarios**:

1. **Given** uma instalação sem organizações além da placeholder `system`, **When** a pessoa informa nome válido e e-mail válido e confirma o "Sprout Workspace", **Then** a organização, o administrador, o papel de administrador, a associação e um workspace inicial são criados e a pessoa é levada à lista de tarefas com a organização já selecionada.
2. **Given** o cenário anterior, **When** a sessão retornada é usada pelo frontend, **Then** ela é aceita nas chamadas autenticadas subsequentes sem qualquer injeção manual de token.

---

### User Story 2 - Impedir reexecução (Priority: P1)

Como operador da instalação, eu quero que o mesmo atalho não possa ser usado duas vezes, para que a primeira identidade não seja substituída ou ampliada por qualquer visitante posterior.

**Why this priority**: é a principal mitigação da superfície não autenticada introduzida pela feature.

**Independent Test**: tentar o bootstrap uma segunda vez após o sucesso e verificar recusa sem alteração dos dados criados.

**Acceptance Scenarios**:

1. **Given** uma instalação que já passou pelo bootstrap, **When** uma nova solicitação de bootstrap é feita com outros dados, **Then** a solicitação é recusada e os dados originais permanecem intactos.
2. **Given** uma instalação que já contém qualquer organização, **When** uma solicitação de bootstrap é feita, **Then** a solicitação é recusada e nenhum objeto é criado.
3. **Given** duas solicitações concorrentes em instalação vazia, **When** ambas são processadas, **Then** no máximo uma cria objetos e a outra é recusada sem gravar nada.

---

### User Story 3 - Falha explícita e recuperável na interface (Priority: P2)

Como pessoa usando a tela inicial, eu quero ver uma mensagem clara quando o bootstrap não puder ser concluído, em vez de um botão que aparenta não funcionar.

**Why this priority**: evita a regressão UX que originou este ciclo; não depende da existência do bootstrap para ser útil.

**Independent Test**: forçar uma recusa e verificar mensagem visível na tela sem mudança de dados.

**Acceptance Scenarios**:

1. **Given** o serviço recusa a criação, **When** a pessoa confirma o formulário, **Then** a tela mostra uma mensagem de erro legível e permanece no formulário com os dados informados.
2. **Given** uma recusa por conflito, **When** a pessoa recarrega a página, **Then** o erro não reaparece automaticamente e o fluxo de login continua disponível.

### Edge Cases

- Nome do workspace ou da organização contém caracteres inválidos, está vazio ou excede limites razoáveis: recusar antes de gravar.
- E-mail do administrador inválido ou já associado a outro usuário: recusar sem criar organização parcial.
- Serviço indisponível ou resposta sem sessão: mostrar erro e não atualizar o estado de autenticação.
- Confirmar duas vezes rapidamente: apenas uma solicitação é processada.
- Instalação com usuários, mas sem organização: o critério de elegibilidade é a ausência de organizações; a criação do administrador deve falhar sem gravar se o e-mail já existir.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE oferecer um único caminho de inicialização utilizável quando não houver organizações além da placeholder `system` de bootstrap do schema.
- **FR-002**: O sistema DEVE aceitar, no formulário inicial, um nome de organização e um e-mail de administrador e validar ambos antes de qualquer gravação.
- **FR-003**: O sistema DEVE criar, em uma única operação atômica, a organização, o usuário administrador, o papel de administrador, a associação do usuário à organização e um workspace inicial.
- **FR-004**: O sistema DEVE emitir uma sessão válida já associada à nova organização como parte do sucesso da inicialização.
- **FR-005**: O sistema DEVE recusar a inicialização sempre que já existir ao menos uma organização não-sistema, sem criar ou modificar objetos.
- **FR-006**: O sistema DEVE impedir que duas inicializações concorrentes criem duas organizações iniciais.
- **FR-007**: O sistema DEVE manter o fluxo de login com provedor externo inalterado para instalações que já possuem organizações.
- **FR-008**: A interface DEVE usar o caminho de inicialização somente no botão "Sprout Workspace" e DEVE manter "Sign In" com o fluxo existente.
- **FR-009**: A interface DEVE exibir mensagem de erro legível e manter os dados informados quando a inicialização for recusada.
- **FR-010**: A interface DEVE inibir submissões duplicadas enquanto uma inicialização está em andamento.
- **FR-011**: O sistema DEVE registrar um evento de auditoria pelo menos com identificador da organização criada e horário, sem registrar segredos ou tokens.
- **FR-012**: O sistema NÃO DEVE verificar posse do e-mail informado neste fluxo; esta limitação DEVE ser explícita na documentação da feature.

### Key Entities *(include if feature involves data)*

- **Organization**: tenant raiz criado na inicialização; identificador, nome, slug derivado do nome.
- **Administrator**: usuário criado com o e-mail informado, associado à nova organização com papel de administrador.
- **Role**: papel de administrador no escopo da nova organização.
- **Membership**: vínculo entre o administrador e a organização, com o papel administrador.
- **Workspace**: workspace inicial da nova organização, associado a ela.
- **Bootstrap Session**: sessão emitida no sucesso, associada ao administrador e à nova organização.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Em instalação vazia, 100% das tentativas válidas pelo formulário terminam na lista de tarefas do workspace recém-criado, sem provedor externo.
- **SC-002**: Nenhuma tentativa após a primeira cria objetos; 100% das tentativas posteriores são recusadas.
- **SC-003**: Nenhuma recusa produz organização, usuário, papel, associação ou workspace parcial.
- **SC-004**: Todos os cenários de aceitação desta spec são cobertos por testes automatizados executáveis no CI.
- **SC-005**: Pessoa que já possui instalação com organização continua usando o fluxo de login existente sem alteração de comportamento.

## Assumptions

- "Instalação vazia" significa zero organizações não-sistema (a migration cria a placeholder `system` com UUID nil); usuários órfãos não impedem a inicialização, mas e-mail duplicado a recusa.
- O nome do workspace inicial é "Main Workspace" e o papel criado é "Admin".
- A sessão emitida pelo bootstrap é equivalente à sessão já usada pela aplicação e já contém a organização selecionada.
- O botão "Sprout Workspace" é o único ponto de entrada deste fluxo; "Sign In" permanece no OIDC existente.
- A ausência de verificação de e-mail é aceita apenas para instalações novas sem provedor; o fluxo não é um mecanismo de autenticação para instalações estabelecidas.
- A feature não altera perfis de implantação nem introduz dependências externas.
