# Quickstart: validar o bootstrap local

**Feature**: 085-local-first-seed | **Date**: 2026-11-20T01:33:00Z

## Pré-requisitos

- PostgreSQL local em `127.0.0.1:5432` com usuário `postgres`.
- Rust 1.98.1 e Bun/Node para o frontend.
- Branch `feature/085-local-first-seed` checkoutada.

## 1. Banco descartável

```bash
export PGPASSWORD=postgres
createdb -h 127.0.0.1 -U postgres flora_demo_085
```

## 2. API

```bash
FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:5432/flora_demo_085' \
FLORA_APP__JWT_SECRET='demo-secret-32-chars-minimum-for-flora' \
cargo run -q -p flora-api
```

Migrations são aplicadas no boot. Confirmar `curl -s http://127.0.0.1:3000/health`.

## 3. Frontend

```bash
cd apps/frontend && bun run dev -- --host 127.0.0.1 --port 5173
```

## 4. Fluxo esperado

1. Abrir `http://127.0.0.1:5173`.
2. Clicar em **Start First Seed Journey**.
3. Preencher nome e e-mail e clicar em **Sprout Workspace**.
4. Resultado: navegação para `/tasks`, workspace `Main Workspace` selecionado e organização já escopada — sem sessão injetada e sem Zitadel.
5. Nova tentativa via API:

```bash
curl -s -i -X POST http://127.0.0.1:3000/api/v1/bootstrap \
  -H 'content-type: application/json' \
  -d '{"organization_name":"Outra Org","admin_email":"outro@teste.test"}' | head -12
```

Esperado: `409 Conflict`.

## 5. Testes automatizados

```bash
FLORA_DATABASE__POSTGRES_URL='postgresql://postgres:postgres@127.0.0.1:5432/flora_demo_085' \
  cargo test -p flora-tests db_bootstrap -- --nocapture
cd apps/frontend && bunx vitest run --project client 'src/routes/(auth)/login-error.svelte.spec.ts'
```

Referências: [contracts/bootstrap.md](./contracts/bootstrap.md) e [data-model.md](./data-model.md).
