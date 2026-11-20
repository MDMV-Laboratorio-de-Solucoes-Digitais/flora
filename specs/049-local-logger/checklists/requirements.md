# Checklist: 049 — Qualidade dos requisitos

**Data:** 2026-11-17T15:06Z · `[x]` = revisão de qualidade, não implementação.

- [x] FR-001 — verificável por `rg` e espia de `fetch`.
- [x] FR-002 — assinaturas comparáveis com o arquivo atual.
- [x] FR-003 — teto 200 mensurável por teste.
- [x] FR-004 — `snapshot()` com cópia defensiva testável.
- [x] FR-005 — contrato 002 editável com diff explícito.
- [x] SC-001 — `rg` sem o caminho do endpoint no frontend.
- [x] SC-002 — teste unitário no projeto `server`.
- [x] SC-003 — gates frontend/Rust e gate autoritativo verdes.
- [x] Não-objetivos e NV declarados.
- [x] Call sites inventariados (8 arquivos); nenhum chamador externo de `flush()`.

## Revisão do checklist

FR 5/5, SC 3/3 com critérios verificáveis; sem ambiguidades. **Aprovado para `tasks`.**
