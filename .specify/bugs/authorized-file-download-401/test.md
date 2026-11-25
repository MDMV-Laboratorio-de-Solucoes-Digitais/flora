# Bug test — authorized-file-download-401

**Data**: 2026-11-25 · **Ciclo**: 123

## Testes

1. **Unitário (Vitest `server`)**: `tests/unit/filesDownload.test.ts` stuba `globalThis.fetch` e afirma que `ApiClient.downloadBlob('/api/v1/files/download/<id>')` envia `Authorization: Bearer <token>`, retorna o `Blob` em 200 e lança `ApiError` com `status=401` após `authState.lockSession()`.
2. **E2E (Playwright/Chromium)**: estender Passo 5b de `seed-flow.e2e.ts`: após upload e reload, capturar `page.waitForResponse` para `/api/v1/files/download/`, verificar `response.status() === 200` e que `response.request().headers()['authorization']` começa com `Bearer`; capturar `page.waitForEvent('download')` e confirmar `suggestedFilename() === uploadedFileName`.
3. **Gate**: `python3 scripts/ci/check_gate_integrity.py` inalterado (87 pisos) e `bun run check`/`bun run lint` sem erros.

## Comandos

```bash
cd apps/frontend && bun run test:unit -- --run --project server tests/unit/filesDownload.test.ts
cd apps/frontend && bun run test:e2e
cd apps/frontend && bun run check && bun run lint
python3 scripts/ci/check_gate_integrity.py
```

## Critério

Critério local parcial: check/lint/unit/gate PASS; E2E `evidence_status: blocked` após três falhas de provisionamento, sem assertion observada. Required checks remotos são a autoridade de merge; automação de provisionamento E2E registrada como próximo passo ao SWE. Evidência em `.specify/validations/123-authorized-file-download/evidence.md`.
