# cargo-vet com 442 exemptions: o que garantir, o que cortar (2026-11-16)

**Objetivo:** decidir, para o Flora (Rust workspace, AGPL-3.0-or-later, self-hosted em VPS barata, equipe pequena), se o `supply-chain/` atual com 442 exemptions e `audits.toml`/`imports.lock` vazios deve ser mantido, cortado ou adiado — e qual política mínima credível de supply-chain substitui ou complementa a ferramenta.

**Método:** fontes primárias (cargo-vet book, cargo-deny docs, CycloneDX, NTIA) + literatura secundária confiável (Microsoft RustTraining, Trail of Bits Testing Handbook) + **cross-check somente leitura** no repositório. Nenhum arquivo de configuração foi alterado. Experimentos com `cargo vet 0.10.3` foram feitos em cópias descartáveis em `/home/luis/.cache/flora-vet-*`; o repositório permanece intocado além deste documento.

**Escopo:** dados de 2026-11-16. Não é parecer jurídico; complementa `docs/research/2026-11-16-seed-license-dependency-audit.md` e responde ao NV "cargo-vet 442 exemptions" de `.specify/releases/025-seed-fase0-verification/release.md` e `.specify/validations/025-seed-fase0-verification/validate.md`.

---

## 1. Resumo executivo (80/20)

| Decisão | Item | Por quê |
|---|---|---|
| **KEEP (condicionado)** | `cargo-vet` como **gate de código novo**, não como programa de auditoria | É exatamente o caso de uso para o qual a ferramenta foi desenhada: `init` exemptiona o passado e passa a bloquear código novo [1][2][5]. Custo de instalação do gate é baixo se o store for versionado. |
| **KEEP (condicionado)** | As 442 exemptions como **backlog visitável**, não como prova de segurança | O próprio manual diz que `exemptions` são "backlog" a reduzir com `cargo vet suggest` [9][6]. Condição: commits versionados + `notes` + `CODEOWNERS` + proibição de `regenerate exemptions` em PR. |
| **CUT** | A meta de auditar as 442 crates "de uma vez" | `cargo vet suggest` estima **8.705.985 linhas** de backlog no estado atual (§3.2). Inviável para equipe pequena. |
| **CUT** | A afirmação implícita de que "442 `safe-to-deploy` = 442 crates auditadas" | No resolvedor, exemption é literalmente desaçucarada como uma auditoria completa; muda a origem para priorização/caveats, não a força do claim [14]. |
| **CUT** | `cargo-vet` como substituto de advisory/licença/SBOM | A ferramenta responde "houve revisão humana?", não "há CVE conhecido?" ou "licença é compatível com AGPL?" [16][17]. |
| **DEFER** | Auditorias manuais em massa, critérios customizados (`crypto-reviewed`), publisher-trust em escala, wildcard audits, import set completo (9 orgs) | Benefício marginal pequeno neste lock (§3.2: 3 imports capturam ~19% de redução; 8 imports, ~21%). |
| **P0 (fazer antes do gate)** | Ligar `cargo deny check` em CI + check diário de advisories | `deny.toml` já existe (cycle 025) e não é executado em CI. É o maior ganho por hora de trabalho [16][18]. |
| **P2** | SBOM CycloneDX em tag/release | Inventário para distribuição AGPL e cadeia de suprimentos; não substitui auditoria [19][20]. |

**Frase-síntese:** para o Flora, o valor do cargo-vet não está nas 442 exemptions — está em impedir que a **443ª** crate entre sem revisão. O que hoje anula esse valor é o store estar **ignorado pelo Git**, desatualizado e sem CI.

---

## 2. O que o cargo-vet realmente garante

### 2.1 Mecanismo

O cargo-vet compara cada dependência de terceiros do grafo de build com auditorias feitas pelo projeto ou por entidades confiáveis. Se houver lacuna, a ferramenta falha e oferece auxílio mecânico para auditar e documentar [1]. O grafo analisado é o **máximo** (`cargo metadata --locked --all-features`): qualquer build real é subconjunto dele, então cobrir o máximo cobre todas as features/plataformas [14].

O resolvedor constrói, por crate, um grafo de versões; nós são versões, arestas são auditorias (progressivas ou delta). A pergunta "esta versão está auditada?" vira "existe caminho da versão-raiz até a versão-alvo certificando o critério?" [14].

### 2.2 Critérios (o que o auditor assina)

| Critério | Texto oficial (resumo) |
|---|---|
| `safe-to-run` | Pode ser compilado/executado/testado em workstation ou automação controlada sem consequências surpreendentes: ler/escrever arquivos sensíveis, instalar software, conectar a endpoints não confiáveis, mineração etc. [4] |
| `safe-to-deploy` | **Não introduzirá vulnerabilidade de segurança séria em software production exposto a input não confiável.** Não exige revisão lógica completa; exige raciocínio sobre todos os blocos `unsafe` e importações poderosas. Implica `safe-to-run` [4]. |

Padrão do cargo-vet: dependências normais e de build exigem `safe-to-deploy`; dependências de dev, `safe-to-run` [10]. É possível relaxar via `[policy.<crate>]` / `dependency-criteria` para subárvores não expostas [10][13].

### 2.3 O que uma `exemption` é de fato

- É um **mecanismo de bootstrap intencional**: `cargo vet init` exemptiona todas as dependências existentes para que o time chegue a "verde" em minutos e ataque o backlog depois; a FAQ dedica uma pergunta específica a isso [2][5].
- O setup output é literalmente `Vetting Succeeded (X exempted)` — o manual avisa: "vetting only succeeds because your list of exceptions contains the exact set of current dependencies used in your project... seek to shrink the list over time" [3].
- No resolvedor, **exemptions são desaçucaradas como auditorias completas** (`Root -> version`), com uma origem especial apenas para deprioritizar a aresta e relatar *caveats*. Mecanicamente, ela satisfaz o mesmo critério que uma auditoria [14].
- Entradas podem ter `notes` (racional) e `suggest = false` (silencia o backlog para itens que não serão tratados) [13].
- `cargo vet certify` **remove automaticamente** a exemption correspondente quando uma auditoria a substitui; `regenerate exemptions` adiciona/remove exemptions para tornar `check` mínimo e pode até **enfraquecer** `safe-to-deploy` para `safe-to-run` [9].

### 2.4 Os três multiplicadores (onde está o 80/20)

1. **Imports** — "the fastest way to shrink the `exemptions` list": aponta para `audits.toml` de outra organização; não é transitivo (confiança direta e explícita); built-in criteria têm o mesmo significado entre projetos [7]. O registry oficial lista 9 conjuntos (Mozilla, Google, Bytecode Alliance, Embark, Zcash, ISRG, Actix, Ariel OS, Fermyon) [15].
2. **Auditorias relativas (delta)** — revisar a diferença entre versões; o manual afirma explicitamente que **deltas funcionam mesmo contra uma base exemptada** e ainda assim dão proteção contra ataques futuros [2]. `suggest` calcula o menor diff possível [6].
3. **Publisher trust / wildcard** — `cargo vet trust` declara confiança no publicador de uma crate, com expiração; é um **heurístico** ("the trusted publisher is not consulted") e exige avaliação do processo de release [8]. `cargo vet trust --all <user>` e `certify --wildcard` escalam isso [9].

### 2.5 O que o cargo-vet NÃO garante

- **Não é scanner de vulnerabilidades nem de malware.** É uma camada de política: "nenhuma crate entra sem registro de revisão humana". Complementa, não substitui, RustSec/`cargo audit` e `cargo deny` [16][17].
- **Não verifica o auditor.** "When someone submits an audit, there is no real way to check their work" — por isso o manual recomenda CODEOWNERS e um grupo pequeno de aprovadores quando >5 pessoas podem aprovar mudanças [12].
- **Não é web-of-trust.** Confiar em um import não implica confiar nas escolhas dele; cada relação é independente [5].
- **Não cobre código primeiro-partido por padrão** (usa `audit-as-crates-io` quando aplicável) [10][13].
- **Não é SBOM nem checagem de licença/AGPL §13** [16][19][20].
- **Exemptions persistentes deixam o projeto vulnerável**: "Even when your project is passing `cargo vet`, lingering entries in `exemptions` could still leave you vulnerable" e "doing so [exemptar com frequência] frequently undermines the value provided by the tool" [6].

Literatura secundária alinha: a Microsoft RustTraining classifica cargo-vet como ferramenta para organizações com requisitos rigorosos (governo, finanças, infraestrutura) e diz que `cargo-deny` é suficiente para a maioria dos times [16]; o Trail of Bits Testing Handbook enquadra ferramentas de vetting como mais úteis para **entender** a cadeia do que para "checar" bugs, e recomenda imports como primeiro passo [17].

---

## 3. Estado do Flora (cross-check read-only)

### 3.1 Inventário

| Item | Estado verificado | Evidência |
|---|---|---|
| `supply-chain/config.toml` | **442 exemptions**: 430 `safe-to-deploy` + 12 `safe-to-run` (a hipótese "442 safe-to-deploy" está errada por 12). Sem `notes`, sem `suggest = false`, sem `[imports]`, sem `[policy]`, sem critérios customizados | `grep -c '^\[\[exemptions\.'` = 442; `grep '^criteria' \| sort \| uniq -c` = 430/12 |
| 12 `safe-to-run` | `bit-set`, `bit-vec`, `linux-raw-sys`, `proptest`, `quick-error`, `rand_xorshift`, `rustix`, `rusty-fork`, `tempfile`, `tokio-test`, `unarray`, `wait-timeout` | `grep -B2` no config |
| `audits.toml` | Vazio (só cabeçalho `[audits]`) | 4 linhas |
| `imports.lock` | Vazio (só cabeçalho) | 2 linhas |
| **Versionamento** | **`supply-chain/` está no `.gitignore:163`, sob "Local tooling / environment artifacts"** | `git ls-files supply-chain` → vazio; `git check-ignore -v` → `.gitignore:163` |
| Consequência do ignore | Clean checkout não tem store: `cargo vet` falha com `You must run 'cargo vet init' (store not found at .../supply-chain)` | teste em cópia sem `supply-chain/` |
| `deny.toml` | Presente e versionado (cycle 025): allow-list SPDX, `confidence-threshold = 0.8`, `advisories version = 2`, `bans`, `sources`. Não é executado em CI | `cat deny.toml`; `git ls-files deny.toml` |
| CI | `rust.yml` = `cargo build`, `cargo test` + jobs DB-gated; `ci.yml` = frontend Bun. **Sem cargo-deny, cargo-audit, cargo-vet, fmt/clippy ou SBOM** | `.github/workflows/*.yml` |
| CODEOWNERS | Ausente | `find .github -iname '*codeowners*'` |
| `Cargo.lock` | 452 pacotes versionados; 442 exemptions ≈ 98% do grafo | `grep -c '^name = '` |
| Ciclo 025 | Release e validation listam explicitamente "cargo-vet exemptions" como NV | `.specify/releases/025-.../release.md`, `.specify/validations/025-.../validate.md` |

### 3.2 Experimentos locais (cargo-vet 0.10.3, 2026-11-16)

Comandos executados **somente em cópias descartáveis**; nenhum arquivo do repo foi modificado.

1. **O store está desatualizado.** `cargo vet --locked --no-minimize-exemptions` → exit 255:
   ```
   Vetting Failed!
   1 unvetted dependencies:
     multer:3.1.0 missing ["safe-to-deploy"]
   ```
   `multer` não aparece no config. Ou seja: as 442 exemptions não cobrem o lock atual.

2. **Backlog de auditoria é inviável.** `cargo vet suggest --no-minimize-exemptions` (com registry) → 443 comandos `cargo vet inspect`, 0 deltas sugeridos, 317 notas de import, 61 "would eliminate this", e:
   ```
   estimated audit backlog: 8705985 lines
   ```
   Exemplos do topo da lista: `tower-http 0.7.0` (32.537 linhas), `zerocopy-derive 0.8.56` (26.614), `rsa 0.9.10` (7.914). Como `audits.toml` está vazio, não há base para deltas — o suggest só oferece auditorias completas.

3. **Imports reduzem, mas não resolvem.** Simulações em sandbox:

   | Config | Backlog estimado | `inspect` | `diff` | notas `trust` |
   |---|---:|---:|---:|---:|
   | baseline (sem imports) | 8.705.985 | 443 | 0 | 0 |
   | 3 imports (mozilla, google, bytecode-alliance) | 7.023.794 (−19,3%) | 258 | 113 | 165 |
   | 8 imports (todos exceto zcash) | 6.837.139 (−21,5%) | 229 | 125 | 163 |

   Leitura: poucos imports já destravam centenas de **deltas pequenos** e sugestões de publisher-trust; importar mais 5 conjuntos custa apenas ~2 pontos percentuais. **3 é o ponto do 80/20.** Esses números são estimativas do tool para o registry em 2026-11-16 e não substituem uma execução real após o store ser populado.

4. **Registry parcialmente quebrado.** `cargo vet` falha ao buscar a URL de `[registry.zcash]` em `registry.toml` (`https://raw.githubusercontent.com/zcash/rust-ecosystem/main/audits.toml`) com **HTTP 404**; `curl` confirma 404 (Mozilla retorna 200). O registry oficial está desatualizado.

5. **Bug/aresta da ferramenta.** `cargo vet suggest --locked --no-minimize-exemptions` → exit 101, `cargo vet panicked ... Option::unwrap() on a None value, at src/storage.rs:837:40`. O mesmo comando sem `--locked` funciona. Isso é relevante porque `--locked` é exatamente o modo recomendado para CI [11] — veja NV5.

6. **Gate de CI impossível hoje.** Sem o store versionado, um runner de CI limpo não consegue nem iniciar o cargo-vet (item 1 da tabela anterior).

---

## 4. Blanket exemptions anulam o propósito?

**Não por si só — e sim se virarem estado permanente.**

- **Não anulam** quando são o mecanismo documentado de bootstrap: o manual incentiva explicitamente começar verde com o passado exemptionado e ratchetar com `suggest` [1][2][5]. O valor imediato é **bloquear código novo**: qualquer crate/versão nova entra vermelha no CI e exige decisão [2][11].
- **Anulam** quando:
  - viram álibi ("está verde, então está auditado") — o resolvedor as trata como auditorias [14], mas a única evidência é a própria afirmação do time;
  - novas exemptions são adicionadas por reflexo (`cargo vet regenerate exemptions` a cada PR) — o manual chama isso de "solução mais agressiva" e recomenda evitar [9];
  - nunca são podadas — `suggest` existe exatamente para medir e reduzir o backlog residual [6];
  - o store não é versionado/protegido, então ninguém vê nem revisa o que foi exemptionado (caso Flora hoje).

A distinção útil é: **exemption = dívida com juro zero, mas sem garantia**. Enquanto o saldo é conhecido, visível e decrescente, a ferramenta entrega valor. Quando o saldo é invisível (gitignore) ou crescente, ela é apenas um selo verde.

Para o Flora especificamente: o problema não são as 442; é (a) estarem fora do Git, (b) uma estar desatualizada (`multer`), (c) nenhuma auditoria/import real tê-las desafiado e (d) nenhum gate existir.

---

## 5. Alternativas e complementos

| Opção | O que entrega | Custo | Veredito para o Flora |
|---|---|---|---|
| **cargo-deny only** (advisories + licenses + bans + sources) | CVEs conhecidos via RustSec, allow-list SPDX, versões duplicadas/banidas, registries/git sources [16][18] | Baixo: `deny.toml` já existe; falta ligar no CI | **P0.** Maior cobertura por hora. Observação: `version = 2` torna advisories de vulnerabilidade erro por padrão; `ignore` aceita `reason`/`expiry` [18]. |
| **cargo-audit agendado** | Freshness: advisory DB é buscado em runtime; `audit-check@v2` diário [16] | Muito baixo | **P0/P1.** Complementa o deny (que o time pode rodar com DB fixado). Use `cargo audit --deny warnings`. |
| **SBOM CycloneDX** | Inventário de componentes (supplier/componente/versão/purl/deps/timestamp) anexado ao artefato de release [19][20] | Baixo: `cargo install cargo-cyclonedx`, job em tag | **P2.** Importante para distribuição/AGPL e para respondedor de incidente; **não** prova código revisado. Atenção: a ferramenta invoca Cargo e pode executar código de build [20]. |
| **cargo-vet gate (backlog + imports)** | Política "nova crate só entra com revisão"; deltas e imports barateiam o futuro [1][2][7] | Médio-baixo se o store for commitado; alto se o objetivo for zerar 442 | **P1, condicionado.** Ver §6. |
| **cargo-crev** | Reviews criptograficamente assinados, web-of-trust | Alto: identidade, configuração manual, sem delta-trust [17] | **DEFER.** Excesso de maquinaria para equipe pequena. |
| **cackle / cargo-unmaintained / cargo-geiger** | APIs sensíveis usadas por deps, crates sem manutenção, uso de `unsafe` | Baixo-médio | **DEFER.** Útil como investigação pontual, não como gate. |

**AGPL-3.0 §13:** nenhuma dessas ferramentas resolve a obrigação de oferecer Corresponding Source quando código modificado é exposto em rede. Isso continua em `docs/research/2026-11-16-seed-license-dependency-audit.md` (itens 1, 8) e não deve ser misturado com política de supply-chain.

---

## 6. Política recomendada para equipe pequena (Flora)

### P0 — antes de qualquer discussão de auditoria

1. **Ligar `cargo deny check` em `rust.yml`** com o `deny.toml` já versionado (advisories + licenses + bans + sources). Falha de licença/advisory bloqueia merge. Validar localmente antes (`cargo deny check` esteve NV no ciclo 025 por 403 ao índice crates.io — NV7).
2. **Adicionar job diário de `cargo audit --deny warnings`** (ou `rustsec/audit-check@v2`) para advisories frescos; manter o `cargo deny` como política determinística [16].
3. **Não prometer "supply-chain segura" com base no `supply-chain/` atual.**

### P1 — cargo-vet como gate de código novo (custo ~1–2 h)

4. **Tirar `supply-chain/` do `.gitignore`** e commitar `config.toml`, `audits.toml`, `imports.lock`. Sem isso, não há gate, nem CODEOWNERS, nem histórico — corte o cargo-vet inteiro se essa etapa for recusada.
5. **Corrigir o desatualizado:** `multer 3.1.0` precisa de auditoria (2.817 linhas segundo o suggest) ou exemption explícita com `notes`. `cargo vet --locked` deve passar **antes** de ligar o gate.
6. **Adicionar 3 imports** no `config.toml` — `mozilla`, `google`, `bytecode-alliance` (URLs em [15]); gerar `imports.lock`; **não** adicionar `zcash` enquanto a URL do registry estiver 404 [NV4]. Depois rodar `cargo vet suggest` e medir o backlog real.
7. **Adicionar o passo de CI do manual**: `cargo vet --locked` [11]. Na primeira semana, rode em modo **informativo** (allow-failure) para medir o impacto em PRs; vire bloqueante assim que o gate estiver estável.
8. **Regras de governança do gate** (o que impede o gate de virar teatro):
   - `CODEOWNERS` para `/supply-chain/` com 1–2 mantenedores [12];
   - **nunca** commitar `cargo vet regenerate exemptions` como resolução padrão de PR [9];
   - toda exemption nova exige `notes` com justificativa e data [13];
   - marcar `suggest = false` apenas para crates que nunca serão auditadas (ex.: plataformas que o Flora não entrega), com `notes` explicando — e documentar o critério;
   - revisão trimestral: `cargo vet suggest` + `cargo vet prune` e registrar quantas exemptions foram removidas (meta: **diminuir**, nunca aumentar).

### P1.5 — o que auditar primeiro se sobrar tempo (o 20% crítico)

Não auditar por ordem alfabética. Priorizar crates que processam input não confiável ou fazem crypto/auth, e preferir **deltas** a full audits [2][6]:

1. `jsonwebtoken`, `rsa`, `sha1`/`sha2`, `hkdf`, `digest`, `crypto-common`, `const-oid`, `argon2` (auth/crypto);
2. `tower-http`, `axum`, `axum-core`, `reqwest`, `sqlx`/`sqlx-macros`, `multer` (fronteira de red/DB — onde o `safe-to-deploy` realmente importa);
3. `rustls*`, `aws-lc-rs`/`aws-lc-sys` (TLS), se não cobertos por imports.

Mesmo uma auditoria delta curta contra uma base exemptionada já tem valor documentado [2]. Registrar cada auditoria com `who`, data e `notes`.

### P2 — inventário e distribuição

9. Gerar SBOM CycloneDX em tag/release e anexar ao GHCR/artefato [19][20] — sem isso, não há inventário auditável para responder "qual versão foi entregue?".

### DEFER (com gatilhos para reavaliar)

- Zerar as 442 exemptions (só se entrar em mercado regulado ou se um cliente exigir).
- Critérios customizados (`crypto-reviewed`, `no-network-io`) e publisher-trust em escala [8][10].
- Importar os 9 orgs do registry; 3 já dão ~90% do benefício observado (§3.2).
- cargo-crev, cackle, cargo-geiger como gates.

**Gatilhos de escalada:** (a) Flora passar a processar dados sensíveis de terceiros sob contrato; (b) distribuição de binário/container para clientes externos (agrava AGPL §6(d) e a necessidade de SBOM); (c) mais de 5 pessoas puderem aprovar mudanças em `supply-chain/` — aí CODEOWNERS deixa de ser opcional [12].

---

## 7. NVs (não validado nesta pesquisa)

- **NV1** — `cargo vet --locked` com store versionado em CI limpo não foi executado de ponta a ponta; validar após o item 4/5.
- **NV2** — `multer 3.1.0`: auditoria (2.817 linhas) vs exemption com justificativa — decisão pendente; o número de linhas é estimativa do `suggest`.
- **NV3** — Os números de backlog (8,7M → 7,0M → 6,8M) dependem do estado do registry em 2026-11-16 e de `--all-features`; reexecutar após imports reais. Não são meta, são diagnóstico.
- **NV4** — URL de `[registry.zcash]` retorna 404; não se sabe se é movimentação temporária do repo ou abandono do conjunto. Não importar até confirmar.
- **NV5** — `cargo vet suggest --locked` panica em `src/storage.rs:837` com `audits.toml`/`imports.lock` vazios; testar com versão mais nova do cargo-vet e com store populado. Isso pode impedir o uso de `--locked` em certos fluxos.
- **NV6** — Sugestões de publisher-trust/wildcard não foram avaliadas uma a uma; confiar em publicador é heurístico e exige verificação do processo de release [8].
- **NV7** — `cargo deny check` completo (licenças/advisories) continua sem verificação local/CI neste ambiente (403 ao índice no ciclo 025); a allow-list do `deny.toml` não foi confrontada com as 452 entradas do lock.
- **NV8** — Efeito de `regenerate exemptions` sobre o config atual não foi medido (poderia remover exemptions e/ou enfraquecer `safe-to-deploy` → `safe-to-run` [9]); é preciso um diff revisado antes de qualquer execução.
- **NV9** — `deny.toml` usa `confidence-threshold = 0.8`; a política de licenças do Flora e o impacto de `MPL-2.0`/`Unicode-3.0`/`AGPL-3.0` na allow-list não foram revalidados aqui (ver doc de licenciamento).
- **NV10** — Custo real de CI do cargo-vet (instalação + cache do binário) não foi medido no runner do projeto; a receita oficial usa cache de tool [11].

---

## Fontes primárias (acesso 2026-11-16)

1. Mozilla — *Cargo Vet, Introduction* (sharing, relative audits, deferred audits): https://mozilla.github.io/cargo-vet/
2. Mozilla — *How it Works* (`init` em ~5 min, bloqueio de código novo, footnote sobre deltas contra exemptions): https://mozilla.github.io/cargo-vet/how-it-works.html
3. Mozilla — *Setup* (`Vetting Succeeded (X exempted)`, shrink the list): https://mozilla.github.io/cargo-vet/setup.html
4. Mozilla — *Built-In Criteria* (`safe-to-run`, `safe-to-deploy`, `implies`): https://mozilla.github.io/cargo-vet/built-in-criteria.html
5. Mozilla — *FAQ* (por que `init` exemptiona tudo; sem web-of-trust): https://mozilla.github.io/cargo-vet/faq.html
6. Mozilla — *Performing Audits* (exemptions enfraquecem o valor; `suggest`; registry): https://mozilla.github.io/cargo-vet/performing-audits.html
7. Mozilla — *Importing Audits* (fastest way to shrink; não transitivo; registry): https://mozilla.github.io/cargo-vet/importing-audits.html
8. Mozilla — *Trusting Publishers* (trust entries com expiração; heurístico): https://mozilla.github.io/cargo-vet/trusting-publishers.html
9. Mozilla — *Commands* (`suggest`, `certify`, `regenerate exemptions`, `prune`, `trust --all`, `wildcard`, `record-violation`): https://mozilla.github.io/cargo-vet/commands.html
10. Mozilla — *Specifying Policies* + *Audit Criteria* (default criteria; `dependency-criteria`; conjuntos Google `ub-risk-N`): https://mozilla.github.io/cargo-vet/specifying-policies.html ; https://mozilla.github.io/cargo-vet/audit-criteria.html
11. Mozilla — *Configuring CI* (`cargo vet --locked`): https://mozilla.github.io/cargo-vet/configuring-ci.html
12. Mozilla — *Curating Your Audit Set* (CODEOWNERS, >5 aprovadores, "no real way to check their work"): https://mozilla.github.io/cargo-vet/curating-your-audit-set.html
13. Mozilla — *Configuration* (`notes`/`suggest` em exemptions, `policy`, `imports.exclude`, `criteria-map`): https://mozilla.github.io/cargo-vet/config.html
14. Mozilla — *The Algorithm* (exemptions desaçucaradas como auditorias, caveats, grafo maximal `--all-features`): https://mozilla.github.io/cargo-vet/algorithm.html
15. Mozilla — `registry.toml` (9 conjuntos de auditores): https://raw.githubusercontent.com/mozilla/cargo-vet/main/registry.toml
16. Microsoft — *Rust Engineering Practices, ch. 6 Dependency Management and Supply Chain Security* (cargo-vet para requisitos rigorosos; cargo-deny suficiente para maioria; pipeline audit/deny): https://microsoft.github.io/RustTraining/engineering-book/ch06-dependency-management-and-supply-chain-s.html
17. Trail of Bits — *Testing Handbook, Rust Supply Chain Analysis* (vetting é mais "understanding" que "checking"; imports recomendados; cargo audit; cackle): https://appsec.guide/docs/languages/rust/supply-chain-analysis/
18. Embark — *cargo-deny, `[advisories]` config* (RustSec por padrão, `ignore` com reason/expiry, `version = 2`): https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html ; visão geral: https://embarkstudios.github.io/cargo-deny/ ; nota de escopo cargo-audit vs cargo-deny: https://github.com/EmbarkStudios/cargo-deny/issues/386
19. OWASP CycloneDX — *CycloneDX Rust (Cargo) Plugin* (gera SBOM; chama Cargo internamente): https://github.com/CycloneDX/cyclonedx-rust-cargo
20. NTIA — *The Minimum Elements For a Software Bill of Materials (SBOM)* (2021): https://www.ntia.doc.gov/report/2021/minimum-elements-software-bill-materials-sbom
21. Flora — cross-check local somente leitura: `supply-chain/*`, `Cargo.lock`, `deny.toml`, `.gitignore`, `.github/workflows/{ci,rust}.yml`, `.specify/releases/025-*/release.md`, `.specify/validations/025-*/validate.md`, `docs/research/2026-11-16-seed-license-dependency-audit.md`; comandos `cargo vet 0.10.3` executados em cópias descartáveis.
