# Decision: 071 — remover stub morto AuthService

**Verdict:** go (scoped)

## Decisão

Executar a remoção mecânica descrita em `shape.md`, restrita aos 11 arquivos Rust declarados, artefatos do ciclo e `STATE.md`. Nenhuma autorização adicional: código morto, verificável por compilação e gates.

## Delimitadores

- Zero mudança em handlers de auth, fluxo OIDC, migrations, workflows, gates, pisos, ADRs ou constituição.
- Manter `Error::ServiceUnavailable`, `AuthProvider` e `UserInfo`.
- Merge apenas com CI verde observada ao vivo; sem auto-merge/force-push.

## NV

CI futura, Forgejo, produção e aprovações: **[NV]**.
