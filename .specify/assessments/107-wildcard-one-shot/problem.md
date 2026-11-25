# Problem Definition: Recuperação wildcard irrestrita e não one-shot

- **Slug**: 107-wildcard-one-shot
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md | shape.md

## Problem Statement

O opt-in temporário `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` foi introduzido nos ciclos 100/101 no startup e no login sem filtro de papel e sem desligamento após o primeiro login, deixando janela indefinida de escalada (intake.md:10-14; research.md:12,20; define.md:5). Como `Config` é imutável em `AppState.config` (`flora-api/src/state/app_state.rs:28`), um estado apenas em memória não sobrevive a restart (research.md:35-36). A seleção precisa ocorrer sem vazar enumeração de e-mail ou papel em log/resposta, e a autenticação após o consumo precisa falhar fechado (research.md:14,41).

Os papéis administrativos built-in registrados são `Admin` e `Owner`, com FK garantindo `roles.organization_id == memberships.organization_id` (research.md:37-38). O risco residual é que um marcador persistente não protege contra restauração de backup anterior ao consumo; por isso a expiração documentada em `docs/adr/0007-wildcard-recovery-expiration.md` (`2026-11-29T23:59:59Z`) condiciona o desligamento seguro à remoção das variáveis (research.md:29,46). Staging, produção, credenciais reais e restart multiréplica não foram observados: **[NV]** (decision.md:15; research.md:45). O veredito `go` e os FRs em `define.md` são registro histórico do que foi autorizado, não reavaliação deste enquadramento.

## Affected Users & Stakeholders

- **Users**: o SE demandante, via revisão interagentes (bug `d11-wildcard-unbounded`); administradores/owners das organizações elegíveis ao seletor e quem usa a conta selecionada para entrar; os demais usuários dessas organizações, expostos à janela de escalada (research.md:12-15; intake.md:18-19).
- **Stakeholders**: o guardião do gate ADR-0003 e de `check_gate_integrity.py`, que precisa de evidência determinística sem required check novo; quem opera as variáveis temporárias na Coolify e deve removê-las na expiração; os revisores interagentes que autorizaram a correção (research.md:15,20,23,29).

## Goals

- Reduzir a janela de escalada a um único consumo verificável, mantendo a condição de papel administrativo built-in (`Admin`/`Owner` em organização não-sistema) registrada em research.md:21,37.
- Evitar que a flag volte a valer após restart sem depender de estado apenas em memória (research.md:35-36).
- Manter falha fechada no login após o consumo (401 genérico, `invalid_credentials`) e ausência de enumeração em log/resposta (define.md:9; research.md:40-41).
- Preservar required checks e pisos do gate; no guarda atual `bootstrap` é 13 e `auth` é 25 (decision.md:11; research.md:39).
- Preservar a expiração documentada e a remoção das variáveis como condição de desligamento seguro, incluindo a limitação de restauração de backup (research.md:29,46).

## Non-Goals

- Não alterar gate/required check, formato de sessão, rota de login, credenciais reais, deploy ou outros ciclos do INBOX (define.md:13).
- Não afirmar comportamento em staging, produção ou múltiplas réplicas sem observação direta: **[NV]** (decision.md:15; research.md:45).
- Não reescrever registro append-only nem tratar a citação histórica `auth 23` como piso corrente (`auth 25`) (research.md:39,48).

## Success Metrics

- Sessões emitidas via opt-in wildcard sob tentativas concorrentes: alvo no máximo 1; baseline: unknown (critério US2 em research.md:14; sem medição remota).
- Logins com o seletor após o consumo: alvo 0, com 401 genérico; baseline: unknown (research.md:49 registra ausência de medição em produção).
- Contas selecionáveis por `*`: alvo apenas `Admin`/`Owner` built-in fora de organização de sistema; baseline: qualquer e-mail, sem filtro de papel (research.md:12,20).
- Ocorrências de enumeração de e-mail/papel em log ou resposta: alvo 0; baseline: unknown (research.md:41).
- Pisos do gate: alvo inalterados (`bootstrap 13`, `auth 25` no guarda atual); baseline: 13/25 (research.md:39) — o registro histórico `auth 23` permanece apenas como citação.

## Cost of Inaction

Sem restrição de papel e consumo persistente, qualquer bootstrap com `*` continua elegível após o primeiro login e após restart, mantendo a janela de escalada indefinida (research.md:12,20,35). A negação sem resposta genérica pode revelar se uma conta existe; a expiração do ADR sem remoção das variáveis deixa o marcador reativável por backup (research.md:29,46). Como nenhum required check observa esse comportamento (decision.md:11), o gate pode seguir PASS enquanto o risco operacional permanece, e o próximo item autorizado do INBOX fica bloqueado (intake.md:19).

## Open Questions

- [NEEDS CLARIFICATION: a expiração temporal do ADR deve ser 7 dias?] — o ADR 0007 fixou 2026-11-29, mas a dúvida do intake permanece registrada (intake.md:23; research.md:54).
- [NEEDS CLARIFICATION: o consumo deve ocorrer antes ou depois da emissão da sessão?] — o plano diz "antes, na mesma transação", sem marcação de respondida no intake (intake.md:24; research.md:55).
- [NEEDS CLARIFICATION: restart/replica é coberto pelo marcador persistente?] — depende de staging real e restart multiréplica **[NV]** (intake.md:25; research.md:56).
- [NEEDS CLARIFICATION: duas tentativas concorrentes produzem no máximo uma sessão também sob latência de replicação?] (research.md:57).
- [NEEDS CLARIFICATION: como conciliar a citação histórica `auth 23` com o piso atual `auth 25` sem reescrever o registro append-only?] (research.md:58).
