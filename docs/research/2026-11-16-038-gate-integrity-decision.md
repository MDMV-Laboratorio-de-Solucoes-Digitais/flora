# 038 — Integridade do gate de merge autônomo: decisão 80/20 (pesquisa multilíngue)

**Data:** 2026-11-16. **Tipo:** research + decisão; executa o bug flow `.specify/bugs/gate-integrity-unguarded/`.
**Método:** sete subagentes em segundo plano — auditoria de estado, desenho de CI/CD residual, fontes primárias EN/PT, corroboração ES/FR/DE/ZH/RU/JA/KO, operacional da Dark Factory, descoberta de skills (`npx skills find`/`use`, sem instalação) e verificação remota. Skills aplicadas: `research`, `source-driven-development`, `source-driven` de CI (`ci-cd-and-automation`), `security-and-hardening`, `verification-before-completion`, `mdmv-rust-thin`, `doubt-driven-development`.

## 1) Decisão

**Ciclo 038: tornar o gate de merge autônomo auto-defensível.** O merge já é decisão de máquina (ADR-0003, PR #252), mas a ADR nomeia dois riscos residuais que ninguém vigiava: (a) GitHub trata `skipped` como sucesso em required check, e (b) renomear um job órfão o required check e trava todos os PRs. Adicionalmente, nada impedia que um futuro PR adicionasse `continue-on-error` a um job obrigatório ou removesse os pisos de testes — exatamente a classe de defeito que o ciclo 037 corrigiu, agora na camada de configuração.

Entregas:
- `scripts/ci/check_gate_integrity.py` — verificador determinístico, stdlib-only, executado como step do job **obrigatório** `rust-build` (sem criar novo nome de check, evitando deadlock de proteção).
- Holdout da Dark Factory provando que um filtro sem correspondência falha com `passed=0 floor=12` (`/.specify/holdout/`).
- Reconciliação de `OBJETIVO.md`, `ROADMAP.md` e `036-authorization-matrix.md` com ADR-0003 (o material antigo ainda exigia autorização humana para merge).

## 2) Por que este e não outro

| Candidato | Impacto | Esforço | Autônomo | Veredito |
|---|---:|---:|---|---|
| **Integridade do gate + reconciliação de docs** | alto (protege o próprio merge) | baixo | sim | **escolhido** |
| SHA-pin de 16 actions | médio (supply chain) | médio | sim | 2º lugar; item de manutenção contínua |
| RLS sem policies em 11 tabelas | alto (tenancy) | médio/alto | parcial | depende do role real da app (NV); comportamento arriscado |
| RBAC endpoints placeholder | alto (P0) | médio | sim, mas muda comportamento | ciclo separado |
| Promover `cargo-vet`/`e2e-browser` a gates | médio | médio | sim | prematuro: 455 exemptions sem CODEOWNERS |
| FASE 7 Documents | alto | alto | sim | fora do apetite |

O gate integrity vence porque é o único item que **impede regressão silenciosa da confiança** conquistada em 037 — o próprio princípio "verificação é o gargalo" da Dark Factory. Custo: ~150 linhas de script + um step + docs.

## 3) Achados que sustentam a decisão (fontes primárias e multilíngue)

- **Skipped = sucesso:** documentação oficial do GitHub (status checks) trata `skipped`/`neutral` como satisfeito para required checks. Consequência: um `if:` mal colocado em job obrigatório é um gate vazio. Fontes independentes em JA (issoh.co.jp) e EN (community discussion #209881) confirmam; uma afirmação contrária em everydayprep.jp foi marcada NV.
- **Renomear job trava PRs:** required checks casam por **nome exato**; rename órfão deixa o check "Expected" para sempre. Sequência segura: add job → land → add required name → remove old → delete job. Fonte: GitHub troubleshooting docs; ittrip.xyz (JA) mostra o caso real de ruleset com nome obsoleto.
- **SHA pinning:** GitHub recomenda "pin to a full-length commit SHA, the only way to use an action as an immutable release" (secure-use). **16 de 19** `uses:` do Flora são mutáveis; apenas `release.yml` tem 3 SHAs. Exceção: `google/osv-scanner-action/...@v2` não existe como tag — `@v2` provavelmente quebrado, ainda sem runs (NV). Adiado para manutenção, não bloqueante do merge.
- **RLS:** docs oficiais PostgreSQL — sem policy há *default deny* para não-owner, mas **owner/bypassrls ignoram RLS**; sem `FORCE ROW LEVEL SECURITY` em nenhum lugar do repo. `20240818000001_init_schema.sql:200-210` habilita RLS em 11 tabelas com zero policies ativas. O role real da aplicação é **NV**; corrigir às cegas pode zerar todas as queries. Mantido como ciclo separado com investigação de role + pgTAP.
- **cargo-vet não é gate:** passa com 455 exemptions, 0 audits, 0 imports; 13 notas dizem "informational gate only". Promover agora seria abençoar dívida sem dono (sem CODEOWNERS).

Corroboração por idioma: **FR** (stephane-robert.info, tadata.fr — SHA + cooldown), **ES** (secure-pipelines.com), **DE** (Microsoft Zero Trust DS_032 → OWASP CICD-SEC-3 / NIST SSDF PS.3.1), **ZH** (Alibaba Hologres, InfoQ, Huawei RDS — RLS default-deny), **RU** (sysadminium.ru), **JA** (Qiita, issoh.co.jp, ittrip.xyz — rename deadlock), **KO** (hgryoo.dev — RLS como injeção de predicado em tempo de rewrite; trustedoss.github.io — "pinning protege apenas o ref fixado"). Vários resultados apenas traduzem fontes EN — marcados como tal.

## 4) Evidência executada

```
python3 scripts/ci/check_gate_integrity.py     -> gate-integrity: PASS (7 verificações)
python3 <DF>/scripts/holdout/run_holdout.py ... -> 1 scenario
python3 <DF>/scripts/holdout/checker.py ...     -> holdout-check: PASS scenarios=1
  observed: exit_code=1, "auth: passed=0 floor=12", "0 passed ... 89 filtered out"
python3 -c 'import yaml; ...'                   -> yaml-ok
```

Os gates habituais (`fmt`, `clippy -D warnings`, `deny`, `build`, `test --workspace`) seguem sendo executados pelo PR #254. O check `--remote` do verificador compara as proteções ao vivo via `gh api`; é opcional e nunca roda no runner.

## 5) Fontes

Primárias: docs.github.com/en/actions/reference/security/secure-use · docs.github.com/en/pull-requests/reference/status-checks · .../troubleshooting-required-status-checks · github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions · postgresql.org/docs/current/ddl-rowsecurity.html · .../sql-createpolicy.html · .../sql-altertable.html · mozilla cargo-vet docs · embarkstudios cargo-deny checks.
Secundárias independentes: issoh.co.jp, ittrip.xyz, Qiita, hgryoo.dev, trustedoss.github.io, stephane-robert.info, tadata.fr, sysadminium.ru, Alibaba Hologres, InfoQ CN, Huawei RDS.
Internas: `docs/adr/0003-autonomous-merge-under-deterministic-gates.md`, `docs/research/2026-11-16-037-rust-toolchain-pin-bump-decision.md`, `docs/research/2026-11-16-036-{authorization-matrix,ci-baseline}.md`, `.github/workflows/{rust,ci,docker-staging,osv-scan}.yml`.

## 6) NV

- Semântica do runner GitHub e estado ao vivo do Forgejo não exercitados pelo holdout (o holdout replays o shell do workflow, não o Actions).
- `osv-scan.yml` sem runs no window; tag `@v2` aparentemente inválida — confirmar antes de tocar.
- Role de conexão da aplicação com Postgres (dono vs não-dono) não inspecionado.
- Code scanning/secret scanning do repositório NV.
