# Decision: 068 — refresh verificado de documentação

**Verdict:** go (scoped)

## Decisão

Executar o sweep documental descrito em `shape.md`, restrito aos paths declarados em `define.md` FR-001..FR-005. Nenhuma autorização adicional é necessária: trata-se de correção de estado verificado com evidência determinística local e de um comentário contradito pelo próprio código. A emenda da constituição permanece escalada **[NV]** por exigir aprov maintainer (Constituição §Governance).

## Delimitadores

- 5 arquivos de documentação/comentário + artefatos do ciclo.
- Zero alteração em `.github/workflows/`, migrations, ADRs, constituição, required checks, deploy, tag, segredo ou produção.
- `STATE.md` recebe apenas apêndices datados.
- Merge apenas após CI verde observada ao vivo; auto-merge e force-push proibidos.

## NV

CI futura, branch protection em mutação, Forgejo, produção e aprovações: **[NV]** — reconsultar ao vivo no Bloco 7 antes do merge.
