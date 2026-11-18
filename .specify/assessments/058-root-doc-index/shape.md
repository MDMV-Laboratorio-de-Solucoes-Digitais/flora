# Shape: 058 — Root doc index

## Forma escolhida

`index.md` com cabeçalho datado, uma frase de escopo, lista agrupada
("Direção", "Estado verificado", "Referência") com links relativos e uma nota
final `[NV]`. Sem tabelas de contagem (já em `PROGRESSO.md`).

## Alternativas descartadas

- README como índice: mudaria a apresentação do projeto.
- Índice exaustivo de specs/assessments: detalhe demais para a raiz.
- Conteúdo duplicado: drift garantido.
- ADR drift no mesmo PR: governança, aprovação pendente.

## Risco e reversão

Risco baixo: arquivo novo, nenhum código/gate. Reversão: `git revert` do commit.
Nenhuma credencial envolvida.

## Verificação

Validação de links por Python, contagem de linhas, verificador isolado, gates.
