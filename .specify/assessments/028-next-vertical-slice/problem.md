# Problem Definition: FASE 0 e verificação do Smoke sem fecho executável

- **Slug**: 028-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

A auditoria pós-027 considera os gates do Seed VERIFICADOS, com duas exceções: o smoke integrado não pode ser executado porque o JWT estático do `docs/dev-smoke.md` expirou em 2026-11-16, e resta desvio documental (`intake.md:8`; `research.md:7`). O repositório é público e o Dependabot já está ativo — 83 alertas, 4 no `Cargo.lock` raiz —, mas não há branches protegidos nem processo de release/tag (`intake.md:8`).

O último item literal do marco FASE 0 segue sem aplicação machine-enforced: o store do `cargo-vet` está ignorado pelo Git, então um checkout limpo não tem gate, e a única isenção necessária para `cargo vet --locked --no-minimize-exemptions` — `multer 3.1.0` — ainda não está registrada (`research.md:3`). Também não há oráculo independente de advisories: a verificação depende de sinais já existentes, e um job informativo de varredura semanal ainda não foi estabelecido (`research.md:4`). Como não existem required checks hoje, qualquer reforço precisa ser não-bloqueante por padrão (`research.md:4`).

Há, por fim, contradições residuais: o ADR-0001 continua descrevendo OTel como inutilizável, README está desatualizado e os snapshots de pesquisa do 027 não trazem banner de superseeding após a anexação do `TraceLayer` (`research.md:7`). O problema é a ausência de evidência reproduzível e de sinais independentes, não a falta das capacidades em si.

## Affected Users & Stakeholders

- **Users**: desenvolvedores que tentam reproduzir o smoke a partir da documentação e revisores que precisam confiar na triagem de dependências. [ASSUMPTION: perfis inferidos de `intake.md:8` e `research.md:3-4`; sem entrevistas registradas.]
- **Stakeholders**: mantenedores da FASE 0 e do ADR-0001, equipe de supply-chain/CI, responsáveis pela documentação de dev-smoke e pelos alertas do Dependabot.

## Goals

- Tornar a evidência do `cargo-vet` versionada e reproduzível em checkout limpo, sem impor bloqueio de merge (`research.md:3`).
- Estabelecer sinais de advisory independentes e não-bloqueantes, aproveitando o fato de o repositório ser público (`research.md:4`).
- Substituir credencial estática expirada por procedimento de emissão determinístico, para que o smoke possa ser verificado por qualquer operador (`decision.md:11`).
- Fechar as contradições residuais entre ADR, README e snapshots de pesquisa (`research.md:7`).

## Non-Goals

- Fora do enquadramento: e2e de browser com minter feature-gated, SBOM de release (bloqueado por ausência de tag), ADR do Tauri, gate bloqueante do `cargo-vet` com CODEOWNERS, exporter OTLP, validação de carga/restauração em 2vCPU/4GB e clippy estrito com `--all-targets` (`decision.md:16`).
- Também não compõem o problema novos serviços obrigatórios, reversão de trabalho paralelo e checks que bloqueiem merges (`intake.md:16-17`). A execução local do smoke não é pré-requisito deste enquadramento quando as portas 3000/5432 estão ocupadas (`research.md:7`).

## Success Metrics

- O store do `cargo-vet` está versionado e a checagem com `--locked` passa com a isenção registrada (baseline: store ignorado e `multer` sem isenção — `research.md:3`).
- Existe revisão de dependências em PR e varredura OSV semanal, ambas não-bloqueantes (baseline: nenhuma — `research.md:4`).
- O procedimento de smoke não contém JWT expirado e pode ser emitido de forma determinística (baseline: token expirado em 2026-11-16 — `intake.md:8`; `research.md:7`).
- ADR-0001, README e snapshots do 027 não se contradizem (baseline: três focos de desvio — `research.md:7`).

## Cost of Inaction

Sem store versionado, a verificação de supply-chain desaparece em checkout limpo e a FASE 0 permanece literalmente incompleta. Sem sinais de advisory independentes, 83 alertas existentes não são triados com critério comum. Um smoke com credencial expirada torna impossível reproduzir a última gate do ADR, deixando a classificação "verificado" sem execução correspondente. As contradições documentais forçam cada leitor a escolher uma fonte, perpetuando o desvio.

## Open Questions

- [NEEDS CLARIFICATION: após que janela de observação o gate do `cargo-vet` pode tornar-se bloqueante? `research.md:3` sugere uma semana, sem critério de avaliação definido.]
- [NEEDS CLARIFICATION: devem ser criados required checks ou branch protection neste marco? `intake.md:8` registra a ausência, sem decisão de política.]
- [NEEDS CLARIFICATION: como priorizar os 83 alertas do Dependabot, dos quais 4 estão no `Cargo.lock` raiz? `intake.md:8` não fornece critério de triagem.]
- [NEEDS CLARIFICATION: a desativação do license-check na revisão de dependências é aceitável como política? `research.md:4` a propõe, sem justificativa registrada.]
- [NEEDS CLARIFICATION: qual porta isolada deve ser usada para nova tentativa de smoke? `research.md:7` registra 3000/5432 ocupadas, sem definir alternativa.]
