# Define: 107 — wildcard one-shot

## Problema

O opt-in `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` não exige papel administrativo e não se desliga após o primeiro login, permitindo escalada e janela indefinida.

## Escopo

FR-001 papel Admin/Owner built-in; FR-002 consumo atômico one-shot; FR-003 startup não reaplica hash após consumo; FR-004 login falha fechado com 401 genérico; FR-005 ADR com expiração e testes sem mudar pisos.

## Fora de escopo

Mudança de gate/required check, formato de sessão, rota de login, credenciais reais, deploy, outros ciclos do INBOX.
