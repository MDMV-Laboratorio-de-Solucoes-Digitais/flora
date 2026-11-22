# ADR-0008 — Zitadel/passkey: deferimento no Flora e quatro gates de reentrada

**Data**: 2026-11-22 · **Status**: Aceito (deferimento) · **Origem**: item MINOR do INBOX autorizado pelo SE

## Contexto

O Flora já usa OIDC PKCE S256 com `state`/`nonce` server-side, validação de ID token via JWKS e sessões em DB com `jti` revogável. A autenticação de **máquina** e o não-repúdio de decisões humanas não estão implementados. O plano multiagente propõe service accounts Zitadel (RFC 7523) e passkeys/WebAuthn; não há provedor configurado no Seed e não há evidência de que o Zitadel self-hosted esteja disponível no ambiente do projeto.

## Decisão

**DEFER**. Manter PKCE + sessão DB como autoridade no Seed. Não prometer não-repúdio, assinatura de operador ou identidade de agente sem WebAuthn verificada. Nenhuma dependência, endpoint ou credencial nova é adicionada por este ADR.

## Quatro gates de reentrada

1. **Provedor**: Zitadel self-hosted com descoberta OIDC, JWKS e issuer/audience validados em runtime; `[NV]` até configuração real.
2. **Humanos**: registro e verificação de passkey/WebAuthn (attestation, origin, `userVerification`) no fluxo de login — somente então é possível falar em não-repúdio.
3. **Máquinas**: service accounts com Private Key JWT (RFC 7523) e `aud` igual ao issuer identifier do authorization server, conforme RFC 10070 (nov/2026); segredos em `.tokens/` 0o600, nunca no LLM.
4. **Auditoria**: recibo HITL emitido pelo Flora (`HitlReceiptPort`, ciclo 106) com aprovação ligada ao principal WebAuthn verificado, `jti`, `exp`, hash do payload e trilha append-only — antes disso, o recibo prova autenticidade de origem, não não-repúdio humano.

## Consequências

RBAC humano permanece; `cedar-policy` para máquinas é tratado no assessment como ADAPT futuro, não como dependência deste ADR. Rollback é simples: nenhuma mudança de código foi feita.

## [NV]

Disponibilidade do Zitadel, registro de passkeys, RFC 10070 no deployment específico e execução em staging/produção.
