# 031 — Promoção do gate cargo-vet de informativo para bloqueante (2026-11-16)

**Data:** 2026-11-16
**Escopo:** pesquisa somente leitura + cross-check somente leitura no checkout local e no repositório remoto `github`. **Nenhum workflow, `.github/CODEOWNERS`, `supply-chain/`, `Cargo.lock`, config de branch protection ou ruleset foi alterado.** O único arquivo escrito por esta tarefa é este documento.
**Pergunta:** quais são os passos exatos para promover o job informativo `supply-chain-vet` a gate bloqueante sem quebrar a experiência de contribuição — e essa promoção pode ser feita sem autorização explícita de mantenedor?
**Antecedentes:** `2026-11-16-026-cargo-vet-exemptions.md`, `2026-11-16-027-cargo-vet-gate-feasibility.md` (experimentos em cópia descartável), `2026-11-16-028-cargo-vet-gate-plan.md` (Fatia A executada; Fatia B pendente).
**Idiomas consultados:** EN (canônico), pt, es, de, fr, zh-CN, ja. As versões localizadas da documentação do GitHub foram reconsultadas como verificação cruzada; nenhuma tradução foi usada como fonte de afirmação operacional (§8).

**Convenção de evidência:** `[L#]` = leitura direta no checkout local; `[G#]` = consulta somente leitura à API do GitHub (`gh api` / `api.github.com`) em 2026-11-16; `[D#]` = GitHub Docs em EN, reconsultado hoje; `[T#]` = versões localizadas dos GitHub Docs; `[U#]` = upstream cargo-vet (book/API). **`VERIFICADO`** = observado diretamente; **`INFERIDO`** = deduzido de documentação; **`NV`** = não validado (§7).

---

## 0. Recomendação: **DEFERIR a promoção agora** (não é NO-GO)

**Frase-síntese:** a promoção é tecnicamente barata e o caminho está claro, mas hoje ela depende de pré-condições que **não estão satisfeitas e não podem ser satisfeitas por agente sem autorização**: (1) owner de CODEOWNERS válido, (2) decisão de admin sobre protection/ruleset, (3) janela de observação real do job informativo. Além disso, o próprio `config.toml` carrega 13 exemptions rotuladas `"informational gate only"` — uma delas diz literalmente *"revisit when cargo-vet gate becomes blocking"*. Promover antes de revisar esse rótulo transformaria dívida explicitamente pendente em baseline sancionada.

| Item | Veredito | Justificativa |
|---|---|---|
| Promover **neste ciclo** (remover `continue-on-error` + marcar check como required + ativar CODEOWNERS) | **DEFER** | Sem owner validado, sem decisão de admin e com ~25 min de janela informativa `[G5]`. |
| Preparar tudo agora (documento, diff proposto, plano de rollback, ratchet) | **GO** | É exatamente este documento; nenhuma permissão especial necessária. |
| Promover **após** satisfazer C1–C6 (§1) | **GO condicionado** | Execução estimada: 1 PR de código + 1 mudança de settings + 1 PR de teste. |
| Zerar as 455 exemptions antes de promover | **NO-GO** | Backlog estimado em ~8,7M linhas no doc 027; fora de escopo. O correto é revisar as **13** rotuladas, não as 455. |
| Usar `continue-on-error: true` **e** check required | **NO-GO** | O job passa a reportar sucesso mesmo falhando; o required check nunca bloqueia `[D2][D3]`. |
| Adicionar filtro `paths:` ao workflow do vet | **NO-GO** | Check fica `Pending` e **bloqueia** todo PR que não tocar os paths `[D2]`. |

**Critério de aceite para promover:** C1–C6 da §1 todos `VERIFICADO`, e um PR de teste (§3, passo 6) comprovando que um crate novo sem decisão de vet **bloqueia** o merge.

---

## 1. Estado real (cross-check local + remoto) — corrige o briefing

> O briefing recebido dizia "store versionado em cycle 028 (455 exemptions após cycle 030), `continue-on-error: true`, **sem branch protection, sem CODEOWNERS**". Os dois primeiros fatos confirmam-se; o terceiro é **impreciso**: existe proteção clássica mínima nas branches `dev` e `main`, apenas **sem required status checks**. Isso muda o plano: é uma **edição** de regra existente (ou criação de ruleset em camada), não uma criação do zero.

| Item | Estado `VERIFICADO` | Evidência |
|---|---|---|
| Job id do gate | `supply-chain-vet` — **não** `cargo-vet` (o doc 028 propôs `cargo-vet`, mas o código mergeado usa outro nome). É esse o nome que vai aparecer como required check. | `[L1]` `rust.yml:87`; `[G3]` check runs no HEAD de `dev` |
| `continue-on-error` | `true` no **job** `supply-chain-vet` (`rust.yml:89`). Há outro `continue-on-error: true` no job `e2e-browser` (`rust.yml:116`) — **não deve ser tocado**. | `[L1]`; `[G1]` remote `dev` idêntico |
| Comando do gate | `cargo vet --locked --no-minimize-exemptions` (`rust.yml:95`). **Sem** `--no-all-features` → grafo maximal; o NV2 dos docs 027/028 está resolvido. | `[L1][G1]` |
| Instalação da ferramenta | `taiki-e/install-action@cargo-vet` (`rust.yml:92-93`). Nos 12 runs recentes o step `Install cargo-vet` e o step `Vet (informational, Fatia A)` terminaram `success` → o NV6 dos docs anteriores está **resolvido na prática**. Pinar versão continua desejável. | `[L1][G5]` |
| Store | Versionado: `supply-chain/{config.toml,audits.toml,imports.lock}` presentes localmente e no remoto `dev`. | `[L2][G2]` |
| Exemptions | **455** = 443 `safe-to-deploy` + 12 `safe-to-run`. Nenhum `[imports]`, `imports.lock` com 2 linhas (só cabeçalho), `audits.toml` com 4 linhas (só `[audits]`). Ou seja: baseline é 100% exemption, zero auditoria, zero import. | `[L2][L3][G2]` |
| Dívida rotulada | **13** blocos com `notes` contendo `"informational gate only"` (12 "Cycle 030: transitive via dev-mint clap helper" + 1 `multer`, Cycle 028: *"revisit when cargo-vet gate becomes blocking"*). 455 − 442 = 13: toda a dilatação do baseline veio de itens explicitamente marcados como pendentes. | `[L2][L4]` |
| CODEOWNERS | Ausente localmente e no remoto `dev` (`contents/.github/CODEOWNERS?ref=dev` → 404). | `[L5][G6]` |
| Proteção de branch (remoto) | `dev` **e** `main` protegidas no modo clássico. `dev`: só "no force push / no deletion"; **sem** required checks, **sem** required reviews. `main`: 1 aprovação + dismiss stale; **sem** `require_code_owner_reviews`, **sem** required checks. `enforce_admins: false` em ambas. | `[G4]` |
| Rulesets | `GET /rulesets` → `[]`. Nenhuma ruleset. | `[G4]` |
| Repositório remoto | `MDMV-Laboratorio-de-Solucoes-Digitais/flora`: **público, não arquivado, não desabilitado**, default branch `dev`, último push `2026-11-16T17:14:53Z`. Ou seja: o fato de o checkout local estar sob `projetos/Arquivados/` **não** significa que o alvo remoto esteja inativo. | `[G0]` |
| Divergência local/remoto | HEAD local `bb16ba9a9` vs `github/dev` `856a3345da8e` (checkout desatualizado). Os arquivos relevantes (`rust.yml`, `config.toml`) são **idênticos** nos dois lados. Qualquer edição futura deve sincronizar antes. | `[L6][G1][G2][G3]` |
| Estabilidade do job informativo | 12/12 runs recentes: step `Vet (informational, Fatia A)` = `success`. **Mas** a janela é de `2026-11-16T16:49:58Z` a `17:14:51Z` — **~25 minutos**, não 1 semana. | `[G5]` |
| Conclusão do check no HEAD | `supply-chain-vet` = `success`; `build` = `success`; `dependency-review` = `skipped` (job só roda em PR). | `[G3]` |
| Upstream cargo-vet | issue #705 (`suggest --locked` panica) **aberta**; PR #706 aberta, `mergeable_state: dirty`; release latest `v0.10.3` (2026-10-22). Inalterado desde os docs 027/028. | `[U1][U2][U3]` |

**Leitura:** o gate hoje é verde, barato e determinístico. O bloqueio é **procedural e de governança**, não técnico — exatamente como o doc 028 já concluíra para a Fatia B.

### Pré-condições (todas precisam virar `VERIFICADO` antes da promoção)

- **C1 — Autorização:** mantenedor com permissão de admin (ou custom role "edit repository rules") aprova explicitamente a mudança de poder de merge. Pesquisa/autonomia de agente não cobre isso (§6).
- **C2 — Canonicalidade:** confirmar que o alvo é o repositório GitHub público acima (e não o Forgejo `origin` espelhado), e como os pushes chegam lá (ver NV-H).
- **C3 — Dívida rotulada:** revisar as 13 exemptions `"informational gate only"`, no mínimo `multer 3.1.0` (cuja nota pede revisit). Cada uma: auditar/importar, ou aceitar com `notes` revisada contendo responsável + data.
- **C4 — Janela:** ≥ 7 dias **e** ≥ 20 PRs com o **step** de vet em `success` (não apenas o job — `continue-on-error` mascara o job).
- **C5 — Owners:** 2 handles/times reais, com `write` explícito no repositório, confirmados para o CODEOWNERS (NV-A).
- **C6 — UX:** texto curto de CONTRIBUTING + PR de teste (§3 passo 6) validando bloqueio e review request.

---

## 2. O que "tornar bloqueante" significa (dois planos independentes)

Promover não é um único interruptor. São duas superfícies:

1. **Plano do workflow:** remover `continue-on-error: true` do job. Sem isso o job reporta `success` mesmo quando o step falha — literalmente *"Prevents a workflow run from failing when a job fails. Set to `true` to allow a workflow run to pass when this job fails"* `[D2]`. Um check required configurado sobre ele **nunca bloquearia**.
2. **Plano do GitHub:** o check `supply-chain-vet` precisa ser selecionado como **required status check** na proteção da branch de merge (`dev`) — ou numa ruleset que a cubra. Sem isso, um job vermelho apenas pinta a UI de vermelho: qualquer pessoa com write faz merge na mesma `[D1]`.

Fatos do GitHub que afetam diretamente a decisão:

- Required checks aceitam conclusão `successful`, `skipped` **ou** `neutral` `[D1]`; um job skipping reporta `Success` e **não bloqueia** `[D3]`. Por isso `continue-on-error` e filtros de path são armadilhas gêmeas.
- Workflows skipados por `paths:`/`branches:` deixam o check em `Pending` e **bloqueiam** o PR `[D2]`. **Não adicionar `paths:` ao job do vet.**
- Nomes de jobs devem ser **únicos entre todos os workflows**; nome duplicado gera resultado ambíguo e bloqueia merges `[D1]`. `supply-chain-vet` é único no repo `[L7]`.
- CODEOWNERS sozinho apenas **solicita** review. Para bloquear é preciso "Require review from Code Owners" na proteção `[D4][D5]`. Aprovação de **qualquer um** dos owners satisfaz o requisito `[D4]`.
- Owners (e aprovadores) precisam ter **write access** explícito `[D4]`; o CODEOWNERS precisa estar na **branch base** do PR `[D4]`; proteja o próprio `/.github/CODEOWNERS` `[D4]`.
- Rulesets têm status **Active/Disabled** (rollback em um clique, sem deletar config), são visíveis para qualquer usuário com read, e **camadas se agregam com a regra mais restritiva vencendo** `[D6][D7]`. Preferível à proteção clássica para este caso, justamente pelo rollback.
- Em ruleset, ao marcar "Require status checks before merging", é preciso **digitar o nome do check e clicar no `+`** para adicioná-lo; "Require branches to be up to date" só tem efeito se houver ao menos um check definido `[D7]`.

---

## 3. Passos exatos (sequência, com gates)

Todos os comandos na raiz do repo. Regra HQ: **nunca `git add -A` / `git add .`**; indexar só paths explícitos. Nada abaixo foi executado nesta tarefa.

### Passo 0 — autorização e sincronização (gate C1/C2)
1. Obter approval explícito do mantenedor/admin para: editar `rust.yml`, adicionar CODEOWNERS e **alterar regras de merge**. Registrar em issue/ADR quem autorizou e em qual repositório.
2. `git fetch github dev && git checkout dev && git pull --ff-only github dev` (o checkout local está desatualizado — §1).
3. `gh api repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora/branches/dev/protection` e `.../rulesets` novamente antes de mexer, para capturar drift.

### Passo 1 — sanear a dívida rotulada (gate C3)
```bash
cargo vet --locked --no-minimize-exemptions
grep -n 'informational gate only' supply-chain/config.toml
```
Para `multer 3.1.0` (o caso que pede revisit explícito): preferir `cargo vet inspect multer 3.1.0` + `certify`. Se o custo não couber, trocar a `notes` por uma decisão nominal datada, ex.:
`"Cycle 031: revisado por <handle> em 2026-11-16; parser multipart do axum-extra; aceito como dívida com reavaliação em 2027-02-16."`
Repetir para os outros 12. **Nunca** `cargo vet regenerate exemptions` como atalho (doc 027/028: o book chama a solução "mais agressiva" e pode enfraquecer critérios).

Opcional, se quiser reduzir o backlog futuro: `cargo vet import mozilla`, `google`, `bytecode-alliance` (URLs oficiais no registry; não importar `zcash` sem revalidar), seguido de `cargo vet --locked --no-minimize-exemptions`. Imports são otimização, não pré-requisito do gate. **Não** rodar `suggest --locked` (issue #705): usar `suggest --locked --no-registry-suggestions`.

### Passo 2 — cumprir a janela (gate C4)
Verificar o **step**, não o job:
```bash
gh api repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/workflows/rust.yml/runs?per_page=20 \
  --jq '.workflow_runs[].id' | while read -r id; do
  gh api repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/$id/jobs?per_page=100 \
    --jq '[.jobs[] | select(.name=="supply-chain-vet") | .steps[] | select(.name|test("Vet")) | .conclusion] | join(",")'
done
```
Critério: 20/20 `success` em ≥ 7 dias corridos. Hoje: 12/12 em ~25 min (§1) → **insuficiente**.

### Passo 3 — preparar UX (gate C6, parte 1)
Adicionar parágrafo curto em `docs/agents/` ou `CONTRIBUTING` (a ser criado, se não houver):

> Se o CI falhar em `supply-chain-vet`, rode `cargo vet --locked --no-minimize-exemptions`. Escolha, nesta ordem: (1) `cargo vet import <org>` para audit sets confiáveis; (2) `cargo vet inspect <crate> <ver>` + `certify`; (3) `cargo vet add-exemption <crate> <ver> --criteria safe-to-deploy --notes "<motivo, data, responsável>"`. **Nunca** use `regenerate exemptions` para destravar um PR. Mudanças em `supply-chain/` exigem review do CODEOWNER.

Decisões de UX já tomadas / recomendadas:
- **Não renomear** o job durante a promoção. Se quiser um label amigável depois, faça em PR separado atualizando o nome do required check em conjunto — nome divergente = PR travado `[D1]`.
- **Não usar `skip-checks: true` / `[skip ci]`** em commits que dependem do gate.
- **Deixar "Require branches to be up to date before merging" desmarcado no início** (modo *loose* `[D1]`). Justificativa: o workflow já roda em `push` na `dev`, então uma incompatibilidade pós-merge é detectada no próximo run; isso evita o churn de "Update branch" em equipe pequena. Reavaliar se houver merge de deps incompatíveis.
- Check required deve rodar em **todo PR**, inclusive forks: o job usa `pull_request`, não `pull_request_target`, não precisa de secrets, e `--locked` não faz fetch de imports `[U4]`.

### Passo 4 — PR de código (pode ser mergeado pelas regras atuais; sem admin)
Diff mínimo em `.github/workflows/rust.yml`:

```diff
   supply-chain-vet:
     runs-on: ubuntu-latest
-    continue-on-error: true
     steps:
       - uses: actions/checkout@v4
       - name: Install cargo-vet
         uses: taiki-e/install-action@cargo-vet
       - name: Vet (informational, Fatia A)
         run: cargo vet --locked --no-minimize-exemptions
```
Atenção: a linha 116 (`e2e-browser`) continua `continue-on-error: true` — **não remover**. Atualizar o `name:` do step de "informational" para refletir o novo estado é opcional e **não altera o nome do check** (que vem do job id).

Adicionar `.github/CODEOWNERS` (placeholders — preencher só com handles reais validados, C5):

```text
# Gate de supply-chain: toda mudança no store exige aprovador designado.
/supply-chain/       @<org>/<time-ou-usuario-1> @<org>/<time-ou-usuario-2>
/.github/CODEOWNERS  @<org>/<admin-owner>
```
Verificações obrigatórias antes de commitar: handles existem, são visíveis e têm `write` explícito; arquivo está na branch base; a segunda linha protege o próprio CODEOWNERS `[D4]`. Se a org não tiver time visível, usar 2 usuários com write.

Ratchet mecânico opcional (PR separado, depois da promoção): arquivo `supply-chain/EXEMPTION_BASELINE` com o número congelado e um step que falha se `grep -c '^\[\[exemptions\.' supply-chain/config.toml` exceder o baseline sem uma `notes` nova datada. Isso torna o ratchet auditável sem depender de memória humana.

### Passo 5 — settings do GitHub (somente admin; gate C1)
**Recomendado: ruleset**, pela capacidade de desligar sem destruir (rollback em um clique) `[D6]`:

1. `Settings → Rulesets → New ruleset → New branch ruleset`; nome: `flora-supply-chain-vet`.
2. Enforcement status: **`Disabled`** primeiro (configura tudo sem afetar ninguém).
3. Target branches: incluir `dev` (e, por decisão do admin, `main` — hoje `main` já exige 1 aprovação mas nenhum check `[G4]`).
4. Bypass list: **vazia** se o objetivo é bloquear também admins; se precisar de válvula de emergência, listar explicitamente 1 ator e registrar (isso enfraquece a garantia — ver §7).
5. Branch protections: marcar **Require status checks before merging** → digitar `supply-chain-vet` → **clicar no `+`**; marcar **Require review from Code Owners**; deixar "Require branches to be up to date" desmarcado (§3 passo 3).
6. Revisar o preview, salvar. Mudar enforcement para **`Active`** somente depois do passo 6.

Alternativa (se a equipe preferir proteção clássica): editar a regra existente de `dev`, marcar **Require status checks to pass before merging**, selecionar `supply-chain-vet` na busca, marcar **Require review from Code Owners** e **Do not allow bypassing the above settings** (equivalente a `enforce_admins: true`, hoje `false` `[G4]`). Atenção: só **uma** regra clássica se aplica por branch `[D1]`.

### Passo 6 — prova de bloqueio antes de ativar
Com a ruleset ainda `Disabled` (ou a regra clássica salva mas com check ainda não aplicado de fato):
1. Abrir PR de teste que adicione uma dependência nova fictícia/sem auditoria (`cargo add <crate>` em branch descartável) → `supply-chain-vet` deve ficar **vermelho no step**.
2. Confirmar que o review do CODEOWNER foi **solicitado** (PR de teste tocando `supply-chain/`).
3. Confirmar que um PR de fork com `pull_request` também reporta o check.
4. Só então alternar a ruleset para `Active` (ou aplicar a regra clássica). Se qualquer item falhar: manter `Disabled` e corrigir.

### Passo 7 — monitoria pós-promoção
Acompanhar por 7 dias: taxa de falha do step, tempo até aprovação do CODEOWNER, número de exemptions (ratchet), e se alguém usou `regenerate exemptions` ou bypass. Aplicar §5 se disparar gatilho.

---

## 4. Ratchet de revisão de exemptions (o que impede o gate virar teatro)

1. **Congelar a baseline** em 455 na data da promoção, com `notes` nominal para cada item rotulado (C3). Registrar o número em `docs/agents/` ou `EXEMPTION_BASELINE`.
2. **Toda exemption nova** exige `notes` com: motivo, data, responsável e tipo de decisão (`import` / `audit` / `exemption aceita`). Sem isso, PR não passa pelo CODEOWNER.
3. **Nunca `regenerate exemptions` como resolução de PR.** `cargo vet prune` é permitido apenas em commit revisado com diff visível.
4. **Meta declarada: o número diminui.** Revisão trimestral com `cargo vet suggest --locked --no-registry-suggestions` → escolher deltas → `certify` → `prune`. Marcar `suggest = false` só com critério documentado e `notes` (doc 026 §6).
5. **Prioridade se sobrar tempo:** fronteira de input não confiável (`multer`, `tower-http`, `axum`, `reqwest`, `sqlx`) e auth/crypto (`jsonwebtoken`, `rsa`, `argon2`, `rustls*`) — sempre delta antes de full `[U5]`.
6. **Mecanização (opcional, PR separado):** step de CI que compara `grep -c '^\[\[exemptions\.'` com o baseline e falha em aumento não justificado. Sem isso, o ratchet depende de disciplina humana + CODEOWNER.

---

## 5. Gatilhos de rollback

| Gatilho | Ação (em ordem de preferência) |
|---|---|
| Step `Vet` falha de forma **não reproduzível localmente** em **2 PRs consecutivos** | 1) ruleset → `Disabled` (ou re-adicionar `continue-on-error: true` no job); 2) manter `cargo deny` como gate único; 3) investigar com `--verbose info`. |
| Alguém propõe `regenerate exemptions` para destravar PR | Pausar a promoção; abrir issue; **não** aceitar o regenerate. |
| `cargo vet check --locked` passa a panicar (regressão além da issue #705) | Voltar a informativo, pinar/repinar `0.10.3`, reportar upstream. |
| `taiki-e/install-action@cargo-vet` falha no runner | Trocar pela receita canônica do book (`CARGO_VET_VERSION` + tool cache + `cargo install`); é troca de instalação, não de política `[U6]`. |
| Owner do CODEOWNERS sai da org / perde write | Review request deixa de ser atribuído; substituir handle **antes** do próximo merge. |
| Check fantasma (fica `Pending`/não reportado em forks) | Validar em PR de teste; se não reportar, desativar regra até corrigir o trigger. |
| Ferramenta abandonada / instabilidade recorrente | Ruleset `Disabled`; manter `cargo deny check bans licenses sources` + advisory agendado. |
| Pressão para "zerar as 455" | Recusar; apresentar o backlog ~8,7M linhas e oferecer 13 rotuladas + imports. |

**Nunca usar como rollback:** apagar `supply-chain/`, desfazer o versionamento do store ou `git rm supply-chain/*`. Isso destrói a baseline e torna qualquer reativação mais cara (docs 027/028).

---

## 6. Autorização: a resposta direta

**Sim, a promoção exige autorização explícita de mantenedor/admin — e ela não foi dada nesta tarefa.** O escopo recebido era **research-only** e o usuário proíbe ações não autorizadas. A autonomia concedida cobre: ler, consultar docs/API, redigir este documento e propor diff/plano. Não cobre:

| Ação | Quem pode | Por quê |
|---|---|---|
| Editar `.github/workflows/rust.yml` e commitar `.github/CODEOWNERS` | Qualquer pessoa com write (via PR), mas o **merge** precisa de aprovação conforme as regras vigentes | É mudança de código; ainda permitida hoje porque `dev` não exige checks/reviews `[G4]` |
| Criar/editar ruleset ou branch protection | Admin do repo ou custom role com "edit repository rules" `[D1][D8]` | API não autenticada retornou `401 Requires authentication` `[G4]`; `gh` está autenticado como `lfgranja`, mas o token `repo`+`workflow` **não prova** permissão de admin — isso é NV-B |
| Escolher o dono do CODEOWNERS | Mantenedor humano | Exige identidade real e `write` explícito; inventar handle é ação não autorizada (NV-A) |
| Mudar enforcement de `Disabled` → `Active` | Admin | É o ato que **altera o poder de merge** de todas as pessoas no repo |

Portanto: nenhuma dessas ações foi executada. O documento é um plano pronto para ser aprovado, não uma promoção disfarçada.

---

## 7. Riscos e NVs (não validado nesta pesquisa)

- **NV-A (bloqueante)** — Handles/times reais para o CODEOWNERS e seus `write` permissions não verificados. Impede o passo 4.
- **NV-B (bloqueante)** — Identidade com permissão de "edit repository rules" no GitHub não confirmada (o token `gh` autentica, mas não prova admin). Impede o passo 5.
- **NV-C (bloqueante)** — Canonicalidade ambígua: checkout local fica em `projetos/Arquivados/flora` (cujo `AGENTS.md` diz não modernizar arquivados sem tarefa explícita), enquanto o remoto `github` está público, ativo e com push hoje. Confirmar qual é a fonte da verdade antes de qualquer mudança.
- **NV-D** — Janela de observação: 12/12 verdes, mas ~25 min. A condição "1 semana" do doc 028 **não** está satisfeita.
- **NV-E** — As 13 exemptions `"informational gate only"` não foram auditadas/aceitas nesta tarefa; uma delas (`multer`) pede revisit explícito.
- **NV-F** — Sem imports e sem auditorias (0 `[imports]`, `audits.toml` vazio), o gate bloqueia apenas dependências **novas**; não há cobertura adicional para o passado. Número de backlog (~8,7M linhas) vem do doc 027 e depende de estado externo — diagnóstico, não meta.
- **NV-G** — upstream: #705 aberta, #706 aberta e em conflito. `check --locked` não panica, mas o pin exato da versão via `taiki-e/install-action@cargo-vet` não foi validado (o uso **sem pin** funciona — 12/12).
- **NV-H** — `origin` é Forgejo (`forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`) e o STATE.md do HQ menciona push mirroring Forgejo → GitHub. **Não foi verificado** se contribuidores pushed direto no GitHub, se o mirror pode reescrever `dev` e se a proteção do GitHub bloqueia/é contornada por esse caminho. Isso pode tornar o "gate bloqueante" ineficaz na prática — investigar no passo 0.
- **NV-I** — Comportamento de required check + CODEOWNERS em PR de fork não testado (previsto no passo 6).
- **NV-J** — Custo de tempo do CODEOWNER (review de toda mudança em `supply-chain/`) não medido; com 2 owners e equipe pequena, risco de gargalo.
- **NV-K** — Ratchet é hoje apenas procedural (sem step automático); não há garantia mecânica contra crescimento silencioso.
- **NV-L** — Checkout local desatualizado em relação a `github/dev`; edições sem `fetch`/`pull` podem conflitar.
- **NV-M** — Se o job ganhar `name:` no futuro, o nome do required check muda e precisa ser atualizado em conjunto (§3 passo 3).
- **NV-N** — Não executei `cargo vet`, `cargo vet import`, `npm`/`bun`, nem qualquer comando com efeito de escrita no repo; todos os resultados de gate vêm de leitura de arquivos e da API do GitHub.

---

## 8. Cross-check multilíngue (pt, es, de, fr, zh-CN, ja)

As páginas localizadas dos GitHub Docs foram reconsultadas em 2026-11-16 para verificar se alguma tradução altera a mecânica (por exemplo, se "required" virasse "recomendado", ou se "qualquer owner" virasse "todos"). **Nenhuma divergência operacional encontrada.**

| Idioma | "required status checks" | "code owners" | Observação |
|---|---|---|---|
| EN | required status checks | code owners | canônico `[D1][D4]` |
| pt | verificações de status (de passagem) | proprietários de código | mesmo aviso de nomes de job únicos `[T-pt]` |
| es | comprobaciones de estado | propietarios de código | "nombres de trabajo únicos" `[T-es]` |
| de | Statusüberprüfungen | Code-Eigentümer / Codebesitzer | "Auftragsnamen ... eindeutig" `[T-de]` |
| fr | vérifications d'état / contrôles d'état | propriétaires de code | "nom de tâche" único `[T-fr]` |
| zh-CN | 状态检查 | 代码所有者 | "作业名称都是唯一的" `[T-zh]` |
| ja | ステータスチェック | コードオーナー | "ジョブ名がすべてのワークフローで一意" `[T-ja]` |

Consistências verificadas em todos os idiomas consultados:
1. **Aviso de nomes de job únicos** entre workflows (evita resultado de check ambíguo e merge bloqueado).
2. **Aprovadores/owners precisam de permissão de escrita** no repositório.
3. **Aprovação de qualquer um dos owners** satisfaz o requisito de code owner review.

Nota de acesso: o locale português dos docs é `/pt/` (não `/pt-br/`, que retorna 404); os demais são `/es/`, `/de/`, `/fr/`, `/zh/`, `/ja/`.

---

## 9. Fontes (reconsultadas em 2026-11-16)

**Local (somente leitura)**
- `[L1]` `.github/workflows/rust.yml` (job `supply-chain-vet` linhas 87–95; `e2e-browser` linha 116).
- `[L2]` `supply-chain/config.toml` — 455 blocos `[[exemptions.*]]`; 443 `safe-to-deploy`, 12 `safe-to-run`; 0 `[imports]`.
- `[L3]` `supply-chain/imports.lock` (2 linhas), `supply-chain/audits.toml` (4 linhas).
- `[L4]` `grep -n 'informational gate only' supply-chain/config.toml` — 13 ocorrências.
- `[L5]` `find/.github` — nenhum CODEOWNERS.
- `[L6]` `git status`, `git rev-parse HEAD`, `git remote -v` — HEAD `bb16ba9a9`, remotos `github`/`origin`.
- `[L7]` `grep -rn 'supply-chain-vet' .github/workflows/*.yml` — ocorrência única.

**GitHub API / `gh` (somente leitura)**
- `[G0]` `GET /repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora` — público, `archived: false`, `default_branch: dev`.
- `[G1]` `GET /repos/.../contents/.github/workflows/rust.yml?ref=dev` — idêntico ao local nos pontos relevantes.
- `[G2]` `GET /repos/.../contents/supply-chain/config.toml?ref=dev` e `/contents/supply-chain` — 455, 3 arquivos versionados.
- `[G3]` `GET /repos/.../commits/856a3345da8e/check-runs` e `/status`.
- `[G4]` `GET /repos/.../branches/{dev,main}/protection` e `GET /repos/.../rulesets`; tentativa não autenticada → 401.
- `[G5]` `GET /repos/.../actions/workflows/rust.yml/runs?per_page=12` + `GET /actions/runs/{id}/jobs` — 12/12 step `success`, janela 16:49–17:14 UTC.
- `[G6]` `GET /repos/.../contents/.github/CODEOWNERS?ref=dev` → 404.

**GitHub Docs (EN)**
- `[D1]` About protected branches — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- `[D2]` Workflow syntax (`jobs.<job_id>.continue-on-error`, filtros `paths`/`branches`) — https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
- `[D3]` Status checks (`skipped` reporta Success) — https://docs.github.com/en/pull-requests/reference/status-checks
- `[D4]` About code owners — https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- `[D5]` Managing a branch protection rule — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule
- `[D6]` About rulesets — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets
- `[D7]` Creating rulesets for a repository — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- `[D8]` permissão "edit repository rules" citada em `[D1]`.

**Localizações verificadas**
- `[T-pt]` https://docs.github.com/pt/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- `[T-es]` https://docs.github.com/es/... (同上)
- `[T-de]` https://docs.github.com/de/... (同上)
- `[T-fr]` https://docs.github.com/fr/... (同上)
- `[T-zh]` https://docs.github.com/zh/... (同上)
- `[T-ja]` https://docs.github.com/ja/... (同上)

**Upstream cargo-vet**
- `[U1]` issue #705 (aberta) — https://github.com/mozilla/cargo-vet/issues/705
- `[U2]` PR #706 (aberta, `dirty`) — https://github.com/mozilla/cargo-vet/pull/706
- `[U3]` release v0.10.3 (2026-10-22) — https://api.github.com/repos/mozilla/cargo-vet/releases/latest
- `[U4]` Importing Audits (`--locked` pula fetch; não transitivo) — https://mozilla.github.io/cargo-vet/importing-audits.html
- `[U5]` Performing Audits (deltas, backlog) — https://mozilla.github.io/cargo-vet/performing-audits.html
- `[U6]` Configuring CI — https://mozilla.github.io/cargo-vet/configuring-ci.html

---

## 10. Resumo executivo

- **DEFERIR a promoção neste ciclo.** Não é uma objeção técnica: o gate está verde (12/12), barato e determinístico.
- O impedimento é de **governança e autorização**: sem owner de CODEOWNERS validado, sem admin que possa editar regras de merge, sem janela de 7 dias e sem revisão das 13 exemptions rotuladas como informativas, promover mudaria o poder de merge com base em dívida não assumida.
- A sequência correta é: **autorizar → sincronizar → sanear 13 rótulos → observar 7 dias/20 PRs → PR de código (remover só a linha 89) → CODEOWNERS → ruleset `Disabled` → prova de bloqueio → `Active`**.
- O nome do check a ser exigido é **`supply-chain-vet`** (job id), não `cargo-vet`.
- **Não** adicionar filtros `paths:`, **não** renomear o job no mesmo PR, **não** confundir o `continue-on-error` do `e2e-browser` (linha 116).
- rollback é barato se usar ruleset: basta alternar `Active → Disabled`. Nunca apagar o store.
- **Nenhuma mudança foi aplicada** nesta tarefa; a promoção depende de aprovação explícita de mantenedor/admin (§6).
