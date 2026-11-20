# Define: 049 — Local logger

## Problema

O logger frontend promete envio ao backend, mas o endpoint não existe: cada flush
gera um 404 silencioso e descarta entradas. Há ainda drift documental no contrato 002.

## Requisitos funcionais

- **FR-001**: O logger NÃO DEVE emitir requisições de rede (`fetch`/`sendBeacon`).
- **FR-002**: `debug/info/warn/error` DEVEM continuar com a mesma assinatura.
- **FR-003**: O buffer DEVE ser limitado (no máximo 200 entradas) para evitar crescimento ilimitado.
- **FR-004**: `snapshot(): readonly LogEntry[]` DEVE expor uma cópia para diagnóstico local.
- **FR-005**: O contrato em `specs/002-.../contracts/logger-contract.md` DEVE refletir o comportamento local.

## Não-objetivos

- Não criar rota backend, não adicionar OTel/exporter.
- Não introduzir `console.log` nem quebrar a regra no-console.
- Não alterar os 8 call sites, constituição, ADRs ou PR #262.

## Critérios de sucesso

- `rg '/api/v1/telemetry/logs' apps/frontend/src` → 0 ocorrências.
- Teste unitário no projeto `server` prova ausência de `fetch` e limite do buffer.
- Gates frontend (check/lint/test/build) e Rust permanecem verdes.
