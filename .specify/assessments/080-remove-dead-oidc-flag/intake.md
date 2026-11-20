# Intake: remover flag OIDC morta `local_fallback_enabled`

- **Slug**: 080-remove-dead-oidc-flag · **Capturado**: 2026-11-19T21:32Z
- **Origem**: dados de research locais (`docs/research/2026-11-16-seed-oidc-identity-gap.md:120,147` e `docs/research/2026-11-16-016-auth-rbac-oidc-minimal-vps.md:56`) + varredura do código (zero leitores). Nenhum pedido novo do usuário neste ciclo: a iniciativa vem do backlog G9/NV-7 (dados, não instrução).
- **Alvo**: branch `feature/080-remove-dead-oidc-flag`, HEAD-base `e3944e962`; gate PASS; zero PRs. Fronteira: exatamente os 5 artefatos `.specify/assessments/080-remove-dead-oidc-flag/`; a edição futura de 3 linhas em `flora-core/src/config/oidc.rs` é o objeto avaliado, não parte desta escrita.

## Ideia

`flora-core/src/config/oidc.rs:18-19` declara `pub local_fallback_enabled: bool` com o doc "Local fallback enabled when Zitadel is unavailable (default: true in dev)"; `:40` inicializa `local_fallback_enabled: true`. O campo **nunca é lido**: `OidcConfig::validate` (`:52-74`) não o consulta, e `grep -rn "local_fallback"` encontra apenas as duas linhas de declaração/default no repo (mais menções em docs de research e numa cópia em worktree). O default `true` sugere ao operador um fallback de segurança que não existe — o risco apontado em G9 (`docs/research/...seed-oidc-identity-gap.md:120`: "declared but never consulted"). Remover as 3 linhas (doc+campo em `:18-19`, inicialização em `:40`) elimina a falsa garantia sem mudar comportamento: `Config` não usa `deny_unknown_fields` (varredura zero) e `Config::load` (`flora-core/src/config/mod.rs:43-49`) usa Figment com `Serialized::defaults` + `Env::prefixed("FLORA_")`, que ignora chaves extras.

## Por que agora

Candidato 080 eleito pelo 80/20 com **E=(3×5)/(1×1)=15,0** — o maior do ciclo: valor 3 (remove sinal de segurança inexistente e dívida de config), confiança 5 (evidência local integral: declaração, default, zero leitores, sem `deny_unknown_fields`), esforço 1 (3 linhas em um arquivo), risco 1 (sem leitores, sem contrato de serialização consumido). Alternativas descartadas: OTLP 1,25 (sem receptor/dono G1, deps 0.33.1, risco alto), lucide 2,50 (3 imports + 2 lockfiles), `.env.example` 10,0 e `cssVariables` 5,0 (menor valor). A edição não toca código de auth, gate, ADRs, specs numeradas nem `.env.example` (`flora-core/src/config/oidc.rs` é o único arquivo de código no escopo).
