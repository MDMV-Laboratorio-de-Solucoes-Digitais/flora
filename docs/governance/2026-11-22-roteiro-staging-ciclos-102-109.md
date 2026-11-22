# Roteiro complementar de testes — ciclos 102–109

**Data**: 2026-11-22 · **Base**: `docs/governance/2026-11-21-roteiro-staging-flora.md` (ciclos 096–101)
**Escopo**: funcionalidades implementadas após o roteiro original: UI de arquivos (105), recibo HITL (106), wildcard one-shot (107), proptests (108), Briefing de Compreensão (109) e assessment (102–104).

> Regras do roteiro original permanecem: sem valores reais de credenciais em arquivo/PR/log; registrar data/hora; `[NV]` em todo item não executado ao vivo; modelo de falha F-XXX com cenário, passo, esperado, obtido, URL, timestamp e browser/versão.

## R1 — Emissor de recibo HITL (ciclo 106, PR #352)

1. Com o segredo HMAC configurado no ambiente (via cofre, nunca no repo), emitir `VerificationReceipt` para uma decisão Tier 4 válida.
2. **Esperado**: envelope com 8 campos; `producer_system="flora"`, `status="verified"`, `station` em MAIÚSCULAS, `next_stage` minúsculo da allowlist.
3. **Esperado**: assinatura igual ao vetor canônico do validador Python (`ba563acc…` para a fixture de teste) quando usado o segredo de teste.
4. **Esperado**: nenhum segredo/`operator_id` em log, erro ou resposta. `HitlReceiptAdapter` imprime `[REDACTED]` no `Debug`.
5. **[NV]**: round-trip ao vivo com a Autônoma (`POST /receipts/flora-hitl`) depende de URL + segredo compartilhado — ver INBOX enviado à Autônoma.

## R2 — Wildcard de recuperação (ciclo 107, PR #353)

**Ordem obrigatória:** executar os passos 1–5 **antes** da remoção/rotação de `FLORA_BOOTSTRAP__ADMIN_EMAIL` solicitada à Autônoma (INBOX #133). Após a remoção, não repetir os passos 1–5 como falha: o resultado esperado passa a ser apenas 401 por ausência de opt-in. O passo 6 registra a transição e depende de confirmação ao vivo — **[NV]**.

1. Com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e senha temporária: login com `email="*"` selects apenas usuário ativo com papel `Admin`/`Owner`.
2. **Esperado**: 200 + `session_token` na primeira tentativa.
3. Repetir a mesma chamada: **esperado 401** genérico (`invalid_credentials`), sem vazamento de qual condição falhou.
4. Tentar com usuário sem papel administrativo: **esperado 401**, sem enumeração.
5. Após reinício da API com a env ainda setada: **esperado 401** (marcador persistente).
6. Confirmar que a env temporária foi removida do Coolify e rotacionada (ADR-0007, expira 2026-11-29) — **[NV]** até confirmação da SE/Autônoma.

## R3 — Arquivos (ciclo 105, PR #348)

1. Selecionar workspace e fazer upload de arquivo < 10 MiB: **esperado** 201 e arquivo listado com "Baixar".
2. Tentar upload de 11 MiB: **esperado** erro pt-BR “Arquivo muito grande. O tamanho máximo é 10 MB.” e nada gravado.
3. Clicar em "Baixar": **limitação conhecida** — `<a download>` não injeta `Authorization: Bearer`; validar download via `curl -H "Authorization: Bearer …" "$API_URL/api/v1/files/download/<id>"` → 200 e conteúdo íntegro. **[NV]** para clique autenticado no navegador.
4. Confirmar que a lista é somente da sessão (sem endpoint de listagem) — registrar como limitação, não bug.

## R4 — Qualidade e processo (ciclos 108–109)

1. `cargo test -p flora-core` (55 testes) e `PROPTEST_CASES=500 cargo test -p flora-core proptests` (4 propriedades): **esperado exit 0**.
2. `python3 scripts/ci/check_gate_integrity.py`: **esperado PASS**, 10 legs / 77 pisos.
3. Verificar que PRs em domínios críticos (auth/RBAC/tenant/bootstrap/HITL) incluem Briefing de Compreensão preenchido a partir de `.specify/templates/briefing-compreensao.md` — **prospectivo a partir do ciclo 109**; ciclos anteriores são grandfathered.

## R5 — Itens ainda [NV] do roteiro original

| Item | Bloqueio | Responsável |
|---|---|---|
| Cenário 6 (DMs com 2º usuário) | fixture do segundo usuário não entregue | Autônoma (INBOX #133) |
| Cenário 8 (upload inválido/limite em FQDN real) | staging real não entregue | Autônoma (INBOX #133) |
| Cenário 12 (reset/remoção de fixture) | comandos concretos não entregues | Autônoma (INBOX #133) |
| Round-trip HITL ao vivo | URL + segredo compartilhado | Autônoma (INBOX #133) |

## Critério de aceite

Executar R1–R5 no staging real com data/hora e evidências (curl + screenshots); falhas registradas no modelo F-XXX; `[NV]` mantido até verificação ao vivo. Nenhuma credencial real em arquivo, PR, log ou mensagem.
