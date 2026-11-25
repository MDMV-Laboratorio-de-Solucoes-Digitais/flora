# Concept: FASE 0 com fecho executável

- **Slug**: 028-next-vertical-slice
- **Created**: 2026-11-25
- **Recommended option**: Fecho não-bloqueante da FASE 0

## Options

### Option A — Fecho não-bloqueante da FASE 0
- **Sketch**: A verificação de dependências passa a existir em checkout limpo e é versionada, sem impedir merges; sinais independentes de advisories surgem na revisão de mudanças e em uma varredura semanal. Quem tenta reproduzir o smoke encontra um procedimento de emissão de credencial que funciona, em vez de um segredo vencido. As contradições residuais entre ADR, README e registros de pesquisa desaparecem.
- **Appetite**: medium
- **Trade-offs**: Fecha o último item literal do marco que ainda não é machine-enforced e dá critério comum para triar os alertas, ao custo de mais sinais informativos no dia a dia e de um procedimento de emissão que precisa de segredo bem gerenciado. Nada aqui passa a bloquear merge, por decisão explícita.
- **Rabbit holes**: promover o gate a bloqueante com CODEOWNERS antes da janela de observação; auditar massivamente o histórico de dependências; duplicar o gate RustSec já existente; criar tag/release para produzir SBOM; insistir em executar o smoke nas portas ocupadas.

### Option B — Menor fecho que poderia funcionar
- **Sketch**: Versionar a evidência de dependências com a única isenção necessária e substituir a credencial vencida do smoke por emissão determinística; não acrescentar jobs de advisory nem tocar nos documentos.
- **Appetite**: small
- **Trade-offs**: Resolve a falha literal da FASE 0 e devolve a qualquer operador a capacidade de reproduzir o smoke; deixa porém os alertas sem triagem comum e as contradições documentais intactas.
- **Rabbit holes**: chamar o procedimento de "verificação executada" sem tê-lo executado; esquecer que emissão determinística não substitui execução em ambiente isolado.

### Option C — Não fazer: terceirizar e aceitar segredo estático
- **Sketch**: Não versionar nada: continuar confiando apenas nos alertas já ativos da plataforma e trocar a credencial vencida por outra de longa validade.
- **Appetite**: small (incerto: nenhuma evidência de que isso satisfaça a auditoria)
- **Trade-offs**: Custo imediato quase zero, mas nenhuma evidência versionada em checkout limpo, nenhuma revisão de dependências em mudanças e um segredo de longa duração que é ele próprio um risco.
- **Rabbit holes**: renovação periódica manual de credenciais, confiança cega em alertas não triados, dependência de recursos pagos da plataforma.

## Recommendation

Recomendo a **Option A**: as métricas exigem store versionado e checagem com `--locked`, revisão de dependências em PR e varredura OSV semanal ambas não-bloqueantes, smoke sem JWT expirado e ADR/README/registros sem contradição. A Option B cobre apenas metade; a Option C contradiz a exigência de evidência reproduzível e cria um segredo durável.

## Out of Scope (for the recommended option)

- E2E de browser com minter feature-gated; SBOM de release (bloqueado por ausência de tag); ADR do Tauri.
- Gate bloqueante do `cargo-vet` com CODEOWNERS; exporter OTLP; validação de carga/restauração em 2vCPU/4GB.
- Clippy estrito com `--all-targets`.
- Novos serviços obrigatórios, reversão de trabalho paralelo e checks que bloqueiem merges.

## Assumptions to Validate

- Que uma única isenção registrada é suficiente para a checagem versionada passar com `--locked`.
- Que o repositório ser público dispensa recursos pagos de segurança e que os alertas já ativos são sinal suficiente para começar.
- Que advisory e revisão de dependências devem ser não-bloqueantes porque não existem required checks hoje.
- Que a execução local do smoke fica como NV devido às portas 3000/5432 ocupadas, mesmo com runbook disponível.
- Após qual janela de observação o gate poderia tornar-se bloqueante, e se a desativação do license-check na revisão é aceitável como política.
- Critério de priorização dos alertas existentes, política de required checks e porta isolada para tentativa futura de smoke.
