# Requirements Quality Checklist — 072

> Unit tests for requirements writing: `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída.

## Completude

- [x] FR-001 define os quatro documentos e as seis contagens esperadas, com âncora explícita.
- [x] FR-002 cobre os ciclos 069/070 e o intervalo 001–070 sem reabrir a correção 137/137.
- [x] FR-003 define a invariante append-only de forma verificável.
- [x] FR-004 classifica todas as afirmações remotas como `[NV]`.
- [x] FR-005 inclui STATE.md e index.md da raiz na rastreabilidade.

## Clareza e consistência

- [x] SC-001..SC-004 usam comandos concretos (`grep`, `find`, `git diff`, guardião do alvo).
- [x] O escopo exclui explicitamente código, CI, constituição e o worktree 071.
- [x] Sem termos vagos como “atualizar quando possível”; cada nota tem data e critério.
- [x] `[NV]` não é misturado com evidência local verificada.

## Edge cases

- [x] E se outra sessão mutar o repo durante a execução? Releitura de estado antes de cada bloco e âncora registrada.
- [x] E se o apêndice já existir? Verificar antes de escrever; nada é sobrescrito.
- [x] E se `gh` falhar? PR fica **[NV]**/bloqueado; nenhum merge é feito.

## Portarias

- [x] R1 `go (scoped)` em `decision.md` antes de `specify`.
- [x] R3 zero perguntas, registradas em `clarify.md`.
- [x] R4 `validate.md` aprovado antes de `release.md`.
- [x] R7 release com SemVer, changelog por IDs e rollback.
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`.
