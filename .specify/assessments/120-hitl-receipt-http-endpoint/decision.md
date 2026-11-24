# Decision — ciclo 120 (`120-hitl-receipt-http-endpoint`)

**Data**: 2026-11-24 · **Veredito: go**

## Pontuação 80/20 (Bloco 5, denominador `3125`)

- **Candidato escolhido (`IMPLEMENTAÇÃO`, fecha `BF-03`)**: `impacto (5) × confiança (5) × fechamento (5) × esforço⁻¹ (6−2=4) × risco⁻¹ (6−2=4)` = **`2000/3125`**.
- **Comparativo**:
  - `BF-04` (seletor interativo de tema em `/settings`): `4 × 5 × 3 × (6 − 1) × (6 − 1)` = `1500/3125`.
  - `BF-02` remanescente (UI de convites e atribuição de papéis a membros): `4 × 4 × 4 × (6 − 4) × (6 − 3)` = `384/3125`.
- **Justificativa**: `BF-03` completa a ponte HTTP/RBAC para `HitlReceiptAdapter` sem alterar schema SQL nem lockfile, fecha integralmente um item de backlog com consumidor real (`flora-api` + `MDMV-Autonoma/src/contracts/flora_bridge.py` + `db-tests`), reforça invariantes de servidor (`operator_id` via sessão e `503` sanitizado) e mantém entrega focada (~250–350 LOC de código + testes).
