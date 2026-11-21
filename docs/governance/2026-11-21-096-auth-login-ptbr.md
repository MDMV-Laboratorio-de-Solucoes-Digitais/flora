# Nota de governança — 2026-11-21 (ciclo 096)

**Assunto:** usuário `luis.granja@mdmv.com.br` (criado via bootstrap) não consegue entrar em `https://flora.mdmv.qzz.io/` — diagnóstico OIDC (defaults `localhost:8080`, allowlist vazia, nenhum IdP disponível, nenhum fallback de senha), evidências locais e ao vivo, e decisão de fallback local Argon2id com recuperação por env no startup e tela de login pt-BR.

**Âncora:** branch `feature/096-auth-login-ptbr` @ `1910d1c7d`; `.specify/feature.json` → `specs/096-auth-login-ptbr`. Snapshot documental `2026-11-21T00:57:59Z`. Registro append-only; nenhuma linha histórica reescrita.

## (a) Sintoma verificado

- **Ao vivo:** o usuário `luis.granja@mdmv.com.br`, criado pelo fluxo público `POST /api/v1/bootstrap` (FR-012, sem verificação de posse do e-mail — ver `docs/governance/2026-11-20-085-fr-012-bootstrap-sem-verificacao-email.md`), não consegue autenticar no frontend de produção. O botão **Sign In** inicia o redirecionamento OIDC e não há caminho de senha funcional.
- **Ao vivo (medição registrada às 2026-11-21T00:15:48Z, ciclo 095):** `https://flora.mdmv.qzz.io/` → **200** com a tela em inglês "Welcome to Flora / Enter your credentials to access your workspace. / Sign In". A mesma interface persistentemente em inglês é a barreira de idioma relatada no diagnóstico do ciclo 096.
- **Local:** `apps/frontend/src/routes/(auth)/+page.svelte:145-159` confirma as strings "Welcome to Flora", "Enter your credentials to access your workspace.", "Email", "Password" e "Sign In"; `:127` "Admin Email" e `:140` "Back to Login". O campo de senha existe no markup, mas `handleLogin()` (`:38`) apenas monta `GET /api/v1/auth/login?redirect_uri=...` — ou seja, o input de senha não autentica nada.

## (b) Causa raiz: OIDC default, allowlist vazia, nenhum IdP, nenhum fallback de senha

| Evidência | Local / medição | Leitura |
|---|---|---|
| Issuer OIDC em default `localhost:8080` | `.env.example:59` `FLORA_OIDC__ISSUER_URL=http://localhost:8080`, `:60` `FLORA_OIDC__CLIENT_ID=flora@localhost`, `:61` `FLORA_OIDC__CLIENT_SECRET=your-zitadel-client-secret`; `flora-core/src/config/oidc.rs` `OidcConfig::default()` usa `ZITADEL_ISSUER_URL` ou `http://localhost:8080`, `flora@localhost` e `client_secret` vazio. Não há `.env` no checkout — apenas `.env.example`. | Sem sobrescrita, o runtime herda o placeholder: o endpoint de autorização aponta para `localhost:8080`, que não existe em produção. Nenhum IdP real está configurado. |
| Allowlist vazia | `flora-core/src/config/oidc.rs` `OidcConfig::default()` → `allowed_redirect_uris: Vec::new()`; `.env.example:64` `FLORA_OIDC__ALLOWED_REDIRECT_URIS` está comentado; `flora-api/src/routes/auth.rs:105` valida allowlist exata (callback configurado + URIs listadas). | Nenhuma URI adicional permitida; o fluxo depende somente do callback de `FLORA_APP__BASE_URL`, sem margem para redirecionamentos alternativos de recuperação. |
| Nenhum IdP disponível | Não há Zitadel nem qualquer provedor OIDC materializado no alvo; os registros de assessment (003–016) tratam OIDC/JWKS e2e como **NV**/fora de escopo. `OidcConfig::validate()` (`flora-core/src/config/oidc.rs`) checa apenas `issuer_url`, `client_id`, `scopes` e `S256` — não testa alcançabilidade do issuer. | A configuração passa na validação mesmo com issuer inalcançável; a falha só aparece no navegador, após o clique em **Sign In**. **[NV]** reconsulta ao vivo da disponibilidade real do IdP. |
| Nenhum fallback de senha | `flora-api/src/routes/auth.rs` → `create_auth_router()` registra apenas `/login`, `/callback`, `/logout`, `/refresh`, `/select-organization` (T019/T023/T116). `specs/001-flora-seed-v0-1/spec.md:271` **FR-022** exigia autenticação local email/senha como fallback quando Zitadel estivesse indisponível — requisito nunca implementado. Ciclo 041 removeu `argon2`/`bcrypt` como "crates dead" (`specs/041-rust-dead-crates/spec.md:13-16`; `.specify/validations/041-rust-dead-crates/validate.md:23`). | Não existe rota, hash nem crate para autenticar por senha. O usuário bootstrap, sem vínculo OIDC, fica sem caminho de entrada — exatamente o sintoma relatado. |
| Ausência de coluna de hash | Nenhuma migration em `flora-*/migrations` define `password_hash`, embora `specs/001-flora-seed-v0-1/data-model.md:19` previsse `password_hash: String (Hashed)`. | Hoje não há onde persistir/verificar um hash local; a decisão (c) precisa materializar esse campo. |
| Interface somente em inglês | `apps/frontend/src/routes/(auth)/+page.svelte:127,140,145-159` (strings listadas em (a)); `login-error.svelte.spec.ts` ancora os testes no rótulo em inglês `'Sign In'`. | Além de não autenticar, a tela não atende ao usuário pt-BR; a tradução também exige atualizar os testes de login. |

## (c) Decisão registrada — fallback local pt-BR

1. **Reintroduzir `argon2` (Argon2id)** em `flora-core`/`flora-api` para o fallback local de senha, revertendo, apenas no que toca a auth, a remoção do ciclo 041. `bcrypt` permanece removido. Isso materializa o **FR-022** de `specs/001` (fallback local quando Zitadel indisponível), com escopo explícito de instalação bootstrap/single-tenant, e dá ao usuário criado via bootstrap um caminho de entrada.
2. **Recuperação de credencial por variável de ambiente no startup:** ler e-mail e hash Argon2id da credencial de recuperação em chaves `FLORA_*` dedicadas no boot (senha hasheada em memória, nunca em texto puro versionado). Se a env estiver ausente, o fallback local permanece **desligado** e o fluxo OIDC segue como único caminho. Nenhuma credencial foi copiada para o repositório, logs, artefatos ou para esta nota.
3. **Tela de login pt-BR** em `apps/frontend/src/routes/(auth)/+page.svelte`: traduzir "Welcome to Flora", "Enter your credentials to access your workspace.", "Email", "Password", "Sign In", "Admin Email", "Back to Login" e as mensagens de erro de `login-error`; expor o campo de senha como controle funcional **somente quando o fallback local estiver habilitado**. Atualizar `login-error.svelte.spec.ts` para os novos rótulos.
4. **OIDC continua primário:** o fluxo authorization-code + PKCE/S256 existente não é substituído nem relaxado. O fallback local é complementar e só opera quando o IdP estiver indisponível/não configurado — nunca como atalho que enfraqueça a validação do provedor.
5. **Fora do gate de merge (ADR-0003):** nenhum required check novo. A única autoridade de gate é `python3 scripts/ci/check_gate_integrity.py`. Esta nota não toca validation/release, workflow, gate, ADR, constituição, PR, merge, tag ou deploy.

## (d) Estado **[NV]** e gates esperados

**[NV] até deploy final:** disponibilidade/configuração real do IdP em produção, alcançabilidade do issuer e allowlist efetiva, persistência do `password_hash`, comportamento do fallback local após deploy, tela pt-BR em produção, E2E autenticado com a credencial real de `luis.granja@mdmv.com.br`, e CI/proteção/Coolify/aprovações ao vivo. Nenhum deploy foi executado por esta nota.

Gates e verificações esperados, nesta ordem, antes de declarar concluído:

1. `python3 scripts/ci/check_gate_integrity.py` → **PASS** (registrado no diagnóstico do ciclo; reexecutar no checkout final) e `git diff --check` limpo. Não continuar se vermelho.
2. Testes Rust do crate de auth: hash/verify Argon2id, leitura da env no startup (inclusive o caso "env ausente → fallback desligado") e rota de login local; contrato de login e sessão verdes.
3. `npm --prefix apps/frontend run check` e vitest da tela de login (`+page.svelte` / `login-error.svelte.spec.ts`) com os rótulos pt-BR → 0 erros.
4. Build Docker de `flora-web`/API e medição local: `/`, `/config.js` e `/health` → **200**; login local com credencial de teste → sessão emitida e `GET /api/v1/tasks` com token → **200**; segundo fluxo OIDC inalterado quando configurado.
5. Após o deploy final, medir em navegador real: tela pt-BR, login com `luis.granja@mdmv.com.br`, navegação autenticada a `/tasks`, e `/health`/CORS da API. Registrar data/hora, commit e resultado em nova nota append-only.
6. **Rollback:** reapontar o app Coolify para a imagem/commit anterior e remover a env de credencial de recuperação, sem force-push; revalidar `/`, `/health` e o fluxo OIDC.

## Histórico e escopo

Documento novo e append-only. Nenhuma linha de `STATE.md`, `PROGRESSO.md`, ADRs, constituição, `specs/`, `.specify/validations/` ou `.specify/releases/` foi reescrita. Nenhuma credencial foi copiada ou exposta. Afirmações sobre estado remoto/Coolify/DNS/CI/IdP não remediadas nesta nota permanecem **[NV]** até reconsulta ao vivo e deploy final.
