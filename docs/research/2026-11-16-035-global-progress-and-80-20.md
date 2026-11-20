# 035: progresso global e próximo 80/20 (síntese com fontes primárias)

**Data:** 2026-11-16. **Tipo:** síntese decisória; não altera código, CI nem deploy.
**Método:** evidência executada localmente + pesquisa web em fontes primárias (documentação oficial), em inglês, português, alemão e russo. Skills aplicadas: `research` (agente de fundo, fontes primárias), `firecrawl-deep-research` (tier *Exhaustive*: 10+ consultas, 25+ fontes, visões contrárias), `documentation-writer` (Diátaxis) e `writing-guidelines` (Vercel).
**Leia também:** `2026-11-16-035-stack-current-state-global.md`, `2026-11-16-035-vps-operability-global.md` e os relatórios 035 de landscape, mecanismos opcionais e segurança/supply-chain.

## Resumo

O caminho percorrido é real: 34 specs, 34 listas de tarefas, 31 assessments, 25 validações, 25 releases e 32 ciclos mergeados. O que falta é sobretudo **verificação**, não código novo. O maior avanço com menor esforço é corrigir o gate de testes do CI: hoje `cargo test` roda **0 testes** no workspace raiz; trocar uma linha para `cargo test --workspace` passa a executar **104 testes** em cada PR. Custo: minutos. Risco: nenhum. Comando SpecKit: `/speckit.bug.assess`.

## 1) Quanto do caminho já percorremos

Contagem executada neste checkout em 2026-11-16:

| Artefato | Quantidade | Leitura |
|---|---|---|
| Specs (`spec.md`) / tarefas (`tasks.md`) | 34 / 34 | Cobertura SDD contínua de 001 a 034 |
| Planos (`plan.md`) / checklists | 24 / 20 | Cerimônia reduzida a partir de 021 e 025 |
| Assessments (`intake`) com `problem`/`concept` | 31 (14 / 14) | "Define" e "shape" abandonados após o ciclo 023 |
| Validações / releases | 25 / 25 | Rastreabilidade de execução por ciclo |
| Entradas em `.specify/bugs` | 1 | `forgejo-rust-ci-rust-toolchain-action-not-found`, com `test.md` = `partial` |
| Documentos em `docs/research/` | 104 | Base de decisão acumulada |
| PRs no GitHub / issues abertas | 82 / 168 | Tracker de issues obsoleto e sem triagem |

Verificação executada localmente nesta sessão: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build -p flora-api` e `cargo test --workspace` passaram (104 testes, 43 ignorados). A API respondeu `/health`, `/health/ready`, `/health/live`, `/metrics` e `/llms.txt` com 200 contra PostgreSQL 18 real; `/api/v1/organizations` sem token respondeu 401. O frontend passou em `check`, `lint`, `build` e nos 30 testes do projeto `server`.

## 2) O problema de maior alavancagem

`rust.yml:35` executa `cargo test`. A documentação oficial do Cargo define que, em workspace não virtual com `[package]` na raiz e sem `default-members`, o pacote selecionado por padrão é **apenas o crate raiz**. A raiz de Flora é o placeholder `src/main.rs`, com 0 testes. Os 104 testes dos 7 crates só rodam com `--workspace`.

Prova local: `cargo metadata --no-deps` reporta `workspace_default_members = [pacote raiz]`; `cargo test` imprime `running 0 tests`; `cargo test --workspace` imprime 104 testes aprovados.

A ADR-0001 exige "evidência executada, não auto-certificação" e define gates de verificação. Hoje o gate mais visível do CI é verde sem executar a suíte. Isso afeta diretamente 82 PRs e toda alegação de "CI verde" usada para fechar ciclos, specs e ADRs.

**Causa-raiz:** o comando de teste ficou no padrão do Cargo quando o workspace foi criado, enquanto o clippy recebeu `--workspace --all-targets` explicitamente (`rust.yml:27`).

## 3) Priorização RICE adaptada

A escala é a definida pelo autor do RICE na Intercom: `Impact` 3/2/1/0,5/0,25; `Confidence` 100/80/50%; `Effort` em pessoas-mês, com valores fracionários para trabalho bem inferior a um mês. A fórmula é `(Alcance × Impacto × Confiança) / Esforço`. O próprio autor recomenda usar a pontuação para tomada de decisão informada, não como regra rígida, e a literatura posterior alerta contra falsa precisão. Aqui o "alcance" é a contagem verificável de decisões ou execuções afetadas por trimestre, não usuários finais: Flora não tem telemetria, então `Reach` de features de produto fica explicitamente marcado como NV.

| Candidato | Comando SpecKit | Alcance | Impacto | Confiança | Esforço | Leitura |
|---|---|---:|---:|---:|---:|---|
| **A. Corrigir `cargo test` → `cargo test --workspace`** | `/speckit.bug.assess` | 82 PRs | 3 | 100% | ~0,05 | **Veredito: fazer agora** |
| B. Corrigir `docker-staging.yml` (`secrets` em `if:`) | `/speckit.bug.assess` | 100 runs | 2 | 100% | ~0,05 | Bloqueado por guardrail de deploy |
| C. Fechar 4 NVs de VPS (carga/restore/limites) | `/speckit.assess.intake` | 1 deployment | 2 | 80% | 0,5–1 | 3–6 dias; depende de host e autorização |
| D. FASE 7 Documents | `/speckit.specify` | NV (sem usuários) | 2 | 50% | 2–3 | Design pronto, mas demanda não medida |
| E. Artefato `analyze` do ciclo 034 | `/speckit.specify` | 1 processo | 0,5 | 100% | ~0,02 | Higiene de processo, sem efeito em produto |

O ponto A vence porque combina confiança máxima (o defeito foi reproduzido localmente), custo de uma linha e efeito transversal sobre todas as alegações de verificação. O princípio do Shape Up casa com isso: o "appetite" é de um *small batch* (uma ou duas semanas); uma correção de uma linha cabe folga dentro desse apetite e não precisa de pitch novo.

## 4) Decisão e execução exata

**Próximo passo: `/speckit.bug.assess`** para o bug "CI `test` step executa 0 testes".

Arquivos e mudança:

```yaml
# .github/workflows/rust.yml:35
- name: Test
-   run: cargo test
+   run: cargo test --workspace
```

Sequência sugerida no fluxo de bug:

1. `/speckit.bug.assess` com o log local (`running 0 tests` vs 104) e a citação da doc do Cargo.
2. `/speckit.bug.fix`: uma linha no workflow; nenhuma mudança de código de produto.
3. `/speckit.bug.test`: confirmar no log do job `build` que os 104 testes aparecem; fechar com evidência executada, não com "CI verde" genérico.

Variação opcional, ainda no mesmo apetite: usar `cargo test --workspace --all-targets` para alinhar o comando de teste ao de clippy, já que `--all-targets` equivale a `--lib --bins --tests --benches --examples`.

**Companheiro de mesmo apetite:** o CI fixa exatamente Rust `1.98.0` em quatro pontos de `rust.yml` e no `Dockerfile`. A 1.98.1 corrigiu uma compilação incorreta de trait object que podia gerar ponteiro nulo na vtable (UB); a estável atual é 1.100.0. Subir o pin para uma versão corrigida tem justificativa de correção, não de novidade. Fontes oficiais e o impacto completo estão em `2026-11-16-035-stack-current-state-global.md` (S1–S5). Manter 1.98.0 é aceitável apenas se a equipe aceitar compilar com UB conhecido.

**Segundo colocado, somente com autorização:** `docker-staging.yml:43` usa `secrets.COOLIFY_DEPLOY_WEBHOOK` dentro de `if:` no nível de step, o que a documentação oficial do GitHub Actions não permite; o padrão correto é mover o segredo para `env` e comparar a variável de ambiente. O workflow falha em 100 dos 100 runs recentes e nunca cria jobs. Corrigi-lo pode publicar a imagem no GHCR e disparar o webhook do Coolify, e os guardrails do projeto proíbem deploy/produção sem autorização explícita. Registre como bug separado; não execute sem permissão.

**Não fazer agora:** FASE 7 Documents, reabilitação de Meilisearch, exporter OTLP e testes de carga/restore. Todos são plausíveis, mas nenhum tem o mesmo custo-benefício de uma linha que restaura a confiança de toda a cadeia de verificação.

## 5) Riscos e perguntas em aberto

- O `db-tests` já roda as seis suítes com `--ignored`; `--workspace` não muda isso. A correção amplia os testes unitários e de contrato CPU-only, não substitui cobertura de banco.
- 43 testes continuam ignorados fora do job `db-tests`. Isso é intencional e documentado.
- A contagem de 104 testes é deste checkout; ela cresce com novos testes e o número exato no log do CI deve ser usado como evidência.
- Não há telemetria de uso real, então nenhuma decisão de feature de produto pode reivindicar alto `Reach` com evidência. Essa é a principal lacuna de dados para o próximo ciclo de produto.

## 6) Achados externos que afetam a direção

- A tese de debloat se sustenta: o Macro está hiperativo (8.881 commits, último push em 2026-11-16), é hosted-first e sua stack de desenvolvimento exige OpenSearch, Kafka, FusionAuth, Cloudflare e OpenBao. Isso valida a decisão de não perseguir paridade.
- Há uma janela de licenciamento: Macro migrou de BSL para AGPL, Mattermost limita binários gratuitos a 250 usuários, NocoDB não é OSI, AppFlowy-Cloud foi arquivado e Outline usa BSL. AGPL puro é um diferencial de fato, não apenas preferência.
- O rival mais próximo no perfil Seed é o Docmost (AGPL, três containers). A cunha honesta da Flora continua sendo monólito single-node, Postgres-autoritativo e pegada baixa, ainda não provada em 2 vCPU/4 GB.
- Contrapontos registrados: custo de tempo e energia do self-hosting, banimento de AGPL em algumas empresas e pools de talento menores para Rust e Svelte. Detalhes e 30+ fontes primárias em `2026-11-16-035-landscape-macro-alternatives-global.md`.

**Implicação para o 80/20:** nenhum desses achados desloca a recomendação. A janela de licenciamento só vira vantagem se a verificação for confiável; corrigir o gate de testes continua sendo o pré-requisito.

## 7) Fontes primárias selecionadas

- Intercom, "RICE: Simple prioritization for product managers", 2018-01-05: https://www.intercom.com/blog/rice-simple-prioritization-for-product-managers (acesso 2026-11-16). Define escalas e fórmula.
- Basecamp, "Shape Up" (Ryan Singer), capítulos 2 e 3: https://basecamp.com/shapeup/1.1-chapter-02 e https://basecamp.com/shapeup/1.2-chapter-03 (acesso 2026-11-16). Define *appetite*, *small batch* e *fixed time, variable scope*.
- Cargo, "Workspaces" e `cargo test`: https://doc.rust-lang.org/cargo/reference/workspaces.html e https://doc.rust-lang.org/cargo/commands/cargo-test.html (via relatório 035 de stack, acesso 2026-11-16). Definem root package, `default-members`, `--workspace` e `--all-targets`.
- Documentação oficial do GitHub Actions, contexto de expressões e segredos (via relatório 035 de segurança, acesso 2026-11-16).
- Internas: `docs/adr/0001-seed-v01-mechanism-substitutions.md`, `.github/workflows/rust.yml`, `AGENTS.md`, `ROADMAP.md`, `OBJETIVO.md`.
