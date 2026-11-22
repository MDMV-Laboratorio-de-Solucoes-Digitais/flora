# Validação R4 — ciclo 106 (recibo HITL)

**Data**: 2026-11-22 · **Branch**: `feature/106-hitl-receipt-port` @ `0582883da`

## Veredito: APROVADO

| Comando | Resultado (saída observada) |
|---|---|
| `cargo check -p flora-api` | exit 0 |
| `cargo test -p flora-api hitl_receipt` | `2 passed, 38 filtered out`, exit 0 |
| `cargo fmt --all -- --check` | exit 0 (após formatar o atributo `#[expect]`) |
| `cargo clippy -p flora-api --all-targets -- -D warnings` | exit 0 |
| `python3 scripts/ci/check_gate_integrity.py` | PASS, `exit=0`, 10 legs / 77 pisos |

### Critérios

- **SC-001** envelope aceito pelo contrato Python (vetor + revalidação local) — **PASS**.
- **SC-002** entradas inválidas rejeitadas antes de assinar; sem segredo/PII em log — **PASS**.
- **SC-003** gate inalterado e nenhuma credencial real versionada — **PASS**.

### [NV]

Round-trip ao vivo, transporte, staging/produção e credenciais reais: **[NV]**.

## Conclusão

**R4 APROVADO.** Segue para R7 release.
