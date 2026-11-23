# Checklist: 116 — identidade real de autores no chat

- [x] Todos os `FR-001`..`FR-005` mapeados a cenários verificáveis `SC-001`..`SC-005`
- [x] Critério 80/20 registrado em `.specify/memory/STATE.md` (`240/625`)
- [x] Lookup de autores restringido exclusivamente a `sender_id`s de canal já autorizado pela `org_id` da sessão (sem diretório irrestrito)
- [x] Piso `messaging` planejado em lockstep (`4 -> 5`, total `79`) em `check_gate_integrity.py` e `.github/workflows/rust.yml`
- [x] Sem clarificações pendentes (R3 = 0)
