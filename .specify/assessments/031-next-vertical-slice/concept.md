# Concept: Publicável sem publicar

- **Slug**: 031-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Prontidão de release sem tag

## Options

### Option A — Prontidão de release sem tag
- **Sketch**: O workspace passa a ter uma única fonte de versão, de modo que manifests e travamento não possam divergir. O repositório ganha um mecanismo de release preparado para disparar somente quando uma tag existir, com inventário de componentes rastreável e publicação em rascunho — mas nenhuma tag é criada e nada é publicado. A governança passa a registrar a contagem correta de isenções, os jobs existentes e a lane de navegador já fechada.
- **Appetite**: medium
- **Trade-offs**: Transforma "Seed verificado" em "Seed publicável" sem exigir autorização de publicação, ao custo de tocar em todos os manifests e de adicionar um fluxo com permissão de escrita latente. Sem tag, o gerador de inventário não tem execução real, apenas validação em CI.
- **Rabbit holes**: criar tag, publicar release ou empurrar tags; promover `cargo-vet` a required; abrir frente de CORS, load/restore ou OTLP; fazer dry-run do gerador e decidir imutabilidade ou `--target`; alterar proteção de branch, CODEOWNERS ou rulesets.

### Option B — Menor coisa que poderia funcionar
- **Sketch**: Centralizar a versão e corrigir os documentos de governança, sem adicionar mecanismo de release nem inventário de componentes. A fonte de versão deixa de ser ambígua e os auditores leem números corretos, mas a publicação futura continua artesanal.
- **Appetite**: small
- **Trade-offs**: É o corte mais seguro e resolve a divergência de versão e a documentação; deixa porém a rastreabilidade de componentes sem mecanismo, que é justamente o que separa "verificado" de "publicável".
- **Rabbit holes**: tratar a versão centralizada como se o release estivesse pronto; acumular correções documentais sem data de validade.

### Option C — Não fazer: comprar pipeline de release externa
- **Sketch**: Não preparar release no repositório: contratar pipeline gerenciada que gere versão, tags e inventário quando a publicação for autorizada.
- **Appetite**: large (incerto: sem evidência de custo, autorização ou integração)
- **Trade-offs**: Evita tocar em oito manifests e em fluxos com permissão de escrita, mas cria dependência externa para um ato que ainda não foi autorizado e não corrige a documentação do repositório.
- **Rabbit holes**: chaves e contratos de fornecedor, política de imutabilidade de terceiros, sincronização entre versão externa e manifests internos.

## Recommendation

Recomendo a **Option A**: as métricas pedem versão centralizada nos oito manifests com travamento consistente, mecanismo tag-only presente com zero tags criadas, ADR com 455 isenções e lane e2e resolvida, `AGENTS.md` listando o job e CI verde sem publicação. A Option B entrega metade; a Option C introduz dependência externa sem autorização.

## Out of Scope (for the recommended option)

- Não criar tag, não publicar release e não empurrar tags; capacidade de release permanece latente.
- Não entregar allowlist CORS, promoção de `cargo-vet` a required, load/restore, OTLP, dry-run do gerador de SBOM ou decisão de `--target`.
- Não alterar proteção de branch, CODEOWNERS ou rulesets sem autorização.
- Não afirmar imutabilidade de release sem que exista release publicada.

## Assumptions to Validate

- Quem detém a autorização explícita para criar a primeira tag e publicar um release.
- Que o uso de chaves pontuais de versão na raiz evita duplicação de chave, conforme verificado localmente.
- Que os oito manifests e a contagem de 455 isenções refletem o estado atual; número e contagem podem ter mudado.
- Que adicionar um fluxo disparado por tag é seguro enquanto não existir tag.
- Se dry-run do gerador é necessário antes da primeira tag ou se basta validação do artefato em CI.
- Quem valida que `ROADMAP`, `OBJETIVO` e os registros de pesquisa do ciclo anterior não reintroduzem números obsoletos.
- Resultado de CI remoto e Forgejo [NV].
