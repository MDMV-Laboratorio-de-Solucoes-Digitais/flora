# Seed FASE 0 — Licenciamento + auditoria de dependências (2026-11-16)

**Objetivo:** definir o mínimo credível de FASE 0 para Flora (AGPL-3.0-or-later, Rust workspace + Svelte 5, self-hosted em VPS barata).  
**Método:** fontes primárias (GNU/FSF, SPDX, Cargo Book, docs cargo-deny/cargo-vet, CycloneDX, NTIA) + cross-check **somente leitura** no repositório. Nenhum arquivo do repo foi modificado por esta pesquisa além deste documento.  
**Escopo:** dados de 2026-11-16; não é parecer jurídico (FSF explicitamente não presta legal advice [2]).

## 1) AGPL-3.0 §13: self-hosted vs hospedado

| Situação | Obrigação real | Base |
|---|---|---|
| Uso interno da organização, sem `convey` | Sem obrigação de publicar modificações; rodar programa é permitido sem condições (§0 define `convey` como propagação que permite a outros fazer/receber cópias; **interação em rede sem transferência de cópia não é conveying**) | [1] §0, §2; [2] #InternalDistribution |
| Self-host **sem modificações** | §13 não dispara (ele começa com *"if you modify the Program"*); continua valendo a obrigação de fornecer texto da licença/avisos se houver convey (ex.: distribuir binário/container) | [1] §4–§6, §13; [3] |
| Self-host **modificado**, acessado por usuários na rede | Deve **oferecer de forma proeminente** a todos os usuários que interagem remotamente a oportunidade de receber o **Corresponding Source da versão modificada**, via servidor de rede, sem custo, por meio padrão/usual de cópia | [1] §13; [2] #UnreleasedModsAGPL |
| `Corresponding Source` inclui quê? | Definição do §1: fontes para gerar/instalar/rodar/modificar, incluindo scripts e código de bibliotecas vinculadas **salvo System Libraries**; bibliotecas modificadas devem ser fornecidas | [1] §1; [2] #AGPLv3CorrespondingSource |
| "Interagir remotamente" | Programas expressamente projetados para aceitar requisições e responder pela rede (servidores web/mail, aplicações web interativas) se enquadram; SSH/X remoto não, por si só | [2] #AGPLv3InteractingRemotely; [3] |
| Frontend servido ao navegador | JS entregue ao navegador **é convey** (o usuário recebe cópia); a regra de "uso em rede não é convey" vale para o código no servidor, não para o bundle baixado | [3] |
| Mistura com GPLv3 | Permissão **unidirecional e limitada** do §13: pode-link/combinar; AGPL continua valendo na parte AGPL, GPLv3 na parte GPLv3. Não é relicenciamento geral (AGPL ≠ GPLv3; AGPL não é compatível com GPLv2) | [1] §13; [2] #AGPLGPL; [5] #AGPL |

**Implicação para Flora:** enquanto o deploy for **flora upstream sem customizações**, a obrigação central é de licenciamento/distribuição (avisos + texto), não de publicação forçada de source. Assim que uma PME/consultor modificar o código e expor a UI na rede, entra a obrigação de oferecer o source daquela versão — o desenho mínimo é (a) registrar modificações com data/aviso (§5a) e (b) ter um link "Source" proeminente no UI + repositório público. O texto canônico do GNU recomenda exatamente esse padrão para apps web [1] "How to Apply These Terms".

## 2) LICENSE e campos SPDX — o que é "correto"

- `SPDX` é o formato esperado: o campo `license` do Cargo é uma **expressão SPDX** (`AND`/`OR`/`WITH`); `license-file` é alternativa para licença não padrão [4]. O identificador **`AGPL-3.0-or-later`** existe e significa AGPL-3.0 ou versão posterior, distinto de `AGPL-3.0-only` [7].
- Texto canônico da licença inclui, após `END OF TERMS AND CONDITIONS`, o apêndice **"How to Apply These Terms to Your New Programs"**; o FAQ do GNU diz que preâmbulo e instruções são partes integrais e não devem ser omitidos [1][2] #GPLOmitPreamble.
- Ter só o arquivo `LICENSE` não deixa inequívoco que os arquivos do repo estão sob a licença; o FAQ recomenda aviso em cada arquivo-fonte, sendo uma declaração clara no README **legalmente suficiente** [2] #LicenseCopyOnly, #NoticeInSourceFile.
- `apps/frontend/package.json` está `"private": true` e **sem campo `license`**; `apps/web/tauri/src-tauri/Cargo.toml` tem `license = ""` — ambos são sinais ambíguos para ferramentas automáticas [4][6].

## 3) Ferramentas realistas em CI (sem tooling proprietário)

| Ferramenta | Custo | O que entrega | Adequação Flora / FASE 0 |
|---|---|---|---|
| `cargo deny check licenses` | FOSS; action oficial `EmbarkStudios/cargo-deny-action@v1` | Allow-list de SPDX, `confidence-threshold`, `clarify`, `exceptions`, checa dev/build deps; trata GNU licenses de forma pedantic desde 0.18.4 | **Primeira escolha.** Exige criar `deny.toml` (não existe hoje). Rodar `cargo deny check` falha sem config. [6] |
| `cargo vet --locked` | FOSS; receita de CI pronta (install + cache + `cargo vet --locked`) | Auditorias por crate + **exemptions ratchetáveis**;Share/relative/deferred audits | **Segunda camada:** suprir auditoria de supply-chain. Já há `supply-chain/`, mas sem CI e com `audits.toml`/`imports.lock` vazios [8][9] |
| `cargo-cyclonedx` (CycloneDX Rust) | FOSS (Apache-2.0), `cargo install cargo-cyclonedx` | SBOM CycloneDX de todas as dependências a partir do Cargo | **SBOM mínimo realista**: gerar JSON em tag/release e anexar ao artefato/GHCR. Não é "check de licença", é inventário [10] |
| `cargo metadata` / `Cargo.lock` | Nativo | Lista de pacotes/versões | Útil para inventário rápido, **não** é SBOM completo (NTIA exige fornecedor, componente, versão, identificadores únicos, dependências, autor do SBOM e timestamp) [11] |
| Ferramentas comerciais de compliance | Caro / proprietary | Scan de código, relatórios, legal review | **Fora do FASE 0**; VPS barata + FOSS não comporta; substituir por allow-list + revisão humana [6][8] |

**Armadilhas de compatibilidade a codificar na allow-list** [5][6]:
- Apache-2.0 é compatível com GPLv3, **não** com GPLv2; AGPLv3 não é compatível com GPLv2 [5].
- `AGPL-3.0-only` vs `AGPL-3.0-or-later` e `GPL-3.0` (deprecated) vs `GPL-3.0-only` — cargo-deny trata GNU licenses pedanticamente; allow-list errada gera falso verde [6].
- Licenças free mas GPL-incompatíveis: BSD 4-clause (advertising), CDDL-1.0, OSL, MPL-1.1, EPL-1.0/2.0 sem designação secondary, **MPL-2.0 com "Incompatible With Secondary Licenses"**, EUPL-1.2 sem conversão [5].
- Crate sem `license`/`license-file` → "unlicensed" no cargo-deny; usar `[[licenses.clarify]]` com hash ou abrir issue upstream [6].
- `dev-dependencies` são ignoradas por padrão no cargo-deny; `build-dependencies` entram por padrão e podem afetar o artefato [6].
- `"MIT"` é ambíguo (X11 vs Expat) na linguagem corrente; exigir ID SPDX explícito [5][4].

## 4) Estado atual do Flora (cross-check read-only)

| Item | Estado | Evidência / detalhe |
|---|---|---|
| Arquivo `LICENSE` | **Presente, mas incompleto** | `LICENSE` (31.6 KB, 619 linhas) = texto AGPL-3.0 verbatim até `END OF TERMS AND CONDITIONS` (linha 619). **Falta o apêndice "How to Apply These Terms to Your New Programs"** presente no texto canônico [1]. Sem `NOTICE`. |
| Declaração de licença no repo | **Parcial** | `README.md:71`: "Flora is licensed under ... AGPL-3.0"; `README.md:32`: `- [ ] License: AGPL` (checkbox ainda desmarcado); badge no topo. Suficiente no nível README [2], mas **zero cabeçalhos `SPDX-License-Identifier`** nos ~169 arquivos `.rs`/`.svelte`/`.ts`. |
| `Cargo.toml` raiz | **Correto** | `package.license = "AGPL-3.0-or-later"` em `[workspace]`; `[package] license.workspace = true`. Expressão SPDX válida [4][7]. |
| Crates do workspace | **Corretos** | `flora-{core,api,messaging,tasks,files,search,notifications,tests}/Cargo.toml`: `license.workspace = true`. |
| Apps fora do workspace | **Gap** | `apps/web/tauri/src-tauri/Cargo.toml:5` → `license = ""`; `apps/frontend/package.json` → `"private": true`, sem `license`. `apps/web` é legado/não canônico, mas o campo vazio é armadilha para scanners. |
| `supply-chain/` | **Presente, inefetivo** | `config.toml` v0.10 com **442 exemptions**, todas `criteria = "safe-to-deploy"`; `audits.toml` e `imports.lock` contêm apenas cabeçalhos vazios. Sem `[imports]`, sem critérios customizados, sem CI. `Cargo.lock` tem 452 pacotes → cobertura real de auditoria ≈ 0. |
| `deny.toml` | **Ausente** | Nenhum `deny.toml`/`deny.exceptions.toml` no repo (excluindo `target/`). Binários `cargo-deny` e `cargo-vet` existem no ambiente local, mas não há config nem chamada em CI. |
| `.github/workflows/rust.yml` | **Sem gates de licença/SBOM** | `cargo build` + `cargo test` + jobs DB-gated. Sem `cargo fmt/clippy`, `cargo deny`, `cargo vet`, `cargo audit` ou geração de SBOM. |
| `.github/workflows/ci.yml` | **Frontend sem auditoria de licenças** | Só `bun install`, `check`, `lint`, `test:unit`, `build` em `apps/frontend`. |
| `.github/workflows/docker-staging.yml` | **Sem SBOM/provenance** | Publica `ghcr.io/.../flora:staging` e `:sha`, dispara Coolify; sem attest/SBOM. |
| UI §13 ("Source" link) | **Ausente** | Nenhum link de código-fonte/aviso AGPL encontrado em `apps/frontend/src`. Aceitável se não houver modificações; obrigatório no primeiro deploy modificado [1][2]. |
| Inventário de dependências | **Parcial** | `Cargo.lock` presente e versionado (452 pacotes). Sem inventário de licenças effective nem SBOM. |

## 5) Checklist mínimo FASE 0 (ordenado, verificável)

1. [ ] **Normalizar `LICENSE`** para o texto canônico completo (incluir apêndice "How to Apply These Terms"), com `git diff` mostrando apenas o acréscimo.
2. [ ] **Explicitar licença em todos os manifests**: manter `AGPL-3.0-or-later` no workspace; corrigir `license = ""` do Tauri legado (ou marcar `publish = false` com justificativa); adicionar `"license": "AGPL-3.0-or-later"` em `apps/frontend/package.json`.
3. [ ] **Avisos por arquivo**: decidir política mínima — README + cabeçalho SPDX nos `lib.rs`/`main.rs` de cada crate e nos entrypoints Svelte; gerar com script, não à mão [2].
4. [ ] **Criar `deny.toml` com allow-list fechada** (MIT/Apache-2.0/BSD/ISC/Unicode/etc. conforme inventário real), `confidence-threshold = 0.95`, `include-build = true`, `include-dev` explícito, `[[licenses.exceptions]]`/`clarify` apenas com hash; rodar `cargo deny check licenses` local até passar [6].
5. [ ] **Adicionar gate em `rust.yml`**: `cargo deny check licenses` (e `bans`/`sources` se quiser, no mesmo passo). Falha de licença bloqueia merge.
6. [ ] **Revalidar `supply-chain/`**: `cargo vet prune` / regenerar exemptions contra o `Cargo.lock` atual; substituir o blanket `safe-to-deploy` por auditorias reais nas 10–20 crates mais críticas e adicionar `cargo vet --locked` ao `rust.yml` [8][9].
7. [ ] **SBOM de release**: gerar CycloneDX JSON com `cargo-cyclonedx` num job de tag/release e anexar ao artefato/GHCR, com supplier/component/version/purl/dependencies/timestamp [10][11].
8. [ ] **Política §13 documentada** (1 página em `docs/`): "modificações expostas em rede = oferecer Corresponding Source"; link "Source" no rodapé do frontend apontando para o repo; aviso de modificação com data no código/CHANGELOG quando aplicável [1][2].
9. [ ] **Política de ingestão de dependências**: permitir apenas IDs SPDX da allow-list; qualquer GPLv2-only/CDDL/OSL/MPL-incompatível ou crate "unlicensed" exige revisão explícita antes do merge [5][6].

**Definição de pronto (FASE 0):** CI verde em `push`/`PR` com itens 5 e 6; `LICENSE` completo; manifests consistentes; SBOM gerado em tag; decisão §13 registrada. Itens 3 e 8 podem ser fatias separadas se o bloqueio for tempo, mas o item 5 não.

## NVs (não validado nesta pesquisa / próximas verificações)

- **NV1** — `cargo deny check licenses` não foi executado: o allow-list final depende do inventário real das 452 entradas do `Cargo.lock`. Validar com execução real antes de fechar o item 4.
- **NV2** — `cargo vet --locked` não foi executado: 442 exemptions podem estar desatualizadas em relação ao lock atual; confirmar se o comando passa hoje e quantas exemptions sobram após `prune`.
- **NV3** — Interpretação de "modify" para customizações pequenas (config, tema, patches locais) e para deploys apenas internos é questão jurídica; FAQ/FSF não é legal advice [2]. Registrar a decisão de política, não inventar interpretação.
- **NV4** — Inventário de licenças do frontend (`bun pm ls` / lockfile) não foi auditado; bundles Svelte com dependências copyleft podem exigir avisos adicionais por serem **convey** ao navegador [3].
- **NV5** — Titularidade de copyright dos commits MDMV (EMP/contrato/CLA/DCO) não verificada; afeta avisos §4/§5 e a capacidade de relicenciar [1][2].
- **NV6** — A imagem pública `ghcr.io/.../flora:staging` é distribuição externa? Se sim, avaliar §6(d) (oferecer Corresponding Source no mesmo lugar, sem custo extra), não apenas §13 [1].
- **NV7** — Compatibilidade do `cargo-cyclonedx` com Rust 1.98 / lockfile v4 não testada; validar instalação e saída JSON antes de prometer o item 7.

## Fontes primárias (acesso 2026-11-16)

1. GNU — *GNU Affero General Public License v3.0* (§0, §1, §4–6, §13, apêndice): https://www.gnu.org/licenses/agpl-3.0.en.html
2. GNU/FSF — *Frequently Asked Questions about the GNU Licenses* (#UnreleasedModsAGPL, #AGPLv3CorrespondingSource, #AGPLv3InteractingRemotely, #AGPLGPL, #LicenseCopyOnly, #NoticeInSourceFile, #InternalDistribution, #GPLOmitPreamble): https://www.gnu.org/licenses/gpl-faq.html
3. FSF — *The fundamentals of the AGPLv3* (convey vs rede; JS no navegador): https://www.fsf.org/bulletin/2021/fall/the-fundamentals-of-the-agplv3
4. Rust — *The Cargo Book, The Manifest Format* (`license`, `license-file`, expressões SPDX): https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields
5. GNU/FSF — *Various Licenses and Comments about Them* (compatibilidade Apache/AGPL/GPL, MPL-2.0, CDDL, etc.): https://www.gnu.org/licenses/license-list.html
6. Embark Studios — *cargo-deny, The `[licenses]` section* (allow-list, GNU pedantic, clarify, private, dev/build): https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html ; visão geral + GitHub Action: https://embarkstudios.github.io/cargo-deny/
7. SPDX — *AGPL-3.0-or-later* (identificador oficial e distinção de `-only`): https://spdx.org/licenses/AGPL-3.0-or-later.html
8. Mozilla — *Cargo Vet, Introduction* (audits compartilhados, relativos e deferred/exemptions): https://mozilla.github.io/cargo-vet/
9. Mozilla — *Cargo Vet, Configuring CI* (`cargo vet --locked`): https://mozilla.github.io/cargo-vet/configuring-ci.html
10. OWASP CycloneDX — *CycloneDX Rust (Cargo) Plugin*: https://github.com/CycloneDX/cyclonedx-rust-cargo
11. NTIA — *The Minimum Elements For a Software Bill of Materials (SBOM)* (2021): https://www.ntia.doc.gov/report/2021/minimum-elements-software-bill-materials-sbom
