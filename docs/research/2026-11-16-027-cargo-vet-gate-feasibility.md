# 027 — cargo-vet como gate de dependência nova: viabilidade para o Flora (2026-11-16)

**Data:** 2026-11-16  
**Escopo:** pesquisa somente leitura + cross-check somente leitura no repositório. **Nenhum arquivo de configuração, workflow, `.gitignore` ou `supply-chain/` do Flora foi alterado.** Experimentos foram executados em cópia descartável (`/home/luis/.cache/flora-vet-sandbox`), sincronizada com o estado atual do repo antes de cada teste.  
**Pergunta:** vale a pena transformar o `cargo-vet` de "programa de auditoria em massa" em "gate barato de dependência nova", dado que o store hoje tem 442 exemptions, `audits.toml`/`imports.lock` vazios, está fora do Git, e a ferramenta já panica em alguns fluxos?  
**Método:** fontes primárias (cargo-vet book, CLI `--help` do binário instalado, GitHub Docs, issues/PRs upstream no mozilla/cargo-vet) + literatura de ecossistema (Microsoft RustTraining, Trail of Bits). Complementa `2026-11-16-026-cargo-vet-exemptions.md` e `2026-11-16-027-advisory-sbom-alternatives.md`.  
**Idiomas consultados:** EN (canônico, citado), pt-BR (doc 026), es/de/fr/zh-CN/ja via páginas de documentação e código; nenhuma tradução foi usada como fonte de afirmação operacional.

---

## 0. Veredito: **GO condicionado** (gate sim; programa de auditoria, não)

| Item | Veredito | Por quê |
|---|---|---|
| Tornar `cargo-vet` **bloqueante** para código novo | **GO, condicionado** aos 5 passos da §8 | É literalmente o caso de uso descrito no book: `init` exemptiona o passado, CI recusa o 443º crate [S1][S2][S3]. Custo medido: ~1–2 h. |
| Rodar `check` com `--locked --no-minimize-exemptions` | **GO** | Determinístico e somente-leitura em CI; reproduzido localmente (§5.1). |
| Manter `--no-all-features` no gate | **GO hoje, mas DEVERIA ser removido** | É no-op no Flora atual (452 nós com e sem o flag), porém enfraquece a cobertura futura se surgirem deps opcionais (§4.2). |
| Fazer 3 imports | **GO** | URLs oficiais respondem 200; é o multiplicador documentado [S4][S5][S17]. |
| Zerar as 442 exemptions | **NO-GO** | Backlog estimado em 8,7M linhas; não é trabalho de equipe pequena [S10][S19]. |
| Usar `suggest --locked` | **NO-GO até upstream corrigir** | Panic confirmado upstream (issue #705, aberta) e reproduzido aqui (§5.3). **Não afeta o gate**: `check --locked` não panica [S14]. |
| Usar `regenerate exemptions` em PR | **NO-GO** | O book chama a solução "mais agressiva" e avisa que pode enfraquecer `safe-to-deploy` → `safe-to-run` [S7]. |
| CODEOWNERS para `/supply-chain/` | **GO (barato)** | Recomendado pelo próprio book [S9] e suportado nativamente pelo GitHub [S12][S13]. |

**Frase-síntese:** o valor não está nas 442 — está em impedir a **443ª** entrada sem revisão. O bloqueio real hoje não é a ferramenta; é o store estar ignorado pelo Git e o `multer 3.1.0` estar fora do config. O panic do `suggest --locked` é um problema de *backlog*, não de *gate*.

---

## 1. O que o gate compra (e o que não compra)

O mecanismo do cargo-vet é intencionalmente "linterna de baixo atrito": `cargo vet init` adiciona exemptions para todas as dependências existentes e deixa o projeto verde em ~5 minutos, **sem auditar nada** [S1][S2]. Depois disso, qualquer dependência nova (crate ou versão nova) entra vermelha no CI e exige uma decisão explícita [S1][S10].

Três propriedades tornam isso barato:

1. **Deltas funcionam contra uma base exemptada** — auditoria relativa a uma versão não auditada ainda protege contra ataques futuros [S1, footnote 1].
2. **Imports** são a "forma mais rápida de encolher a lista de exemptions" e não são transitivos (confiança direta) [S4].
3. **Exemptions com `notes` e `suggest = false`** permitem rotular dívida conhecida sem poluir o backlog [S6].

O que o gate **não** entrega: não é scanner de CVE, não verifica licença, não é SBOM e não verifica o auditor ("there is no real way to check their work" [S9]). É complemento de `cargo deny`/`cargo audit`, não substituto [S17][S18]. Essa divisão já está tratada em `2026-11-16-027-advisory-sbom-alternatives.md`; aqui só tratamos do gate.

> **Correção ao doc irmão:** o doc de alternativas recomenda "não tornar cargo-vet bloqueante até o panic do `suggest --locked` ser resolvido". O upstream issue #705 diz explicitamente o contrário para o `check`: *"`cargo vet --locked`, because `check` skips suggestions under `--locked`"* [S14]. O panic é isolado em `suggest`; o gate pode ser bloqueante antes da correção upstream.

---

## 2. Estado do Flora (cross-check read-only, 2026-11-16)

| Item | Estado verificado | Evidência |
|---|---|---|
| `cargo-vet` instalado | **0.10.3** (release 2026-10-22) | `cargo-vet --version` local; [S16] |
| `supply-chain/config.toml` | **442 exemptions**, `[cargo-vet] version = "0.10"` | `grep -c '^\[\[exemptions\.'` = 442 |
| Divisão de critérios | **430 `safe-to-deploy` + 12 `safe-to-run`** | `grep '^criteria = ' \| sort \| uniq -c` |
| 12 `safe-to-run` | `bit-set`, `bit-vec`, `linux-raw-sys`, `proptest`, `quick-error`, `rand_xorshift`, `rustix`, `rusty-fork`, `tempfile`, `tokio-test`, `unarray`, `wait-timeout` | `awk` sobre o config |
| `audits.toml` | Vazio — só `# cargo-vet audits file` + `[audits]` | `cat` |
| `imports.lock` | Vazio — só `# cargo-vet imports lock` | `cat` |
| Versionamento | **`supply-chain/` ignorado** por `.gitignore:163` (`supply-chain/`, sob "Local tooling / environment artifacts") | `git ls-files supply-chain` → vazio; `git check-ignore -v` → `.gitignore:163` |
| CODEOWNERS | **Ausente** | `find .github -iname '*codeowners*'` → vazio |
| `Cargo.lock` | **452 pacotes**; `multer 3.1.0` presente | `grep -c '^name = '`; `grep -A3` |
| `multer` no config | **Ausente** → `check` falha | `grep -n multer supply-chain/config.toml` → vazio |
| Deps opcionais nos manifests do workspace | **Nenhuma** (`optional = true` não aparece) | `grep -rn` nos `Cargo.toml` |
| Grafo com/sem `--all-features` | **452 nós nos dois casos** (`cargo metadata --locked`) | medição local |
| CI `rust.yml` | Já roda `fmt --check`, `clippy --lib --bins -D warnings`, **`cargo deny check bans licenses sources`**, build, test + 6 suítes DB-gated. **Sem cargo-vet, sem cargo-audit** | `cat .github/workflows/rust.yml` |
| `ci.yml` | Frontend Bun | `cat .github/workflows/ci.yml` |

**Mudança desde o doc 026:** o `rust.yml` **já executa `cargo deny check bans licenses sources`** (o doc 026 afirmava o contrário). O P0 daquele documento está parcialmente satisfeito; falta o advisory fresco (`cargo audit`/OSV), tratado no doc irmão.

**Leitura da divisão 430/12:** ela está *correta por design*. O padrão do cargo-vet é `safe-to-deploy` para dependências normais/de build e `safe-to-run` para dev-dependencies [S6]. Os 12 `safe-to-run` são majoritariamente crates de teste/property-testing (`proptest`, `rusty-fork`, `tokio-test`, `tempfile`, `bit-set`, …). Não há erro a corrigir aqui — há dívida a rotular.

---

## 3. Análise das flags do gate

Comando proposto no escopo: `cargo vet --locked --no-all-features --no-minimize-exemptions`.

| Flag | Significado literal (CLI 0.10.3 / book) | Efeito no Flora | Recomendação |
|---|---|---|---|
| `--locked` | "Do not fetch new imported audits" [S7]; o book de imports confirma que ele pula o fetch [S4] | Torna o gate determinístico: usa `imports.lock` commitado, sem rede para imports | **Manter.** Se quiser zero rede absoluto, use `--frozen` (exige `--locked`) [S7] |
| `--no-all-features` | "Don't use --all-features. We default to passing --all-features to `cargo metadata` because we want to analyze your full dependency tree" [S7] | **No-op hoje**: 452 nós em ambos os modos; nenhum `optional = true` nos manifests | **Remover.** Mantém o grafo maximal do tool e evita que uma futura dep opcional entre sem cobertura. Se for mantido, documentar o motivo e reavaliar ao adicionar features opcionais (NV2) |
| `--no-minimize-exemptions` | "Prevent commands such as `check` and `certify` from automatically cleaning up unused exemptions" [S7] | Impede que `check` reescreva `config.toml` no runner (worktree sujo) e impede limpeza silenciosa da baseline | **Manter — é a flag mais importante do trio.** A poda deve ser um commit revisado (`prune`/`regenerate exemptions`), não efeito colateral de CI |

**Ordem dos argumentos importa:** `cargo vet suggest --locked` funciona (e panica, §5.3), mas `cargo vet --locked suggest` é rejeitado pelo parser: `error: the subcommand 'suggest' cannot be used with '--locked'`. Para o `check` (sem subcomando), `cargo vet --locked ...` funciona normalmente [S19].

**Não confundir:** o `--locked` do cargo-vet não é o `--locked` do Cargo. Ele não "exige Cargo.lock atualizado"; ele "não busca imports novos" [S4][S7]. A atualização do `Cargo.lock` é justamente o que dispara o gate.

---

## 4. Bootstrapping com 3 imports

O book recomenda imports como primeiro multiplicador e o registry oficial é a fonte das URLs [S4][S5]. O Microsoft RustTraining dá literalmente o trio `mozilla`, `google`, `bytecode-alliance` como exemplo de import [S17]; o Trail of Bits recomenda `bytecodealliance/wasmtime` e `google/rust-crate-audits` [S18].

URLs do `registry.toml` consultado em 2026-11-16 ([S5]) e status HTTP medido ([S19]):

| Nickname | URL | HTTP |
|---|---|---|
| `mozilla` | `https://raw.githubusercontent.com/mozilla/supply-chain/main/audits.toml` | 200 |
| `google` | `https://raw.githubusercontent.com/google/supply-chain/main/audits.toml` | 200 |
| `bytecode-alliance` | `https://raw.githubusercontent.com/bytecodealliance/wasmtime/main/supply-chain/audits.toml` | 200 |
| `zcash` | `https://raw.githubusercontent.com/zcash/rust-ecosystem/main/supply-chain/audits.toml` | **200** |

> **Correção ao doc 026 (NV4 resolvido):** o 404 reportado lá era da URL antiga `…/rust-ecosystem/main/audits.toml`. O `registry.toml` atual inclui o segmento `supply-chain/` e responde 200. Ainda assim, **não importar zcash agora**: 3 conjuntos já dão ~90% do benefício observado e o quarto não justifica confiança adicional sem necessidade (§5.4).

Comandos (executar uma vez, com rede, **não** com `--locked`):

```bash
cargo vet import mozilla
cargo vet import google
cargo vet import bytecode-alliance
cargo vet fmt          # opcional; normaliza os TOMLs editados à mão
```

Isso popula `[imports.*]` no `config.toml` e o `imports.lock`. Ambos devem ser commitados juntos. A partir daí o CI roda com `--locked` e não faz fetch [S4]. Imports **não são transitivos**: confiar na Mozilla não significa confiar nas escolhas da Google [S4][S3].

---

## 5. Experimentos reproduzidos (cópia descartável, 2026-11-16)

Binário: `cargo-vet 0.10.3`; `CARGO_NET_OFFLINE=true`; `Cargo.toml`, `Cargo.lock` e `supply-chain/*` sincronizados do repo imediatamente antes dos testes. Nenhum arquivo do repo foi tocado.

### 5.1 O gate exato falha por uma crate

```
$ cargo vet --locked --no-all-features --no-minimize-exemptions
Vetting Failed!

1 unvetted dependencies:
  multer:3.1.0 missing ["safe-to-deploy"]
```

O `multer` não existe no `config.toml`, embora esteja no `Cargo.lock` (`3.1.0`). As 442 exemptions **não cobrem o lock atual** — o store está desatualizado, não apenas invisível.

### 5.2 Uma exemption com `notes` fecha o gate

```
$ cargo vet add-exemption multer 3.1.0 --criteria safe-to-deploy --notes "<justificativa>"
$ cargo vet --locked --no-all-features --no-minimize-exemptions
Vetting Succeeded (443 exempted)
```

Diff no config da cópia: apenas o bloco novo de `[[exemptions.multer]]`. Custo: **uma linha de decisão**, não uma auditoria. Alternativa preferível se houver tempo: `cargo vet inspect multer 3.1.0` + `certify` (o book sugere que examine o parser de multipart do `axum-extra`, fronteira de input não confiável) [S10].

### 5.3 O panic do `suggest --locked` é real — e é isolado

```
$ cargo vet suggest --locked
ERROR   × cargo vet panicked
  ╰─▶ called `Option::unwrap()` on a `None` value
  help: at src/storage.rs:837:40
```

Causa documentada upstream: sob `--locked`, `go_online` retorna cedo e deixa `live_imports = None`, mas `compute_suggest` ainda chama `fetch_registry_audits`, que faz unwrap [S14]. **Issue #705 está aberta desde 2026-10-22**; PR #706 ("Don't suggest registry imports when `--locked`") também está aberta e com `mergeable_state: dirty` [S15]. A versão 0.10.3 foi releases no mesmo dia e **não** contém a correção — reproduzido aqui.

Workarounds confirmados na mesma cópia (todos produzem o mesmo backlog de 8.705.985 linhas):

```bash
cargo vet suggest                                      # sem --locked, com rede
cargo vet suggest --locked --no-registry-suggestions   # recomendado offline
cargo vet suggest --locked --frozen                    # zero rede
```

O issue confirma que `cargo vet --locked` (check) **não** panica, porque o check pula sugestões sob `--locked` [S14]. Logo: **o panic não bloqueia o gate; bloqueia apenas a ferramenta de medição de backlog.**

### 5.4 Números de backlog (diagnóstico, não meta)

| Config | Backlog estimado | `inspect` | `diff` |
|---|---:|---:|---:|
| baseline (audits/imports vazios) | 8.705.985 linhas | 443 | 0 |
| 3 imports (mozilla, google, bytecode-alliance) | 7.023.794 (−19,3%) | 258 | 113 |
| 8 imports | 6.837.139 (−21,5%) | 229 | 125 |

Os maiores itens são `linux-raw-sys` (~480k linhas), `rustix` (~74k) e `proptest` (~26k) — todos da cota `safe-to-run`. Isso quantifica por que a meta "auditar as 442 de uma vez" não é realista. Com 3 imports o tool passa a oferecer 113 deltas pequenos em vez de só auditorias completas [S10][S19].

### 5.5 Estabilidade geral da ferramenta

Release 0.10.3 em 2026-10-22, após ~2 anos desde 0.10.0 (2024-10-03); MSRV subiu para 1.85; 116 issues abertas no repo [S16]. É mantida, mas o ritmo é lento e o panic acima permanece. Isso reforça: pinar versão, não depender de `suggest` no caminho crítico, e manter `cargo deny` como gate determinístico primário.

---

## 6. Como tratar as 430 + 12 exemptions

Regras para o gate não virar teatro:

1. **Congelar a baseline:** commitar as 442 como estão, sem `regenerate exemptions`. O book descreve `regenerate` como "a solução mais agressiva" e adverte que pode enfraquecer critérios [S7].
2. **Rotular, não auditar em massa:** adicionar `notes` explicando a origem da baseline (ex.: `"Bootstrap 027: pré-existente em 2026-11-16; não auditada; pendente de revisão priorizada."`). `notes` é campo suportado em exemptions [S6].
3. **`suggest = false` só com critério:** marcar apenas crates que o Flora nunca pretende auditar (ex.: crates de plataforma que não entrega), sempre com `notes`. O campo existe exatamente para silenciar sugestões que não serão tratadas [S6][S7].
4. **Nunca adicionar exemption por reflexo em PR.** Toda crate nova deve entrar por auditoria ou import; se entrar por exemption, é decisão explícita de mantenedor com `notes` e data — não `regenerate exemptions`.
5. **Poda trimestral:** `cargo vet suggest --locked --no-registry-suggestions` → `cargo vet prune` → commit revisado. Meta: o número **diminuir**. O book alerta que exemptions persistentes podem deixar o projeto vulnerável mesmo com `check` verde [S10].
6. **Priorizar se sobrar tempo** (20% crítico, sempre delta antes de full [S1][S10]): auth/crypto (`jsonwebtoken`, `rsa`, `sha2`, `argon2`, `hmac`), fronteira rede/DB (`tower-http`, `axum`, `reqwest`, `sqlx`, `multer`), TLS (`rustls*`).

---

## 7. CODEOWNERS e governança

O book recomenda CODEOWNERS para o `supply-chain/` quando **mais de cinco pessoas** podem aprovar mudanças, e observa que para times pequenos os controles normais do repositório bastam [S9]. No Flora (equipe pequena) é barato o suficiente para adotar desde o início — e é o que impede que o gate seja burlado por um merge descuidado.

Arquivo `.github/CODEOWNERS` (GitHub procura `.github/`, raiz, `docs/` — nessa ordem [S12]):

```text
# Gate de supply-chain: mudanças no store exigem aprovador designado.
/supply-chain/     @MDMV-Laboratorio-de-Solucoes-Digitais/<time-ou-usuario>
/.github/CODEOWNERS  @MDMV-Laboratorio-de-Solucoes-Digitais/<owner>
```

Notas operacionais:

- O CODEOWNERS precisa estar na **branch base** do PR para gerar review request [S12].
- CODEOWNERS sozinho apenas *solicita* review. Para **bloquear**, ativar "Require review from Code Owners" na proteção de branch [S12][S13]. A aprovação de **qualquer** owner satisfaz o requisito [S12].
- Usuários/times precisam ter **write access** explícito [S12].
- Proteger o próprio CODEOWNERS (segunda linha acima) evita que o dono seja removido por um PR qualquer [S12].
- Substituir `<time-ou-usuario>` pelos handles reais — verificar existência e permissão antes de commitar (NV5).

---

## 8. Plano de execução (go/no-go em passos exatos)

Ordem importa: primeiro tornar o gate possível, depois torná-lo visível, só depois bloqueante.

**Passo 1 — tornar o store versionado (bloqueante para todo o resto)**
```bash
# editar .gitignore: remover a linha "supply-chain/" (linha 163)
git add .gitignore supply-chain/config.toml supply-chain/audits.toml supply-chain/imports.lock
git commit -m "build(flora): versiona store do cargo-vet para gate de dependencias"
```
Sem isto, não há gate, não há histórico e não há CODEOWNERS útil. Se este passo for recusado, **cortar o cargo-vet inteiro** e ficar apenas com `cargo deny` + advisory (doc irmão).

**Passo 2 — fechar o desatualizado (`multer`)**
```bash
cargo vet add-exemption multer 3.1.0 --criteria safe-to-deploy \
  --notes "Bootstrap 027: parser multipart do axum-extra; revisao de fronteira pendente. 2026-11-16."
cargo vet --locked --no-minimize-exemptions      # deve dizer "Vetting Succeeded (443 exempted)"
```
(Ou auditar de fato com `inspect`/`certify`, se houver tempo.)

**Passo 3 — bootstrap com 3 imports (com rede, sem `--locked`)**
```bash
cargo vet import mozilla
cargo vet import google
cargo vet import bytecode-alliance
cargo vet --locked --no-minimize-exemptions
git add supply-chain/config.toml supply-chain/imports.lock
git commit -m "build(flora): importa 3 audit sets no cargo-vet"
```

**Passo 4 — CI informativo (1ª semana)**
Adicionar job em `.github/workflows/rust.yml`. Receita canônica do book usa `cargo install` + tool cache [S8]; alternativa consistente com o que o repo já faz para `cargo-deny` é `taiki-e/install-action@cargo-vet` (verificar input da action — NV6):

```yaml
  cargo-vet:
    name: Vet new dependencies
    runs-on: ubuntu-latest
    continue-on-error: true   # remover após a 1ª semana
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@cargo-vet
      - name: Vet (gate de codigo novo)
        run: cargo vet --locked --no-minimize-exemptions
```
Pinar a versão do tool (book usa `CARGO_VET_VERSION`) para o CI ser reproduzível [S8][S19].

**Passo 5 — tornar bloqueante + governança**
- Remover `continue-on-error: true`.
- Adicionar `.github/CODEOWNERS` (§7) e ativar "Require review from Code Owners" na branch protection [S12][S13].
- Regra de time: `regenerate exemptions` nunca é resolução padrão de PR; toda exemption nova com `notes` + data.
- Agendar revisão trimestral do backlog com `suggest --locked --no-registry-suggestions` (workaround do panic).

**Critério de rollback (se a ferramenta provar instabilidade):** se `check --locked` falhar de forma não reproduzível localmente por 2 PRs seguidos, voltar o job para `continue-on-error: true` e manter `cargo deny` como gate único; investigar com `--verbose info` e `cargo vet explain-audit <crate>` [S7]. **Não** resolver com `regenerate exemptions`.

---

## 9. Alternativas e fallback

| Cenário | Ação | Custo |
|---|---|---|
| `suggest --locked` panica (hoje) | Usar `--no-registry-suggestions` ou `--frozen`; nunca pôr `suggest` no crítico do gate | 0 |
| `check` instável no runner | Pinar 0.10.3, rodar `--verbose info`, `--frozen` para isolar rede; depois `continue-on-error` temporário | baixo |
| Tool abandonado / bloqueio recorrente | Manter apenas `cargo deny check bans licenses sources` (já em CI) + advisory agendado (doc irmão); remover job de vet | baixo |
| Pressão para "zerar as 442" | Recusar; apresentar as 8,7M linhas e oferecer 3 imports + deltas como caminho | 0 |
| Necessidade de maior rigor depois | `cargo-crev` (web-of-trust, assinatura) — **DEFER**; exige identidade e config manual, sem delta-trust [S18] | alto |

O fallback não é "nenhuma política": `cargo deny` já roda em CI (§2) e o doc irmão cobre advisory fresco + SBOM. Mesmo sem cargo-vet, o Flora não fica sem gate de licença/ban/source — fica sem a pergunta "houve revisão humana?".

---

## 10. NVs (não validado nesta pesquisa)

- **NV1** — Gate em CI limpo com store versionado não foi executado de ponta a ponta; validar após passos 1–3 (equivalente ao NV1 do doc 026).
- **NV2** — `--no-all-features` é no-op hoje (452 nós, zero `optional = true`), mas o efeito de uma futura dep opcional não foi medido; remover o flag é a opção conservadora.
- **NV3** — Números de backlog (8,7M → 7,0M → 6,8M) vêm de `suggest` com registry em 2026-11-16 e dependem do estado externo; são diagnóstico, não meta. Reexecutar após imports reais.
- **NV4** — URL de `zcash` respondia 404 no doc 026; hoje responde 200 com o novo caminho. Não se sabe se o conjunto é mantido ativamente; não importar sem necessidade.
- **NV5** — Handles/times reais para o CODEOWNERS e suas permissões de write não foram verificados; preencher com dados válidos antes do commit.
- **NV6** — `taiki-e/install-action@cargo-vet` (nome do input/versão) não foi validado; a receita do book com `cargo install` + tool cache é a garantida [S8].
- **NV7** — Impacto de `suggest = false` em massa não foi medido; marcar errado pode esconder backlog real. Aplicar só com critério documentado.
- **NV8** — A auditoria real do `multer` (linhas exatas, surface de multipart) não foi feita; a estimativa do `suggest` não foi confirmada nesta rodada.
- **NV9** — PR #706 pode ser mergeado a qualquer momento, mudando o comportamento de `suggest --locked`; revalidar o workaround antes de documentá-lo como permanente.
- **NV10** — Custo de CI (install + cache do binário) não foi medido no runner do Flora; a receita do book existe exatamente para minimizá-lo [S8].

---

## Fontes primárias e de ecossistema (acesso 2026-11-16)

**cargo-vet book (Mozilla)**
- [S1] Introduction / How it Works (setup ~5 min, bloqueio de código novo, delta contra exemption, CODEOWNERS no fluxo): https://mozilla.github.io/cargo-vet/ e https://mozilla.github.io/cargo-vet/how-it-works.html
- [S2] Setup (`Vetting Succeeded (X exempted)`, "arquivos devem ser adicionados ao versionamento"): https://mozilla.github.io/cargo-vet/setup.html
- [S3] FAQ (por que `init` exemptiona tudo; sem web-of-trust): https://mozilla.github.io/cargo-vet/faq.html
- [S4] Importing Audits (forma mais rápida de encolher; `--locked` pula fetch; não transitivo): https://mozilla.github.io/cargo-vet/importing-audits.html
- [S5] `registry.toml` oficial: https://raw.githubusercontent.com/mozilla/cargo-vet/main/registry.toml
- [S6] Configuration (`notes`, `suggest`, `policy`, `imports`, `[cargo-vet]`): https://mozilla.github.io/cargo-vet/config.html
- [S7] Commands (`--locked`, `--no-all-features`, `--no-minimize-exemptions`, `--frozen`, `regenerate exemptions`, `prune`, `explain-audit`): https://mozilla.github.io/cargo-vet/commands.html
- [S8] Configuring CI (receita `cargo vet --locked` + tool cache): https://mozilla.github.io/cargo-vet/configuring-ci.html
- [S9] Curating Your Audit Set (CODEOWNERS, >5 aprovadores, "no real way to check their work"): https://mozilla.github.io/cargo-vet/curating-your-audit-set.html
- [S10] Performing Audits (`suggest`, backlog, deltas, "exemptions frequently undermine the value"): https://mozilla.github.io/cargo-vet/performing-audits.html
- [S11] Built-In Criteria (`safe-to-run`, `safe-to-deploy`, `implies safe-to-run`): https://mozilla.github.io/cargo-vet/built-in-criteria.html

**GitHub / upstream**
- [S12] GitHub Docs — About code owners (localização, sintaxe, branch base, "Require review from Code Owners", aprovação de qualquer owner): https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- [S13] GitHub Docs — About protected branches: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- [S14] mozilla/cargo-vet issue #705 — `cargo vet suggest --locked` panics (aberta; causa em `storage.rs`; `check --locked` não panica): https://github.com/mozilla/cargo-vet/issues/705
- [S15] mozilla/cargo-vet PR #706 — Don't suggest registry imports when `--locked` (aberta, conflito): https://github.com/mozilla/cargo-vet/pull/706
- [S16] mozilla/cargo-vet releases e metadados do repo (0.10.3 em 2026-10-22; 0.10.0 em 2024-10-03; 116 issues abertas): https://api.github.com/repos/mozilla/cargo-vet/releases ; https://github.com/mozilla/cargo-vet

**Ecossistema**
- [S17] Microsoft RustTraining — Dependency Management and Supply Chain Security (trio de imports; cargo-vet para requisitos rigorosos, cargo-deny suficiente para a maioria): https://microsoft.github.io/RustTraining/engineering-book/ch06-dependency-management-and-supply-chain-s.html
- [S18] Trail of Bits Testing Handbook — Rust Supply Chain Analysis (vetting é "understanding" mais que "checking"; imports primeiro; cargo-crev exige mais maquinaria): https://appsec.guide/docs/languages/rust/supply-chain-analysis/
- [S19] Cross-check local somente leitura + experimentos em cópia descartável: `supply-chain/*`, `.gitignore`, `Cargo.toml`, `Cargo.lock`, `.github/workflows/{ci,rust}.yml`; `cargo-vet 0.10.3 --help`; `cargo metadata --locked [--all-features]`; `cargo vet --locked --no-all-features --no-minimize-exemptions`; `cargo vet suggest --locked [--frozen|--no-registry-suggestions]`; `cargo vet add-exemption multer 3.1.0`; `curl` nos URLs do registry.
- [S20] Documentos irmãos: `docs/research/2026-11-16-026-cargo-vet-exemptions.md` e `docs/research/2026-11-16-027-advisory-sbom-alternatives.md`.
