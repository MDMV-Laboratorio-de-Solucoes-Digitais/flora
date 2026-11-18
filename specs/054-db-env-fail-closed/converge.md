# Converge: 054 — DB env fail-closed

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-18T00:32Z — entrada 1

- Decisão R1 `go (scoped)`; assess (intake/research/define/shape/decision), spec,
  clarify, plan, checklist, tasks e analyze criados. `analyze` PASS na primeira
  execução, sem conflitos.
- Evidência inicial: 9 `return Ok(None)` em 8 módulos de contrato e 50
  `return Ok(())` — sem env, os 50 testes contam como `passed`; o guardião não
  verificava as linhas de env.
- Implementação (2 arquivos, 6 linhas adicionadas, nenhum `.rs` tocado):
  - `rust.yml`: duas expansões `: "${VAR:?…}"` dentro de `run_leg`, antes do cargo.
  - `check_gate_integrity.py`: exige que o bloco `db-tests` contenha as duas
    chaves de env; detectado e corrigido um falso casamento com a própria linha
    `:?` durante o teste de mutação (`startswith` no lugar de `in`).
- Verificações determinísticas:
  - Guardião normal → PASS (8 pisos/49).
  - Cópia do workflow em `/tmp` sem as duas env → guardião **FAIL** com os dois
    erros; remoção isolada de `DATABASE_URL` → **FAIL** com o erro correto.
  - Script `run:` extraído do step e executado sem env → **exit 1 antes do cargo**
    (0 invocações de cargo no log).
  - Mesmo script com env apontando para Postgres 18.6 isolado (`flora_054_verify`)
    → 8 legs verdes: auth 13/12, messaging 4/4, websocket 8/8, search 8/8,
    notifications 5/5, files 6/6, tasks 4/4, workspace 2/2 (50 testes).
- Gates locais (00:31Z): guardião, fmt, clippy, deny, build, test e os quatro
  gates frontend — todos exit 0.
- DB temporário `flora_054_verify` removido ao final; DB `flora` alheio intacto.
- Factory lida em `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`; nada materializado.
