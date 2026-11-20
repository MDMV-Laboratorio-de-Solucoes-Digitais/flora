# Shape: 049 — Local logger

## Forma escolhida

Reescrever `logger.ts` como coletor local em memória: remover import de `ApiClient`,
`flush()`, `setInterval`, listener `beforeunload` e `sendBeacon`; manter o buffer
com teto de 200 entradas e acrescentar `snapshot()`. Atualizar o contrato 002 e
acrescentar um teste `logger.spec.ts` no projeto `server`.

## Alternativas descartadas

- Adicionar rota backend: 150–250 LOC, ingestão não autenticada e nenhum coletor real.
- Manter o POST mas engolir 404: falha silenciosa persistente e drift documental.
- Usar `localStorage`: persistência desnecessária e risco de crescimento ilimitado.

## Risco e reversão

Risco baixo: nenhuma superfície HTTP, nenhuma autoria, API pública preservada.
Reversão: `git revert` dos commits do ciclo.

## Verificação

`rg` de ausência + teste unitário + gates frontend/Rust + `check_gate_integrity.py`.
