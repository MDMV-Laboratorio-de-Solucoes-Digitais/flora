> **Superseded by PR #236/#237 (cycle 028):** this is a pre-implementation snapshot; check committed files for current state.

# 028 — Plano de implementação exato do gate cargo-vet para o Flora (2026-11-16)

**Data:** 2026-11-16
**Escopo:** pesquisa somente leitura + auditoria somente leitura do estado atual do repo. **Nenhum arquivo de configuração, workflow, `.gitignore`, `supply-chain/`, `Cargo.lock` ou código foi alterado.** O único arquivo escrito por esta tarefa é este documento.
**Antecedentes:** complementa `2026-11-16-026-cargo-vet-exemptions.md`, `2026-11-16-027-advisory-sbom-alternatives.md` e, sobretudo, `2026-11-16-027-cargo-vet-gate-feasibility.md` (doc 027), que reproduziu empiricamente o comportamento do gate em cópia descartável.
**Pergunta:** qual é a sequência exata de menor risco para transformar o cargo-vet em gate de dependência nova no Flora, com imports, CI informativo primeiro, CODEOWNERS e gatilhos de rollback?

**Convenção de evidência:** `[S#]` = fonte primária reconsultada hoje (§9). `[V#]` = verificação somente leitura feita nesta auditoria no checkout real (§2). **`VERIFICADO`** = observado diretamente; **`INFERIDO`** = deduzido de documento/documentação, sem execução no checkout real; **`NV`** = não validado, listado em §7.

> **Nota de rede:** ao contrário do receio inicial, a rede estava disponível nesta sessão. Todos os URLs primários em §9 responderam HTTP 200 em 2026-11-16, incluindo `registry.toml`, as páginas do book, os docs do GitHub e as APIs de issue/PR/release do upstream. Ainda assim, o plano §4 inclui fallback explícito para ambiente sem rede, porque o `cargo vet import` depende de `raw.githubusercontent.com`.

---

## 0. Veredito e recomendação go/no-go

| Fatia | Veredito | Justificativa |
|---|---|---|
| **Fatia A (passos 0–4)**: versionar store, fechar `multer`, 3 imports, job CI **informativo** | **GO** | Pequena, reversível, verificável em um único PR. Reproduzida no doc 027. Corrige o bloqueio real (store invisível + `multer` fora do config). |
| **Fatia B (passo 5)**: tornar bloqueante + CODEOWNERS + branch protection | **GO condicionado** a 1 semana de job informativo verde e a handle de owner válida (NV5). | Barato, mas muda o poder de merge; não deve estrear no mesmo PR da Fatia A. |
| Zerar as 442 exemptions | **NO-GO** | Backlog estimado em 8,7M linhas [S10][S19]. Fora de escopo. |
| Usar `suggest` no caminho crítico | **NO-GO** | Issue #705 aberta; `suggest --locked` panica [S14]. `check --locked` não panica. |
| `regenerate exemptions` como resolução de PR | **NO-GO** | O book chama a solução "mais agressiva" e adverte enfraquecimento de critérios [S7]. |

**Recomendação:** aprovar **apenas a Fatia A** agora, como um PR atômico verificável. Não aprovar a Fatia B no mesmo ciclo. O critério de aceite da Fatia A é objetivo: `cargo vet --locked --no-minimize-exemptions` termina com `Vetting Succeeded (443 exempted)` localmente e o job informativo aparece verde no primeiro PR após o merge.

**Frase-síntese:** o gate não depende de auditoria nova nem de imports — depende de (1) o store entrar no Git, (2) `multer 3.1.0` deixar de ser a única lacuna. Os 3 imports são multiplicador opcional, não pré-requisito.

---

## 1. Auditoria do estado atual (somente leitura, 2026-11-16)

| Item | Estado `VERIFICADO` | Evidência |
|---|---|---|
| Branch / HEAD | `dev` @ `e9ef62589` (`Merge remote-tracking branch 'github/dev' into dev`); working tree limpo | `[V1]` `git status --short` vazio |
| `supply-chain/` existe | Sim: `audits.toml` (35 B), `config.toml` (~31.9 KB), `imports.lock` (26 B) | `[V2]` `ls -la supply-chain/` |
| `supply-chain/` versionado? | **Não.** `git ls-files supply-chain` vazio; `git check-ignore -v` aponta `.gitignore:163:supply-chain/` | `[V3]` |
| `.gitignore:163` | `supply-chain/`, sob a seção `# Local tooling / environment artifacts` (linhas 160–163) | `[V4]` leitura direta |
| `config.toml` | `[cargo-vet] version = "0.10"`; **442** blocos `[[exemptions.*]]` | `[V5]` `grep -c '^\[\[exemptions\.'` |
| Divisão de critérios | **430 `safe-to-deploy` + 12 `safe-to-run`** | `[V5]` `grep '^criteria = ' | sort | uniq -c` |
| 12 `safe-to-run` | `bit-set`, `bit-vec`, `linux-raw-sys`, `proptest`, `quick-error`, `rand_xorshift`, `rustix`, `rusty-fork`, `tempfile`, `tokio-test`, `unarray`, `wait-timeout` | `[V6]` `awk` sobre o config |
| `audits.toml` | Vazio: `# cargo-vet audits file` + `[audits]` | `[V2]` leitura direta |
| `imports.lock` | Vazio: `# cargo-vet imports lock` | `[V2]` leitura direta |
| `multer` no config | **Ausente** | `[V5]` `grep -n multer` → vazio |
| `Cargo.lock` | **451** pacotes (`grep -c '^name = '` = 451, `grep -c '^\[\[package\]\]'` = 451); `multer 3.1.0` presente com source registry | `[V7]` |
| Diff vs doc 027 | Doc 027 afirmou **452** pacotes. Hoje são **451**. `multer` segue ausente do config, então a conclusão do doc 027 (gate falha por `multer`) permanece válida. A causa da diferença (remoção de dep entre a pesquisa e a auditoria, ou divergência de contagem) **não foi investigada** — ver NV11. | `[V7]` + doc 027 §2 |
| `.github/workflows/` | `ci.yml` (frontend Bun), `rust.yml` (Rust), `docker-staging.yml` | `[V8]` `ls -la` |
| `rust.yml` | Já roda fmt check, clippy estrito (`--lib --bins`), `taiki-e/install-action@cargo-deny`, `cargo deny check bans licenses sources`, build, test, 6 suítes DB-gated em `db-tests`. **Sem cargo-vet, sem cargo-audit.** | `[V9]` leitura direta |
| CODEOWNERS | **Ausente** em todo o repo | `[V10]` `find ... -iname '*codeowners*'` vazio |
| Remotes | `github` → `github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora.git`; `origin` → Forgejo interno | `[V11]` `git remote -v` |
| Versão do binário local | `cargo vet --version` → **0.10.3** | `[V12]` |
| Upstream | Release latest `v0.10.3` (2026-10-22); issue #705 **aberta**; PR #706 **aberta, `mergeable_state: dirty`** (conflito), não mergeada | `[S14][S15][S16]` via API GitHub 2026-11-16 |

**Leitura:** o estado é idêntico ao retratado no doc 027, com a única delta material sendo 451 vs 452 pacotes no lock. Nada mudou que invalide o plano: o gate hoje **falharia** por `multer:3.1.0`.

---

## 2. Por que esta ordem (princípio de desenho)

1. **Tornar possível** → store versionado (passo 1).
2. **Tornar verde** → `multer` (passo 2). O comando do doc 027 é o único que foi executado com sucesso em cópia descartável; replicá-lo reduz risco.
3. **Tornar mais barato** → imports (passo 3), que só **adicionam** cobertura e portanto não podem fazer um `check` verde ficar vermelho.
4. **Tornar visível** → CI informativo (passo 4), para medir estabilidade sem bloquear merges.
5. **Tornar bloqueante** → passo 5, em PR separado, depois da janela de observação.

Inverter essa ordem (começar pelo job bloqueante) faria o primeiro PR de todo contribuidor falhar por `multer` e pelo store ausente — o oposto do objetivo de "baixo atrito".

---

## 3. Plano exato — Fatia A (GO agora)

Todos os comandos abaixo são executados na raiz do repo. **Não usar `git add -A` / `git add .`** (regra HQ): indexar apenas os paths explícitos listados.

### Passo 0 — pré-condição (reproduzir a falha no checkout real, somente leitura)

```bash
cargo vet --locked --no-all-features --no-minimize-exemptions
```

Esperado (doc 027 §5.1): `Vetting Failed!` / `1 unvetted dependencies: multer:3.1.0 missing ["safe-to-deploy"]`.

Se **não** falhar por `multer`, parar e investigar antes de continuar — significa que o estado divergiu do auditado aqui (NV11). Este comando é seguro: `--no-minimize-exemptions` impede escrita no `config.toml` e `--locked` impede fetch de imports [S7].

### Passo 1 — tornar o store versionado (bloqueante para todo o resto)

Editar `.gitignore`: **remover a linha 163** (`supply-chain/`) e, de preferência, substituir por uma linha de comentário explicando que o store agora é versionado. Não remover as linhas 161–162 (`.clean-room/`, `.commandcode.inactive-*/`).

```bash
git add .gitignore supply-chain/config.toml supply-chain/audits.toml supply-chain/imports.lock
git commit -m "build(flora): versiona store do cargo-vet para gate de dependências"
```

Notas:
- Commitar `audits.toml` e `imports.lock` **mesmo vazios**: o book exige que os arquivos do store entrem no versionamento [S2]. É o que dá histórico e torna o CODEOWNERS útil depois.
- Se este passo for recusado, **cortar o cargo-vet inteiro** e manter apenas `cargo deny` + advisory (doc irmão). Sem store versionado não há gate auditável.

### Passo 2 — fechar a única lacuna (`multer`)

```bash
cargo vet add-exemption multer 3.1.0 --criteria safe-to-deploy \
  --notes "Bootstrap 028: pré-existente em 2026-11-16; parser multipart do axum-extra; revisão de fronteira de input pendente."
cargo vet --locked --no-minimize-exemptions
```

Esperado: `Vetting Succeeded (443 exempted)`.

Alternativa, **se houver tempo e revisão real**: `cargo vet inspect multer 3.1.0` + `certify`, examinando o parser de multipart (fronteira de input não confiável) [S10]. Isso é preferível no mérito, mas aumenta o tamanho da fatia; para um slice pequeno e verificável, a exemption com `notes` é a escolha correta e está alinhada ao doc 027 §6 (rotular dívida, não auditar em massa).

```bash
git add supply-chain/config.toml
git commit -m "build(flora): exempt multer 3.1.0 com nota de bootstrap"
```

> Mesmo com imports vindo depois, o número de exempted pode mudar; o critério de aceite é **sucesso**, não o número exato de exemptions.

### Passo 3 — bootstrap com 3 imports (com rede; **sem** `--locked`)

```bash
cargo vet import mozilla
cargo vet import google
cargo vet import bytecode-alliance
cargo vet fmt                       # opcional; normaliza TOMLs editados pela ferramenta
cargo vet --locked --no-minimize-exemptions
git add supply-chain/config.toml supply-chain/imports.lock
git commit -m "build(flora): importa 3 audit sets (mozilla, google, bytecode-alliance)"
```

URLs oficiais do `registry.toml` [S5], reconsultados hoje (`VERIFICADO`):

| Nickname | URL | Status hoje |
|---|---|---|
| `mozilla` | `https://raw.githubusercontent.com/mozilla/supply-chain/main/audits.toml` | 200 |
| `google` | `https://raw.githubusercontent.com/google/supply-chain/main/audits.toml` | 200 |
| `bytecode-alliance` | `https://raw.githubusercontent.com/bytecodealliance/wasmtime/main/supply-chain/audits.toml` | 200 |
| `zcash` (não importar) | `https://raw.githubusercontent.com/zcash/rust-ecosystem/main/supply-chain/audits.toml` | 200 |

Decisão sobre os 3: **GO**. O trio é o exemplo literal do Microsoft RustTraining [S17] e do Trail of Bits [S18], e o book chama imports a "forma mais rápida de encolher a lista de exemptions" [S4]. Não importar `zcash` agora: quarto conjunto não justifica confiança adicional sem necessidade (doc 027 §4/NV4).

Duas propriedades que tornam isso seguro [S4]:
- **Não transitivo**: confiar na Mozilla não confia nas escolhas da Google; cada import é unilateral.
- **`--locked` pula o fetch**: depois dos imports commitados, o CI usa `imports.lock` e não faz rede. Portanto imports **não introduzem** dependência de rede no gate.

**Fallback de risco de rede (importante):** se `cargo vet import` falhar por falta de acesso a `raw.githubusercontent.com`, **não bloquear a Fatia A**. Pular o passo 3 inteiro, commitar apenas passos 1–2 e rodar o gate com `imports.lock` vazio (o gate funciona igual — imports são otimização, não pré-requisito). Retry dos imports em PR separado. Isso é superior a editar `imports.lock` à mão, cujo formato é gerado pela ferramenta e não é estável para escrita manual.

### Passo 4 — job CI informativo (primeira semana)

Adicionar job ao final de `.github/workflows/rust.yml` (arquivo já existente; não criar outro workflow):

```yaml
  cargo-vet:
    name: Vet new dependencies (informational)
    runs-on: ubuntu-latest
    continue-on-error: true   # remover na Fatia B, após 1 semana
    steps:
      - uses: actions/checkout@v4
      - name: Install cargo-vet
        uses: taiki-e/install-action@cargo-vet
      - name: Vet (gate de código novo)
        run: cargo vet --locked --no-minimize-exemptions
```

Decisões de desenho:
- **`continue-on-error: true` na primeira semana**: mede estabilidade no runner sem sequestrar merges. É a fatia verificável — o job aparece como sucesso (amarelo) mesmo se o vet falhar, e o log vira evidência.
- **Comando idêntico ao validado localmente**, exceto pela remoção de `--no-all-features` (no-op hoje: 451 nós com e sem o flag no estado auditado; doc 027 §2 mediu 452 nos dois modos). Se preferir aderência literal ao experimento, usar `cargo vet --locked --no-all-features --no-minimize-exemptions`; o resultado é o mesmo. **Não remover `--no-minimize-exemptions`**: é a flag que impede o CI de reescrever `config.toml` e podar a baseline silenciosamente [S7].
- **Instalação**: `taiki-e/install-action@cargo-vet` é consistente com o `taiki-e/install-action@cargo-deny` já usado em `rust.yml` linha 31. O input exato da action **não foi validado** (NV6). A receita **garantida** pelo book é `CARGO_VET_VERSION: 0.10.3` + tool cache + `cargo install --version ... cargo-vet` [S8]. Se a action falhar no primeiro run, trocar pela receita do book — não é uma decisão de arquitetura, só de instalação.
- **Pinar a versão** (`0.10.3`) na Fatia B, quando o job virar bloqueante, para reprodutibilidade [S8].
- **Ordem dos argumentos**: `cargo vet --locked ...` (sem subcomando) é válido para `check`; o problema de parsing só ocorre em `cargo vet --locked suggest` [S19]. O job acima não usa `suggest`, então está seguro.

```bash
git add .github/workflows/rust.yml
git commit -m "ci(flora): adiciona cargo-vet informativo ao Rust CI"
```

### Aceite da Fatia A

1. `git ls-files supply-chain` lista os 3 arquivos.
2. `cargo vet --locked --no-minimize-exemptions` → `Vetting Succeeded` no checkout pós-merge.
3. `git status --short` limpo após o comando (prova de que `--no-minimize-exemptions` funcionou).
4. Job `cargo-vet` aparece no primeiro PR após o merge, sem erro de instalação.
5. `git diff` do PR contém apenas: `.gitignore`, `supply-chain/*`, `.github/workflows/rust.yml` — nenhum `Cargo.lock`, nenhum código.

---

## 4. Fatia B — bloqueante + governança (GO condicionado, PR separado)

Só executar após 1 semana com o job informativo estável e com owner real validado.

1. Remover `continue-on-error: true` de `cargo-vet` em `rust.yml`.
2. Adicionar `.github/CODEOWNERS`:

```text
# Gate de supply-chain: mudanças no store exigem aprovador designado.
/supply-chain/       @MDMV-Laboratorio-de-Solucoes-Digitais/<time-ou-usuario>
/.github/CODEOWNERS  @MDMV-Laboratorio-de-Solucoes-Digitais/<owner>
```

Fatos verificados sobre CODEOWNERS [S12][S13]:
- GitHub procura em `.github/`, raiz, `docs/` — nessa ordem, e usa o primeiro encontrado.
- O arquivo **precisa estar na branch base** do PR para gerar review request. Como o merge é em `dev`, commitar em `dev`.
- CODEOWNERS sozinho **apenas solicita** review. Para **bloquear**, ativar "Require review from Code Owners" na proteção da branch `dev` [S12][S13].
- A aprovação de **qualquer um** dos owners satisfaz o requisito [S12].
- Usuário/team precisa ter **write access** explícito [S12]. `<time-ou-usuario>` e `<owner>` devem ser preenchidos com handles reais — ver NV5.
- Proteger `/.github/CODEOWNERS` com segunda linha evita que o dono seja removido por PR comum [S12].

3. Regra de time (documentar em `docs/agents/` ou no próprio CODEOWNERS como comentário):
   - `regenerate exemptions` **nunca** é resolução padrão de PR [S7].
   - Toda exemption nova exige `notes` + data; auditoria ou import são preferidos a exemption.
   - Revisão trimestral do backlog: `cargo vet suggest --locked --no-registry-suggestions` → `cargo vet prune` → commit revisado. **Nunca** `suggest --locked` puro (panic, §5).

---

## 5. Gatilhos de rollback

| Gatilho | Ação | Por quê |
|---|---|---|
| `cargo vet --locked --no-minimize-exemptions` falha de forma **não reproduzível localmente** em **2 PRs consecutivos** | Voltar o job para `continue-on-error: true`; manter `cargo deny` como gate único; investigar com `--verbose info` | Doc 027 §8. Evita gate fantasma. |
| Um PR só fica verde com `regenerate exemptions` | Pausar o gate bloqueante; abrir issue; **não** aceitar o regenerate | O book adverte enfraquecimento de critérios [S7]. |
| `check --locked` passa a panicar (além do `suggest`) | Reverter para `continue-on-error`, pinar/repinar versão,_reportar upstream | Issue #705 é isolada em `suggest` [S14]; novo panic seria regressão. |
| `taiki-e/install-action@cargo-vet` falha no runner | Substituir pela receita `cargo install` + tool cache do book [S8] | NV6; é troca de instalação, não de política. |
| Rede indisponível para imports | Pular imports; manter gate com `imports.lock` vazio | §3 passo 3; imports não são pré-requisito. |
| Ferramenta abandonada / instabilidade recorrente | Remover job de vet; manter `cargo deny check bans licenses sources` + advisory do doc irmão | Fallback não é "sem política" [S18][S19]. |
| Pressão para "zerar as 442" | Recusar; apresentar 8,7M linhas; oferecer 3 imports + deltas | Doc 027 §5.4. |

**Não usar como rollback:** apagar o store, desfazer o passo 1, ou `git rm supply-chain/*`. Isso destruiria a baseline e tornaria qualquer reativação mais cara.

---

## 6. Riscos e NVs

Herdados do doc 027 (revalidados onde aplicável):

- **NV1** — Gate em CI limpo com store versionado não foi executado de ponta a ponta. O aceite §3 cobre isso.
- **NV2** — `--no-all-features` é no-op hoje; efeito de futura dep opcional não medido. Remover o flag é a opção conservadora (adotado no job proposto).
- **NV3** — Números de backlog (8,7M → 7,0M → 6,8M) vêm de `suggest` com registry e dependem de estado externo; são diagnóstico, não meta.
- **NV5** — Handles/times reais para CODEOWNERS e suas permissões de write **não verificados**. Impede a Fatia B até serem confirmados. (Org confirmada via remote `[V11]`; time não.)
- **NV6** — `taiki-e/install-action@cargo-vet` (input/versão) **não validado**; receita do book é a garantida.
- **NV7** — Impacto de `suggest = false` em massa não medido; aplicar só com critério documentado.
- **NV8** — Auditoria real do `multer` (linhas exatas, surface de multipart) não feita.
- **NV9** — PR #706 pode ser mergeado a qualquer momento, mudando `suggest --locked`. **Revalidado hoje:** ainda aberta e em conflito (`dirty`) `[S15]`, então o workaround segue necessário.
- **NV10** — Custo de CI (install + cache) não medido no runner do Flora.

Novos nesta auditoria:

- **NV11** — `Cargo.lock` tem **451** pacotes hoje vs **452** no doc 027. A causa da diferença não foi investigada. Impacto esperado: nenhum (`multer` segue presente e fora do config), mas o passo 0 existe para confirmar.
- **NV12** — Não executei `cargo vet add-exemption`, `cargo vet import` nem `cargo vet check` no checkout real (restrição de somente leitura). Todos os resultados são do experimento em cópia descartável do doc 027, reproduzido com o mesmo binário 0.10.3 `[V12]`.
- **NV13** — Não verifiquei se a proteção da branch `dev` existe ou qual é sua configuração atual; "Require review from Code Owners" pode já estar ativa ou não. Checar antes da Fatia B.
- **NV14** — `git status` limpo `[V1]` garante que não havia trabalho local não commitado no momento da auditoria, mas as Fatias A/B devem ser feitas após sincronizar com `github/dev` para evitar conflito com agentes paralelos.

**Risco de magnitude:** o store `config.toml` tem ~31.9 KB e 442 entradas. Adicionar 1 exemption + 3 imports é diff pequeno e revisável. O risco não é o tamanho do diff; é o **procedimento** (store fora do Git e CI bloqueante precoce). O plano ataca exatamente esses dois pontos.

---

## 7. Fontes primárias (reconsultadas 2026-11-16; acesso HTTP 200)

**cargo-vet book (Mozilla)**
- [S1] Introduction / How it Works — https://mozilla.github.io/cargo-vet/ e https://mozilla.github.io/cargo-vet/how-it-works.html
- [S2] Setup ("Vetting Succeeded (X exempted)", arquivos devem ser versionados) — https://mozilla.github.io/cargo-vet/setup.html
- [S4] Importing Audits ("--locked skips the fetch", "not transitive") — https://mozilla.github.io/cargo-vet/importing-audits.html
- [S5] registry.toml oficial (URLs dos 3 imports + zcash) — https://raw.githubusercontent.com/mozilla/cargo-vet/main/registry.toml
- [S7] Commands (`--locked` = "Do not fetch new imported audits"; `--no-all-features`; `--no-minimize-exemptions` = "Prevent commands such as check and certify from automatically cleaning up unused exemptions"; `--no-registry-suggestions`; `--frozen` requer `--locked`; `regenerate exemptions`) — https://mozilla.github.io/cargo-vet/commands.html
- [S8] Configuring CI (receita canônica com `CARGO_VET_VERSION` + tool cache + `cargo vet --locked`) — https://mozilla.github.io/cargo-vet/configuring-ci.html
- [S9] Curating Your Audit Set (CODEOWNERS, >5 aprovadores, "no real way to check their work") — https://mozilla.github.io/cargo-vet/curating-your-audit-set.html
- [S10] Performing Audits (`suggest`, backlog, deltas, "exemptions frequently undermine the value") — https://mozilla.github.io/cargo-vet/performing-audits.html

**GitHub / upstream**
- [S12] GitHub Docs — About code owners (localização/ordem, base branch, "Require review from Code Owners", aprovação de qualquer owner, write access) — https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- [S13] GitHub Docs — About protected branches — https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- [S14] mozilla/cargo-vet issue #705 — `cargo vet suggest --locked` panics; **aberta** em 2026-11-16 — https://github.com/mozilla/cargo-vet/issues/705
- [S15] mozilla/cargo-vet PR #706 — Don't suggest registry imports when `--locked`; **aberta, `mergeable_state: dirty`** — https://github.com/mozilla/cargo-vet/pull/706
- [S16] Release latest `v0.10.3` (2026-10-22) — https://api.github.com/repos/mozilla/cargo-vet/releases/latest ; https://github.com/mozilla/cargo-vet

**Ecossistema**
- [S17] Microsoft RustTraining — Dependency Management and Supply Chain Security (trio mozilla/google/bytecode-alliance) — https://microsoft.github.io/RustTraining/engineering-book/ch06-dependency-management-and-supply-chain-s.html
- [S18] Trail of Bits Testing Handbook — Rust Supply Chain Analysis — https://appsec.guide/docs/languages/rust/supply-chain-analysis/
- [S19] Doc 027 (experimentos em cópia descartável, argument order, contagens, fallback do panic) — `docs/research/2026-11-16-027-cargo-vet-gate-feasibility.md`

**Verificações locais desta auditoria**
- `[V1]`–`[V12]`: `git status/branch/log/ls-files/check-ignore/remote`, `ls -la supply-chain .github .github/workflows`, leitura de `.gitignore`, `supply-chain/{config,audits,imports}`, `.github/workflows/{rust,ci,docker-staging}.yml`, `grep/awk` sobre config e `Cargo.lock`, `cargo vet --version`. Nenhum comando com efeito de escrita.

---

## 8. Resumo executivo

- O gate é viável e barato; o bloqueio real é procedural, não técnico.
- **GO para a Fatia A**, em um PR atômico: `.gitignore` + 3 arquivos do store + `multer` exemption + (se rede permitir) 3 imports + job informativo.
- **GO condicionado para a Fatia B**, em PR separado após 1 semana: remover `continue-on-error`, adicionar CODEOWNERS, ativar proteção de branch.
- `multer 3.1.0` é a única lacuna verificada; 442 exemptions cobrem o resto do lock atual.
- `suggest --locked` panica (issue #705 aberta), mas `check --locked` não — o gate pode ser bloqueante antes da correção upstream.
- Se a rede falhar para imports, **pular imports**, não o gate.
- Se o job provar instabilidade, **voltar a informativo**, não apagar o store.
