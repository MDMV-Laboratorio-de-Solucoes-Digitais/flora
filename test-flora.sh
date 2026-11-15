#!/usr/bin/env bash
set -euo pipefail

# Flora API smoke script (updated for JWT/Bearer auth, cycle 007)
# Usage:
#   chmod +x test-flora.sh
#   FLORA_BASE_URL=http://localhost:3000 FLORA_TOKEN=<jwt> ORG_ID=<uuid> ./test-flora.sh
#
# Notes:
# - Tenant-scoped routes require a Bearer JWT with a concrete organization_id.
# - Obtain an initial token via OIDC: GET $FLORA_BASE_URL/api/v1/auth/login
#   (complete callback flow) then select org: POST $FLORA_BASE_URL/api/v1/auth/select-organization
#   with {"organization_id":"<uuid>"} and Authorization: Bearer <initial_token>.
# - OIDC/DB-integrated end-to-end execution is NV without configured provider and seeded data.

BASE_URL="${FLORA_BASE_URL:-http://localhost:3000}"
TOKEN="${FLORA_TOKEN:-}"
ORG_ID="${ORG_ID:-11111111-1111-1111-1111-111111111111}"
WORKSPACE_ID="${WORKSPACE_ID:-88888888-8888-8888-8888-888888888888}"

auth_header=()
if [[ -n "$TOKEN" ]]; then
  auth_header=(-H "Authorization: Bearer $TOKEN")
fi

echo "Flora API smoke (base: $BASE_URL)"

echo "\n== Health checks =="
curl -fsS "$BASE_URL/health" | jq -r '.status? // "ok"'
curl -fsS "$BASE_URL/health/ready" | jq -r '.status? // "ok"'
curl -fsS "$BASE_URL/health/live" | jq -r '.status? // "ok"'

echo "\n== Auth login (OIDC authorize URL) =="
curl -fsS "$BASE_URL/api/v1/auth/login" -H "Accept: application/json" | jq '.authorization_url, .next_step'

if [[ -z "$TOKEN" ]]; then
  echo "\nFLORA_TOKEN not set; skipping tenant-scoped calls."
  echo "After OIDC callback, select org to get org-scoped token:"
  echo "  curl -fsS -X POST '$BASE_URL/api/v1/auth/select-organization' \\
    -H 'Authorization: Bearer <initial_token>' -H 'Content-Type: application/json' \\
    -d '{\"organization_id\":\"$ORG_ID\"}' | jq"
  echo "\nDone (partial)."
  exit 0
fi

echo "\n== Select organization (rotate to org-scoped session) =="
SELECT_RESP=$(curl -fsS -X POST "$BASE_URL/api/v1/auth/select-organization" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "{\"organization_id\":\"$ORG_ID\"}")
echo "$SELECT_RESP" | jq '{user_id, organization_id, expires_in}'
SCOPED_TOKEN=$(echo "$SELECT_RESP" | jq -r '.session_token // empty')
if [[ -z "$SCOPED_TOKEN" ]]; then
  echo "Failed to obtain org-scoped token" >&2
  exit 1
fi
AUTH=(-H "Authorization: Bearer $SCOPED_TOKEN")

echo "\n== List workspaces =="
curl -fsS "${AUTH[@]}" "$BASE_URL/api/v1/workspaces" | jq '.[].name'

echo "\n== List tasks (assignee-scoped in org) =="
curl -fsS "${AUTH[@]}" "$BASE_URL/api/v1/tasks" | jq '[.[] | {id,title,status}]'

echo "\n== Create task =="
NEW_TASK=$(curl -fsS -X POST "$BASE_URL/api/v1/tasks" \
  -H "Content-Type: application/json" "${AUTH[@]}" \
  -d "{\"title\":\"Smoke test task\",\"description\":\"Created by test-flora.sh\",\"workspace_id\":\"$WORKSPACE_ID\"}")
echo "$NEW_TASK" | jq '{id,title,status}'

echo "\n== List tasks (after create) =="
curl -fsS "${AUTH[@]}" "$BASE_URL/api/v1/tasks" | jq '[.[] | {id,title,status}]'

echo "\nSmoke completed."
