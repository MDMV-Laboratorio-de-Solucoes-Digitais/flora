# Clarify: 059 — DB tests fail locally

**Data:** 2026-11-18T03:26Z

## Resoluções

1. **Por que não editar os 50 chamadores?** O `?` já propaga o `Err` do helper;
   os `else` inalcançáveis são inofensivos e manteriam o diff pequeno.
2. **Mensagem de erro:** `anyhow::bail!` com nome das duas env, substituindo
   o `eprintln!("skipping …")` enganoso.
3. **Muda a assinatura?** Não: `Result<Option<...>>` permanece.
4. **CI muda?** Não: já fail-closed desde 054; este ciclo é apenas local.
5. **Identidade 059?** `feature.json` aponta 051; `--dry-run` → 059.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
