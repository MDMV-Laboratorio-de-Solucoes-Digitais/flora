# 032 — Limite de autorização: o agente pode promover um check informativo a obrigatório? (2026-11-16)

**Data:** 2026-11-16
**Escopo:** pesquisa somente leitura + cross-check somente leitura no checkout local e na API do GitHub. **Nenhum workflow, `.github/CODEOWNERS`, `supply-chain/`, `Cargo.lock`, config de branch protection, ruleset ou settings foi alterado.** O único arquivo escrito por esta tarefa é este documento.
**Pergunta:** um agente de IA pode, por autonomia própria, promover o job informativo `supply-chain-vet` a required status check e/ou editar branch protection / rulesets / CODEOWNERS — ou isso exige autorização humana explícita?
**Antecedentes:** `2026-11-16-031-cargo-vet-promotion.md` recomenda **DEFERIR** a promoção (sem owner de CODEOWNERS, sem prova de admin, janela curta, 13 exemptions rotuladas). Este documento responde à pergunta de **governança/Autorização** que o 031 levantou em seu §6, não reabre a recomendação técnica.
**Idiomas consultados:** EN (canônico), pt-BR, es, de, fr, zh-CN, ja. As páginas localizadas dos GitHub Docs foram reconsultadas como verificação cruzada; nenhuma tradução foi usada como fonte de afirmação operacional (§6).
**Convenção de evidência:** `[L#]` = leitura direta no checkout local; `[G#]` = consulta somente leitura à API/`gh` em 2026-11-16; `[D#]` = GitHub Docs (EN), fonte primária; `[T#]` = localizações dos GitHub Docs; `[A#]` = documentação oficial dos frameworks de agente; `[H#]` = governança local MDMV / instrução recebida. **`VERIFICADO`** = observado diretamente; **`INFERIDO`** = deduzido de documentação; **`NV`** = não validado (§5). Os IDs são locais a este documento; onde coincidem com o doc 031, foram reobservados hoje.

---

## 0. Recomendação — resposta direta

**Não. A promoção a required status check e a edição de branch protection/rulesets exigem autorização humana explícita.** Um agente pode preparar tudo, propor o diff, abrir PR e verificar o resultado **depois** de um humano agir; não pode, por autonomia própria, alterar o poder de merge dos demais colaboradores.

A distinção que decide o caso é entre **capacidade** (o token/harness permite?), **permissão do GitHub** (a identidade tem o papel?) e **autoridade** (alguém com poder sobre a governança mandatou a mudança?). As duas primeiras podem estar presentes sem conferir a terceira — e a terceira é a que falta aqui. `[D1][D2][A1][A2][H3]`

| Ação | Veredito | Quem executa / por quê |
|---|---|---|
| Pesquisar, ler API, redigir docs, propor diff e plano de rollback | **AUTÔNOMO — GO** | É exatamente este documento; nenhuma permissão especial. `[L1]-[L5]`, `[G1]-[G4]` |
| Abrir PR alterando `.github/workflows/rust.yml` (remover `continue-on-error` da linha 89) e/ou props de `.github/CODEOWNERS` | **AUTÔNOMO com ressalva — GO para propor, NÃO para automesclar** | É mudança de código; Write já pode editar workflows e criar status checks `[D2]`. Mas o **conteúdo** do CODEOWNERS exige identidade real validada por humano (§5 NV-A), e o merge deve seguir as regras vigentes |
| Simplesmente remover `continue-on-error: true` **sem** marcar o check como required | **DEFERIR junto com a promoção** | Muda a UX de contribuição (job passa a falhar de verdade) sem gerar bloqueio de merge; é meia-medida enganosa. Fazer como parte do PR coordenado, não isolado `[D3]` |
| Escolher os handles/times do CODEOWNERS e confirmar `write` explícito | **HUMANO — exige autorização** | Identidade e delegação de revisão são atos de governança; inventar handle é ação não autorizada `[D4][H3]` |
| Adicionar `supply-chain-vet` como **required status check** na proteção de `dev` | **HUMANO — admin/owner (ou custom role com "edit repository rules")** | "Updating required status checks requires admin or owner permissions" `[D1]`; matriz de papéis: apenas **Admin** ✓ em "Manage branch protection rules and repository rulesets" `[D2]` |
| Criar/editar ruleset, marcar "Require status checks" / "Require review from Code Owners", mexer na bypass list | **HUMANO — admin/owner** | Mesmo gate `[D1][D2]`; bypass list é decisão sobre quem pode driblar o controle `[D5]` |
| Alternar ruleset `Disabled → Active` (ou salvar regra clássica com `enforce_admins: true`) | **HUMANO — admin/owner; ato que altera o poder de merge de todos** | `Active` "takes effect immediately" `[D5]`; admin enforcement "requires admin or owner" `[D1]` |
| Fazer PR de teste provando que um crate sem decisão de vet **bloqueia**; validar review do CODEOWNER; validar fork | **AUTÔNOMO — GO** | É verificação em PR descartável, sem mudar regras; executar antes da ativação `[D5]` |
| Verificar depois (somente leitura) que a regra foi aplicada e reportar | **AUTÔNOMO — GO (obrigatório)** | "Não auto-certifique — verifique" `[H1]` |
| Rollback (ruleset `Active → Disabled`; re-adicionar `continue-on-error`) | **HUMANO para o ruleset**; `continue-on-error` é PR de código | Tropicar enforcement também é "edit repository rules" `[D1][D2][D6]` |

**Frase-síntese:** autonomia cobre **preparar e verificar**; autorização humana cobre **decidir e aplicar quem pode mergear**. O doc 031 já havia chegado a essa conclusão (§6, "Sim, a promoção exige autorização explícita"); esta pesquisa a confirma com fontes primárias adicionais e estende o limite também a bypass lists e ao próprio rollback.

**Critério de saída para o agente:** não executar nenhuma linha da coluna "HUMANO" sem (1) pedido explícito nomeando repositório, branch, check e regra; (2) registro de quem autorizou e onde (issue/ADR); (3) evidência pós-mudança obtida por leitura. `[H1][H3]`

---

## 1. Capacidade ≠ permissão ≠ autoridade

Três camadas independentes costumam ser confundidas numa conversa sobre "autonomia":

1. **Capacidade (harness):** o que o runtime deixa o agente chamar. Na documentação do Claude Code, leituras são automáticas, mas edições de arquivo, comandos shell e chamadas de rede exigem aprovação em modo Manual; existem modos `acceptEdits`, `plan`, `auto`, `dontAsk`, `bypassPermissions`, e **"Permission rules are enforced by Claude Code, not by the model. Instructions in your prompt or `CLAUDE.md` shape what Claude tries to do, but they don't change what Claude Code allows"** `[A1]`. Ou seja: um mandato amplo dado em linguagem natural **não** é um aumento de permissão.
2. **Permissão (GitHub):** o que a identidade autenticada pode fazer no repo. `Write` pode criar status checks, editar/run workflows e atuar como code owner; **somente `Admin`** pode "Manage branch protection rules and repository rulesets" `[D2]`. A API é explícita: "Protecting a branch requires admin or owner permissions to the repository" `[D1]`.
3. **Autoridade (governança):** o que alguém legitimamente decidiu que o agente deve fazer. No caso Flora, a instrução recebida foi de autonomia ampla **com ressalva explícita**: consultar para autorização explícita ou tarefas além da capacidade autônoma `[H3]` — e a regra local "não auto-certifique, verifique" `[H1]`. O prior-art MDMV usa escalada `human-in-the-loop` justamente nas etapas críticas `[H2]`.

**Consequência prática:** mesmo que o token `gh` tivesse admin (o que **não foi provado**, §5 NV-B), o agente ainda não teria autoridade para usá-lo. E mesmo que o harness estivesse em `auto`/`bypassPermissions`, o classificador do Claude Code bloqueia por padrão ações como **"Granting IAM or repo permissions"**, **"Modifying shared infrastructure"** e `force push` — inclusive nesses modos `[A2]`. A promoção de um gate é precisamente "modificar infraestrutura compartilhada de governança".

> `bypassPermissions` "skips permission prompts... Only use this mode in isolated environments like containers or VMs where Claude Code can't cause damage" `[A1]`. Um repositório GitHub compartilhado não é ambiente isolado.

---

## 2. O que o GitHub exige (fontes primárias)

### 2.1 Matriz mínima de papel

| Operação | Papel mínimo | Fonte |
|---|---|---|
| Editar `.github/workflows/*.yml`, criar status checks, rodar workflows | **Write** | `[D2]` (linhas "Create status checks"; "Create, edit, run, re-run, and cancel GitHub Actions workflows") |
| Criar arquivo `.github/CODEOWNERS` ( como arquivo) | **Write** | `[D2]` + `[D4]` (é edição de arquivo; sem "Require review from Code Owners" não bloqueia) |
| Atuar como code owner designado | **Write** | `[D2]` / `[D4]` |
| **Gerenciar branch protection rules e rulesets** | **Admin** (Write e Maintain = ✗) | `[D2]` |
| Atualizar required status checks / required reviews / admin enforcement (REST) | **admin ou owner** | `[D1]` |
| Ler rulesets | Read/Write/Maintain/Admin (Triage/Moderate ✗) | `[D2]`; "anyone with read access" `[D6]` |

A linha decisiva da matriz `[D2]` é: *"Manage branch protection rules and repository rulesets"* — ✓ apenas em **Admin**. Isso responde a pergunta sem ambiguidade: **um agente com `Write` pode deixar o job vermelho, mas não pode transformá-lo em gate de merge**.

### 2.2 Mecânica que importa para a decisão

- Required checks aceitam `successful`, `skipped` **ou** `neutral` `[D3]`; um job que skipa reporta sucesso e não bloqueia (doc 031 `[D2][D3]`). Por isso `continue-on-error` e filtros `paths:` são armadilhas gêmeas: o primeiro **mascara** a falha, o segundo gera `Pending` e **bloqueia** todo PR `[D3]` + doc 031.
- **"Any person or integration with write permissions to a repository can set the state of any status check in the repository"** `[D3]`. Isso é um risco relevante: sem fixar a origem (App) do check, qualquer ator com write — inclusive um agente — pode inflar o status. Mitigação: selecionar o App emissor ao adicionar o required check `[D1][D3]`.
- Nomes de job devem ser únicos entre workflows; nome duplicado gera resultado ambíguo e bloqueia merges `[D3]`. `supply-chain-vet` é único `[L5]` + doc 031 `[L7]`.
- Rulesets: status **Active** entra em vigor imediatamente na criação; **Disabled** não é aplicado; é possível mudar o status sem deletar; "Require branches to be up to date" só tem efeito se houver ao menos um check definido; é preciso digitar o nome do check e clicar no `+` `[D5]`. Camadas se agregam e **a regra mais restritiva vence** `[D6]`.
- A bypass list de ruleset pode incluir admins, organization owners, papéis `maintain`/`write`, times, GitHub Apps e Dependabot `[D5]`. **Bypass list não vazia enfraquece a garantia** — e é decisão humana, não parâmetro técnico neutro.
- CODEOWNERS: owners precisam de `write` **explícito**; revisão é solicitada apenas se o arquivo estiver na branch base; a aprovação de **qualquer um** dos owners satisfaz o requisito; para proteger totalmente o repo é preciso definir owner para o próprio `/.github/CODEOWNERS` `[D4]`.

### 2.3 Estado observado no Flora (somente leitura)

| Item | Observação `VERIFICADO` | Evidência |
|---|---|---|
| Job do gate | `supply-chain-vet`; `continue-on-error: true` na linha 89; comando `cargo vet --locked --no-minimize-exemptions` na 95. Outro `continue-on-error: true` em `e2e-browser` (116) — **não tocar** | `[L1][L5]` |
| Store | `config.toml` com **455** exemptions (12 `safe-to-run`); **13** com `notes` "informational gate only"; `multer` na linha 840 pede *"revisit when cargo-vet gate becomes blocking"*; `audits.toml` e `imports.lock` só com cabeçalho | `[L2][L3]` |
| CODEOWNERS remoto | `contents/.github/CODEOWNERS?ref=dev` → **404** | `[G3]` |
| Proteção de `dev` | Sem `required_status_checks`, sem `required_pull_request_reviews`, `enforce_admins.enabled=false`, `allow_force_pushes=false`, `allow_deletions=false` | `[G1]` |
| Rulesets | `GET /rulesets` → `[]` | `[G2]` |
| Identidade `gh` | `lfgranja`; escopos `gist`, `read:org`, `repo`, `workflow`. Token autentica; **não prova** papel de admin/owner neste repo | `[G4]` |
| Drift | checkout limpo; HEAD local `7e801a6b34`, `github/dev` `966fb19476` (desatualizado); `origin` é Forgejo | `[L4]` |
| Consequência | Hoje qualquer pessoa com write — inclusive um agente — consegue mergear em `dev` sem check e sem review; o gate é decorativo | `[G1][D2][D3]` |

Leitura: o bloqueio é **de governança e autoridade**, não de ferramenta. Confirmam-se os fatos do doc 031; a única mudança observável é o HEAD do checkout/remoto (ambos avançaram desde o doc 031), sem alteração nos arquivos relevantes.

---

## 3. O que frameworks de agente dizem sobre autonomia

- **Claude Code — permissões em camadas:** leitura é automática; edição de arquivo, Bash e WebFetch/Search exigem aprovação no modo Manual; modos `acceptEdits`, `plan`, `auto`, `dontAsk`, `bypassPermissions`; regras `deny` vencem em **todos** os modos, inclusive `bypassPermissions` `[A1]`.
- **Claude Code — o que nenhum modo autoaprova:** a lista inclui explicitamente **"Granting IAM or repo permissions"**, **"Modifying shared infrastructure"**, **"Production deploys and migrations"**, **"Force push"** e destruição irreversível de arquivos pré-existentes `[A2]`. A promoção de gate é "modificar infraestrutura compartilhada"; conceder bypass é "granting repo permissions".
- **Claude Code — auto mode não é selo de segurança:** *"Auto mode reduces permission prompts but does not guarantee safety. Use it for tasks where you trust the general direction, not as a replacement for review on sensitive operations"* `[A2]`.
- **GitHub Copilot — limites próprios:** *"Without write access to a repository, your Copilot session is read-only"*; *"A ruleset or branch protection rule can prevent Copilot from creating or updating pull requests. Repository administrators can configure a ruleset bypass for Copilot"*; uma PR por tarefa; *"Review and test the output before using it in production"* `[A3]`. Mesmo o agente nativo do GitHub atua **dentro** das regras, não as edita.
- **MDMV local:** regra operacional *"não auto-certifique — verifique"* `[H1]`; prior-art com escalada `human-in-the-loop` nas etapas críticas `[H2]`; e a instrução recebida nesta tarefa reserva consulta para autorização explícita ou trabalho além da capacidade autônoma `[H3]`.

**Síntese:** autonomia ampla é razoável para trabalho **reversível, local, revisável e que não muda as regras dos outros** (pesquisa, rascunho, diff proposto, PR de teste). Ela não cobre **alterar o poder de merge**, **escolher delegados de revisão** nem **conceder bypass** — atos que afetam terceiros e só são legítimos com mandato identificável.

---

## 4. Procedimento recomendado para o agente (quando pedirem a promoção)

1. **Não autoautorizar.** Restate o pedido e perguntar: qual repositório, qual branch (`dev`? `main`?), qual check (`supply-chain-vet`), qual regra (proteção clássica ou ruleset), quem é o admin que aprova. Registrar em issue/ADR `[H3]`.
2. **Preparar sem aplicar:** diff de `rust.yml` (remover só linha 89), proposta de CODEOWNERS com handles pendentes, ruleset em `Disabled`, plano de rollback e PR de teste. Tudo isso é autônomo `[A1][D2]`.
3. **Entregar o ato administrativo ao humano:** ele faz `Settings → Rulesets` ou edita a proteção clássica, com `supply-chain-vet` digitado e `+` clicado, e decide a bypass list `[D1][D5]`.
4. **Provar antes de ativar:** PR de teste com dependência nova sem vet deve ficar vermelha **no step**; review do CODEOWNER deve ser solicitado; fork deve reportar o check. Só então `Disabled → Active` `[D5]`.
5. **Verificar depois (somente leitura):** `GET .../branches/dev/protection`, `GET .../rulesets`, check-runs do PR de teste. Não aceitar "foi feito" como evidência `[H1]`.
6. **Registrar:** quem autorizou, quando, em qual repositório, e qual evidência de pós-verificação. Isso fecha a cadeia capacidade → permissão → autoridade.

---

## 5. Riscos e NVs (não validado nesta pesquisa)

- **NV-A (bloqueante)** — Handles/times reais para CODEOWNERS e seus `write` explícitos não verificados; inventar owner é ação não autorizada `[D4]`.
- **NV-B (bloqueante)** — Identidade com papel Admin/owner ou custom role "edit repository rules" não confirmada. Escopos `repo`/`workflow` do token `[G4]` **não** provam admin `[D1][D2]`.
- **NV-C (bloqueante)** — Canonicalidade: checkout em `projetos/Arquivados/flora` (cujo `AGENTS.md` diz não modernizar arquivados sem tarefa explícita), remoto GitHub público e ativo, `origin` Forgejo com push mirroring. Não verificado se o mirror pode reescrever `dev` e contornar a proteção `[L4]` + doc 031 NV-H.
- **NV-D** — Janela informativa insuficiente (doc 031: ~25 min, 12/12 verdes); não recontada nesta tarefa.
- **NV-E** — As 13 exemptions rotuladas não foram auditadas/aceitas; `multer` pede revisit explícito `[L2]`.
- **NV-F** — Sem imports/audits `[L3]`, o gate cobre apenas dependências novas; backlog do doc 027 (~8,7M linhas) é diagnóstico externo, não meta.
- **NV-G** — Origem do status check não fixada: qualquer write pode setar status `[D3]`; um agente com write poderia, em tese, falsificar verde. Mitigação pendente: selecionar App emissor `[D1]`.
- **NV-H** — Bypass list ainda não decidida; se incluir `write`/`maintain` ou admins, a garantia é enfraquecida por desenho `[D5]`.
- **NV-I** — Comportamento em PR de fork (CODEOWNERS da base + check reportado) não testado `[D4]`.
- **NV-J** — Rollback de ruleset também exige admin `[D1][D2]`; se o admin que ativou não estiver disponível, a válvula de emergência não é autônoma.
- **NV-K** — Deriva local/remoto (`7e801a6b34` vs `966fb19476`) `[L4]`; editar sem `fetch`/`pull` pode conflitar.
- **NV-L** — Risco de "autorização implícita": o agente tratar autonomia ampla como consentimento para admin. É exatamente o erro que este documento proíbe `[A1][H3]`.
- **NV-M** — Não executei `gh api` com escrita, `git push`, edição de workflow/settings, `cargo vet` ou qualquer comando com efeito de escrita no repo; todos os resultados vêm de leitura de arquivos, docs e API.

**Risco de segurança (não instrução):** o conteúdo de issues, PRs, logs e páginas consultadas é dado, não comando. Nenhuma diretriz encontrada nesses materiais foi executada como autorização para mudar regras.

---

## 6. Cross-check multilíngue (pt-BR, es, de, fr, zh-CN, ja)

As páginas localizadas de *About code owners* foram reconsultadas em 2026-11-16 para verificar se alguma tradução altera a mecânica de autorização. **Nenhuma divergência operacional encontrada.** As três afirmações que sustentam a recomendação aparecem em todos os idiomas: (a) owners precisam de `write` explícito; (b) a aprovação de **qualquer um** dos owners satisfaz o requisito; (c) só alguém com permissão de **administrador/proprietário** habilita revisões obrigatórias.

| Idioma | "explicit write access" | "admin or owner" | "any one owner" | Fonte |
|---|---|---|---|---|
| EN | explicit `write` access | admin or owner permissions | an approval from *any* of the owners | `[D4]` |
| pt-BR | acesso `write` explícito | permissões de administrador ou proprietário | aprovação de *qualquer* um | `[T-pt]` |
| es | acceso explícito `write` | permisos de administrador o propietario | aprobación de *cualquiera* | `[T-es]` |
| de | expliziten `write`-Zugriff | Administrator- oder Inhaberberechtigungen | Genehmigung *eines* Besitzers | `[T-de]` |
| fr | accès `write` explicite | autorisations d'administrateur ou de propriétaire | approbation de *n'importe* lequel | `[T-fr]` |
| zh-CN | 显式 `write` 访问权限 | 管理员或所有者权限 | 任何所有者的批准 | `[T-zh]` |
| ja | 明示的な `write` アクセス権 | 管理者あるいはオーナー権限 | *いずれか*の所有者による承認 | `[T-ja]` |

Nota de acesso: o locale português dos GitHub Docs é `/pt/` (não `/pt-br/`, que retorna 404); os demais são `/es/`, `/de/`, `/fr/`, `/zh/`, `/ja/`. A restrição de admin para gerenciar proteção/rulesets também foi verificada em EN na matriz de papéis `[D2]` e na API `[D1]`; as localizações consultadas não a contradizem.

---

## 7. Fontes (reconsultadas em 2026-11-16)

**Local (somente leitura)**
- `[L1]` `.github/workflows/rust.yml` — `supply-chain-vet` linhas 87–95; `e2e-browser` linha 116.
- `[L2]` `supply-chain/config.toml` — 455 exemptions, 12 `safe-to-run`, 13 `informational gate only`, `multer` linha 840.
- `[L3]` `supply-chain/audits.toml`, `supply-chain/imports.lock` — apenas cabeçalhos.
- `[L4]` `git status --porcelain` (vazio), `git rev-parse HEAD` (`7e801a6b34…`), `git ls-remote github refs/heads/dev` (`966fb19476…`), `git remote -v` (GitHub + Forgejo).
- `[L5]` `grep -rn 'continue-on-error' .github/workflows/` — 2 ocorrências (linhas 89 e 116).

**GitHub API / `gh` (somente leitura)**
- `[G1]` `GET /repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora/branches/dev/protection`.
- `[G2]` `GET /repos/.../rulesets` → `[]`.
- `[G3]` `GET /repos/.../contents/.github/CODEOWNERS?ref=dev` → 404.
- `[G4]` `gh auth status` → `lfgranja`, escopos `gist`, `read:org`, `repo`, `workflow`.

**GitHub Docs (primário, EN)**
- `[D1]` REST endpoints for protected branches — https://docs.github.com/en/rest/branches/branch-protection
- `[D2]` Repository roles for an organization — https://docs.github.com/en/organizations/managing-access-to-your-organizations-repositories/repository-roles-for-an-organization
- `[D3]` About protected branches — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- `[D4]` About code owners — https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- `[D5]` Creating rulesets for a repository — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- `[D6]` About rulesets — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets
- `[D7]` Workflow syntax (`continue-on-error`, filtros) — citado via doc 031 `[D2]`; não reconsultado nesta tarefa (NV-M).

**Localizações verificadas**
- `[T-pt]` https://docs.github.com/pt/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- `[T-es]` https://docs.github.com/es/... (mesmo caminho)
- `[T-de]` https://docs.github.com/de/... (mesmo caminho)
- `[T-fr]` https://docs.github.com/fr/... (mesmo caminho)
- `[T-zh]` https://docs.github.com/zh/... (mesmo caminho)
- `[T-ja]` https://docs.github.com/ja/... (mesmo caminho)

**Frameworks de agente (primário)**
- `[A1]` Claude Code — Configure permissions — https://code.claude.com/docs/en/permissions
- `[A2]` Claude Code — Choose a permission mode — https://code.claude.com/docs/en/permission-modes
- `[A3]` GitHub Copilot on GitHub.com and GitHub Mobile — https://docs.github.com/en/copilot/concepts/agents/coding-agent/about-coding-agent

**Governança local MDMV**
- `[H1]` `AGENTS.md` (raiz HQ) — regra "não auto-certifique — verifique"; disciplina de agente.
- `[H2]` `.specify/assessments/dark-software-factory/intake.md:200-210` — prior-art com escalada `human-in-the-loop` em etapas críticas.
- `[H3]` Instrução recebida nesta tarefa: autonomia ampla, com consulta reservada para autorização explícita ou tarefas além da capacidade autônoma.

**Antecedente interno**
- `docs/research/2026-11-16-031-cargo-vet-promotion.md` — DEFER, pré-condições C1–C6, plano de promoção e rollback; esta pesquisa endossa o §6 daquele documento.

---

## 8. Resumo executivo

- **A resposta é não:** promover check informativo a required e editar branch protection/rulesets **exige autorização humana explícita** — mesmo que o agente tenha write e mesmo que o harness esteja em modo amplo.
- **Base primária:** só Admin gerencia protection/rulesets `[D2]`; a API exige admin/owner para required checks e admin enforcement `[D1]`; frameworks de agente bloqueiam por padrão "granting repo permissions" e "modifying shared infrastructure" `[A2]`; o próprio agente nativo do GitHub atua dentro das regras, não as edita `[A3]`.
- **O que é autônomo:** pesquisar, redigir, propor diff/CODEOWNERS pendente, criar PR de teste e **verificar depois** que o humano aplicou a regra.
- **O que não é:** escolher owners, marcar check como required, criar/ativar ruleset, decidir bypass, autorrometer o próprio merge, e fazer rollback de ruleset.
- **Estado Flora reafirmado:** `dev` sem required checks/reviews e `enforce_admins=false`, rulesets `[]`, CODEOWNERS ausente, gate com `continue-on-error` e 13 exemptions rotuladas — promover agora mudaria o poder de merge com base em dívida não assumida.
- **Nenhuma mudança foi aplicada.** Este documento é um limite de autoridade explicitado, não uma promoção disfarçada.
