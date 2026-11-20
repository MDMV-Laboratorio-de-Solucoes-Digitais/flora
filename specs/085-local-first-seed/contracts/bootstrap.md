# Contract: `POST /api/v1/bootstrap`

**Feature**: 085-local-first-seed | **Date**: 2026-11-20T01:33:00Z

Rota pública, one-time, usada exclusivamente pelo botão "Sprout Workspace".

## Request

```http
POST /api/v1/bootstrap HTTP/1.1
Content-Type: application/json

{
  "organization_name": "Minha Organização",
  "admin_email": "admin@minhaorg.test"
}
```

| Campo | Tipo | Regra |
|---|---|---|
| `organization_name` | string | 1–255 caracteres após trim |
| `admin_email` | string | e-mail válido; normalizado para lowercase |

Nenhum cabeçalho `Authorization` é necessário ou usado.

## Responses

### 201 Created

```json
{
  "session_token": "<JWT com sub=user_id, jti=<id da sessao>, organization_id e email>",
  "expires_in": 3600,
  "user_id": "<uuid>",
  "organization_id": "<uuid>",
  "workspace_id": "<uuid>"
}
```

O cliente deve persistir `session_token` como sessão autenticada com `organizationId` preenchido e navegar para `/tasks`.

### 409 Conflict

```json
{ "error": "conflict", "message": "duplicate entry: bootstrap already completed" }
```

Ocorre quando já existe organização não-sistema, quando o e-mail já está em uso ou sob concorrência. Nenhum objeto é criado.

### 422 Unprocessable Entity

```json
{ "error": "validation_error", "field": "organization_name", "message": "..." }
```

### 429 Too Many Requests

Sem corpo (middleware de rate limit); limite 5 requisições / 60 s por IP.

### 500 Internal Server Error

Falha de infraestrutura; transação revertida.

## Non-Functional

- **Atomicidade**: todas as gravações em uma transação; rollback em qualquer erro.
- **Concorrência**: `pg_advisory_xact_lock` garante no máximo uma inicialização.
- **Auditoria**: log com `organization_id`, `user_id` e horário; nunca token ou segredo.
- **Verificação de e-mail**: não realizada, por decisão documentada em FR-012.
- **Compatibilidade**: não altera `/api/v1/auth/login`, `/callback`, `/refresh` ou `/select-organization`.
