# Decision: 024 — Next vertical slice (verificação OIDC do ID token)

- **Slug**: 024-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior) | R1 Seed research (7 relatórios, via intake/research)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | ID token apenas decodificado em base64 e confiado (`auth.rs:201-215,398-414`), sem assinatura/`iss`/`aud`/`exp`/`nonce`, identidade por e-mail (`research.md:13-16`). |
| Evidence strength | adequate | Síntese R1 de 8 agentes com citações `arquivo:linha` e plano de verificação determinística com fixtures/JWKS locais; prova com provedor real e rotação de JWKS permanece NV (`research.md:3-4,20-21,28-32`). |
| Value vs. inaction | strong | Única lacuna capaz de admitir sessão não autorizada; sem ela "Users & Roles (OIDC)" resta PARTIAL/aspiracional (`research.md:24-26`; `problem.md:38-40`). |
| Feasibility / appetite | adequate | Opção A (`medium`) reaproveita `openidconnect 4.0` já declarado e a infraestrutura de sessão existente; exige fontes de verdade para issuer/audience/redirect (`research.md:15-16`). |
| Strategic fit | strong | Fecha o marco Seed v0.1 com mecanismo já presente no código, sem novos serviços obrigatórios (`intake.md:16-21,33-35`). |
| Risk posture | adequate | Fail-closed em config incompleta; ressalva TLS do OIDC Core §3.1.3.7 indisponível no perfil padrão `http://localhost:8080`, então `iss`/`aud`/`exp`/`nonce` são MUST (`research.md:28-31`). |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**. O scorecard converge com a decisão anterior: S1 é o maior gap decisivo do Seed, de segurança crítica, com esforço limitado e dependência já declarada; nada nos artefatos inverte essa escolha. A ressalva normativa do perfil local e os NVs (provedor real, rotação de chaves, histórico de identidades por e-mail) reforçam o recorte determinístico em vez de contestá-lo.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas de configuração seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: Estabelecimento de sessão a partir de ID token decodificado e confiado, sem integridade/claims verificadas e com identidade inferida por e-mail (`problem.md:7-11`).
- **Chosen approach**: S1 / Opção A — "Porta única de sessão verificada": descoberta/JWKS com o crate `openidconnect`, `nonce` em transação + callback, allowlist exata de redirect, identidade por `sub` e config fail-closed (`concept.md:9-13,27-29`; decisão anterior:6-9).
- **In scope / out of scope**:
  - **In**: `flora-api/src/routes/auth.rs` (verificação via discovery/JWKS; `nonce` em transação/callback; redirect allowlist; lookup por `sub`; erro fail-closed de config), `flora-core/src/config/oidc.rs` (validação explícita de issuer/audience/redirect conforme necessário), testes com discovery/JWKS mockados + token adulterado + nonce ausente/aud/exp errados mantendo contratos DB de auth verdes; SDD/validação/release, atualização de docs de planejamento, PR/CI verde, merge/sync (decisão anterior:16-21).
  - **Out/NV**: enforcement da FASE 0 no CI, contabilização honesta de OTel, ADR/reconciliação ampla de docs do Seed, mudança de transporte de notificações, RustFS/S3, reativação do Meilisearch, browser e2e/Tauri, e2e com provedor real e exercícios de rotação de JWKS; non-goals: novos serviços obrigatórios, alterações no frontend e tocar nos arquivos `/metrics` do agente paralelo (decisão anterior:23-26).
- **Success metrics**: token adulterado ou com claim inválido não estabelece sessão; identidade por `sub`; evidência determinística com fixtures locais e endpoint mockado; estado de "Users & Roles (OIDC)" alinhado ao comportamento (`problem.md:31-36`).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: emissor/audiência canônicos quando o provedor real for configurado?]; [NEEDS CLARIFICATION: como tratar histórico de identidades por e-mail ao migrar para `sub` (unicidade/backfill de `oidc_subject`)?]; [NEEDS CLARIFICATION: a ressalva TLS se aplica a algum perfil além do padrão?]; [NEEDS CLARIFICATION: fonte de verdade e administração da allowlist de redirect?] (`problem.md:42-47`).
