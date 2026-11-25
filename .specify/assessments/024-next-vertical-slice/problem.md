# Problem Definition: Identidade não verificada no Seed v0.1

- **Slug**: 024-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O estabelecimento de sessão aceita hoje um token de identidade que é apenas decodificado em base64 e tratado como confiável (`flora-api/src/routes/auth.rs:201-215,398-414`, citado em `research.md:13-14`). Não há verificação de assinatura nem de `iss`, `aud`, `exp` e `nonce`, nem descoberta/JWKS e rotação de `kid`, allowlist de redirect ou identidade baseada em `sub` — a identidade é inferida por e-mail (`research.md:14-15`). A dependência `openidconnect 4.0` está declarada, mas não é utilizada (`Cargo.toml:171`; `research.md:15-16`). Já existem PKCE S256, store transacional de uso único, sessões em banco e rotação de `jti` (`research.md:16`), o que torna a lacuna de verificação ainda mais isolada.

Trata-se da única lacuna identificada que pode admitir uma sessão não autorizada, e por isso impede que "Users & Roles (OIDC)" seja reivindicado com evidência executável (`research.md:24`; `research.md:26`). A ressalva normativa de que a validação do servidor TLS pode substituir a assinatura em tokens diretos do endpoint não se aplica ao perfil padrão, cujo emissor é local; `iss`, `aud`, `exp` e `nonce` permanecem requisitos incondicionais (`research.md:29-31`).

Há também uma contradição de escopo do marco: `README.md` e a constituição nomeiam Meilisearch, PGMQ, Valkey e RustFS como "Seed", enquanto a ordem de autoridade do próprio repositório torna normativos código, `Cargo.toml`, CI e os tiers da constituição — Seed = Flora + PostgreSQL, com sistema de arquivos local, PG FTS e notificações in-app (`intake.md:17-19`; `research.md:7-9`). Essa divergência obscurece o que significa "Seed v0.1 verificado".

## Affected Users & Stakeholders

- **Users**: usuários cujas sessões dependem do fluxo OIDC e administradores que configuram o provedor de identidade; potenciais vítimas de acesso indevido caso um token manipulado seja aceito. [ASSUMPTION: perfis inferidos de `research.md:13-16` e `research.md:24`; não há relato de incidente registrado.]
- **Stakeholders**: mantenedores da autenticação, operadores do perfil Seed, responsáveis pela reivindicação do marco "First Milestone: Flora Seed v0.1" e equipe de contratos de segurança.

## Goals

- Fazer com que uma sessão só seja estabelecida a partir de um token cuja integridade e claims obrigatórias foram verificadas (`research.md:13-15`).
- Tornar a reivindicação "Users & Roles (OIDC)" baseada em evidência executável, com verificação determinística sem provedor de identidade ao vivo (`research.md:21`; `decision.md:14`).
- Manter a verificação dentro das restrições registradas: nenhum serviço obrigatório novo e compatibilidade com a infraestrutura de sessão existente (`intake.md:33-35`; `research.md:16`).

## Non-Goals

- Fora deste enquadramento: aplicação da FASE 0 no CI, contabilização honesta de OTel, ADR de decisões do Seed e reconciliação ampla de documentação, mudança de transporte de notificações, RustFS/S3, reativação do Meilisearch, e2e de browser/Tauri, testes com provedor real e exercícios de rotação de JWKS (`decision.md:24-25`).
- Também não compõem o problema a criação de novos serviços obrigatórios, alterações no frontend e interferência no trabalho paralelo em `/metrics` (`decision.md:26`).

## Success Metrics

- Um token manipulado ou com `iss`, `aud`, `exp` ou `nonce` inválidos não estabelece sessão (baseline: nenhuma verificação; token decodificado e confiado — `research.md:13-15`).
- A identidade da sessão é derivada de `sub`, não de e-mail (baseline: identidade por e-mail — `research.md:14-15`).
- Existe evidência determinística de verificação com fixtures locais e endpoint mockado (baseline: nenhum teste negativo de token registrado — `research.md:20-21`).
- O estado documentado de "Users & Roles (OIDC)" corresponde ao comportamento executável (baseline: PARTIAL — `research.md:26`).

## Cost of Inaction

Enquanto o token for apenas decodificado, qualquer conteúdo manipulado pode ser tratado como identidade válida, deixando o sistema exposto a sessões não autorizadas. O marco Seed v0.1 permanece uma afirmação aspiracional, sem evidência que sustente a reivindicação junto de operadores e revisores. A contradição entre documentação e código continua a induzir decisões de infraestrutura incompatíveis com o perfil Seed e com a restrição de VPS barata.

## Open Questions

- [NEEDS CLARIFICATION: qual emissor e audiência devem ser considerados canônicos quando o provedor de identidade real for configurado? `research.md:30` registra o emissor padrão local, sem definir o valor de produção.]
- [NEEDS CLARIFICATION: como tratar o histórico de identidades por e-mail ao migrar para `sub`? `research.md:32` cita unicidade/backfill de `oidc_subject` como NV, sem plano registrado.]
- [NEEDS CLARIFICATION: a ressalva de validação TLS do OIDC Core §3.1.3.7 deve ser considerada em algum perfil além do padrão? `research.md:29-31` a descarta apenas para o perfil local.]
- [NEEDS CLARIFICATION: allowlist de redirect tem fonte de verdade definida? `research.md:14-15` aponta a ausência, sem indicar quem a administra.]
