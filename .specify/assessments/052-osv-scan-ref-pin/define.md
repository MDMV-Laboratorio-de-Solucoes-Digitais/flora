# Define: 052 — OSV scan ref pin

## Problema

O workflow semanal de varredura advisory referencia um ref que não existe no
action upstream. O job não resolve, portanto o sinal de vulnerabilidades de
dependências deixou de ser produzido — sem afetar os checks obrigatórios de merge.

## Requisitos funcionais

- **FR-001**: `.github/workflows/osv-scan.yml` DEVE referenciar um tag existente
  e documentado de `osv-scanner-reusable.yml`.
- **FR-002**: O pin DEVE ser `v2.7.1`, verificado por `git ls-remote` e pela
  documentação oficial do action.
- **FR-003**: Os inputs passados ao reusable workflow NÃO DEVEM mudar.
- **FR-004**: Nenhum required check, ADR, constituição ou outro workflow DEVE ser
  alterado.

## Não-objetivos

- Não promover o OSV scan a required check.
- Não alterar `fail-on-vuln`, agendamento, permissões ou Dependabot.
- Não atualizar ADR-0003 nem a constituição.

## Critérios de sucesso

- `grep -n 'reusable.yml@' .github/workflows/osv-scan.yml` → `v2.7.1`.
- `check_gate_integrity.py` → PASS (4 required checks intactos).
- Gates Rust e frontend verdes; diff de código = 1 linha.
