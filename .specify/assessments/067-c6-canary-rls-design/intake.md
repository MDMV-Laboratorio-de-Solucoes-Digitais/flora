# Intake: 067 — C6 canário RLS (desenho, sem produção)

**Data:** 2026-11-18T15:50Z · **Âncora:** `github/dev@9d5f6656f` · branch `feature/067-c6-canary-rls-design`

## Sinal

C6 é o último item autorizado da tabela de escalonamento (`STATE.md:354,369`).
A auditoria local confirmou: `20240818000001_init_schema.sql:196-215` aplica
`ENABLE ROW LEVEL SECURITY` em 11 tabelas, mas **0 `CREATE POLICY` ativos**,
**0 `set_config`**, **0 papéis de DB (`flora_app`/`flora_owner`)** e
**0 wrapper de transação** — ADR-0005 admite "Enforcement EVOLUTIVE — not
enabled"; ADR-0006 exige promoção em duas etapas, ainda não implementada.

## Autorização

SE autorizou C6 em `STATE.md` (13:04Z, 14:38Z, 14:50Z). Escopo literal:
**desenho de canário, sem produção**. Deploy, credenciais, required checks e
tag/release continuam exigindo autorização explícita do SE (ADR-0006 §3).

## Evidência

- Local: 2 subagentes somente-leitura (realidade do código + lacunas/dívidas),
  ADR-0005, ADR-0006, migração init, `check_gate_integrity.py`.
- Remoto: PR #285 merged, 6 checks verdes — observado ao vivo nesta sessão.
- Factory como fonte: `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3` (HEAD lido,
  nada transplantado como autoridade).

## 80/20

`(I×C)/(E×R)` = (5×5)/(3×1) = **8.33**, o maior candidato. CI verde ⇒
"elevar a CI" não é o nº 1 (regra do BLOCO 5).

## Fronteiras

Documento + artefatos SDD + refresh de docs de estado. **Sem** código,
workflows, migrações, ADRs, constituição, required checks, policies/FORCE em
runtime ou acesso a produção. Nenhuma credencial copiada.
