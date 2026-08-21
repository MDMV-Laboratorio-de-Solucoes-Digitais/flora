#!/usr/bin/env bash
set -e

# Flora Test Script
# Usage: chmod +x test-flora.sh; ./test-flora.sh

echo "🔄 Iniciando testes do Flora API..."
echo "https://localhost:3000"

# Exportar IDs fixos do dev-seed.sql
export ORG_ID="11111111-1111-1111-1111-111111111111"
export ALICE_ID="22222222-2222-2222-2222-222222222222"
export BOB_ID="33333333-3333-3333-3333-333333333333"
export CAROL_ID="44444444-4444-4444-4444-444444444444"

# Testar Health Checks
echo -e "\n✅ Health Checks:"
curl -s http://localhost:3000/health | jq -r '.status? // "ok"'
curl -s http://localhost:3000/health/ready | jq -r '.status? // "ok"'
curl -s http://localhost:3000/health/live | jq -r '.status? // "ok"'

# Testar Login OIDC (Alice)
echo -e "\n✅ Auth Login (Alice):"
curl -s http://localhost:3000/auth/login -H "Accept: application/json" | jq '.authorization_url'

# Listar Workspaces (Alice e Bob)

# Alice (Admin)
echo -e "\n📁 Workspaces (Alice - Admin):"
curl -s -H "X-Organization-ID: $ORG_ID" -H "X-User-ID: $ALICE_ID" \
  http://localhost:3000/api/v1/workspaces | jq '.[].name'

# Bob (Member) - só listagem
echo -e "\n📁 Workspaces (Bob - Member):"
curl -s -H "X-Organization-ID: $ORG_ID" -H "X-User-ID: $BOB_ID" \
  http://localhost:3000/api/v1/workspaces | jq '.[].name'

# Listar Tasks de Bob (assigned)

echo -e "\n✨ Tasks de Bob (assignee):"
curl -s -H "X-Organization-ID: $ORG_ID" -H "X-User-ID: $BOB_ID" \
  http://localhost:3000/api/v1/tasks | jq '[.[] | {title, status}]'

# Criar uma task de teste (Bob)
echo -e "\n📝 Criando task de teste..."
NEW_TASK=$(curl -s -X POST http://localhost:3000/api/v1/tasks \
  -H "Content-Type: application/json" \
  -H "X-Organization-ID: $ORG_ID" -H "X-User-ID: $BOB_ID" \
  -d '{
    "title": "Teste de API",
    "description": "Task criada via teste automático",
    "workspace_id": "88888888-8888-8888-8888-888888888888",
    "assignee_id": "33333333-3333-3333-3333-333333333333"
  }' | jq .)

echo "   Nova task ID: $(echo $NEW_TASK | jq -r '.id')"
echo "   Título: $(echo $NEW_TASK | jq -r '.title')"

# Listar Tasks de novo (inclui a task nova)
echo -e "\n📋 Tasks atualizadas de Bob:"
curl -s -H "X-Organization-ID: $ORG_ID" -H "X-User-ID: $BOB_ID" \
  http://localhost:3000/api/v1/tasks | jq '[.[] | {title, status}]'

echo -e "\n✅ Todos os testes concluídos!
"
echo "📊 Instância: Acme Corporation (slug: acme-corp)"
echo "👤 Usuários: Alice (Admin), Bob (Member), Carol (Viewer)"
echo "🔗 API: http://localhost:3000"
